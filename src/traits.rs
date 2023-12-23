use crate::{password::Passwords, PassrsResult};

/// Trait to run the command
pub trait RunCommand {
    fn run(&self, password_manager: Passwords) -> PassrsResult<()>;
}
