# Command line usage

This section provides a reference for the various commands and options that are
available in the `lprs` command line tool, and how to use them.

But before we dive into the commands, let's take a look at the `lprs` main
options, which are available for all commands.

```
Options:
  -f, --vaults-file <VAULTS_FILE>          The vaults json file
  -v, --verbose                            Show the logs in the stdout
  -m, --master-password <MASTER_PASSWORD>  The master password, or you will prompt it
  -h, --help                               Print help
  -V, --version                            Print version
```

As you can see, the `lprs` command line tool has a few options that are
available for all commands, which are:

- `-f, --vaults-file <VAULTS_FILE>`: The vaults json file, this is the file
  where the vaults are stored. By default, the vaults are stored in the
  program's directory, in a directory called `lprs` and the file called
  `vaults.lprs`.
- `-v, --verbose`: Show the logs in the stdout, this option is useful for
  debugging purposes.
- `-m, --master-password <MASTER_PASSWORD>`: The master password, this is the
  password that is used to encrypt and decrypt the vaults, usful for scripting
  purposes, otherwise you will be prompted for the master password (which is
  better for security reasons)

Now let's take a look at the available commands and how to use them.

- [Adding a vault](commands/add.md)
- [Removing a vault](commands/remove.md)
- [Editing a vault](commands/edit.md)
- [Getting a vault](commands/get.md)
- [Listing all vaults](commands/list.md)
- [Clening the vaults](commands/clean.md)
- [Generating a password](commands/generate-password.md)
- [Importing and exporting vaults](commands/import-export.md)
- [Changing the master password](commands/change-master-password.md)
- [Auto completion](commands/auto-completion.md)

## Donations
You can support the development of my projects by donating me, check out my profile at [git.4rs.nl](https://git.4rs.nl/awiteb#donations)
