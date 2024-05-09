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

use crate::{LprsError, LprsResult};

/// Parse the key & value arguments.
/// ## Errors
/// - If the argument value syntax not `key=value`
pub fn kv_parser(value: &str) -> LprsResult<(String, String)> {
    if let Some((key, value)) = value.split_once('=') {
        Ok((key.trim().to_owned(), value.trim().to_owned()))
    } else {
        Err(LprsError::ArgParse(
            "There is no value, the syntax is `KEY=VALUE`".to_owned(),
        ))
    }
}
