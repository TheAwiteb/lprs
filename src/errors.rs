// Lprs - A local CLI password manager
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

use std::{process::ExitCode, string::FromUtf8Error};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid Json Path Error: {0}")]
    InvalidJsonPath(String),
    #[error("Encryption Error: {0}")]
    Encryption(String),
    #[error("Decryption Error: {0}")]
    Decryption(String),
    #[error(
        "Wrong Master Password Error: Wrong password or you may have played with the password file"
    )]
    WrongMasterPassword,
    #[error("Weak Password Error: {0}")]
    WeakPassword(String),
    #[error("Args Conflict Error: {0}")]
    ArgsConflict(String),
    #[error("Invalid Vault Index Error: {0}")]
    InvalidVaultIndex(String),
    #[error("{0}")]
    Other(String),

    #[error("Invalid Regex: {0}")]
    InvalidRegex(#[from] regex::Error),
    #[error("UTF8 Error: {0}")]
    Utf8(#[from] FromUtf8Error),
    #[error("Base64 Decode Error: {0}")]
    BaseDecodeError(#[from] base64::DecodeError),
    #[error("Json Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Project Folder Error: {0}")]
    ProjectDir(String),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    /// Return the exit code of the error
    pub fn exit_code(&self) -> ExitCode {
        // TODO: Exit with more specific exit code
        ExitCode::FAILURE
    }
}
