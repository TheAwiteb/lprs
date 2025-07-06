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

// This file is not important, it is just a struct that is used to serialize and
// deserialize the vaults from and to the BitWarden format.
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::{Vault, Vaults, cipher::TotpHash};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitWardenLoginData {
    pub username: Option<String>,
    pub password: Option<String>,
    pub uris:     Option<Vec<BitWardenUri>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitWardenUri {
    #[serde(rename = "match")]
    pub mt:  Option<i32>,
    pub uri: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct BitWardenFolder {
    pub id:   String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitWardenNameValue {
    pub name:  String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitWardenPassword {
    #[serde(rename = "type")]
    pub ty:     i32,
    pub name:   String,
    pub login:  Option<BitWardenLoginData>,
    pub notes:  Option<String>,
    #[serde(default)]
    pub fields: Vec<BitWardenNameValue>,
}

/// The bitwarden password struct
#[derive(Default, Deserialize, Serialize)]
pub struct BitWardenPasswords {
    pub encrypted: bool,
    pub folders:   Vec<BitWardenFolder>,
    pub items:     Vec<BitWardenPassword>,
}

impl From<BitWardenPassword> for Vault {
    fn from(value: BitWardenPassword) -> Self {
        Self::new(
            value.name,
            value.login.as_ref().and_then(|l| l.username.as_ref()),
            value.login.as_ref().and_then(|l| l.password.as_ref()),
            value.login.as_ref().and_then(|l| {
                l.uris
                    .as_ref()
                    .and_then(|p| p.first().map(|u| u.uri.clone()))
            }),
            value.notes,
            value
                .fields
                .into_iter()
                .map(|nv| (nv.name, nv.value))
                .collect(),
            None::<String>,
            TotpHash::default(),
        )
    }
}

impl From<Vault> for BitWardenPassword {
    fn from(value: Vault) -> Self {
        Self {
            ty:     1,
            name:   value.name,
            login:  Some(BitWardenLoginData {
                username: value.username,
                password: value.password,
                uris:     value
                    .service
                    .map(|s| vec![BitWardenUri { mt: None, uri: s }]),
            }),
            notes:  value.note,
            fields: value
                .custom_fields
                .into_iter()
                .map(|(name, value)| BitWardenNameValue { name, value })
                .collect(),
        }
    }
}

impl From<Vaults> for BitWardenPasswords {
    fn from(value: Vaults) -> Self {
        Self {
            encrypted: false,
            folders:   Vec::new(),
            items:     value
                .vaults
                .into_iter()
                .map(BitWardenPassword::from)
                .collect(),
        }
    }
}
