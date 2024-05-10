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

use std::str::FromStr;

use clap::Args;

use crate::{
    vault::{Vault, Vaults},
    LprsCommand,
    LprsError,
    LprsResult,
};

#[derive(Debug, Clone, Eq, PartialEq)]
enum VaultGetField {
    Index,
    Name,
    Username,
    Password,
    Service,
    Note,
    Custom(String),
}

impl FromStr for VaultGetField {
    type Err = LprsError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input.to_lowercase().as_str() {
            "index" => Self::Index,
            "name" => Self::Name,
            "username" => Self::Username,
            "password" => Self::Password,
            "service" => Self::Service,
            "note" => Self::Note,
            _ => Self::Custom(input.to_owned()),
        })
    }
}

impl VaultGetField {
    /// Returns the field from the vault
    pub fn get_from_vault<'a>(&self, vault: &'a Vault) -> Option<&'a str> {
        match self {
            Self::Index => None,
            Self::Name => Some(&vault.name),
            Self::Username => vault.username.as_deref(),
            Self::Password => vault.password.as_deref(),
            Self::Service => vault.service.as_deref(),
            Self::Note => vault.note.as_deref(),
            Self::Custom(custom_field) => vault.custom_fields.get(custom_field).map(|x| x.as_str()),
        }
    }

    /// Returns the field as `&str`
    pub fn as_str(&self) -> &str {
        match self {
            Self::Index => "index",
            Self::Name => "name",
            Self::Username => "username",
            Self::Password => "password",
            Self::Service => "service",
            Self::Note => "note",
            Self::Custom(field) => field.as_str(),
        }
    }
}

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
/// Command to get a entire vault or single field from it
pub struct Get {
    /// Whether the index of the vault or its name
    #[arg(value_name = "INDEX-or-NAME")]
    location: String,
    /// A Specific field to get.
    ///
    /// Can be [name,username,password,service,note,"string"] where the string
    /// means a custom field
    #[arg(value_parser = VaultGetField::from_str)]
    field:    Option<VaultGetField>,
}

impl LprsCommand for Get {
    fn run(self, vault_manager: Vaults) -> LprsResult<()> {
        let parsed_index = self.location.trim().parse::<usize>();
        let Some((index, vault)) = (if let Ok(index) = parsed_index {
            vault_manager.vaults.get(index - 1).map(|v| (index, v))
        } else {
            vault_manager
                .vaults
                .iter()
                .enumerate()
                .find(|(_, v)| v.name == self.location)
        }) else {
            return Err(LprsError::Other(format!(
                "There is no vault with the given {} `{}`",
                if parsed_index.is_ok() {
                    "index"
                } else {
                    "name"
                },
                self.location.trim(),
            )));
        };

        if let Some(field) = self.field {
            if field == VaultGetField::Index {
                print!("{index}");
                return Ok(());
            }

            if let Some(value) = field.get_from_vault(vault) {
                print!("{value}")
            } else {
                return Err(LprsError::Other(format!(
                    "There is no value for `{}`",
                    field.as_str()
                )));
            }
        } else {
            println!("{vault}");
        }
        Ok(())
    }
}
