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

use std::{fs::File, io::Error as IoError, io::ErrorKind as IoErrorKind, path::PathBuf};

use clap::Args;

use crate::{
    vault::{vault_state::*, BitWardenPasswords, Format, Vault, Vaults},
    LprsCommand, LprsError, LprsResult,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Import {
    /// The file path to import from
    path: PathBuf,

    /// The format to import from
    #[arg(short, long, default_value_t = Format::Lprs)]
    format: Format,
}

impl LprsCommand for Import {
    fn run(self, mut vault_manager: Vaults<Plain>) -> LprsResult<()> {
        let imported_passwords_len = match self.format {
            Format::Lprs => {
                let vaults = Vaults::try_reload(self.path, vault_manager.master_password.to_vec())?;
                let vaults_len = vaults.vaults.len();

                vault_manager.vaults.extend(vaults.vaults);
                vault_manager.try_export()?;
                vaults_len
            }
            Format::BitWarden => {
                let vaults: BitWardenPasswords = serde_json::from_reader(File::open(&self.path)?)?;
                let vaults_len = vaults.items.len();

                vault_manager
                    .vaults
                    .extend(vaults.items.into_iter().map(Vault::from));
                vault_manager.try_export()?;
                vaults_len
            }
        };
        println!(
            "{imported_passwords_len} vault{s} were imported successfully",
            s = if imported_passwords_len >= 2 { "s" } else { "" }
        );
        Ok(())
    }

    fn validate_args(&self) -> LprsResult<()> {
        if self.path.exists() {
            if self
                .path
                .extension()
                .is_some_and(|e| e.to_string_lossy().eq_ignore_ascii_case("json"))
            {
                Ok(())
            } else {
                Err(LprsError::Io(IoError::new(
                    IoErrorKind::InvalidInput,
                    format!("file `{}` is not a json file", self.path.display()),
                )))
            }
        } else {
            Err(LprsError::Io(IoError::new(
                IoErrorKind::NotFound,
                format!("file `{}` not found", self.path.display()),
            )))
        }
    }
}
