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

use crate::{
    vault::{vault_state::*, Vault, Vaults},
    LprsError, LprsResult, RunCommand,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Edit {
    /// The password index. Check it from list command
    index: NonZeroU64,

    #[arg(short, long)]
    /// The new vault name
    name: Option<String>,
    #[arg(short, long)]
    /// The new vault username
    username: Option<String>,
    #[arg(short, long)]
    /// The new password
    password: Option<String>,
    #[arg(short, long)]
    /// The new vault service
    service: Option<String>,
    #[arg(short = 'o', long)]
    /// The new vault note
    note: Option<String>,
}

impl RunCommand for Edit {
    fn run(&self, mut vault_manager: Vaults<Plain>) -> LprsResult<()> {
        let index = self.index.get() as usize;

        if let Some(vault) = vault_manager.vaults.get_mut(index - 1) {
            if self.name.is_none()
                && self.username.is_none()
                && self.password.is_none()
                && self.service.is_none()
                && self.note.is_none()
            {
                Err(LprsError::Other(
                    "You must edit one option at least".to_owned(),
                ))
            } else {
                *vault = Vault::<Plain>::new(
                    self.name.as_ref().unwrap_or(&vault.name),
                    self.username.as_ref().or(vault.username.as_ref()),
                    self.password.as_ref().or(vault.password.as_ref()),
                    self.service.as_ref().or(vault.service.as_ref()),
                    self.note.as_ref().or(vault.note.as_ref()),
                );
                vault_manager.try_export()
            }
        } else {
            Err(LprsError::InvalidVaultIndex(format!(
                "The index `{}` is greater than the vaults count {}",
                self.index,
                vault_manager.vaults.len()
            )))
        }
    }
}
