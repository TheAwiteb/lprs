// Lprs - A local CLI vaults manager. For human and machine use
// Copyright (C) 2024 Awiteb <a@4rs.nl>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://gnu.org/licenses/gpl-3.0.html>.

use std::{num::NonZeroUsize, str::FromStr};

use clap::Args;
use either::Either;

use crate::{
    clap_parsers::either_parser,
    utils,
    vault::{cipher, Vault, Vaults},
    LprsCommand,
    LprsError,
    LprsResult,
    RESERVED_FIELD_PREFIX,
};

#[derive(Debug, Clone, Eq, PartialEq)]
enum VaultGetField {
    Index,
    Name,
    Username,
    Password,
    Service,
    Note,
    TotpSecret,
    TotpCode,
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
            "totp_secret" => Self::TotpSecret,
            "totp_code" => Self::TotpCode,
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
            Self::TotpSecret => vault.totp_secret.as_deref(),
            Self::TotpCode => None,
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
            Self::TotpSecret => "totp_secret",
            Self::TotpCode => "totp_code",
            Self::Custom(field) => field.as_str(),
        }
    }
}

#[derive(Debug, Args)]
/// Command to get a entire vault or single field from it
pub struct Get {
    /// Whether the index of the vault or its name
    #[arg(name = "INDEX-or-NAME", value_parser = either_parser::<NonZeroUsize, String>)]
    location: Either<NonZeroUsize, String>,

    /// A Specific field to get.
    ///
    /// Can be [name, username, password, service, note, totp_secret, totp_code,
    /// "string"]
    ///
    /// where the string means a custom field
    #[arg(value_parser = VaultGetField::from_str)]
    field: Option<VaultGetField>,
}

impl LprsCommand for Get {
    fn run(self, mut vault_manager: Vaults) -> LprsResult<()> {
        let (index, vault) =
            utils::vault_by_index_or_name(&self.location, &mut vault_manager.vaults)?;

        if let Some(field) = self.field {
            if field == VaultGetField::Index {
                print!("{index}");
                return Ok(());
            }
            if field == VaultGetField::TotpCode {
                if let Some(ref totp_secret) = vault.totp_secret {
                    let totp_code = cipher::totp_now(totp_secret, &vault.totp_hash)?.0;
                    print!("{totp_code}");
                    return Ok(());
                } else {
                    return Err(LprsError::Other(
                        "There is no TOTP secret to get TOTP code".to_owned(),
                    ));
                }
            }

            if let Some(value) = field.get_from_vault(vault) {
                print!("{value}")
            } else {
                return Err(LprsError::Other(format!(
                    "There is no value for `{}` at \"{}\" vault",
                    field.as_str(),
                    vault.name
                )));
            }
        } else {
            if let Some(ref totp_secret) = vault.totp_secret {
                let code = cipher::totp_now(totp_secret, &vault.totp_hash)?.0;
                vault
                    .custom_fields
                    .insert(format!("{RESERVED_FIELD_PREFIX}TOTP Code"), code);
            }
            println!("{vault}");
        }
        Ok(())
    }
}
