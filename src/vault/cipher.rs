// Lprs - A local CLI vaults manager. For human and machine use
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
use base32::Alphabet as Base32Alphabet;
use clap::ValueEnum;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::{LprsError, LprsResult};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

#[derive(Default, Clone, Debug, ValueEnum, Eq, PartialEq, Deserialize, Serialize)]
/// The TOTP hash functions
pub enum TotpHash {
    /// Sha1 hash function
    #[default]
    Sha1,
    /// Sha256 hash function
    Sha256,
    /// Sha512 hash function
    Sha512,
}


/// Create the TOTP code of the current time
///
/// ## Errors
/// - If the given `secret_base32` are vaild base32
pub fn totp_now(secret_base32: &str, hash_function: &TotpHash) -> LprsResult<(String, u8)> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs();
    let remaining = 30 - (now % 30) as u8;
    let secret = base32::decode(Base32Alphabet::RFC4648 { padding: true }, secret_base32)
        .ok_or_else(|| LprsError::Base32("Can't decode the TOTP secret".to_owned()))?;
    Ok(match hash_function {
        TotpHash::Sha1 => {
            (
                totp_lite::totp_custom::<totp_lite::Sha1>(30, 6, &secret, now),
                remaining,
            )
        }
        TotpHash::Sha256 => {
            (
                totp_lite::totp_custom::<totp_lite::Sha256>(30, 6, &secret, now),
                remaining,
            )
        }
        TotpHash::Sha512 => {
            (
                totp_lite::totp_custom::<totp_lite::Sha512>(30, 6, &secret, now),
                remaining,
            )
        }
    })
}

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
