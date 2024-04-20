use serde::{Deserialize, Serialize};

use super::{vault_state::*, Vault, Vaults};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitWardenLoginData {
    pub username: Option<String>,
    pub password: Option<String>,
    pub uris: Option<Vec<BitWardenUri>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitWardenUri {
    #[serde(rename = "match")]
    pub mt: Option<i32>,
    pub uri: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct BitWardenFolder {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitWardenPassword {
    #[serde(rename = "type")]
    pub ty: i32,
    pub name: String,
    pub login: Option<BitWardenLoginData>,
    pub notes: Option<String>,
}

/// The bitwarden password struct
#[derive(Default, Deserialize, Serialize)]
pub struct BitWardenPasswords {
    pub encrypted: bool,
    pub folders: Vec<BitWardenFolder>,
    pub items: Vec<BitWardenPassword>,
}

impl From<BitWardenPassword> for Vault<Plain> {
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
        )
    }
}

impl From<Vault<Plain>> for BitWardenPassword {
    fn from(value: Vault<Plain>) -> Self {
        Self {
            ty: 1,
            name: value.name,
            login: Some(BitWardenLoginData {
                username: value.username,
                password: value.password,
                uris: value
                    .service
                    .map(|s| vec![BitWardenUri { mt: None, uri: s }]),
            }),
            notes: value.note,
        }
    }
}

impl From<Vaults<Plain>> for BitWardenPasswords {
    fn from(value: Vaults<Plain>) -> Self {
        Self {
            encrypted: false,
            folders: Vec::new(),
            items: value
                .vaults
                .into_iter()
                .map(BitWardenPassword::from)
                .collect(),
        }
    }
}
