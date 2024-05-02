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

pub mod add_command;
pub mod clean_command;
pub mod edit_command;
pub mod export_command;
pub mod gen_command;
pub mod import_command;
pub mod list_command;
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
    /// Export the vaults
    Export(export_command::Export),
    /// Import vaults
    Import(import_command::Import),
}

impl_commands!(Commands, Add Remove List Clean Edit Gen Export Import);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The vaults json file
    #[arg(short = 'f', long)]
    pub vaults_file: Option<PathBuf>,
    /// Show the logs in the stdout
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    /// Run the cli
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
            crate::utils::vaults_file()?
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
