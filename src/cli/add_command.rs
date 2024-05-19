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

use clap::Args;

use crate::{
    clap_parsers,
    utils,
    vault::{cipher, Vault, Vaults},
    LprsCommand,
    LprsError,
    LprsResult,
};

#[derive(Debug, Args)]
/// Add command, used to add new vault to the vaults file
pub struct Add {
    #[command(flatten)]
    vault_info:    Vault,
    /// The password, if there is no value you will prompt it
    #[arg(short, long)]
    #[allow(clippy::option_option)]
    password:      Option<Option<String>>,
    /// The TOTP secret, if there is no value you will prompt it
    #[arg(short, long)]
    #[allow(clippy::option_option)]
    totp_secret:   Option<Option<String>>,
    /// Add a custom field to the vault
    #[arg(name = "KEY=VALUE", short = 'c', long = "custom")]
    #[arg(value_parser = clap_parsers::kv_parser)]
    custom_fields: Vec<(String, String)>,
    /// Force add, will not return error if there is a problem with the args.
    ///
    /// For example, duplication in the custom fields and try to adding empty
    /// vault
    #[arg(short, long)]
    force:         bool,
}

impl Add {
    /// Check if there is nothing to add
    fn is_empty(&self) -> bool {
        self.vault_info.is_empty()
            && self.password.is_none()
            && self.totp_secret.is_none()
            && self.custom_fields.is_empty()
    }
}

impl LprsCommand for Add {
    fn run(mut self, mut vault_manager: Vaults) -> LprsResult<()> {
        if !self.is_empty() {
            if let Some(totp_secret) = utils::user_secret(self.totp_secret, "TOTP Secret:", false)?
            {
                cipher::base32_decode(&totp_secret).map_err(|_| {
                    LprsError::Base32("Invalid TOTP secret, must be valid base32 string".to_owned())
                })?;
                self.vault_info.totp_secret = Some(totp_secret);
            }

            self.vault_info.name = self.vault_info.name.trim().to_string();
            self.vault_info.password = utils::user_secret(self.password, "Vault password:", false)?;
            self.vault_info.custom_fields = self.custom_fields.into_iter().collect();
            vault_manager.add_vault(self.vault_info);
            vault_manager.try_export()?;
        }
        Ok(())
    }

    fn validate_args(&self) -> LprsResult<()> {
        if !self.force && self.is_empty() {
            return Err(LprsError::Other("You can't add empty vault".to_owned()));
        }

        if let Some(duplicated_key) = utils::get_duplicated_field(&self.custom_fields) {
            if !self.force {
                return Err(LprsError::Other(format!(
                    "Duplication error: The custom key `{duplicated_key}` is duplicate"
                )));
            }
        }
        if self
            .custom_fields
            .iter()
            .any(|(k, _)| k.starts_with(crate::RESERVED_FIELD_PREFIX))
        {
            return Err(LprsError::ReservedPrefix(crate::RESERVED_FIELD_PREFIX));
        }

        Ok(())
    }
}
