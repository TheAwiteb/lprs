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

use std::fs;

use clap::Args;

use crate::{vault::Vaults, LprsCommand, LprsError, LprsResult};

#[derive(Debug, Args)]
/// Clean command, used to clean the vaults file (remove all vaults)
pub struct Clean;

impl LprsCommand for Clean {
    fn run(self, vault_manager: Vaults) -> LprsResult<()> {
        log::info!(
            "Cleaning the vaults file: {:?}",
            vault_manager.vaults_file.display()
        );
        fs::write(vault_manager.vaults_file, []).map_err(LprsError::Io)
    }
}
