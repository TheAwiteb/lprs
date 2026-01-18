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

use std::{fs, io::Error as IoError, io::ErrorKind as IoErrorKind, path::PathBuf};

use clap::Args;
use sha2::Digest;

use crate::{LprsCommand, LprsError, LprsResult, utils, vault::Vaults};

#[derive(Debug, Args)]
/// Export command. The exported file will be a json file.
pub struct Export {
    // TODO: `force` flag to write on existing file
    /// The path to export to
    path:                PathBuf,
    /// Encryption password of the exported vaults, if there is not, will use
    /// the master password
    #[arg(short = 'p', long)]
    #[allow(clippy::option_option)]
    encryption_password: Option<Option<String>>,
}

impl LprsCommand for Export {
    fn run(self, vault_manager: Vaults) -> LprsResult<()> {
        log::debug!(
            "Exporting vault {} to: {}",
            vault_manager.vaults_file.display(),
            self.path.display(),
        );

        let encryption_key: Option<[u8; 32]> =
            utils::user_secret(self.encryption_password, "Encryption Password:", false)?
                .map(|p| sha2::Sha256::digest(p).into());

        let exported_data = vault_manager.json_export(
            encryption_key
                .as_ref()
                .unwrap_or(&vault_manager.master_password),
        )?;

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

        Ok(())
    }
}
