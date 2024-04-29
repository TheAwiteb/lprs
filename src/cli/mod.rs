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

use std::path::PathBuf;

use clap::Parser;

use crate::{utils, vault::Vaults, LprsCommand, LprsResult};

pub mod add_command;
pub mod clean_command;
pub mod edit_command;
pub mod export_command;
pub mod gen_command;
pub mod import_command;
pub mod list_command;
pub mod remove_command;

crate::create_commands!(
    enum Commands
    "Add new vault", Add => add_command::Add
    "Remove vault", Remove => remove_command::Remove
    "List your vaults and search", List => list_command::List
    "Clean the vaults file", Clean => clean_command::Clean
    "Edit the vault content", Edit => edit_command::Edit
    "Generate a password", Gen => gen_command::Gen
    "Export the vaults", Export => export_command::Export
    "Import vaults", Import => import_command::Import
);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The vaults json file
    #[arg(short, long)]
    vaults_file: Option<PathBuf>,

    // TODO: verbose flag
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    /// Run the cli
    pub fn run(self) -> LprsResult<()> {
        let vaults_file = if let Some(ref path) = self.vaults_file {
            path.clone()
        } else {
            crate::utils::vaults_file()?
        };
        log::debug!("Getting the vaults file: {}", vaults_file.to_string_lossy());

        self.command.validate_args()?;

        let vault_manager = if matches!(self.command, Commands::Clean(..) | Commands::Gen(..)) {
            // Returns empty vault manager for those commands don't need it
            Vaults {
                vaults_file,
                ..Default::default()
            }
        } else {
            let master_password = utils::master_password_prompt(&vaults_file)?;
            Vaults::try_reload(
                vaults_file,
                master_password.into_bytes().into_iter().take(32).collect(),
            )?
        };

        self.command.run(vault_manager)
    }
}
