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

use clap::Args;
use inquire::{Password, PasswordDisplayMode};

use crate::{
    vault::{Vault, Vaults},
    LprsCommand, LprsError, LprsResult,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
/// Add command, used to add new vault to the vaults file
pub struct Add {
    #[command(flatten)]
    vault_info: Vault,
    /// The password, if there is no value for it you will prompt it
    #[arg(short, long)]
    // FIXME: I think replacing `Option<Option<String>>` with custom type will be better
    #[allow(clippy::option_option)]
    password: Option<Option<String>>,
}

impl LprsCommand for Add {
    fn run(mut self, mut vault_manager: Vaults) -> LprsResult<()> {
        match self.password {
            Some(Some(password)) => {
                log::debug!("User provided a password");
                self.vault_info.password = Some(password);
            }
            Some(None) => {
                log::debug!("User didn't provide a password, prompting it");
                self.vault_info.password = Some(
                    Password::new("Vault password:")
                        .without_confirmation()
                        .with_formatter(&|p| "*".repeat(p.chars().count()))
                        .with_display_mode(PasswordDisplayMode::Masked)
                        .prompt()?,
                );
            }
            None => {}
        };

        vault_manager.add_vault(self.vault_info);
        vault_manager.try_export()
    }

    fn validate_args(&self) -> LprsResult<()> {
        if self.vault_info.username.is_none()
            && self.password.is_none()
            && self.vault_info.service.is_none()
            && self.vault_info.note.is_none()
        {
            return Err(LprsError::Other("You can't add empty vault".to_owned()));
        }
        Ok(())
    }
}
