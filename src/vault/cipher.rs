// Lprs - A local CLI vault manager
// Copyright (C) 2024  Awiteb <a@4rs.nl>
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

use std::time::{SystemTime, UNIX_EPOCH};

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{LprsError, LprsResult};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

/// Encrypt the given data by the given key using AES-256 CBC
///
/// Note: The IV will be add it to the end of the ciphertext (Last 16 bytes)
pub(crate) fn encrypt(master_password: &[u8; 32], data: &[u8]) -> Vec<u8> {
    let iv: [u8; 16] = StdRng::seed_from_u64(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs(),
    )
    .gen();

    let mut ciphertext =
        Aes256CbcEnc::new(master_password.into(), &iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
    ciphertext.extend(&iv);
    ciphertext
}

/// Decrypt the given data by the given key, the data should
/// be encrypted by AES-256 CBC. The IV will be extraxted
/// from the last 16 bytes.
pub(crate) fn decrypt(master_password: &[u8; 32], data: &[u8]) -> LprsResult<Vec<u8>> {
    let (ciphertext, iv) = data.split_at(
        data.len()
            .checked_sub(16)
            .ok_or_else(|| LprsError::Decryption)?,
    );

    Aes256CbcDec::new(master_password.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
        .map_err(|_| LprsError::Decryption)
}
