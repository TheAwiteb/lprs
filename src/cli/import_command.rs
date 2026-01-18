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

/// Import command, used to import vaults from the exported files.
#[derive(Debug, Args)]
pub struct Import {
    /// The file path to import from
    path: PathBuf,

    /// Decryption password of the imported vaults, if there is not, will use
    /// the master password
    #[arg(short = 'p', long)]
    #[allow(clippy::option_option)]
    decryption_password: Option<Option<String>>,
}

impl LprsCommand for Import {
    fn run(self, mut vault_manager: Vaults) -> LprsResult<()> {
        log::debug!(
            "Importing vaults from: `{}` to the vault: `{}`",
            self.path.display(),
            vault_manager.vaults_file.display()
        );

        let decryption_key: Option<[u8; 32]> =
            utils::user_secret(self.decryption_password, "Decryption password:", false)?
                .map(|p| sha2::Sha256::digest(p).into());

        let vaults = Vaults::json_reload(
            decryption_key
                .as_ref()
                .unwrap_or(&vault_manager.master_password),
            &fs::read(self.path)?,
        )?;
        let vaults_len = vaults.len();

        vault_manager.vaults = vaults;
        vault_manager.try_export()?;

        println!(
            "{vaults_len} vault{s} were imported successfully",
            s = if vaults_len >= 2 { "s" } else { "" }
        );
        Ok(())
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
        if !self.path.exists() {
            return Err(LprsError::Io(IoError::new(
                IoErrorKind::NotFound,
                format!("file `{}` not found", self.path.display()),
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
