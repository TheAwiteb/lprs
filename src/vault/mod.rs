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

use base64::Engine;
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

use crate::{LprsError, LprsResult};

pub mod cipher;

mod bitwarden;

pub use bitwarden::*;

#[derive(Clone, Debug, ValueEnum)]
pub enum Format {
    Lprs,
    BitWarden,
}

/// The vault struct
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
    pub master_password: [u8; 32],
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
    pub fn new(master_password: [u8; 32], vaults_file: PathBuf, vaults: Vec<Vault>) -> Self {
        Self {
            master_password,
            vaults_file,
            vaults,
        }
    }

    /// Add new vault
    pub fn add_vault(&mut self, vault: Vault) {
        self.vaults.push(vault)
    }

    /// Encrypt the vaults then returns it as json.
    ///
    /// This function used to backup the vaults.
    ///
    /// Note: The returned string is `Vec<Vault>`
    pub fn json_export(&self) -> LprsResult<String> {
        let encrypt = |val: &str| {
            LprsResult::Ok(
                crate::BASE64.encode(cipher::encrypt(&self.master_password, val.as_ref())),
            )
        };

        serde_json::to_string(
            &self
                .vaults
                .iter()
                .map(|v| {
                    LprsResult::Ok(Vault::new(
                        encrypt(&v.name)?,
                        v.username.as_ref().and_then(|u| encrypt(u).ok()),
                        v.password.as_ref().and_then(|p| encrypt(p).ok()),
                        v.service.as_ref().and_then(|s| encrypt(s).ok()),
                        v.note.as_ref().and_then(|n| encrypt(n).ok()),
                    ))
                })
                .collect::<LprsResult<Vec<_>>>()?,
        )
        .map_err(Into::into)
    }

    /// Reload the vaults from json data.
    ///
    /// This function used to import backup vaults.
    pub fn json_reload(master_password: &[u8; 32], json_data: &[u8]) -> LprsResult<Vec<Vault>> {
        let decrypt = |val: &str| {
            String::from_utf8(cipher::decrypt(
                master_password,
                &crate::BASE64.decode(val)?,
            )?)
            .map_err(|err| LprsError::Other(err.to_string()))
        };

        serde_json::from_slice::<Vec<Vault>>(json_data)?
            .into_iter()
            .map(|v| {
                LprsResult::Ok(Vault::new(
                    decrypt(&v.name)?,
                    v.username.as_ref().and_then(|u| decrypt(u).ok()),
                    v.password.as_ref().and_then(|p| decrypt(p).ok()),
                    v.service.as_ref().and_then(|s| decrypt(s).ok()),
                    v.note.as_ref().and_then(|n| decrypt(n).ok()),
                ))
            })
            .collect()
    }

    /// Encrypt the vaults then export it to the file
    pub fn try_export(self) -> LprsResult<()> {
        log::debug!(
            "Trying to export the vaults to the file: {}",
            self.vaults_file.display()
        );
        fs::write(
            &self.vaults_file,
            cipher::encrypt(&self.master_password, &bincode::serialize(&self.vaults)?),
        )
        .map_err(LprsError::Io)
    }

    /// Reload the vaults from the file then decrypt it
    pub fn try_reload(vaults_file: PathBuf, master_password: [u8; 32]) -> LprsResult<Self> {
        let vaults_data = fs::read(&vaults_file)?;

        let vaults: Vec<Vault> = if vaults_data.is_empty() {
            vec![]
        } else {
            bincode::deserialize(&cipher::decrypt(&master_password, &vaults_data)?)?
        };

        Ok(Self::new(master_password, vaults_file, vaults))
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
