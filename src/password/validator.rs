use std::{fs, path::Path};

use crate::PassrsResult;

use super::Password;

/// Return if the password file new file or not
pub fn is_new_password_file(path: &Path) -> PassrsResult<bool> {
    if path.exists() {
        let file_content = fs::read_to_string(path)?;
        if !file_content.is_empty()
            && file_content.trim() != "[]"
            && serde_json::from_str::<Vec<Password>>(&file_content).is_ok()
        {
            return Ok(false);
        }
    }
    Ok(true)
}
