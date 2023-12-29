// Lprs - A local CLI password manager
// Copyright (C) 2024  Awiteb
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
// along with this program.  If not, see <https://www.gnu.org/licenses/gpl-3.0.html>.

use base64::Engine;
use soft_aes::aes::{aes_dec_ecb, aes_enc_ecb};

use crate::{LprsError, LprsResult};

/// Encrypt the string with AEC ECB
pub fn encrypt(master_password: &[u8], data: &str) -> LprsResult<String> {
    let padding = Some("PKCS7");

    aes_enc_ecb(data.as_bytes(), master_password, padding)
        .map(|d| crate::STANDARDBASE.encode(d))
        .map_err(|err| LprsError::Encryption(err.to_string()))
}

/// Decrypt the string with AEC ECB
pub fn decrypt(master_password: &[u8], data: &str) -> LprsResult<String> {
    let padding = Some("PKCS7");

    aes_dec_ecb(
        crate::STANDARDBASE.decode(data)?.as_slice(),
        master_password,
        padding,
    )
    .map_err(|err| {
        if err.to_string().contains("Invalid padding") {
            LprsError::WrongMasterPassword
        } else {
            LprsError::Decryption(err.to_string())
        }
    })
    .map(|d| String::from_utf8(d).map_err(LprsError::Utf8))?
}
