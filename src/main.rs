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

#![doc = include_str!("../README.md")]

use std::env;
use std::process::ExitCode;

use clap::Parser;
use inquire::InquireError;

/// A set of clap vaule parsers used to parse some CLI arguments
pub mod clap_parsers;
/// The main module of the lprs crate, contains the cli and the commands.
pub mod cli;
/// The errors module, contains the errors and the result type.
pub mod errors;
/// The utils module, contains the utility functions of all the modules.
pub mod utils;
/// The vault module, contains the vault struct and the vaults manager.
pub mod vault;

mod macros;
mod traits;

pub use base64::engine::general_purpose::STANDARD as BASE64;
pub use errors::{Error as LprsError, Result as LprsResult};
pub use traits::*;

/// The default vaults file name. Used to store the vaults.
pub const DEFAULT_VAULTS_FILE: &str = "vaults.lprs";

/// The prefix of the reserved custom fields
const RESERVED_FIELD_PREFIX: &str = ".lprsfield.";

fn main() -> ExitCode {
    let lprs_cli = cli::Cli::parse();
    if lprs_cli.verbose {
        env::set_var("RUST_LOG", "lprs");
    }
    pretty_env_logger::init();

    if let Err(err) = lprs_cli.run() {
        if !matches!(
            err,
            LprsError::Inquire(InquireError::OperationCanceled)
                | LprsError::Inquire(InquireError::OperationInterrupted)
        ) {
            eprintln!("{err}");
            return err.exit_code();
        }
    }
    ExitCode::SUCCESS
}
