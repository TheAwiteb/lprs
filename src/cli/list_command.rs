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

use clap::Args;
use inquire::Select;

use crate::{vault::Vaults, LprsCommand, LprsError, LprsResult};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
/// List command, used to list the vaults and search
pub struct List {
    /// Filter the select list
    #[arg(short, long, value_name = "TEXT")]
    filter: Option<String>,
    /// Enable regex when use `--filter` option
    #[arg(short, long)]
    regex:  bool,
}

impl LprsCommand for List {
    fn run(self, vault_manager: Vaults) -> LprsResult<()> {
        if vault_manager.vaults.is_empty() {
            return Err(LprsError::Other(
                "Looks like there is no vaults to list".to_owned(),
            ));
        }

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

        Ok(())
    }

    fn validate_args(&self) -> LprsResult<()> {
        if self.regex && self.filter.is_none() {
            return Err(LprsError::Other(
                "You cannot use the `--regex` flag if you did not use the search option".to_owned(),
            ));
        }
        Ok(())
    }
}
