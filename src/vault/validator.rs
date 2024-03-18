// Lprs - A local CLI password manager
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

use std::{fs, path::Path};

use crate::LprsResult;

use super::{vault_state::*, Vault};

/// Return if the vaults file new file or not
pub fn is_new_vaults_file(path: &Path) -> LprsResult<bool> {
    if path.exists() {
        let file_content = fs::read_to_string(path)?;
        if !file_content.is_empty()
            && file_content.trim() != "[]"
            && serde_json::from_str::<Vec<Vault<Encrypted>>>(&file_content).is_ok()
        {
            return Ok(false);
        }
    }
    Ok(true)
}
