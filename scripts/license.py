#!/usr/bin/env python3

import argparse
import glob
import re
import pathlib

license_header_rust = """
// Copyright 2025 Miku Push! Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
"""

license_header_typescript = """
/**
 * Copyright 2025 Miku Push! Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
"""

print("adding license header to source code")

rust_source_files = glob.glob('src-tauri/**/*.rs', recursive=True) \
                    + glob.glob('lib/**/*.rs', recursive=True)

typescript_source_files = glob.glob('src/**/*.ts', recursive=True) \
                        + glob.glob('src/**/*.tsx', recursive=True) \
                        + glob.glob('src/**/*.css', recursive=True)

rust_regex = re.compile(r"^// Copyright 2025 Miku Push! Team(\n.*)+License.")
typescript_regex = re.compile(r"^/\*\*\n \* Copyright 2025 Miku Push! Team")

exclude_files = [
    "src/vite-env.d.ts",
    "src-tauri/build.rs",
    "src-tauri/src/main.rs",
    "src/components/ui/*.tsx",
    "lib/database/src/schema.rs"
]

def is_source_file_excluded(source_file_path: str):
    for exclude_file_pattern in exclude_files:
        path = pathlib.Path(source_file_path)

        if path.match(exclude_file_pattern):
            return True

    return False


def get_source_code(source_file_path: str):
    with open(source_file_path, "r", encoding="utf-8") as file:
        if is_source_file_excluded(source_file_path):
            print(f"excluding file {source_file_path}")
            return None

        return file.read().strip()


def add_license(source_files: list[str], license_header_content: str, regex: re.Pattern[str]):
    for source_file in source_files:
        source_code = get_source_code(source_file)
        if source_code is None:
            continue

        if regex.match(source_code):
            print(f"file {source_file} already has license header")
            continue

        with open(source_file, "w", encoding="utf-8") as file:
            file.write(license_header_content.strip() + "\n\n" + source_code)


def remove_license(source_files: list[str], license_header_content: str, regex: re.Pattern[str]):
    for source_file in source_files:
        source_code = get_source_code(source_file)
        if source_code is None:
            continue

        with open(source_file, "w", encoding="utf-8") as file:
            file.write(source_code.replace(license_header_content.strip(), "").strip())


parser = argparse.ArgumentParser(description='Add or remove license headers from source code files')
parser.add_argument(
    '--remove',
    action='store_true',
    help='Remove license headers'
)
args = parser.parse_args()

if args.remove:
    print("removing license header from source code")
    remove_license(rust_source_files, license_header_rust, rust_regex)
    remove_license(typescript_source_files, license_header_typescript, typescript_regex)
    print("removed license header from source code")
else:
    print("adding license header to source code")
    add_license(rust_source_files, license_header_rust, rust_regex)
    add_license(typescript_source_files, license_header_typescript, typescript_regex)
    print("added license header to all source code")
