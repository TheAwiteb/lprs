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

use std::collections::BTreeMap;
use std::{fs, path::PathBuf};

use inquire::{validator::Validation, Password, PasswordDisplayMode};
use passwords::{analyzer, scorer};
#[cfg(feature = "update-notify")]
use reqwest::blocking::Client as BlockingClient;
use sha2::Digest;

use crate::vault::Vault;
use crate::{LprsError, LprsResult};

/// Returns the local project dir joined with the given file name
///
/// ## Errors
/// - If the project dir can't be extracted from the OS
/// - If the local project dir can't be created
pub fn local_project_file(filename: &str) -> LprsResult<PathBuf> {
    let local_dir = directories::ProjectDirs::from("", "", "lprs")
        .map(|d| d.data_local_dir().to_path_buf())
        .ok_or_else(|| {
            LprsError::ProjectDir("Can't extract the project_dir from this OS".to_owned())
        })?;
    log::debug!("Local project dir: {:?}", local_dir.display());
    if !local_dir.exists() {
        log::info!("Creating the local project dir: {:?}", local_dir.display());
        fs::create_dir_all(&local_dir)?;
    }
    Ok(local_dir.join(filename))
}

/// Returns the user secret if any
///
/// - If the `secret` is `None` will return `None`
/// - If the `secret` is `Some(None)` will ask the user for a secret in the
///   stdin and return it
/// - If the `secret` is `Some(Some(secret))` will return `Some(secret)`
///
/// ## Errors
/// - When failed to get the secret from stdin
pub fn user_secret(
    secret: Option<Option<String>>,
    prompt_message: &str,
    confirmation: bool,
) -> LprsResult<Option<String>> {
    Ok(match secret {
        None => None,
        Some(Some(p)) => Some(p),
        Some(None) => {
            log::debug!("User didn't provide a secret, prompting it");
            Some(
                Password {
                    enable_confirmation: confirmation,
                    ..Password::new(prompt_message)
                        .with_formatter(&|p| "*".repeat(p.chars().count()))
                        .with_display_mode(PasswordDisplayMode::Masked)
                }
                .prompt()?,
            )
        }
    })
}

/// Returns the default vaults json file
///
/// ## Errors
/// - If the project dir can't be extracted from the OS
/// - If the vaults file can't be created
pub fn vaults_file() -> LprsResult<PathBuf> {
    let vaults_file = local_project_file(crate::DEFAULT_VAULTS_FILE)?;
    if !vaults_file.exists() {
        fs::File::create(&vaults_file)?;
    }
    Ok(vaults_file)
}

/// Validate the password
///
/// ## To pass
/// - The length must be higher than 14 (>=15)
/// - Its score must be greater than 80.0
///
/// ## Errors
/// - There is no errors, just the return type of inquire validator must be
///   Result<Validation, inquire::CustomUserError>
pub fn password_validator(password: &str) -> Result<Validation, inquire::CustomUserError> {
    let analyzed = analyzer::analyze(password);
    Ok(if analyzed.length() < 15 {
        Validation::Invalid("The master password length must be beggier then 15".into())
    } else if scorer::score(&analyzed) < 80.0 {
        Validation::Invalid("Your master password is not stronge enough".into())
    } else {
        Validation::Valid
    })
}

/// Ask the user for the master password, then returns it
///
/// ## Errors
/// - Can't read the password from the user
///
/// Return's the password as 32 bytes after hash it (256 bit)
pub fn master_password_prompt(is_new_vaults_file: bool) -> LprsResult<[u8; 32]> {
    inquire::Password {
        message: "Master Password:",
        enable_confirmation: is_new_vaults_file,
        validators: if is_new_vaults_file {
            vec![Box::new(password_validator)]
        } else {
            vec![]
        },
        ..inquire::Password::new("")
    }
    .with_formatter(&|p| "*".repeat(p.chars().count()))
    .with_display_mode(PasswordDisplayMode::Masked)
    .prompt()
    .map(|p| sha2::Sha256::digest(p).into())
    .map_err(Into::into)
}

