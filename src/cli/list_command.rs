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

use std::num::NonZeroU64;

use clap::Args;

use crate::{
    vault::{vault_state::*, Vaults},
    LprsError, LprsResult, RunCommand,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct List {
    /// Show the clean password
    #[arg(short = 'p', long)]
    unhide_password: bool,
    /// Show the service of the password and search in it if you search
    #[arg(short = 's', long)]
    with_service: bool,
    /// Show the note of the password and search in it if you search
    #[arg(short = 'n', long)]
    with_note: bool,

    /// Return the password with spesifc index
    #[arg(short, long, value_name = "INDEX")]
    get: Option<NonZeroU64>,
    /// Search and display only matching passwords.
    ///
    /// The name and username will be searched. And service and note if included
    #[arg(short = 'e', long, value_name = "TEXT")]
    search: Option<String>,
    /// Enable regex in the search
    #[arg(short, long)]
    regex: bool,
}

impl RunCommand for List {
    fn run(&self, vault_manager: Vaults<Plain>) -> LprsResult<()> {
        if vault_manager.vaults.is_empty() {
            return Err(LprsError::Other(
                "Looks like there is no passwords to list".to_owned(),
            ));
        }

        todo!("https://git.4rs.nl/awiteb/lprs/issues/8")
    }
}
