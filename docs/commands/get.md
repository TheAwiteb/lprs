# Getting a vault

## Usage

```
Usage: lprs get <INDEX-or-NAME> [FIELD]

Arguments:
  <INDEX-or-NAME>
          Whether the index of the vault or its name

  [FIELD]
          A Specific field to get.
          
          Can be [name, username, password, service, note, totp_secret, totp_code, "string"]
          
          where the string means a custom field

Options:
  -h, --help
          Print help (see a summary with '-h')
```

Get a single field from a vault, if the field is not provided, the whole vault
will be printed. If the field is a custom field, you need to provide it as a
string.

Also, if the vault you specified does not contained the field you provided, an
error will be returned.


### Examples
Get the whole vault by its index:
```sh
lprs get 1
```

Get the whole vault by its name:
```sh
lprs get my-vault
```

Get a specific field from a vault by its name:
```sh
lprs get my-vault password
```

Get a custom field from a vault by its name:
```sh
lprs get matrix_home_server "host"
```

## Notes
- The index is one-based (the first vault is 1).