/// Retuns the current lprs version from `crates.io`
///
/// ## Errors
/// - The project dir can't be extracted from the OS
/// - If the last version check file can't be created
#[cfg(feature = "update-notify")]
pub fn lprs_version() -> LprsResult<Option<String>> {
    use std::time::SystemTime;

    let last_version_check_file = local_project_file(crate::LAST_VERSION_CHECK_FILE)?;
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| LprsError::Other("The system time is before UNIX EPOCH!".to_owned()))?
        .as_secs();
    let last_check: u64 = fs::read_to_string(&last_version_check_file)
        .unwrap_or_else(|_| current_time.to_string())
        .parse()
        .map_err(|err| {
            LprsError::Other(format!(
                "Check update file content is invalid time `{}`: {err}",
                last_version_check_file.display()
            ))
        })?;
    fs::write(last_version_check_file, current_time.to_string())?;

    // Check if the last check is before one hour or not
    if (current_time - last_check) >= (60 * 60) || current_time == last_check {
        if let Ok(Ok(response)) = BlockingClient::new()
            .get("https://crates.io/api/v1/crates/lprs")
            .header(
                "User-Agent",
                format!("Lprs <{current_time}> (https://git.4rs.nl/awiteb/lprs)"),
            )
            .send()
            .map(|r| r.text())
        {
            let re = regex::Regex::new(r#""max_stable_version":"(?<version>\d+\.\d+\.\d+)""#)
                .expect("The regex is correct");
            if let Some(cap) = re.captures(&response) {
                return Ok(cap.name("version").map(|m| m.as_str().to_string()));
            }
        }
    }
    Ok(None)
}

/// Returns the duplicated field from the custom field (unprocessed fields)
pub fn get_duplicated_field(fields: &[(String, Option<String>)]) -> Option<&str> {
    fields.iter().find_map(|(key, _)| {
        if fields.iter().filter(|(k, _)| key == k).count() > 1 {
            return Some(key.as_str());
        }
        None
    })
}

/// Apply the edited fields to the vault fields.
/// This will:
/// - Add the field if it's not in the fields map.
/// - Update the field if it's in the map.
/// - Remove the field if its value is empty string.
pub fn apply_custom_fields(
    fields: &mut BTreeMap<String, String>,
    edited_fields: Vec<(String, String)>,
) {
    for (key, value) in edited_fields {
        if fields.contains_key(&key) && value.is_empty() {
            fields.remove(&key);
        } else {
            // The field is not there or its value not empty,
            // so add it or update its value
            fields.insert(key, value);
        }
    }
}

/// Make sure all custom field values are there, if not, ask the user for it
///
/// ## Errors
/// - If can't read the user input
pub fn prompt_custom(
    custom_fields: Vec<(String, Option<String>)>,
) -> LprsResult<Vec<(String, String)>> {
    let mut new_fields = Vec::new();

    for (key, value) in custom_fields {
        if let Some(value) = value {
            new_fields.push((key, value));
        } else {
            let value = inquire::Text::new(&format!("Value of `{key}`:")).prompt()?;
            new_fields.push((key, value));
        }
    }

    Ok(new_fields)
}

/// Returns the vault with its index by its index or name
///
/// ## Errors
/// - If there is no vault with the given index or name
pub fn vault_by_index_or_name<'a>(
    index_or_name: &str,
    vaults: &'a mut [Vault],
) -> LprsResult<(usize, &'a mut Vault)> {
    let parsed_index = index_or_name.parse::<usize>();

    let Some((index, vault)) = (if let Ok(index) = parsed_index {
        index
            .checked_sub(1)
            .and_then(|zeroindex| vaults.get_mut(zeroindex).map(|v| (index, v)))
    } else {
        vaults
            .iter_mut()
            .enumerate()
            .find(|(_, v)| v.name == index_or_name)
    }) else {
        return Err(LprsError::Other(format!(
            "There is no vault with the given {} `{}`",
            if parsed_index.is_ok() {
                "index"
            } else {
                "name"
            },
            index_or_name,
        )));
    };
    Ok((index, vault))
}
