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

use std::{io, process::ExitCode, result, string::FromUtf8Error};

/// The result type used in the whole project
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Encryption Error: {0}")]
    Encryption(String),
    #[error(
        "Decryption Error: The given key cannot decrypt the given data. Either the data has been \
         tampered with or the key is incorrect."
    )]
    Decryption,
    #[error("Wrong Master Password Error: Wrong decryption password")]
    WrongMasterPassword,
    #[error("Weak Password Error: {0}")]
    WeakPassword(String),
    #[error("Args Conflict Error: {0}")]
    ArgsConflict(String),
    #[error("Invalid Vault Index Error: {0}")]
    InvalidVaultIndex(String),
    #[error("{0}")]
    ArgParse(String),
    #[error(
        "Reserved Prefix Error: Sorry, but the following prefix is reserved and cannot be used in \
         custom fields {0}"
    )]
    ReservedPrefix(&'static str),
    #[error("{0}")]
    Other(String),

    #[error("CLI error: {0}")]
    Inquire(#[from] inquire::InquireError),
    #[error("Invalid Regex: {0}")]
    InvalidRegex(#[from] regex::Error),
    #[error("UTF8 Error: {0}")]
    Utf8(#[from] FromUtf8Error),
    #[error("Bincode Error: {0}")]
    Bincode(#[from] bincode::Error),
    #[error("Base64 Decode Error: {0}")]
    BaseDecodeError(#[from] base64::DecodeError),
    #[error("Json Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Project Folder Error: {0}")]
    ProjectDir(String),
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
}

impl Error {
    /// Return the exit code of the error
    pub const fn exit_code(&self) -> ExitCode {
        // TODO: Exit with more specific exit code
        ExitCode::FAILURE
    }
}
