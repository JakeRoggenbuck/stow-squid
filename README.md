# 🦑 stow-squid 0.1.2
Stow your dotfiles

![image](https://user-images.githubusercontent.com/35516367/130694893-177cf2d5-eda1-419a-b6f7-1b2eb1f1d4cf.png)

## Announcements
- Programmatic adding and committing has been added as of version 0.1.2
- To get access to this feature, add a 'gitpath = "/git/path/"' to your config.

end_announcements

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

![image](https://user-images.githubusercontent.com/35516367/130696265-2dde8c08-5bee-41f4-b48d-8b69e15ac184.png)

![image](https://user-images.githubusercontent.com/35516367/130696302-607159a2-2a69-42d4-9f20-505827e32cb1.png)

### Deploy
This is to place all your dotfiles from your git repo to all the various places they might go
```
stow-squid deploy <name>
```

![image](https://user-images.githubusercontent.com/35516367/130696213-d763bd68-2449-4921-8d40-b22c6114f7cb.png)

![image](https://user-images.githubusercontent.com/35516367/130696199-2c57623d-c6b1-4d79-98f3-f0f4b6ae9286.png)

### List
```
stow-squid list
```

## Config
```toml
# Git path
gitpath = "/path/to/git/dir"

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
gitpath = "/home/jake/Build/dotfiles/"

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
