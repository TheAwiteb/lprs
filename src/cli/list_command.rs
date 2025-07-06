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

use clap::Args;
use inquire::{InquireError, Select};

use crate::{
    LprsCommand,
    LprsError,
    LprsResult,
    RESERVED_FIELD_PREFIX,
    vault::{Vaults, cipher},
};

#[derive(Debug, Args)]
/// List command, used to list the vaults and search
pub struct List {
    /// Filter the select list
    #[arg(short, long, value_name = "TEXT")]
    filter: Option<String>,
    /// Enable regex when use `--filter` option
    #[arg(short, long)]
    regex:  bool,
    /// Returns the output as `json` list of vaults
    #[arg(long)]
    json:   bool,
}

impl LprsCommand for List {
    fn run(self, mut vault_manager: Vaults) -> LprsResult<()> {
        if vault_manager.vaults.is_empty() {
            return Err(LprsError::Other(
                "Looks like there is no vaults to list".to_owned(),
            ));
        }

        let pattern = if self.regex {
            self.filter
                .expect("Is required if the `regex` option is `true`")
        } else {
            format!(
                ".*{}.*",
                regex::escape(self.filter.as_deref().unwrap_or_default())
            )
        };
        log::debug!("Listing vaults filtered by: {pattern}");

        let re = regex::Regex::new(&pattern.to_lowercase())?;

        let vaults_list = vault_manager.vaults.iter().enumerate().filter(|(_, v)| {
            re.is_match(&v.name.to_lowercase())
                || v.username
                    .as_deref()
                    .is_some_and(|u| re.is_match(&u.to_lowercase()))
                || v.service
                    .as_deref()
                    .is_some_and(|s| re.is_match(&s.to_lowercase()))
                || v.note
                    .as_deref()
                    .is_some_and(|n| re.is_match(&n.to_lowercase()))
        });

        if self.json {
            print!(
                "{}",
                serde_json::to_string(&vaults_list.map(|(_, v)| v).collect::<Vec<_>>())?
            )
        } else {
            let vault_idx = Select::new(
                "Select a vault to view:",
                vaults_list
                    .map(|(idx, v)| format!("{}) {}", idx + 1, v.list_name()))
                    .collect(),
            )
            .with_formatter(&|s| {
                s.value
                    .split_once(") ")
                    .expect("The bracket are hard coded above")
                    .1
                    .to_owned()
            })
            .prompt()
            .map_err(|err| {
                if matches!(err, InquireError::InvalidConfiguration(_)) {
                    return LprsError::Other("There is no result match your filter".to_owned());
                }
                err.into()
            })?
            .split_once(')')
            .expect("The bracket are hard coded above")
            .0
            .parse::<usize>()
            .unwrap_or_default();

            log::debug!("The user selected the vault at index: {vault_idx}");

            let vault = vault_manager
                .vaults
                .get_mut(vault_idx - 1)
                .expect("The index is correct");

            if let Some(ref totp_secret) = vault.totp_secret {
                let (code, remaining) = cipher::totp_now(totp_secret, &vault.totp_hash)?;
                vault.custom_fields.insert(
                    format!("{RESERVED_FIELD_PREFIX}TOTP Code"),
                    format!("{code} ({remaining}s remaining)"),
                );
            }

            println!("{vault}",);
        }

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
