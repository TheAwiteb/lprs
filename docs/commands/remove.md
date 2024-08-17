# Removing a vault

## Usage

```
Usage: lprs remove [OPTIONS] <INDEX-or-NAME>

Arguments:
  <INDEX-or-NAME>  The vault to remove, index or name

Options:
  -f, --force    Force remove, will not return error if there is no vault with the given index or name
  -h, --help     Print help
```

To remove a vault you need to provide the index or the name of the vault. If you
provide the index, the vault will be removed by its index, if you provide the
name, the vault will be removed the first vault with the given name.

If there is no vault with the given index or name, an error will be returned,
unless you provide the `--force` option, in which case the command will not
return an error if there is no vault with the given index or name.

## Examples
Remove a vault by its index:
```sh
lprs remove 1
```

Remove a vault by its name:
```sh
lprs remove my-vault
```

Force remove a vault by its index:
```sh
lprs remove 234 --force
```

## Notes
- The index is one-based (the first vault is 1).
