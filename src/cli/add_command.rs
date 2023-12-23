use clap::Args;

use crate::{
    password::{Password, Passwords},
    PassrsResult, RunCommand,
};

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct Add {
    #[command(flatten)]
    password_info: Password,
}

impl RunCommand for Add {
    fn run(&self, mut password_manager: Passwords) -> PassrsResult<()> {
        password_manager.add_password(self.password_info.clone());
        password_manager.try_export()
    }
}
