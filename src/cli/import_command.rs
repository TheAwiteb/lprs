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

use std::{fs::File, io::Error as IoError, io::ErrorKind as IoErrorKind, path::PathBuf};

use clap::Args;

use crate::{
    password::{BitWardenPasswords, Format, Password, Passwords},
    LprsError, LprsResult, RunCommand,
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

impl RunCommand for Import {
    fn run(&self, mut password_manager: Passwords) -> LprsResult<()> {
        if self.path.exists() {
            if self
                .path
                .extension()
                .is_some_and(|e| e.to_string_lossy().eq_ignore_ascii_case("json"))
            {
                let imported_passwords_len = match self.format {
                    Format::Lprs => {
                        let passwords = Passwords::try_reload(
                            self.path.to_path_buf(),
                            password_manager.master_password.to_vec(),
                        )?;
                        let passwords_len = passwords.passwords.len();

                        password_manager.passwords.extend(passwords.passwords);
                        password_manager.try_export()?;
                        passwords_len
                    }
                    Format::BitWarden => {
                        let passwords: BitWardenPasswords =
                            serde_json::from_reader(File::open(&self.path)?)?;
                        let passwords_len = passwords.items.len();

                        password_manager
                            .passwords
                            .extend(passwords.items.into_iter().map(Password::from));
                        password_manager.try_export()?;
                        passwords_len
                    }
                };
                println!(
                    "{imported_passwords_len} password{s} were imported successfully",
                    s = if imported_passwords_len >= 2 { "s" } else { "" }
                );

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
