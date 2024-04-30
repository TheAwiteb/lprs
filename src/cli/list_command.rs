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
use inquire::Select;

use crate::{
    vault::{vault_state::*, Vaults},
    LprsCommand, LprsError, LprsResult,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct List {
    /// Return the password with spesifc index
    #[arg(short, long, value_name = "INDEX")]
    get: Option<NonZeroU64>,
    /// Filter the select list
    #[arg(short, long, value_name = "TEXT")]
    filter: Option<String>,
    /// Enable regex when use `--filter` option
    #[arg(short, long)]
    regex: bool,
}

impl LprsCommand for List {
    fn run(self, vault_manager: Vaults<Plain>) -> LprsResult<()> {
        if vault_manager.vaults.is_empty() {
            return Err(LprsError::Other(
                "Looks like there is no vaults to list".to_owned(),
            ));
        }
        if let Some(user_vault_index) = self.get.map(|n| (n.get() - 1) as usize) {
            log::info!("Getting the vault at index: {user_vault_index}");
            if user_vault_index >= vault_manager.vaults.len() {
                return Err(LprsError::Other(
                    "The `--get` index is great then the vaults length".to_owned(),
                ));
            }
            println!(
                "{}",
                vault_manager
                    .vaults
                    .get(user_vault_index)
                    .expect("The index is correct")
            );
        } else {
            let pattern = if self.regex || self.filter.is_none() {
                self.filter.unwrap_or_else(|| ".".to_owned())
            } else {
                format!(
                    ".*{}.*",
                    regex::escape(self.filter.as_deref().unwrap_or(""))
                )
            };
            log::debug!("Listing vaults filtered by: {pattern}");

            let re = regex::Regex::new(&pattern)?;

            let vaults_list = vault_manager
                .vaults
                .iter()
                .enumerate()
                .filter_map(|(idx, v)| {
                    if re.is_match(&v.name)
                        || v.username.as_deref().is_some_and(|u| re.is_match(u))
                        || v.service.as_deref().is_some_and(|s| re.is_match(s))
                        || v.note.as_deref().is_some_and(|n| re.is_match(n))
                    {
                        return Some(format!("{}) {}", idx + 1, v.list_name()));
                    }
                    None
                })
                .collect::<Vec<_>>();

            if vaults_list.is_empty() {
                return Err(LprsError::Other(
                    "There is no result match your filter".to_owned(),
                ));
            }

            let vault_idx = Select::new("Select a vault to view:", vaults_list)
                .with_formatter(&|s| {
                    s.value
                        .split_once(") ")
                        .expect("The bracket are hard coded above")
                        .1
                        .to_owned()
                })
                .prompt()?
                .split_once(')')
                .expect("The bracket are hard coded above")
                .0
                .parse::<usize>()
                .unwrap_or_default();

            log::debug!("The user selected the vault at index: {vault_idx}");

            println!(
                "{}",
                vault_manager
                    .vaults
                    .get(vault_idx - 1)
                    .expect("The index is correct")
            );
        }
        Ok(())
    }

    fn validate_args(&self) -> LprsResult<()> {
        if self.regex && self.filter.is_none() {
            return Err(LprsError::Other(
                "You cannot use the `--regex` flag if you did not use the search option".to_owned(),
            ));
        }
        if self.filter.is_some() && self.get.is_some() {
            return Err(LprsError::Other(
                "You cannot search while you want a vault with a specific index".to_owned(),
            ));
        }
        Ok(())
    }
}
