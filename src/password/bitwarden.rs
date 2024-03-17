use serde::{Deserialize, Serialize};

use super::{Vault, Vaults};

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

impl From<BitWardenPassword> for Vault {
    fn from(value: BitWardenPassword) -> Self {
        Self {
            name: value.name,
            username: value
                .login
                .as_ref()
                .map_or_else(String::new, |l| l.username.to_owned().unwrap_or_default()),
            password: value
                .login
                .as_ref()
                .map_or_else(String::new, |l| l.password.to_owned().unwrap_or_default()),
            service: value
                .login
                .and_then(|l| l.uris.and_then(|p| p.first().map(|u| u.uri.clone()))),
            note: value.notes,
        }
    }
}

impl From<Vault> for BitWardenPassword {
    fn from(value: Vault) -> Self {
        Self {
            ty: 1,
            name: value.name,
            login: Some(BitWardenLoginData {
                username: Some(value.username),
                password: Some(value.password),
                uris: value
                    .service
                    .map(|s| vec![BitWardenUri { mt: None, uri: s }]),
            }),
            notes: value.note,
        }
    }
}

impl From<Vaults> for BitWardenPasswords {
    fn from(value: Vaults) -> Self {
        Self {
            encrypted: false,
            folders: Vec::new(),
            items: value
                .passwords
                .into_iter()
                .map(BitWardenPassword::from)
                .collect(),
        }
    }
}
