# Alabaster for Helix

Minimal Alabaster color themes for the [Helix editor](https://helix-editor.com/).

## Philosophy

These themes follow the [Alabaster philosophy](https://github.com/tonsky/sublime-scheme-alabaster) by Nikita Prokopov: minimal syntax highlighting that emphasizes readability over decoration.

**Only 4 semantic categories get color:**
- Strings (green)
- Constants (magenta)
- Comments (red - they're important!)
- Definitions (blue - functions, types, classes)

Everything else (keywords, variables, operators) uses the default foreground color because the structure of code is already clear from its formatting.

## Themes

- **wolf-alabaster-light.toml** - Light background theme
- **wolf-alabaster-dark.toml** - Dark background theme

## Installation

### Using dotx (recommended)

If you use [dotx](https://pypi.org/project/dotx/) to manage your dotfiles:

```bash
cd ~/<where-you-keep-your-dotfile-collections>/alabaster-for-helix
dotx install helix
```

This creates symlinks in `~/.config/helix/themes/` pointing to the theme files in this repo.

### Manual installation

Copy the theme files to your Helix themes directory:

```bash
cp helix/dot-config/helix/themes/*.toml ~/.config/helix/themes/
```

## Usage

Edit your `~/.config/helix/config.toml`:

```toml
theme = "wolf-alabaster-light"
# or
theme = "wolf-alabaster-dark"
```

## Credits

- Based on [Alabaster](https://github.com/tonsky/sublime-scheme-alabaster) by Nikita Prokopov
- Adapted for Helix by Wolf

## License

MIT
