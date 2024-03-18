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

use crate::{LprsError, LprsResult};

/// Returns the local project dir joined with the given file name
pub fn local_project_file(filename: &str) -> LprsResult<PathBuf> {
    let local_dir = directories::ProjectDirs::from("", "", "lprs")
        .map(|d| d.data_local_dir().to_path_buf())
        .ok_or_else(|| {
            LprsError::ProjectDir("Can't extract the project_dir from this OS".to_owned())
        })?;
    if !local_dir.exists() {
        fs::create_dir_all(&local_dir)?;
    }
    Ok(local_dir.join(filename))
}

/// Returns the default vaults json file
pub fn vaults_file() -> LprsResult<PathBuf> {
    let vaults_file = local_project_file(crate::DEFAULT_VAULTS_FILE)?;
    if !vaults_file.exists() {
        fs::write(&vaults_file, "[]")?;
    }
    Ok(vaults_file)
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
