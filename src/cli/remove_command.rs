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

use std::num::NonZeroUsize;

use clap::Args;
use either::Either;

use crate::{
    clap_parsers::either_parser,
    utils,
    vault::{Vault, Vaults},
    LprsCommand,
    LprsResult,
};

#[derive(Debug, Args)]
/// Remove command, used to remove a vault from the vaults file
pub struct Remove {
    /// The vaults to remove, index or name
    #[arg(name = "INDEX-or-NAME", value_parser = either_parser::<NonZeroUsize, String>)]
    locations: Vec<Either<NonZeroUsize, String>>,

    /// Force remove, will not return error if there is no vault with the given
    /// index or name
    #[arg(short, long)]
    force: bool,
}

impl LprsCommand for Remove {
    fn run(self, mut vault_manager: Vaults) -> LprsResult<()> {
        let indexes = self
            .locations
            .iter()
            .map(|location| {
                utils::vault_by_index_or_name(location, &mut vault_manager.vaults)
                    .map(|(_, v)| v.clone())
            })
            .collect::<LprsResult<Vec<Vault>>>();

        match indexes {
            Ok(indexes) => vault_manager.vaults.retain(|v| !indexes.contains(v)),
            Err(err) => {
                if self.force {
                    log::warn!("Ignoring error: {err}");
                    return Ok(());
                }
                return Err(err);
            }
        }

        vault_manager.try_export()
    }
}
