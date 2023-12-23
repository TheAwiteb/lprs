use base64::Engine;
use soft_aes::aes::{aes_dec_ecb, aes_enc_ecb};

use crate::{PassrsError, PassrsResult};

/// Encrypt the string with AEC ECB
pub fn encrypt(master_password: &[u8], data: &str) -> PassrsResult<String> {
    let padding = Some("PKCS7");

    aes_enc_ecb(data.as_bytes(), master_password, padding)
        .map(|d| crate::STANDARDBASE.encode(d))
        .map_err(|err| PassrsError::Encryption(err.to_string()))
}

/// Decrypt the string with AEC ECB
pub fn decrypt(master_password: &[u8], data: &str) -> PassrsResult<String> {
    let padding = Some("PKCS7");

    aes_dec_ecb(
        crate::STANDARDBASE.decode(data)?.as_slice(),
        master_password,
        padding,
    )
    .map_err(|err| {
        if err.to_string().contains("Invalid padding") {
            PassrsError::WrongMasterPassword
        } else {
            PassrsError::Decryption(err.to_string())
        }
    })
    .map(|d| String::from_utf8(d).map_err(PassrsError::Utf8))?
}
