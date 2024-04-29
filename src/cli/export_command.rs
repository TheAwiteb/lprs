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

use std::{fs, io::Error as IoError, io::ErrorKind as IoErrorKind, path::PathBuf};

use clap::Args;

use crate::{
    vault::{vault_state::*, BitWardenPasswords, Format, Vault, Vaults},
    LprsCommand, LprsError, LprsResult,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Export {
    /// The path to export to
    path: PathBuf,
    /// Format to export vaults in
    #[arg(short, long, value_name = "FORMAT", default_value_t= Format::Lprs)]
    format: Format,
}

impl LprsCommand for Export {
    fn run(self, vault_manager: Vaults<Plain>) -> LprsResult<()> {
        let exported_data = match self.format {
            Format::Lprs => {
                serde_json::to_string::<Vec<Vault<Encrypted>>>(&vault_manager.encrypt_vaults()?)
            }
            Format::BitWarden => serde_json::to_string(&BitWardenPasswords::from(vault_manager)),
        }?;

        fs::write(&self.path, exported_data).map_err(LprsError::from)
    }

    fn validate_args(&self) -> LprsResult<()> {
        if self
            .path
            .extension()
            .is_some_and(|e| e.to_string_lossy().eq_ignore_ascii_case("json"))
        {
            if !self.path.exists() {
                Ok(())
            } else {
                Err(LprsError::Io(IoError::new(
                    IoErrorKind::AlreadyExists,
                    format!("file `{}` is already exists", self.path.display()),
                )))
            }
        } else {
            Err(LprsError::Io(IoError::new(
                IoErrorKind::InvalidInput,
                format!("file `{}` is not a json file", self.path.display()),
            )))
        }
    }
}
