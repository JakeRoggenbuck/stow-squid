# 🦑 stow-squid
Stow your dotfiles

## Verbs
For all verbs, the name is of a specific dotfile and is optional. Including a name will only run the verb on that dotfile. Without a name, it will run the verb on all the dots in the config.

### Save
```
save-squid save <name>
```

### Deploy
```
save-squid deploy <name>
```

### Diff
```
save-squid diff <name>
```

## Config
```toml
[[files]]
name = "dotfile-name"
origin = "path/to/file/from/git/repo"
deployed = "/path/to/where/the/file/is/placed"

[[files]]
name = "dotfile-name-two"
origin = "path/to/other/file/from/git/repo"
deployed = "/path/to/where/the/other/file/is/placed"
```

## Config Example
```toml
[[files]]
name = "bspwm"
origin = "/home/jake/Build/dotfiles/bspwm/bspwmrc"
deployed = "/home/jake/.config/bspwm/bspwmrc"

[[files]]
name = "alacritty"
origin = "/home/jake/Build/dotfiles/alacritty/alacritty.yml"
deployed = "/home/jake/.config/alacritty/alacritty.yml"

[[files]]
name = "bashrc"
origin = "/home/jake/Build/dotfiles/.bashrc"
deployed = "/home/jake/.bashrc"

[[files]]
name = "sxhkd"
origin = "/home/jake/Build/dotfiles/sxhkd/sxhkdrc"
deployed = "/home/jake/.config/sxhkd/sxhkdrc"

```

## Help
```
USAGE:
    drop <verb> [dot]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <verb>
    <dot>
```
