// Lprs - A local CLI password manager
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

use crate::{
    password::{self, Vaults},
    LprsError, LprsResult, RunCommand,
};

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
    "Add new password", Add => add_command::Add
    "Remove password", Remove => remove_command::Remove
    "List your password and search", List => list_command::List
    "Clean the password file", Clean => clean_command::Clean
    "Edit the password content", Edit => edit_command::Edit
    "Generate password", Gen => gen_command::Gen
    "Export the passwords", Export => export_command::Export
    "Import passwords", Import => import_command::Import
);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The vaults json file
    #[arg(short, long)]
    passwords_file: Option<PathBuf>,

    // TODO: verbose flag
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    /// Run the cli
    pub fn run(self) -> LprsResult<()> {
        let passwords_file = if let Some(ref path) = self.passwords_file {
            path.clone()
        } else {
            crate::utils::passwords_file()?
        };
        log::debug!("Getting the vaults file: {}", vaults_file.to_string_lossy());
        let vault_manager = if matches!(self.command, Commands::Clean(..) | Commands::Gen(..)) {
            Vaults {
                passwords_file,
                ..Default::default()
            }
        } else {
            let master_password = scanpw::scanpw!("Master Password: ");

            if vault::is_new_vaults_file(&vaults_file)? {
                let analyzed = passwords::analyzer::analyze(&master_password);
                if analyzed.length() < 15 {
                    return Err(LprsError::WeakPassword(
                        "The master password length must be beggier then 15".to_owned(),
                    ));
                } else if passwords::scorer::score(&analyzed) < 80.0 {
                    return Err(LprsError::WeakPassword(
                        "Your master password is not stronge enough".to_owned(),
                    ));
                }
            }

            let master_password = sha256::digest(master_password);
            Vaults::try_reload(
                passwords_file,
                master_password.into_bytes().into_iter().take(32).collect(),
            )?
        };
        self.command.run(password_manager)?;

        Ok(())
    }
}
