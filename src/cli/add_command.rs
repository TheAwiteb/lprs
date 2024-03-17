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

use clap::Args;

use crate::{
    password::{Password, Passwords},
    LprsResult, RunCommand,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Add {
    #[command(flatten)]
    password_info: Password,
}

impl RunCommand for Add {
    fn run(&self, mut password_manager: Passwords) -> LprsResult<()> {
        password_manager.add_password(self.password_info.clone());
        password_manager.try_export()
    }
}
