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

use std::num::NonZeroUsize;

use clap::Args;
use either::Either;

use crate::{
    clap_parsers::{either_parser, kv_parser},
    utils,
    vault::{cipher, Vaults},
    LprsCommand,
    LprsError,
    LprsResult,
};

#[derive(Debug, Args)]
/// Edit command, used to edit the vault content
pub struct Edit {
    /// The vault to edit, index or name
    #[arg(name = "INDEX-or-NAME", value_parser = either_parser::<NonZeroUsize, String>)]
    location: Either<NonZeroUsize, String>,

    #[arg(short, long)]
    /// The new vault name
    name:          Option<String>,
    #[arg(short, long)]
    /// The new vault username, make it empty string to delete it
    username:      Option<String>,
    #[arg(short, long)]
    /// The new password, make it empty string to delete it
    ///
    /// If there is no value for it you will prompt it
    #[allow(clippy::option_option)]
    password:      Option<Option<String>>,
    #[arg(short, long)]
    /// The new vault service, make it empty string to delete it
    service:       Option<String>,
    #[arg(short = 'o', long)]
    /// The new vault note
    note:          Option<String>,
    /// The TOTP secret, make it empty string to delete it
    ///
    /// If there is no value you will prompt it
    #[arg(short, long)]
    #[allow(clippy::option_option)]
    totp_secret:   Option<Option<String>>,
    /// The custom field, make it empty string to delete it
    ///
    /// If the custom field not exist will created it, if it's will update it,
    /// if there is no value, you will enter it through a prompt (e.g `-c key`)
    #[arg(name = "KEY=VALUE", short = 'c', long = "custom")]
    #[arg(value_parser = kv_parser)]
    custom_fields: Vec<(String, Option<String>)>,
    /// Force edit, will not return error if there is a problem with the args.
    ///
    /// For example, duplication in the custom fields and try to editing nothing
    #[arg(short, long)]
    force:         bool,
}

impl LprsCommand for Edit {
    fn run(self, mut vault_manager: Vaults) -> LprsResult<()> {
        let vault = match utils::vault_by_index_or_name(self.location, &mut vault_manager.vaults) {
            Ok((_, v)) => v,
            Err(err) => {
                if self.force {
                    return Ok(());
                }
                return Err(err);
            }
        };

        log::info!("Applying the new values to the vault");
        if let Some(new_name) = self.name {
            vault.name = new_name;
        }
        if let Some(ref new_password) = self.password {
            if new_password.as_deref().is_some_and(|s| s.is_empty()) {
                vault.password = None;
            } else {
                vault.password = utils::user_secret(self.password, "New vault password:", false)?;
            }
        }
        if let Some(totp_secret) = utils::user_secret(self.totp_secret, "TOTP Secret:", false)? {
            if totp_secret.is_empty() {
                vault.totp_secret = None;
            } else {
                cipher::base32_decode(&totp_secret).map_err(|_| {
                    LprsError::Base32("Invalid TOTP secret, must be valid base32 string".to_owned())
                })?;
                vault.totp_secret = Some(totp_secret);
            }
        }
        if let Some(new_username) = self.username {
            if new_username.is_empty() {
                vault.username = None;
            } else {
                vault.username = Some(new_username);
            }
        }
        if let Some(new_service) = self.service {
            if new_service.is_empty() {
                vault.service = None;
            } else {
                vault.service = Some(new_service);
            }
        }
        if let Some(new_note) = self.note {
            if new_note.is_empty() {
                vault.note = None;
            } else {
                vault.note = Some(new_note);
            }
        }
        utils::apply_custom_fields(
            &mut vault.custom_fields,
            utils::prompt_custom(self.custom_fields)?,
        );

        vault_manager.try_export()
    }

    fn validate_args(&self) -> LprsResult<()> {
        if !self.force
            && self.name.is_none()
            && self.username.is_none()
            && self.password.is_none()
            && self.service.is_none()
            && self.note.is_none()
            && self.totp_secret.is_none()
            && self.custom_fields.is_empty()
        {
            return Err(LprsError::Other(
                "You must edit one option at least".to_owned(),
            ));
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
