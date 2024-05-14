# Changing the master password

## Usage

```
Usage: lprs change-master-password [NEW_PASSWORD]

Arguments:
  [NEW_PASSWORD]  The new master password, if there is no value for it you will prompt it

Options:
  -h, --help  Print help
```

A command to change the master password of the vaults file, you can provide the
new password as an argument or you will be prompted for it.

### Example
Prompt for the new master password
```bash
lprs change-master-password
```

Change the master password to `new-password`
```bash
lprs change-master-password 'new-password'
```

## Note
- The master password is used to encrypt and decrypt the vaults file, so if you
  forget it you will lose all your vaults.
- This action is irreversible, so make sure to remember the new password.