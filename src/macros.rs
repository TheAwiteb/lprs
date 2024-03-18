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

/// Creates commands macro, to create the `Commands` enum and impl `RunCommand` to it.
///
/// ### Notes:
/// The `$command` must impl `RunCommand` trait
///
/// ### Example:
/// ```rust
/// create_commands!(
///     enum TestCommands
///     "Test command", Test => TestArgs
///     "Do something", Some => SomeArgs
/// );
/// ```
/// #### Output
/// ```rust
/// ///The lprs commands
/// pub enum TestCommands {
///     ///Test command
///     Test(TestArgs),
///     ///Do something
///     Some(SomeArgs),
/// }
///
/// impl crate::RunCommand for TestCommands {
///     fn run(
///         &self,
///         vault_manager: crate::vault::Vaults,
///     ) -> crate::LprsResult<()> {
///         match self {
///             Self::Test(command) => command.run(password_manager),
///             Self::Some(command) => command.run(password_manager),
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! create_commands {
    (enum $enum_name: ident $($doc:tt, $varint: ident => $command: ty)+) => {
        #[doc = "The lprs commands"]
        #[derive(Debug, clap::Subcommand)]
        pub enum $enum_name {
            $(
                #[doc = $doc]
                $varint($command),
            )+
        }

        #[automatically_derived]
        impl $crate::RunCommand for $enum_name{
            fn run(&self, password_manager: $crate::vault::Vaults<$crate::vault::vault_state::Plain>) -> $crate::LprsResult<()> {
                match self {
                    $(
                        Self::$varint(command) => command.run(password_manager),
                    )+
                }
            }
        }
    };
}
