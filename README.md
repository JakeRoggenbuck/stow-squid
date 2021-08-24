# 🦑 stow-squid
Stow your dotfiles

![image](https://user-images.githubusercontent.com/35516367/130694893-177cf2d5-eda1-419a-b6f7-1b2eb1f1d4cf.png)

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

![image](https://user-images.githubusercontent.com/35516367/130694801-2850dcaa-355f-489e-8170-566b39095f8b.png)

![image](https://user-images.githubusercontent.com/35516367/130694722-6bc15fd8-9c37-4a1b-af1f-e17b67631063.png)

### Deploy
This is to place all your dotfiles from your git repo to all the various places they might go
```
stow-squid deploy <name>
```

![image](https://user-images.githubusercontent.com/35516367/130694893-177cf2d5-eda1-419a-b6f7-1b2eb1f1d4cf.png)

![image](https://user-images.githubusercontent.com/35516367/130694977-a450efdb-b291-4093-b260-8b7c2340af8f.png)

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
