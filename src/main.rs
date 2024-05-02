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

use clap::Parser;
use inquire::InquireError;
use std::process::ExitCode;

pub mod cli;
pub mod errors;
pub mod utils;
pub mod vault;

mod macros;
mod traits;

pub use base64::engine::general_purpose::STANDARD as BASE64;
pub use errors::{Error as LprsError, Result as LprsResult};
pub use traits::*;

pub const DEFAULT_VAULTS_FILE: &str = "vaults.lprs";

#[cfg(feature = "update-notify")]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "update-notify")]
pub const LAST_VERSION_CHECK_FILE: &str = ".last_version_check";

fn main() -> ExitCode {
    let lprs_cli = cli::Cli::parse();
    if lprs_cli.verbose {
        std::env::set_var("RUST_LOG", "lprs");
    }
    pretty_env_logger::init();

    #[cfg(feature = "update-notify")]
    {
        log::info!("Checking for new version of lprs...");
        match utils::lprs_version() {
            Ok(Some(new_version)) if new_version != VERSION => {
                println!(
                    "Warning: The version you are using of lprs is outdated. There is a newer version, which is `{new_version}`, and your version is `{VERSION}`
                    \rYou can update via: `cargo install lprs --locked`
                    \rOr via git repo: `cargo install --locked --git https://git.4rs.nl/awiteb/lprs.git`
                    \rTo disable update notification: `cargo install lprs --locked --no-default-features`\n\n"
                )
            }
            Err(err) => {
                eprintln!("{err}");
                return ExitCode::FAILURE;
            }
            _ => {
                log::info!("No new version found.");
            }
        }
    }

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
