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

use std::{fs, path::PathBuf};

use clap::Parser;

use crate::{impl_commands, utils, vault::Vaults, LprsCommand, LprsResult};

/// Add command, used to add new vault to the vaults file
pub mod add_command;
/// Clean command, used to clean the vaults file (remove all vaults)
pub mod clean_command;
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
}

impl_commands!(Commands, Add Remove List Clean Edit Gen Get Export Import);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// The lprs cli, manage the CLI arguments and run the commands
pub struct Cli {
    /// The vaults json file
    #[arg(short = 'f', long)]
    pub vaults_file: Option<PathBuf>,
    /// Show the logs in the stdout
    #[arg(short, long)]
    pub verbose:     bool,

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
                    log::info!("Creating the parent vaults file directory");
                    fs::create_dir_all(parent)?;
                }
            }
            fs::File::create(&path)?;
            path
        } else {
            log::info!("Using the default vaults file");
            utils::vaults_file()?
        };
        log::debug!("Vaults file: {}", vaults_file.display());

        self.command.validate_args()?;

        let vault_manager = if matches!(self.command, Commands::Clean(..) | Commands::Gen(..)) {
            log::info!("Running command that don't need the vault manager");
            // Returns empty vault manager for those commands don't need it
            Vaults {
                vaults_file,
                ..Default::default()
            }
        } else {
            log::info!("Reloading the vaults file");
            let master_password =
                utils::master_password_prompt(fs::read(&vaults_file)?.is_empty())?;
            Vaults::try_reload(vaults_file, master_password)?
        };

        self.command.run(vault_manager)
    }
}
