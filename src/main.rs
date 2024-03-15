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

use std::process::ExitCode;

use base64::{
    alphabet,
    engine::{general_purpose::PAD, GeneralPurpose},
};
use clap::Parser;

pub mod cli;
pub mod errors;
pub mod password;
pub mod utils;

mod macros;
mod traits;

pub use errors::{Error as LprsError, Result as LprsResult};
pub use traits::*;

pub const STANDARDBASE: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, PAD);
pub const DEFAULT_PASSWORD_FILE: &str = "passwords.json";

#[cfg(feature = "update-notify")]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(feature = "update-notify")]
pub const LAST_VERSION_CHECK_FILE: &str = ".last_version_check";

fn main() -> ExitCode {
    pretty_env_logger::init();

    #[cfg(feature = "update-notify")]
    {
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
            _ => {}
        }
    }

    if let Err(err) = cli::Cli::parse().run() {
        eprintln!("{err}");
        err.exit_code()
    } else {
        ExitCode::SUCCESS
    }
}
