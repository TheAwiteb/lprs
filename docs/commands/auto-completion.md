# Auto completion

## Usage

```
Usage: lprs completion <SHELL>

Arguments:
  <SHELL>  Shell to generate completion for [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help  Print help
```

The `completion` command generates completion scripts for the specified shell. The output is written to `stdout`, so you can redirect it to a file and source it in your shell configuration file.

## How to use

| Shell      | Command to generate completion script | Command location (the file to write the command in) |
|------------|---------------------------------------|----------------------------------------------------|
| Bash       | `eval "$(lprs completion bash)"`      | `~/.bashrc`                                        |
| Elvish     | `eval (lprs completion elvish \| slurp)` | `~/.elvish/rc.elv`                                |
| Fish       | `lprs completion fish \| source`      | `~/.config/fish/config.fish`                       |
| Powershell | `Invoke-Expression (& { (lprs completion powershell \| Out-String) })` | run `echo $PROFILE` |
| Zsh        | `eval "$(lprs completion zsh)"`       | `~/.zshrc`                                         |
