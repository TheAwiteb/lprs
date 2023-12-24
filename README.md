# Passrs

Passrs is a local password manager designed to securely store and manage your passwords.

## Installation

To install Passrs, you will need to have the Cargo package manager installed. If you do not have Cargo installed, you can install it by following the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html). Note the Minimum Supported Rust Version (MSRV) for Passrs is `1.70.0`.

1. Clone the Passrs repository:
    ```bash
    cargo install --locked --git https://github.com/theawiteb/passrs.git
    ```

2. Run Passrs:
    ```bash
    passrs --help
    ```

## Uninstallation
```bash
cargo uninstall passrs
```

## Usage

Passrs provides a command-line interface for managing your passwords. The following commands are available:

```
Local CLI password manager

Usage: passrs [OPTIONS] <COMMAND>

Commands:
  add    Add new password
  list   List your password and search
  clean  Clean the password file
  edit   Edit the password content
  help   Print this message or the help of the given subcommand(s)

Options:
  -p, --passwords-file <PASSWORDS_FILE>
          The passwords json file, default: $HOME/.local/share/passrs/passwords.json
  -h, --help
          Print help
  -V, --version
          Print version
```

<!--
### Backup

It is important to regularly backup your passwords to prevent data loss. Passrs does not provide an automatic backup feature. To backup your passwords, you can use the export command provided by Passrs. This command allows you to export your encrypted passwords to a json file, which you can then manually backup to a secure location. -->


## Contributing

Contributions to Passrs are welcome! If you would like to contribute, please follow the guidelines outlined in the [CONTRIBUTING](CONTRIBUTING.md) file.

## License

Passrs is licensed under the GPL-3.0 License. This means that you are free to use, modify, and distribute the software under the terms of this license. Please refer to the [LICENSE](LICENSE) file for more details.
