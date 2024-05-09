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

use std::num::NonZeroU64;

use clap::Args;

use crate::{clap_parsers, utils, vault::Vaults, LprsCommand, LprsError, LprsResult};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
/// Edit command, used to edit the vault content
pub struct Edit {
    /// The password index. You can get it from the list command
    index: NonZeroU64,

    #[arg(short, long)]
    /// The new vault name
    name:              Option<String>,
    #[arg(short, long)]
    /// The new vault username
    username:          Option<String>,
    #[arg(short, long)]
    /// The new password, if there is no value for it you will prompt it
    #[allow(clippy::option_option)]
    password:          Option<Option<String>>,
    #[arg(short, long)]
    /// The new vault service
    service:           Option<String>,
    #[arg(short = 'o', long)]
    /// The new vault note
    note:              Option<String>,
    /// The custom field, make its value empty to delete it
    ///
    /// If the custom field not exist will created it, if it's will update it
    #[arg(name = "KEY=VALUE", short = 'c', long = "custom")]
    #[arg(value_parser = clap_parsers::kv_parser)]
    pub custom_fields: Vec<(String, String)>,
}

impl LprsCommand for Edit {
    fn run(self, mut vault_manager: Vaults) -> LprsResult<()> {
        let index = self.index.get() as usize;
        log::debug!("Editing vault at index: {index}");

        let Some(vault) = vault_manager.vaults.get_mut(index - 1) else {
            return Err(LprsError::InvalidVaultIndex(format!(
                "The index `{}` is greater than the vaults count {}",
                self.index,
                vault_manager.vaults.len()
            )));
        };

        log::info!("Applying the new values to the vault");
        if let Some(new_name) = self.name {
            vault.name = new_name;
        }
        if self.password.is_some() {
            vault.password = utils::user_password(self.password, "New vault password:")?;
        }
        if let Some(new_username) = self.username {
            vault.username = Some(new_username);
        }
        if let Some(new_service) = self.service {
            vault.service = Some(new_service);
        }
        if let Some(new_note) = self.note {
            vault.note = Some(new_note);
        }
        utils::apply_custom_fields(&mut vault.custom_fields, self.custom_fields);

        vault_manager.try_export()
    }

    fn validate_args(&self) -> LprsResult<()> {
        if self.name.is_none()
            && self.username.is_none()
            && self.password.is_none()
            && self.service.is_none()
            && self.note.is_none()
            && self.custom_fields.is_empty()
        {
            return Err(LprsError::Other(
                "You must edit one option at least".to_owned(),
            ));
        }
        if let Some(duplicated_key) = utils::get_duplicated_field(&self.custom_fields) {
            return Err(LprsError::Other(format!(
                "Duplication error: The custom key `{duplicated_key}` is duplicate"
            )));
        }

        Ok(())
    }
}
