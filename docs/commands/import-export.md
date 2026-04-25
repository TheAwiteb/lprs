# Importing and exporting vaults

## Import usage

```
Usage: lprs import [OPTIONS] <PATH>

Arguments:
  <PATH>
          The file path to import from. Use `-` to import from the stdin

Options:
  -p, --decryption-password [<DECRYPTION_PASSWORD>]
          Decryption password of the imported vaults, if there is not, will use the master password

  -h, --help
          Print help (see a summary with '-h')
```

## Export usage

```
Usage: lprs export [OPTIONS] <PATH>

Arguments:
  <PATH>
          The path to export to. Use `-` to export to the stdout

Options:
  -p, --encryption-password [<ENCRYPTION_PASSWORD>]
          Encryption password of the exported vaults, if there is not, will use the master password

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

To import and export vaults you need to provide the path to the file to import
from or export to.

For the import command, you can provide the decryption password of the imported
vaults, if there is no decryption password provided, the master password will
be used.

For the export command, you can provide the encryption password, if there is no
encryption password provided, the master password will be used.

## Examples
Import vaults from a file:
```sh
lprs import /path/to/vaults.json
```

Import vaults from a file with a decryption password (You will be prompted for
the decryption password):
```sh
lprs import /path/to/vaults.json -p
```

Export vaults to a file:
```sh
lprs export /path/to/vaults.json
```

export vaults to a file with an encryption password (You will be prompted for
the encryption password):
```sh
lprs export /path/to/vaults.json -p
```

## Notes
- The imported or exported file must be a `.json` file.
- The imported vaults will be added to the current vaults.
- The imported vaults must don't have a custom field prefixed with `.lprsfield.`
  because it's reserved for backwards compatibility.
