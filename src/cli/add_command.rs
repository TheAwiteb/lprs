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
    vault::{Vault, Vaults},
    LprsCommand,
    LprsError,
    LprsResult,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
/// Add command, used to add new vault to the vaults file
pub struct Add {
    #[command(flatten)]
    vault_info:    Vault,
    /// The password, if there is no value for it you will prompt it
    #[arg(short, long)]
    #[allow(clippy::option_option)]
    password:      Option<Option<String>>,
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

impl LprsCommand for Add {
    fn run(mut self, mut vault_manager: Vaults) -> LprsResult<()> {
        if !self.vault_info.is_empty() {
            self.vault_info.name = self.vault_info.name.trim().to_string();
            self.vault_info.password = utils::user_secret(self.password, "Vault password:")?;
            self.vault_info.totp_secret = utils::user_secret(self.totp_secret, "TOTP Secret:")?;
            self.vault_info.custom_fields = self.custom_fields.into_iter().collect();
            vault_manager.add_vault(self.vault_info);
            vault_manager.try_export()?;
        }
        Ok(())
    }

    fn validate_args(&self) -> LprsResult<()> {
        if !self.force && self.vault_info.is_empty() {
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
