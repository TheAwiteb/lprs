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

use std::{fs, marker::PhantomData, path::PathBuf};

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

use crate::{LprsError, LprsResult};
use vault_state::*;

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

/// The states of the vaults
pub mod vault_state {
    /// Means the vault is encrypted
    #[derive(Clone, Debug, Default)]
    pub struct Encrypted;
    /// Means the vault is not encrypted
    #[derive(Clone, Debug, Default)]
    pub struct Plain;
}

/// The vault struct
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, Parser)]
pub struct Vault<T>
where
    T: std::fmt::Debug + Clone,
{
    /// The name of the vault
    #[arg(short, long)]
    pub name: String,
    /// The username
    #[arg(short, long)]
    pub username: Option<String>,
    /// The password
    #[arg(short, long)]
    pub password: Option<String>,
    /// The service name. e.g the website url
    #[arg(short, long)]
    pub service: Option<String>,
    /// Add a note to the vault
    #[arg(short = 'o', long)]
    pub note: Option<String>,

    /// State phantom
    #[serde(skip)]
    #[arg(skip)]
    phantom: PhantomData<T>,
}

/// The vaults manager
#[derive(Default)]
pub struct Vaults<T>
where
    T: std::fmt::Debug + Clone,
{
    /// Hash of the master password
    pub master_password: Vec<u8>,
    /// The json vaults file
    pub vaults_file: PathBuf,
    /// The vaults
    pub vaults: Vec<Vault<T>>,
}

impl<T> Vault<T>
where
    T: std::fmt::Debug + Clone,
{
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
            phantom: std::marker::PhantomData,
        }
    }
}

impl Vault<Encrypted> {
    /// Decrypt the vault
    pub fn decrypt(&self, master_password: &[u8]) -> LprsResult<Vault<Plain>> {
        Ok(Vault::<Plain>::new(
            cipher::decrypt(master_password, &self.name)?,
            cipher::decrypt_some(master_password, self.username.as_ref())?,
            cipher::decrypt_some(master_password, self.password.as_ref())?,
            cipher::decrypt_some(master_password, self.service.as_ref())?,
            cipher::decrypt_some(master_password, self.note.as_ref())?,
        ))
    }
}

impl Vault<Plain> {
    /// Encrypt the vault
    pub fn encrypt(&self, master_password: &[u8]) -> LprsResult<Vault<Encrypted>> {
        Ok(Vault::<Encrypted>::new(
            cipher::encrypt(master_password, &self.name)?,
            cipher::encrypt_some(master_password, self.username.as_ref())?,
            cipher::encrypt_some(master_password, self.password.as_ref())?,
            cipher::encrypt_some(master_password, self.service.as_ref())?,
            cipher::encrypt_some(master_password, self.note.as_ref())?,
        ))
    }
}

impl<T> Vaults<T>
where
    T: std::fmt::Debug + Clone,
{
    /// Create new [`Vaults`] instnce
    pub fn new(master_password: Vec<u8>, vaults_file: PathBuf, vaults: Vec<Vault<T>>) -> Self {
        Self {
            master_password,
            vaults_file,
            vaults,
        }
    }
}

impl Vaults<Plain> {
    /// Encrypt the vaults
    pub fn encrypt_vaults(&self) -> LprsResult<Vec<Vault<Encrypted>>> {
        self.vaults
            .iter()
            .map(|p| p.encrypt(&self.master_password))
            .collect()
    }

    /// Reload the vaults from the file then decrypt it
    pub fn try_reload(vaults_file: PathBuf, master_password: Vec<u8>) -> LprsResult<Self> {
        let vaults =
            serde_json::from_str::<Vec<Vault<Encrypted>>>(&fs::read_to_string(&vaults_file)?)?
                .into_iter()
                .map(|p| p.decrypt(master_password.as_slice()))
                .collect::<LprsResult<Vec<Vault<Plain>>>>()?;

        Ok(Self::new(master_password, vaults_file, vaults))
    }

    /// Encrypt the vaults then export it to the file
    pub fn try_export(self) -> LprsResult<()> {
        fs::write(
            &self.vaults_file,
            serde_json::to_string(&self.encrypt_vaults()?)?,
        )
        .map_err(LprsError::Io)
    }

    /// Add new vault
    pub fn add_vault(&mut self, vault: Vault<Plain>) {
        self.vaults.push(vault)
    }
}

impl ToString for Format {
    fn to_string(&self) -> String {
        self.to_possible_value()
            .expect("There is no skiped values")
            .get_name()
            .to_owned()
    }
}
