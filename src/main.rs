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

pub use {macros::*, traits::*};

pub use errors::{Error as PassrsError, Result as PassrsResult};

pub const STANDARDBASE: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, PAD);

fn main() -> ExitCode {
    pretty_env_logger::init();

    if let Err(err) = cli::Cli::parse().run() {
        eprintln!("{err}");
        err.exit_code()
    } else {
        ExitCode::SUCCESS
    }
}
