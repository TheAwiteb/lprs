use clap::Args;
use comfy_table::Table;
use regex::Regex;

use crate::{password::Passwords, PassrsError, PassrsResult, RunCommand};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct List {
    /// Show the clean password
    #[arg(short = 'p', long)]
    unhide_password: bool,
    /// Show the service of the password and search in it if you search
    #[arg(short = 's', long)]
    with_service: bool,
    /// Show the note of the password and search in it if you search
    #[arg(short = 'n', long)]
    with_note: bool,

    /// Return the password with spesifc index
    #[arg(short, long, value_name = "INDEX")]
    get: Option<usize>,
    /// Search and display only matching passwords.
    ///
    /// The name and username will be searched. And service and note if included
    #[arg(short = 'e', long, value_name = "TEXT")]
    search: Option<String>,
    /// Enable regex in the search
    #[arg(short, long)]
    regex: bool,
}

impl RunCommand for List {
    fn run(&self, password_manager: Passwords) -> PassrsResult<()> {
        if password_manager.passwords.is_empty() {
            println!("Looks like there is no passwords to list")
        } else {
            if self.get.is_some() && self.search.is_some() {
                return Err(PassrsError::ArgsConflict(
                    "You cannot use `--get` arg with `--search` arg".to_owned(),
                ));
            }
            if self.regex && self.search.is_none() {
                return Err(PassrsError::ArgsConflict(
                    "You cannot use `--regex` without `--search` arg".to_owned(),
                ));
            }

            let mut table = Table::new();
            let mut header = vec!["Index", "Name", "Username", "Password"];
            if self.with_service {
                header.push("Service");
            }
            if self.with_note {
                header.push("Note");
            }
            let re = Regex::new(self.search.as_deref().unwrap_or("."))?;

            table.set_header(header);
            let passwords = password_manager
                .passwords
                .iter()
                .enumerate()
                .filter(|(idx, pass)| {
                    if let Some(index) = self.get {
                        return (idx + 1) == index;
                    }
                    if let Some(ref pattern) = self.search {
                        if self.regex {
                            return re.is_match(&pass.name)
                                || re.is_match(&pass.username)
                                || (self.with_service
                                    && pass.service.as_ref().is_some_and(|s| re.is_match(s)))
                                || (self.with_note
                                    && pass.note.as_ref().is_some_and(|n| re.is_match(n)));
                        } else {
                            let pattern = pattern.to_lowercase();
                            return pass.name.to_lowercase().contains(&pattern)
                                || pass.username.to_lowercase().contains(&pattern)
                                || (self.with_service
                                    && pass
                                        .service
                                        .as_ref()
                                        .is_some_and(|s| s.to_lowercase().contains(&pattern)))
                                || (self.with_note
                                    && pass
                                        .note
                                        .as_ref()
                                        .is_some_and(|n| n.to_lowercase().contains(&pattern)));
                        }
                    }

                    true
                });
            for (idx, password) in passwords {
                let hide_password = "*".repeat(password.password.chars().count());
                let idx = (idx + 1).to_string();
                let mut row = vec![
                    idx.as_str(),
                    password.name.as_str(),
                    password.username.as_str(),
                    if self.unhide_password {
                        password.password.as_str()
                    } else {
                        hide_password.as_str()
                    },
                ];
                if self.with_service {
                    row.push(password.service.as_deref().unwrap_or("Not Set"))
                }
                if self.with_note {
                    row.push(password.note.as_deref().unwrap_or("Not Set"))
                }
                table.add_row(row);
            }
            println!("{table}");
        }
        Ok(())
    }
}
