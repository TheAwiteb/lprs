use serde::{Deserialize, Serialize};

use super::{Password, Passwords};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitWardenLoginData {
    pub username: String,
    pub password: String,
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
    pub login: BitWardenLoginData,
    pub notes: Option<String>,
}

/// The bitwarden password struct
#[derive(Default, Deserialize, Serialize)]
pub struct BitWardenPasswords {
    pub encrypted: bool,
    pub folders: Vec<BitWardenFolder>,
    pub items: Vec<BitWardenPassword>,
}

impl From<Password> for BitWardenPassword {
    fn from(value: Password) -> Self {
        Self {
            ty: 1,
            name: value.name,
            login: BitWardenLoginData {
                username: value.username,
                password: value.password,
                uris: value
                    .service
                    .map(|s| vec![BitWardenUri { mt: None, uri: s }]),
            },
            notes: value.note,
        }
    }
}

impl From<Passwords> for BitWardenPasswords {
    fn from(value: Passwords) -> Self {
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
