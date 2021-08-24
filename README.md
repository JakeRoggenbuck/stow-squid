# 🦑 stow-squid
Stow your dotfiles

![image](https://user-images.githubusercontent.com/35516367/130694552-c0a30c2e-910d-47e1-8fe2-f4509578c56f.png)


## Install
```sh
git clone https://github.com/JakeRoggenbuck/stow-squid.git

cargo install --path .

# Add config to ~/.config/stow-squid/stow-squid.toml
mkdir -p ~/.config/stow-squid/ && cp example-config.toml ~/.config/stow-squid/stow-squid.toml
```

## Verbs
For all verbs, the name is of a specific dotfile and is optional. Including a name will only run the verb on that dotfile. Without a name, it will run the verb on all the dots in the config.

### Save
This is to update your dotfiles (that are scattered around your machine) to your git repo 
```
stow-squid save <name>
```

### Deploy
This is to place all your dotfiles from your git repo to all the various places they might go
```
stow-squid deploy <name>
```

### List
```
stow-squid list
```

## Config
```toml
# Structure

# [[files]]
# name = "dotfile-name"
# origin = "path/to/file/from/git/repo"
# deployed = "/path/to/where/the/file/is/placed"


# Example

# [[files]]
# name = "bspwm"
# origin = "/home/jake/Build/dotfiles/bspwm/bspwmrc"
# deployed = "/home/jake/.config/bspwm/bspwmrc"


# Add you first dotfile here
[[files]]
name = ""
origin = ""
deployed = ""
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
