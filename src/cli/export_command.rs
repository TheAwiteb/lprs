// Lprs - A local CLI vaults manager. For human and machine use
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
use sha2::Digest;

use crate::{
    utils,
    vault::{BitWardenPasswords, Format, Vaults},
    LprsCommand,
    LprsError,
    LprsResult,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
/// Export command, used to export the vaults in `lprs` format or `BitWarden`
/// format. The exported file will be a json file.
pub struct Export {
    // TODO: `force` flag to write on existing file
    /// The path to export to
    path:                PathBuf,
    /// Format to export vaults in
    #[arg(short, long, value_name = "FORMAT", default_value_t= Format::Lprs)]
    format:              Format,
    /// Encryption password of the exported vaults (in `lprs` format)
    /// if there is not, will use the master password
    #[arg(short = 'p', long)]
    #[allow(clippy::option_option)]
    encryption_password: Option<Option<String>>,
}

impl LprsCommand for Export {
    fn run(self, vault_manager: Vaults) -> LprsResult<()> {
        log::debug!(
            "Exporting vault {} to: {} with format: {}",
            vault_manager.vaults_file.display(),
            self.path.display(),
            self.format
        );

        let encryption_key: Option<[u8; 32]> =
            utils::user_secret(self.encryption_password, "Encryption Password:")?
                .map(|p| sha2::Sha256::digest(p).into());

        let exported_data = match self.format {
            Format::Lprs => {
                vault_manager.json_export(
                    encryption_key
                        .as_ref()
                        .unwrap_or(&vault_manager.master_password),
                )?
            }
            Format::BitWarden => serde_json::to_string(&BitWardenPasswords::from(vault_manager))?,
        };

        fs::write(&self.path, exported_data).map_err(LprsError::from)
    }

    fn validate_args(&self) -> LprsResult<()> {
        if !self
            .path
            .extension()
            .is_some_and(|e| e.to_string_lossy().eq_ignore_ascii_case("json"))
        {
            return Err(LprsError::Io(IoError::new(
                IoErrorKind::InvalidInput,
                format!("file `{}` is not a json file", self.path.display()),
            )));
        }
        if self.path.exists() {
            return Err(LprsError::Io(IoError::new(
                IoErrorKind::AlreadyExists,
                format!("file `{}` is already exists", self.path.display()),
            )));
        }
        if self.path.is_dir() {
            return Err(LprsError::Io(IoError::new(
                IoErrorKind::InvalidInput,
                format!("file `{}` is a directory", self.path.display()),
            )));
        }
        if self.encryption_password.is_some() && self.format != Format::Lprs {
            return Err(LprsError::Other(
                "You only can to use the encryption password with `lprs` format".to_owned(),
            ));
        }

        Ok(())
    }
}
