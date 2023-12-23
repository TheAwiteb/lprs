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
        ExitCode::FAILURE
    }
}
