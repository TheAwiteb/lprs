# Importing and exporting vaults

## Import usage

```
Usage: lprs import [OPTIONS] <PATH>

Arguments:
  <PATH>
          The file path to import from

Options:
  -f, --format <FORMAT>
          The format to import from
          
          [default: lprs]

          Possible values:
          - lprs:       The lprs format, which is the default format and is is the result of the serialization/deserialization of the Vaults struct
          - bit-warden: The BitWarden format, which is the result of the serialization/deserialization of the BitWardenPasswords struct

  -p, --decryption-password [<DECRYPTION_PASSWORD>]
          Decryption password of the imported vaults (in `lprs` format) if there is not, will use the master password

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Export usage

```
Usage: lprs export [OPTIONS] <PATH>

Arguments:
  <PATH>
          The path to export to

Options:
  -f, --format <FORMAT>
          Format to export vaults in
          
          [default: lprs]

          Possible values:
          - lprs:       The lprs format, which is the default format and is is the result of the serialization/deserialization of the Vaults struct
          - bit-warden: The BitWarden format, which is the result of the serialization/deserialization of the BitWardenPasswords struct

  -p, --encryption-password [<ENCRYPTION_PASSWORD>]
          Encryption password of the exported vaults (in `lprs` format) if there is not, will use the master password

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

To import and export vaults you need to provide the path to the file to import
from or export to, and you can provide the format to import from or export to,
the format can be `lprs` or `bit-warden`, and the imported or exported file must
be `.json` file.

For the import command, you can provide the decryption password of the imported
vaults if they are exported in the `lprs` format, if there is no decryption
password provided, the master password will be used.

For the export command, you can provide the encryption password of the exported
vaults in the `lprs` format, if there is no encryption password provided, the
master password will be used.

## Examples
Import vaults from a file in the `lprs` format:
```sh
lprs import /path/to/vaults.json
```

Import vaults from a file in the `lprs` format with a decryption password (You
will be prompted for the decryption password):
```sh
lprs import /path/to/vaults.json -p
```

Import vaults from a file in the `bit-warden` format:
```sh
lprs import /path/to/vaults.json -f bit-warden
```

Export vaults to a file in the `lprs` format (is the default format):
```sh
lprs export /path/to/vaults.json
```

export vaults to a file in the `lprs` format with an encryption password (You
will be prompted for the encryption password):
```sh
lprs export /path/to/vaults.json -p
```

Export vaults to a file in the `bit-warden` format:
```sh
lprs export /path/to/vaults.json -f bit-warden
```

## Notes
- The imported or exported file must be a `.json` file.
- The imported vaults will be added to the current vaults.
- The imported vaults must don't have a custom field prefixed with `.lprsfield.`
  because it's reserved for backwards compatibility.
