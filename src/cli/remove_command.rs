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

use crate::{password::Passwords, LprsError, LprsResult, RunCommand};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Remove {
    /// The password index
    index: NonZeroU64,

    /// Force remove, will not return error if there is no password with this index
    #[arg(short, long)]
    force: bool,
}

impl RunCommand for Remove {
    fn run(&self, mut password_manager: Passwords) -> LprsResult<()> {
        let index = (self.index.get() - 1) as usize;
        if index > password_manager.passwords.len() {
            if !self.force {
                return Err(LprsError::Other(
                    "The index is greater than the passwords counts".to_owned(),
                ));
            }
        } else {
            password_manager.passwords.remove(index);
            password_manager.try_export()?;
        }
        Ok(())
    }
}
