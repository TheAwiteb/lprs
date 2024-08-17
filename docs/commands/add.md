# Adding a vault

## Usage

```
Usage: lprs add [OPTIONS] <NAME>

Arguments:
  <NAME>
          The name of the vault

Options:
  -u, --username <USERNAME>
          The username

  -s, --service <SERVICE>
          The service name. e.g the website url

  -n, --note <NOTE>
          Add a note to the vault

      --totp-hash <HASH_FUNCTION>
          The TOTP hash function
          
          [default: sha1]

          Possible values:
          - sha1:   Sha1 hash function
          - sha256: Sha256 hash function
          - sha512: Sha512 hash function

  -p, --password [<PASSWORD>]
          The password, if there is no value you will prompt it

  -t, --totp-secret [<TOTP_SECRET>]
          The TOTP secret, if there is no value you will prompt it

  -c, --custom <KEY(=VALUE)?>
          Add a custom field to the vault
          
          If there is no value, you will enter it through a prompt

  -f, --force
          Force add, will not return error if there is a problem with the args.
          
          For example, duplication in the custom fields and try to adding empty vault

  -h, --help
          Print help (see a summary with '-h')
```

So, to add a vault you need to provide a name for the vault, and you can provide
a username, service name, note, password, TOTP secret, and custom fields.

For secrets like the password and TOTP secret, you can provide them as arguments
or you will be prompted for them.

### Custom fields
You can't add a custom field prefixed with `.lprsfield.` because it's reserved
for backwards compatibility.

## Examples
Add a vault:
```sh
lprs add my-vault1 -u my-username -s my-service -n 'My super secret note' \
    -p my-password -t 'JFWG65TFKJ2XG5BO' \
    -c key1=value1 -c key2=value2
```

Add a vault with a username and a password, but prompt for the password:
```sh
# -p without a value will prompt you for the password
lprs add my-vault2 -u my-username -p
```

Add a vault with a username, a password, and custom fields:
```sh
# The password will be prompted
lprs add my-vault3 -u my-username -p \
    -c key1=value1 \
    -c 'long key'='long value'
```

## Notes
- You must provide a name for the vault and at least one of the following:
  username, password, TOTP secret, or custom fields.
- If you provide a password or TOTP secret as an argument, it will be visible in
  the shell history.
- You can use existing vault names, and it will not be overwritten, so if you
  edit, get, or remove a vault by its name, it will be the first one found, so
  be careful with the names.
