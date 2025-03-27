# chameleon 🦎
Config switcher for linux!

# Description
chameleon is a cli tool for switching files located in `~/.config` and `~/.local/bin`, and switching wallpapers using swww!

It was created to facilitate reproduction of ricing themes, and the nomenclature used in the project reflects that.

## Themes 🎨
chameleon works by switching different themes.

A theme is a collection of files in a structured manner, described by a Cham.toml file in the root of the directory.

In a theme you can include scripts to be linked to `~/.local/bin`, files to be linked to `~/.config`, and a wallpaper to be set by swww!

## Installation 📥
You can install chameleon by running:
```cargo install chameleon-th```

## Usage 🛠️
To select a theme you can run:
```
chameleon -s <theme_name>
```

The theme will be searched in `~/.local/share/chameleon/themes/`.

## Theme creation 📝
The theme must have a Cham.toml file in the root of the directory.

It needs to have a name so that it can be found by the set command.

The structure goes as the following example:
```toml
name = 'NixBlue'
description = "Perfect Blue"

config = 'config'
bin = 'bin'

fixup_inline = ["eww open bar --restart","makoctl reload"]
wallpaper = 'wallpapers/perfect_blue.png'
```

config and bin must be paths to the directory of files to be linked to the respective directories.

fixup_inline is a list of commands to be ran after the files are switched.

wallpaper is a path to a wallpaper to be set when the theme is selected.


