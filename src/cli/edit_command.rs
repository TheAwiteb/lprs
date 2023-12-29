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

use std::num::NonZeroU64;

use clap::Args;

use crate::{
    password::{Password, Passwords},
    LprsError, LprsResult, RunCommand,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Edit {
    /// The password index. Check it from list command
    index: NonZeroU64,

    #[arg(short, long)]
    /// The new password name
    name: Option<String>,
    #[arg(short, long)]
    /// The new password username
    username: Option<String>,
    #[arg(short, long)]
    /// The new password
    password: Option<String>,
    #[arg(short, long)]
    /// The new password service
    service: Option<String>,
    #[arg(short = 'o', long)]
    /// The new password note
    note: Option<String>,
}

impl RunCommand for Edit {
    fn run(&self, mut password_manager: Passwords) -> LprsResult<()> {
        let index = self.index.get() as usize;

        if let Some(password) = password_manager.passwords.get_mut(index - 1) {
            if self.name.is_none()
                && self.username.is_none()
                && self.password.is_none()
                && self.service.is_none()
                && self.note.is_none()
            {
                Err(LprsError::Other(
                    "You must edit one option at least".to_owned(),
                ))
            } else {
                *password = Password {
                    name: self.name.as_ref().unwrap_or(&password.name).to_string(),
                    username: self
                        .username
                        .as_ref()
                        .unwrap_or(&password.username)
                        .to_string(),
                    password: self
                        .password
                        .as_ref()
                        .unwrap_or(&password.password)
                        .to_string(),
                    service: self.service.as_ref().or(password.service.as_ref()).cloned(),
                    note: self.note.as_ref().or(password.note.as_ref()).cloned(),
                };
                password_manager.try_export()
            }
        } else {
            Err(LprsError::InvalidPasswordIndex(format!(
                "The index `{}` is greater than the passwords count {}",
                self.index,
                password_manager.passwords.len()
            )))
        }
    }
}
