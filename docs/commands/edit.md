# Editing a vault

## Usage

```
Usage: lprs edit [OPTIONS] <INDEX-or-NAME>

Arguments:
  <INDEX-or-NAME>
          The vault to edit, index or name

Options:
  -n, --name <NAME>
          The new vault name

  -u, --username <USERNAME>
          The new vault username, make it empty string to delete it

  -p, --password [<PASSWORD>]
          The new password, make it empty string to delete it
          
          If there is no value for it you will prompt it

  -s, --service <SERVICE>
          The new vault service, make it empty string to delete it

  -o, --note <NOTE>
          The new vault note

  -t, --totp-secret [<TOTP_SECRET>]
          The TOTP secret, make it empty string to delete it
          
          If there is no value you will prompt it

  -c, --custom <KEY=VALUE>
          The custom field, make it empty string to delete it
          
          If the custom field not exist will created it, if it's will update it, if there is no value, you will enter it through a prompt (e.g `-c key`)

  -f, --force
          Force edit, will not return error if there is a problem with the args.
          
          For example, duplication in the custom fields and try to editing nothing

  -h, --help
          Print help (see a summary with '-h')
```

To edit a vault you need to provide the index or the name of the vault. If you
provide the index, the vault will be edited by its index, if you provide the
name, the vault will be edited the first vault with the given name.

You can edit the vault name, username, password, service, note, TOTP secret, and
custom fields.

For secrets like the password and TOTP secret, you can provide them as arguments
or you will be prompted for them.

## Field removal
If you want to remove a field from the vault, you can provide an empty value for
it, e.g. `-o ""`.

## Custom fields
If you want to add a custom field to the vault, you can use the `-c, --custom`
option, and provide the key-value pair. If you want to delete a custom field,
you can provide the key with an empty value, e.g. `-c key=""`. If the custom
field not exist it will be created, if it's exist it will be updated.

You can't add a new custom field prefixed with `.lprsfield.` because it's
reserved for backwards compatibility.

## Examples
Edit a vault by its index:
```sh
lprs edit 1 -n new-vault-name -u new-username -p new-password -s new-service -o new-note -t new-totp-secret -c key1=value1 -c key2=value2
```

Edit a vault password by its name:
```sh
# You will be prompted for the new password
lprs edit my-vault -p
```

Remove a custom field from a vault by its name:
```sh
lprs edit my-vault -c key1=""
```

## Notes
- The index is one-based (the first vault is 1).
