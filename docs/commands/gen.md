# Generating a password

## Usage

```
Usage: lprs get <INDEX-or-NAME> [FIELD]

Arguments:
  [LENGTH]  The password length [default: 18]

Options:
  -u, --uppercase  With uppercase letters (A-Z)
  -l, --lowercase  With lowercase letters (a-z)
  -n, --numbers    With numbers (0-9)
  -s, --symbols    With symbols (!,# ...)
  -h, --help       Print help
  -V, --version    Print version
```

Generate a password with the specified length, by default the length is `18`,
you can specify the length by passing the `LENGTH` argument.

This command is useful when you need to generate a password for a new vault.

## Examples
Generate a password of length 20 with uppercase letters, lowercase letters,
numbers, and symbols:
```sh
lprs gen 20 -ulns
```

Generate a password of length 20 for a new vault:
```sh
lprs add my-vault -u 'username' -p $(lprs gen 20 -ulns)
```

## Notes
- You must specify at least one of the options to generate a password.
