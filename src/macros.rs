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
/// ///The passrs commands
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
///         password_manager: crate::password::Passwords,
///     ) -> crate::PassrsResult<()> {
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
        #[doc = "The passrs commands"]
        #[derive(Debug, clap::Subcommand)]
        pub enum $enum_name {
            $(
                #[doc = $doc]
                $varint($command),
            )+
        }

        #[automatically_derived]
        impl $crate::RunCommand for $enum_name{
            fn run(&self, password_manager: $crate::password::Passwords) -> $crate::PassrsResult<()> {
                match self {
                    $(
                        Self::$varint(command) => command.run(password_manager),
                    )+
                }
            }
        }
    };
}
