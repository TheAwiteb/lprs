use std::{fs, path::PathBuf};

use crate::{PassrsError, PassrsResult};

/// Return the default passwords json file
pub fn passwords_file() -> PassrsResult<PathBuf> {
    if let Some(path) = directories::ProjectDirs::from("", "", "passrs")
        .map(|d| d.data_local_dir().to_path_buf().join("passwords.json"))
    {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if !path.exists() {
            fs::write(&path, "[]")?;
        }
        Ok(path)
    } else {
        Err(PassrsError::ProjectDir(
            "Can't extract the project_dir from this OS".to_owned(),
        ))
    }
}
