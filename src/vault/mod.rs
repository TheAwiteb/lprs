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

use std::{fs, path::PathBuf};

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

use crate::{LprsError, LprsResult};

pub mod cipher;

mod bitwarden;
mod validator;

pub use bitwarden::*;
pub use validator::*;

#[derive(Clone, Debug, ValueEnum)]
pub enum Format {
    Lprs,
    BitWarden,
}

/// The vault struct
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, Parser)]
pub struct Vault {
    /// The name of the vault
    #[arg(short, long)]
    pub name: String,
    /// The username
    #[arg(short, long)]
    pub username: Option<String>,
    /// The password
    #[arg(skip)]
    pub password: Option<String>,
    /// The service name. e.g the website url
    #[arg(short, long)]
    pub service: Option<String>,
    /// Add a note to the vault
    #[arg(short = 'o', long)]
    pub note: Option<String>,
}

/// The vaults manager
#[derive(Default)]
pub struct Vaults {
    /// Hash of the master password
    pub master_password: Vec<u8>,
    /// The json vaults file
    pub vaults_file: PathBuf,
    /// The vaults
    pub vaults: Vec<Vault>,
}

impl Vault {
    /// Create new [`Vault`] instance
    pub fn new(
        name: impl Into<String>,
        username: Option<impl Into<String>>,
        password: Option<impl Into<String>>,
        service: Option<impl Into<String>>,
        note: Option<impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            username: username.map(Into::into),
            password: password.map(Into::into),
            service: service.map(Into::into),
            note: note.map(Into::into),
        }
    }

    /// Decrypt the vault
    pub fn decrypt(&self, master_password: &[u8]) -> LprsResult<Vault> {
        Ok(Vault::new(
            cipher::decrypt(master_password, &self.name)?,
            cipher::decrypt_some(master_password, self.username.as_ref())?,
            cipher::decrypt_some(master_password, self.password.as_ref())?,
            cipher::decrypt_some(master_password, self.service.as_ref())?,
            cipher::decrypt_some(master_password, self.note.as_ref())?,
        ))
    }

    /// Encrypt the vault
    pub fn encrypt(&self, master_password: &[u8]) -> LprsResult<Vault> {
        Ok(Vault::new(
            cipher::encrypt(master_password, &self.name)?,
            cipher::encrypt_some(master_password, self.username.as_ref())?,
            cipher::encrypt_some(master_password, self.password.as_ref())?,
            cipher::encrypt_some(master_password, self.service.as_ref())?,
            cipher::encrypt_some(master_password, self.note.as_ref())?,
        ))
    }

    /// Return the name of the vault with the service if there
    pub fn list_name(&self) -> String {
        use std::fmt::Write;
        let mut list_name = self.name.clone();
        if let Some(ref username) = self.username {
            write!(&mut list_name, " <{username}>").expect("String never fail");
        }
        if let Some(ref service) = self.service {
            write!(&mut list_name, " ({service})").expect("String never fail");
        }
        if self.username.is_none() && self.password.is_none() && self.note.is_some() {
            write!(&mut list_name, " *Note").expect("String never fail");
        }
        list_name
    }
}

impl Vaults {
    /// Create new [`Vaults`] instnce
    pub fn new(master_password: Vec<u8>, vaults_file: PathBuf, vaults: Vec<Vault>) -> Self {
        Self {
            master_password,
            vaults_file,
            vaults,
        }
    }

    /// Encrypt the vaults
    pub fn encrypt_vaults(&self) -> LprsResult<Vec<Vault>> {
        self.vaults
            .iter()
            .map(|p| p.encrypt(&self.master_password))
            .collect()
    }

    /// Reload the vaults from the file then decrypt it
    pub fn try_reload(vaults_file: PathBuf, master_password: Vec<u8>) -> LprsResult<Self> {
        let vaults = serde_json::from_str::<Vec<Vault>>(&fs::read_to_string(&vaults_file)?)?
            .into_iter()
            .map(|p| p.decrypt(master_password.as_slice()))
            .collect::<LprsResult<Vec<Vault>>>()?;

        Ok(Self::new(master_password, vaults_file, vaults))
    }

    /// Encrypt the vaults then export it to the file
    pub fn try_export(self) -> LprsResult<()> {
        log::debug!(
            "Trying to export the vaults to the file: {}",
            self.vaults_file.display()
        );
        fs::write(
            &self.vaults_file,
            serde_json::to_string(&self.encrypt_vaults()?)?,
        )
        .map_err(LprsError::Io)
    }

    /// Add new vault
    pub fn add_vault(&mut self, vault: Vault) {
        self.vaults.push(vault)
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_possible_value()
                .expect("There is no skiped values")
                .get_name()
        )
    }
}

impl std::fmt::Display for Vault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}", self.name)?;
        if let Some(ref username) = self.username {
            write!(f, "\nUsername: {username}")?;
        }
        if let Some(ref password) = self.password {
            write!(f, "\nPassword: {password}")?;
        }
        if let Some(ref service) = self.service {
            write!(f, "\nService: {service}")?;
        }
        if let Some(ref note) = self.note {
            write!(f, "\nNote:\n{note}")?;
        }

        Ok(())
    }
}
