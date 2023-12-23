use std::fs;

use clap::Args;

use crate::{password::Passwords, PassrsError, PassrsResult, RunCommand};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Clean {}

impl RunCommand for Clean {
    fn run(&self, password_manager: Passwords) -> PassrsResult<()> {
        fs::write(password_manager.passwords_file, "[]").map_err(PassrsError::Io)
    }
}
