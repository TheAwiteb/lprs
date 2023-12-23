use std::{fs, path::PathBuf};

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{PassrsError, PassrsResult};

pub mod cipher;
mod validator;

pub use validator::*;

/// The passwords manager
#[derive(Deserialize, Serialize)]
pub struct Passwords {
    /// Hash of the master password
    #[serde(skip)]
    pub master_password: Vec<u8>,
    /// The json passwords file
    #[serde(skip)]
    pub passwords_file: PathBuf,
    /// The passwords
    pub passwords: Vec<Password>,
}

/// The password struct
#[derive(Clone, Debug, Deserialize, Serialize, Parser)]
pub struct Password {
    /// The name of the password
    #[arg(short, long)]
    pub name: String,
    /// The username
    #[arg(short, long)]
    pub username: String,
    /// The password
    #[arg(short, long)]
    pub password: String,
    /// The service name. e.g the website url
    #[arg(short, long)]
    pub service: Option<String>,
    /// The note of the password
    #[arg(short = 'o', long)]
    pub note: Option<String>,
}

impl Password {
    /// Encrypt the password data
    pub fn encrypt(self, master_password: &[u8]) -> PassrsResult<Self> {
        Ok(Self {
            name: cipher::encrypt(master_password, &self.name)?,
            username: cipher::encrypt(master_password, &self.username)?,
            password: cipher::encrypt(master_password, &self.password)?,
            service: self
                .service
                .map(|url| cipher::encrypt(master_password, &url))
                .transpose()?,
            note: self
                .note
                .map(|note| cipher::encrypt(master_password, &note))
                .transpose()?,
        })
    }

    /// Decrypt the password data
    pub fn decrypt(self, master_password: &[u8]) -> PassrsResult<Self> {
        Ok(Self {
            name: cipher::decrypt(master_password, &self.name)?,
            username: cipher::decrypt(master_password, &self.username)?,
            password: cipher::decrypt(master_password, &self.password)?,
            service: self
                .service
                .map(|url| cipher::decrypt(master_password, &url))
                .transpose()?,
            note: self
                .note
                .map(|note| cipher::decrypt(master_password, &note))
                .transpose()?,
        })
    }
}

impl Passwords {
    /// Create new Passwords instnce
    pub fn new(
        master_password: Vec<u8>,
        passwords_file: PathBuf,
        passwords: Vec<Password>,
    ) -> Self {
        Self {
            master_password,
            passwords_file,
            passwords,
        }
    }

    /// Encrypt the passwords
    fn encrypt(self) -> PassrsResult<Self> {
        Ok(Self {
            passwords: self
                .passwords
                .into_iter()
                .map(|p| p.encrypt(&self.master_password))
                .collect::<PassrsResult<Vec<Password>>>()?,
            ..self
        })
    }

    /// Reload the passwords from the file
    pub fn try_reload(passwords_file: PathBuf, master_password: Vec<u8>) -> PassrsResult<Self> {
        let passwords =
            serde_json::from_str::<Vec<Password>>(&fs::read_to_string(&passwords_file)?)?
                .into_iter()
                .map(|p| p.decrypt(master_password.as_slice()))
                .collect::<PassrsResult<Vec<Password>>>()?;

        Ok(Self::new(master_password, passwords_file, passwords))
    }

    /// Export the passwords to the file
    pub fn try_export(self) -> PassrsResult<()> {
        let path = self.passwords_file.to_path_buf();
        fs::write(path, serde_json::to_string(&self.encrypt()?.passwords)?).map_err(PassrsError::Io)
    }

    /// Add new password
    pub fn add_password(&mut self, password: Password) {
        self.passwords.push(password)
    }
}
