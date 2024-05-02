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

use std::{fs, path::PathBuf};

use inquire::validator::Validation;
use sha2::Digest;

use crate::{LprsError, LprsResult};

/// Returns the local project dir joined with the given file name
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

/// Returns the default vaults json file
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
pub fn password_validator(password: &str) -> Result<Validation, inquire::CustomUserError> {
    let analyzed = passwords::analyzer::analyze(password);
    if analyzed.length() < 15 {
        return Ok(Validation::Invalid(
            "The master password length must be beggier then 15".into(),
        ));
    } else if passwords::scorer::score(&analyzed) < 80.0 {
        return Ok(Validation::Invalid(
            "Your master password is not stronge enough".into(),
        ));
    }
    Ok(Validation::Valid)
}

/// Ask the user for the master password, then returns it
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
    .with_display_mode(inquire::PasswordDisplayMode::Masked)
    .prompt()
    .map(|p| sha2::Sha256::digest(p).into())
    .map_err(Into::into)
}

/// Retuns the current lprs version from `crates.io`
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
        if let Ok(Ok(response)) = reqwest::blocking::Client::new()
            .get("https://crates.io/api/v1/crates/lprs")
            .header(
                "User-Agent",
                format!("Lprs <{current_time}> (https://git.4rs.nl/awiteb/lprs)"),
            )
            .send()
            .map(|r| r.text())
        {
            let re =
                regex::Regex::new(r#""max_stable_version":"(?<version>\d+\.\d+\.\d+)""#).unwrap();
            if let Some(cap) = re.captures(&response) {
                return Ok(cap.name("version").map(|m| m.as_str().to_string()));
            }
        }
    }
    Ok(None)
}
