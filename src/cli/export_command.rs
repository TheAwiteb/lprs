// Lprs - A local CLI password manager
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

use std::{fs, path::PathBuf};

use clap::Args;

use crate::{
    password::{BitWardenPasswords, Format, Passwords},
    LprsError, LprsResult, RunCommand,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Export {
    /// The path to export to
    path: PathBuf,
    /// Format to export passwords in
    #[arg(short, long, value_name = "FORMAT", default_value_t= Format::Lprs)]
    format: Format,
}

impl RunCommand for Export {
    fn run(&self, password_manager: Passwords) -> LprsResult<()> {
        let exported_data = match self.format {
            Format::Lprs => serde_json::to_string(&password_manager.encrypt()?.passwords),
            Format::BitWarden => serde_json::to_string(&BitWardenPasswords::from(password_manager)),
        }
        .map_err(LprsError::from)?;

        fs::write(&self.path, exported_data).map_err(LprsError::from)
    }
}
