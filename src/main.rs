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
