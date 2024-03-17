# Lprs

Lprs is a local password manager designed to securely store and manage your passwords.

### MSRV
The Minimum Supported Rust Version (MSRV) is `1.70.0`.

## Installation

To install Lprs, you will need to have the Cargo package manager installed. If you do not have Cargo installed, you can install it by following the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).


1. Install using [cargo-install](https://doc.rust-lang.org/cargo/commands/cargo-install.html):
```bash
cargo install lprs --locked
```
This will enable the update notifications for Lprs. If you don't want to enable update notifications, you can install Lprs using:
```bash
cargo install lprs --locked --no-default-features
```

2. Run Lprs:
```bash
lprs --help
```

## Uninstallation
```bash
cargo uninstall lprs
```

## Usage

Lprs provides a command-line interface for managing your passwords. The following commands are available:

```
A local CLI password manager

Usage: lprs [OPTIONS] <COMMAND>

Commands:
  add     Add new password
  remove  Remove password
  list    List your password and search
  clean   Clean the password file
  edit    Edit the password content
  gen     Generate password
  export  Export the passwords
  import  Import passwords
  help    Print this message or the help of the given subcommand(s)

Options:
  -p, --passwords-file <PASSWORDS_FILE>
          The passwords json file, default: $HOME/.local/share/lprs/passwords.json
  -h, --help
          Print help
  -V, --version
          Print version
```

### Example
```bash
lprs add -n "Gmail" -u "some@gmail.com" -p $(lprs gen 19 -u -l -s) -s "https://mail.google.com"
```

#### Result
This is the result when search for it
```
$ lprs list -e "mail" -p -s
Master Password: ***************
+-------+-------+----------------+---------------------+-------------------------+
| Index | Name  | Username       | Password            | Service                 |
+================================================================================+
| 31    | Gmail | some@gmail.com | >NC`q$%+Nno<y&<y]VB | https://mail.google.com |
+-------+-------+----------------+---------------------+-------------------------+
```


### Backup

It is important to regularly backup your passwords to prevent data loss. Lprs does not provide an automatic backup feature. To backup your passwords, you can use the `export` command provided by Lprs. This command allows you to export your encrypted passwords to a json file, which you can then manually backup to a secure location.

#### Formats
The format of the exported file can be specified using the `--format` option. The following formats are supported:

-  `lprs`: The default format used by Lprs. This format is encrypted and can be imported back into Lprs using the `import` command. This is the recommended format to use for backups as it is encrypted and can be imported back into Lprs.
- `bit-warden`: The format used by [Bitwarden](https://bitwarden.com/). This format is not encrypted and can be imported into Bitwarden. This format is useful if you want to switch to Bitwarden or another password manager that supports this format.



## Contributing

Contributions to Lprs are welcome! If you would like to contribute, please follow the guidelines outlined in the [CONTRIBUTING]
(CONTRIBUTING.md) file.

## Mirrors
This repository is mirrored on the following platforms:
- [GitHub](https://github.com/TheAwiteb/lprs)
- [Codeberg](https://codeberg.org/awiteb/lprs)

## License

Lprs is licensed under the GPL-3.0 License. This means that you are free to use, modify, and distribute the software under the terms of this license. Please refer to the [LICENSE](LICENSE) file for more details.

---

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/awiteb)