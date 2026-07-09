// Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
// Copyright (C) 2025  Miku Push! Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use log::debug;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use zip::CompressionMethod;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

#[derive(Debug, Clone)]
pub struct ZipDirectoryResult {
    pub path: PathBuf,
    pub size: u64,
}

pub fn zip_directory(
    directory_path: &Path,
    output_directory: &Path,
) -> Result<ZipDirectoryResult, String> {
    if !directory_path.is_dir() {
        return Err(format!(
            "path is not a directory: {}",
            directory_path.display()
        ));
    }

    fs::create_dir_all(output_directory)
        .map_err(|err| format!("failed to create zip output directory: {}", err))?;

    let zip_path = output_directory.join(format!("{}.zip", Uuid::new_v4()));
    debug!(
        "compressing directory {} to {}",
        directory_path.display(),
        zip_path.display()
    );

    let file = File::create(&zip_path)
        .map_err(|err| format!("failed to create zip file {}: {}", zip_path.display(), err))?;
    let mut writer = ZipWriter::new(file);
    let force_zip64 = directory_requires_zip64(directory_path)?;
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

    let base_path = directory_path
        .parent()
        .ok_or_else(|| "failed to resolve directory parent".to_string())?;
    add_path_to_zip(&mut writer, directory_path, base_path, options, force_zip64)?;
    writer
        .finish()
        .map_err(|err| format!("failed to finish zip file: {}", err))?;

    let size = fs::metadata(&zip_path)
        .map_err(|err| format!("failed to read zip metadata: {}", err))?
        .len();

    Ok(ZipDirectoryResult {
        path: zip_path,
        size,
    })
}

fn add_path_to_zip(
    writer: &mut ZipWriter<File>,
    path: &Path,
    base_path: &Path,
    options: SimpleFileOptions,
    force_zip64: bool,
) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|err| format!("failed to read metadata for {}: {}", path.display(), err))?;

    if metadata.file_type().is_symlink() {
        debug!(
            "skipping symlink while zipping directory: {}",
            path.display()
        );
        return Ok(());
    }

    let name = path
        .strip_prefix(base_path)
        .map_err(|err| format!("failed to create zip entry path: {}", err))?;
    let name = zip_entry_name(name);

    if metadata.is_dir() {
        if !name.is_empty() {
            writer
                .add_directory(name, options.unix_permissions(0o755).large_file(force_zip64))
                .map_err(|err| format!("failed to add zip directory entry: {}", err))?;
        }

        let entries = fs::read_dir(path)
            .map_err(|err| format!("failed to read directory {}: {}", path.display(), err))?;
        for entry in entries {
            let entry = entry.map_err(|err| format!("failed to read directory entry: {}", err))?;
            add_path_to_zip(writer, &entry.path(), base_path, options, force_zip64)?;
        }

        return Ok(());
    }

    if metadata.is_file() {
        let file_options = options
            .unix_permissions(0o644)
            .large_file(force_zip64 || metadata.len() >= zip::ZIP64_BYTES_THR);
        writer
            .start_file(name, file_options)
            .map_err(|err| format!("failed to add zip file entry: {}", err))?;
        let mut file = File::open(path)
            .map_err(|err| format!("failed to open file {}: {}", path.display(), err))?;
        io::copy(&mut file, writer)
            .map_err(|err| format!("failed to write file to zip: {}", err))?;
    }

    Ok(())
}

fn directory_requires_zip64(directory_path: &Path) -> Result<bool, String> {
    let mut size = 0u64;
    let mut entries = 0u64;
    collect_directory_size(directory_path, &mut size, &mut entries)?;

    let estimated_zip_overhead = entries.saturating_mul(256);
    Ok(size.saturating_add(estimated_zip_overhead) >= zip::ZIP64_BYTES_THR)
}

fn collect_directory_size(path: &Path, size: &mut u64, entries: &mut u64) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|err| format!("failed to read metadata for {}: {}", path.display(), err))?;

    if metadata.file_type().is_symlink() {
        return Ok(());
    }

    *entries = entries.saturating_add(1);

    if metadata.is_file() {
        *size = size.saturating_add(metadata.len());
        return Ok(());
    }

    if metadata.is_dir() {
        let children = fs::read_dir(path)
            .map_err(|err| format!("failed to read directory {}: {}", path.display(), err))?;
        for child in children {
            let child = child.map_err(|err| format!("failed to read directory entry: {}", err))?;
            collect_directory_size(&child.path(), size, entries)?;
        }
    }

    Ok(())
}

fn zip_entry_name(path: &Path) -> String {
    path.components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<&str>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn zip_directory_should_include_directory_root_and_files() {
        let base_path = std::env::temp_dir().join(format!("mikupush-test-{}", Uuid::new_v4()));
        let directory_path = base_path.join("folder");
        let nested_path = directory_path.join("nested");
        fs::create_dir_all(&nested_path).unwrap();
        fs::write(directory_path.join("file.txt"), "hello").unwrap();
        fs::write(nested_path.join("inner.txt"), "world").unwrap();

        let output_directory = base_path.join("output");
        let result = zip_directory(&directory_path, &output_directory).unwrap();

        let file = File::open(result.path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        assert_eq!(
            0o755,
            archive.by_name("folder/").unwrap().unix_mode().unwrap() & 0o777
        );
        assert_eq!(
            0o755,
            archive
                .by_name("folder/nested/")
                .unwrap()
                .unix_mode()
                .unwrap()
                & 0o777
        );
        assert_eq!(
            0o644,
            archive
                .by_name("folder/file.txt")
                .unwrap()
                .unix_mode()
                .unwrap()
                & 0o777
        );

        let mut contents = String::new();
        archive
            .by_name("folder/nested/inner.txt")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert_eq!("world", contents);

        let _ = fs::remove_dir_all(base_path);
    }

    #[test]
    fn directory_requires_zip64_should_detect_large_directories() {
        let base_path = std::env::temp_dir().join(format!("mikupush-test-{}", Uuid::new_v4()));
        let directory_path = base_path.join("folder");
        fs::create_dir_all(&directory_path).unwrap();
        let large_file = File::create(directory_path.join("large.bin")).unwrap();
        large_file.set_len(zip::ZIP64_BYTES_THR + 1).unwrap();

        assert!(directory_requires_zip64(&directory_path).unwrap());

        let _ = fs::remove_dir_all(base_path);
    }
}
