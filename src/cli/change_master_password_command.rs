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
use sha2::{Digest, Sha256};

use crate::{utils, vault::Vaults, LprsCommand, LprsResult};

#[derive(Debug, Args)]
/// Change master password, reencrypt the vaults with new password
pub struct ChangeMasterPassword {
    /// The new master password, if there is no value for it you will prompt it
    #[allow(clippy::option_option)]
    new_password: Option<String>,
}

impl LprsCommand for ChangeMasterPassword {
    fn run(self, mut vault_manager: Vaults) -> LprsResult<()> {
        vault_manager.master_password =
            utils::user_secret(Some(self.new_password), "New master password:", true)?
                .map(|s| Sha256::digest(s).into())
                .expect("We wrap it in `Some`, so is will return a secret");
        vault_manager.try_export()?;
        Ok(())
    }
}
