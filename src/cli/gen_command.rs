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

use std::num::NonZeroU64;

use clap::Args;

use crate::{
    vault::{vault_state::*, Vaults},
    LprsError, LprsResult, RunCommand,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Gen {
    /// The password length
    #[arg(default_value_t = NonZeroU64::new(18).unwrap())]
    length: NonZeroU64,

    /// With uppercase letters (A-Z)
    #[arg(short, long)]
    uppercase: bool,
    /// With lowercase letters (a-z)
    #[arg(short, long)]
    lowercase: bool,
    /// With numbers (0-9)
    #[arg(short, long)]
    numbers: bool,
    /// With symbols (!,# ...)
    #[arg(short, long)]
    symbols: bool,
}

impl RunCommand for Gen {
    fn run(&self, _password_manager: Vaults<Plain>) -> LprsResult<()> {
        if self.uppercase || self.lowercase || self.numbers || self.symbols {
            println!(
                "{}",
                passwords::PasswordGenerator::new()
                    .length(self.length.get() as usize)
                    .uppercase_letters(self.uppercase)
                    .lowercase_letters(self.lowercase)
                    .numbers(self.numbers)
                    .symbols(self.symbols)
                    .strict(true)
                    .generate_one()
                    .expect("The length cannot be zero")
            );
            Ok(())
        } else {
            Err(LprsError::Other(
                "You need to enable at least one kind of characters".to_owned(),
            ))
        }
    }
}
