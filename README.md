
<div align="center">

# Lprs
A local vault manager designed to securely store and manage your vaults.

[![Docs](https://img.shields.io/badge/docs-lprs.4rs.nl-purple)](https://lprs.4rs.nl)
[![Crates.io](https://img.shields.io/crates/v/lprs?color=orange)](https://crates.io/crates/lprs)
[![Forgejo CI Status](https://git.4rs.nl/awiteb/lprs/badges/workflows/ci.yml/badge.svg)](https://git.4rs.nl/awiteb/lprs)
[![Forgejo CD Status](https://git.4rs.nl/awiteb/lprs/badges/workflows/cd.yml/badge.svg)](https://git.4rs.nl/awiteb/lprs)

</div>

## Features
- Auto checks for updates (Can be disabled).
- Passing the master password as an argument and via stdin.
- Changing the master password.
- Generating a passwords.
- Store username, password, service name and notes in a vault.
- Custom fields, you can store any key-value pair in a vault.
- TOTP (Time-based One-Time Password) generation. Which can be used to generate
  2FA codes.
- Searching for vaults. And list all vaults in json format.
- Importing and exporting encrypted vaults (in json format).
- Importing and exporting from/to Bitwarden json format. (Unencrypted)
- Editing vaults. (The secrets can be passed as arguments or via stdin)
- Deleting vaults.
- Getting single field from a vault. (Useful for scripts)
- Ability to edit, get and remove a vault using its index or name.
- Auto completion for bash, elvish, fish, powershell and zsh
- Ability to import and export vaults with different master passwords. (Useful
  for sharing vaults with others)

## Installation

### Build from source (MSRV: `1.74.0`)
```bash
# From crates.io
cargo install lprs 
# From source (after cloning the repository)
# The binary will be in target/release/lprs
cargo build --release
```

This will build Lprs with update checking enabled. If you want to disable update checking, you can build Lprs without the default features by passing the `--no-default-features` flag.

### Pre-built binaries
Pre-built binaries are available for Linux only, you can download them from the [releases page](https://git.4rs.nl/awiteb/lprs/releases/latest).

## Usage

Lprs provides a command-line interface for managing your vaults. The following commands are available:

```
A local CLI vaults manager. For human and machine use

Usage: lprs [OPTIONS] <COMMAND>

Commands:
  add                     Add new vault
  remove                  Remove vault [alias `rm`]
  list                    List your vaults and search [alias `ls`]
  clean                   Clean the vaults file
  edit                    Edit the vault content
  gen                     Generate a password
  get                     Get a entire vault or single field from it
  export                  Export the vaults
  import                  Import vaults
  change-master-password  Change the master password
  completion              Generate shell completion
  help                    Print this message or the help of the given subcommand(s)

Options:
  -f, --vaults-file <VAULTS_FILE>          The vaults json file
  -v, --verbose                            Show the logs in the stdout
  -m, --master-password <MASTER_PASSWORD>  The master password, or you will prompt it
  -h, --help                               Print help
  -V, --version                            Print version
```

## Documentation
You can find the full documentation for Lprs on the official website at https://lprs.4rs.nl.

## Contributing

Contributions to Lprs are welcome! If you would like to contribute, please follow the guidelines outlined in the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Mirrors
This repository is mirrored on the following platforms:
- [GitHub](https://github.com/TheAwiteb/lprs)
- [Codeberg](https://codeberg.org/awiteb/lprs)

## License

Lprs is licensed under the GPL-3.0 License. This means that you are free to use, modify, and distribute the software under the terms of this license. Please refer to the [LICENSE](LICENSE) file for more details.

---

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/awiteb)
