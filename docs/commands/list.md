# Listing all vaults

## Usage

```
Usage: lprs list [OPTIONS]

Options:
  -f, --filter <TEXT>  Filter the select list
  -r, --regex          Enable regex when use `--filter` option
      --json           Returns the output as `json` list of vaults
  -h, --help           Print help
  -V, --version        Print version
```

Lprs `list` command is used to list all vaults in the vaults file, you can also
filter the list by using the `--filter` option, and you can enable regex by
using the `--regex` flag. Also you can get the output as `json` by using the
`--json` flag (this is useful when you want to use the output in a script and
work with it with `jq`).


### Examples
<script src="https://asciinema.org/a/eEVkDi0NroBjKNILg7KW3hSKY.js" id="asciicast-eEVkDi0NroBjKNILg7KW3hSKY" async="true" data-cols=48 data-rows=10></script>
