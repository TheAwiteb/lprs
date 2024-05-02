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

/// Impl `LprsCommand` to the given subcommand.
///
/// ### Notes:
/// The `$command` must impl `LprsCommand` trait
///
/// ### Example:
/// ```rust
/// create_commands!(
///     TestCommands,
///     Test Some
/// );
/// ```
/// #### Output
/// ```rust
/// impl crate::LprsCommand for TestCommands {
///     fn run(
///         &self,
///         vault_manager: crate::vault::Vaults,
///     ) -> crate::LprsResult<()> {
///         match self {
///             Self::Test(command) => command.run(vault_manager),
///             Self::Some(command) => command.run(vault_manager),
///         }
///     }
///
///     fn validate_args(&self) -> crate::LprsResult<()> {
///         match self {
///             Self::Test(command) => command.validate_args(),
///             Self::Some(command) => command.validate_args(),
///         }
///     }
/// }

/// ```
#[macro_export]
macro_rules! impl_commands {
    ($enum_name: ident, $($varint: ident)+) => {
        #[automatically_derived]
        impl $crate::LprsCommand for $enum_name{
            fn run(self, vault_manager: $crate::vault::Vaults) -> $crate::LprsResult<()> {
                match self {
                    $(
                        Self::$varint(command) => command.run(vault_manager),
                    )+
                }
            }

            fn validate_args(&self) -> LprsResult<()> {
                match self {
                    $(
                        Self::$varint(command) => command.validate_args(),
                    )+
                }
            }
        }
    };
}
