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
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use rand::RngCore;
use std::io::{Error, ErrorKind, Result};

pub const ENCRYPTION_BLOCK_SIZE: usize = 8 * 1024 * 1024;
pub const NONCE_SIZE: usize = 12;
pub const TAG_SIZE: usize = 16;
const KEY_SIZE: usize = 32;

pub fn generate_key() -> String {
    let mut key = [0u8; KEY_SIZE];
    rand::rngs::OsRng.fill_bytes(&mut key);

    key.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn encrypted_size(plaintext_size: u64) -> u64 {
    let blocks = plaintext_size.div_ceil(ENCRYPTION_BLOCK_SIZE as u64);

    plaintext_size + blocks * (NONCE_SIZE + TAG_SIZE) as u64
}

pub struct Cipher {
    aes: Aes256Gcm,
}

impl Cipher {
    pub fn new(hex_key: &str) -> Result<Self> {
        if hex_key.len() != KEY_SIZE * 2 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid encryption key length",
            ));
        }

        let mut key = [0u8; KEY_SIZE];

        for (index, pair) in hex_key.as_bytes().chunks_exact(2).enumerate() {
            let pair = std::str::from_utf8(pair)
                .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?;
            key[index] = u8::from_str_radix(pair, 16)
                .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?;
        }

        let aes = Aes256Gcm::new_from_slice(&key)
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "invalid encryption key"))?;

        Ok(Self { aes })
    }

    pub fn encrypt_block(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let nonce = generate_nonce();
        let encrypted = self
            .aes
            .encrypt(Nonce::from_slice(&nonce), plaintext)
            .map_err(|_| Error::other("file encryption failed"))?;

        let mut block = Vec::with_capacity(NONCE_SIZE + encrypted.len());
        block.extend_from_slice(&nonce);
        block.extend_from_slice(&encrypted);

        Ok(block)
    }
}

fn generate_nonce() -> [u8; NONCE_SIZE] {
    let mut nonce = [0u8; NONCE_SIZE];
    rand::rngs::OsRng.fill_bytes(&mut nonce);

    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypted_size_counts_nonce_and_tag_for_each_block() {
        let block_size = ENCRYPTION_BLOCK_SIZE as u64;
        let overhead = (NONCE_SIZE + TAG_SIZE) as u64;

        assert_eq!(encrypted_size(0), 0);
        assert_eq!(encrypted_size(block_size), block_size + overhead);
        assert_eq!(
            encrypted_size(block_size + 1),
            block_size + 1 + 2 * overhead
        );
    }
}
