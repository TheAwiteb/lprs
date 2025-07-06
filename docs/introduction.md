# Lprs Documentation

Welcome to the Lprs documentation. This documentation is intended to help you
get started with the Lprs command line tool `lprs`, and to provide you with a
reference for the various commands and options that are available.

Lprs is a vault manager that allows you to store and retrieve secrets from a
vault. It is designed to be simple to use, and to provide a secure way to manage
your secrets.

Lprs is not for human use only, it is also designed to be used in scripts as
well.

## Encryption
In Lprs, we use the AES-256 CBC encryption algorithm to encrypt and decrypt the
vaults, we ask you for master password, which is used to encrypt and decrypt the
vaults, the master password will be hashed using SHA-256 and the hash will be
used as the key for the AES-256 CBC encryption algorithm. Also we don't store
the master password anywhere (even the hash of it), so if you forget it you will
lose all your vaults.

## Storage
The vaults are stored in the program's directory, in a directory called `lprs`
and the file called `vaults.lprs`.

### File location
The file location is dependent on the operating system you are using, here are
the locations for the different operating systems:

| OS     | Location                                      |
|--------|-----------------------------------------------|
| Linux  | `$HOME/.local/share/lprs/vaults.lprs`         |
| MacOS  | `$HOME/Library/Application Support/lprs/vaults.lprs` |
| Windows| `{FOLDERID_LocalAppData}\lprs\vaults.lprs`    |


### File format
The list of vaults is stored in encrypted binary format.

## Features
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
