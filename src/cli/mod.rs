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

use std::{fs, path::PathBuf};

use clap::Parser;
use sha2::{Digest, Sha256};

use crate::{impl_commands, utils, vault::Vaults, LprsCommand, LprsResult};

/// Add command, used to add new vault to the vaults file
pub mod add_command;
/// Change master password, reencrypt the vaults with new password
pub mod change_master_password_command;
/// Clean command, used to clean the vaults file (remove all vaults)
pub mod clean_command;
/// Generate shell completion
pub mod completion_command;
/// Edit command, used to edit the vault content
pub mod edit_command;
/// Export command, used to export the vaults
/// in `lprs` format or `BitWarden` format
pub mod export_command;
/// Generate command, used to generate a password
pub mod gen_command;
/// Command to get a entire vault or single field from it
pub mod get_command;
/// Import command, used to import vaults from the exported files, `lprs` or
/// `BitWarden`
pub mod import_command;
/// List command, used to list the vaults and search
pub mod list_command;
/// Remove command, used to remove vault from the vaults file
pub mod remove_command;

/// The lprs commands
#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Add new vault
    Add(add_command::Add),
    /// Remove vault [alias `rm`]
    #[command(alias = "rm")]
    Remove(remove_command::Remove),
    /// List your vaults and search [alias `ls`]
    #[command(alias = "ls")]
    List(list_command::List),
    /// Clean the vaults file
    Clean(clean_command::Clean),
    /// Edit the vault content
    Edit(edit_command::Edit),
    /// Generate a password
    Gen(gen_command::Gen),
    /// Get a entire vault or single field from it
    Get(get_command::Get),
    /// Export the vaults
    Export(export_command::Export),
    /// Import vaults
    Import(import_command::Import),
    /// Change master password, reencrypt the vaults with new password
    ChangeMasterPassword(change_master_password_command::ChangeMasterPassword),
    /// Generate shell completion
    Completion(completion_command::Completion),
}

impl_commands!(Commands, Add Remove List Clean Edit Gen Get Export Import ChangeMasterPassword Completion);

/// Header message, used in the help message
const HEADER: &str = r#"Copyright (C) 2024 Awiteb <a@4rs.nl>
License GNU GPL-3.0-or-later <https://gnu.org/licenses/gpl-3.0.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Git repository: https://git.4rs.nl/awiteb/lprs
Documentation: https://lprs.4rs.nl"#;

/// Footer message, used in the help message
const FOOTER: &str = r#"Please report bugs to <https://git.4rs.nl/awiteb/lprs/issues>."#;

#[derive(Parser, Debug)]
#[command(about, version, before_long_help = HEADER, after_help = FOOTER)]
/// A local CLI vaults manager. For human and machine use
pub struct Cli {
    /// The vaults json file
    #[arg(short = 'f', long)]
    pub vaults_file:     Option<PathBuf>,
    /// Show the logs in the stdout
    #[arg(short, long)]
    pub verbose:         bool,
    /// The master password, or you will prompt it
    #[arg(short, long)]
    pub master_password: Option<String>,

    #[command(subcommand)]
    /// The provided command to run
    pub command: Commands,
}

impl Cli {
    /// Run the cli
    ///
    /// # Errors
    /// - If can't get the default vaults file
    /// - If the vaults file can't be created
    /// - If the user provide a worng CLI arguments
    /// - If failed to write in the vaults file
    /// - (errors from the commands)
    pub fn run(self) -> LprsResult<()> {
        let vaults_file = if let Some(path) = self.vaults_file {
            log::info!("Using the given vaults file");
            if let Some(parent) = path.parent() {
                if parent.to_str() != Some("") && !parent.exists() {
                    log::info!(
                        "Creating the parent vaults file directory: {}",
                        parent.display()
                    );
                    fs::create_dir_all(parent)?;
                }
            }
            if !path.exists() {
                fs::File::create(&path)?;
            }
            path
        } else {
            log::info!("Using the default vaults file");
            utils::vaults_file()?
        };
        log::debug!("Vaults file: {}", vaults_file.display());

        self.command.validate_args()?;

        let vault_manager = if matches!(
            self.command,
            Commands::Clean(..) | Commands::Gen(..) | Commands::Completion(..)
        ) {
            log::info!("Running command that don't need the vault manager");
            // Returns empty vault manager for those commands don't need it
            Vaults {
                vaults_file,
                ..Default::default()
            }
        } else {
            log::info!("Reloading the vaults file");
            let master_password = if let Some(plain_master_password) = self.master_password {
                Sha256::digest(plain_master_password).into()
            } else {
                utils::master_password_prompt(fs::read(&vaults_file)?.is_empty())?
            };
            Vaults::try_reload(vaults_file, master_password)?
        };

        self.command.run(vault_manager)
    }
}
