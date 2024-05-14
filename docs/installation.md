# Installation

Lprs can be installed using the pre-built binaries or by building from source
via `cargo-install` or `cargo build`, by default the lprs will check for
updates, if you don't want this behavior, you can disable it by passing the
`--no-default-features` flag to the `cargo-install` or `cargo build` command.
And for pre-built binaries, you will find a binary without `lprs-update-notify`,
this binary will not check for updates.

- [Pre-built binaries](https://git.4rs.nl/awiteb/lprs/releases/latest)
- Install using `cargo-install`: `cargo install lprs`
- Building from source, clone the repository and run `cargo build --release` and
  copy the binary from the `target/release` directory.