// Local CLI password manager
// Copyright (C) 2024  Awiteb
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
    password::{self, Passwords},
    PassrsError, PassrsResult, RunCommand,
};

pub mod add_command;
pub mod clean_command;
pub mod edit_command;
pub mod list_command;

crate::create_commands!(
    enum Commands
    "Add new password", Add => add_command::Add
    "List your password and search", List => list_command::List
    "Clean the password file", Clean => clean_command::Clean
    "Edit the password content", Edit => edit_command::Edit
    // TODO: Remove command
    // TODO: Export command
    // TODO: Import command
);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The passwords json file, default: $HOME/.local/share/passrs/passwords.json
    #[arg(short, long)]
    passwords_file: Option<PathBuf>,

    // TODO: verbose flag
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    /// Run the cli
    pub fn run(self) -> PassrsResult<()> {
        let passwords_file = if let Some(ref path) = self.passwords_file {
            path.clone()
        } else {
            crate::utils::passwords_file()?
        };
        log::debug!(
            "Getting password file: {}",
            passwords_file.to_string_lossy()
        );
        let password = scanpw::scanpw!("Master Password: ");

        if password::is_new_password_file(&passwords_file)? {
            let analyzed = passwords::analyzer::analyze(&password);
            if analyzed.length() < 15 {
                return Err(PassrsError::WeakPassword(
                    "The password length must be beggier then 15".to_owned(),
                ));
            } else if passwords::scorer::score(&analyzed) < 80.0 {
                return Err(PassrsError::WeakPassword(
                    "Your password is not stronge enough".to_owned(),
                ));
            }
        }

        let master_password = sha256::digest(password);
        let password_manager = Passwords::try_reload(
            passwords_file,
            master_password.into_bytes().into_iter().take(32).collect(),
        )?;
        self.command.run(password_manager)?;

        Ok(())
    }
}
