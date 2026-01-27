# Alabaster for Helix

Minimal Alabaster color themes for the [Helix editor](https://helix-editor.com/).

## Philosophy

These themes follow the [Alabaster philosophy](https://github.com/tonsky/sublime-scheme-alabaster) by Nikita Prokopov: minimal syntax highlighting that emphasizes readability over decoration.

**Only 4 semantic categories get color:**
- Strings (green)
- Constants (magenta)
- Comments (dark themes: yellow; light themes: red - they're important!)
- Definitions (blue - functions, types, classes)

Everything else (keywords, variables, operators) uses the default foreground color because the structure of code is already clear from its formatting.

## Design Decisions

### Function calls vs definitions

Tonsky's original Alabaster highlights function **definitions** but not function **calls**. Helix's tree-sitter queries don't always distinguish these—some languages (Lua, Haskell) use `function.call` for calls, but most (Rust, Python, JavaScript) use bare `function` for both.

This theme sets `function.call` and `function.method.call` to the default foreground color, which helps in languages that distinguish them.

### Builtin functions

Helix uses `function.builtin` exclusively at call sites (builtins like `print()` don't have user-visible definitions). Since these are calls, not definitions, we don't highlight them—consistent with tonsky's philosophy.

### Bracket matching

Tonsky's Sublime theme highlights **both** brackets in a matching pair with blue foreground + underline. Helix's model is different: `ui.match` styles the *other* bracket (not under the cursor), while `ui.cursor.match` only affects the cursor's appearance, not the bracket character itself.

This theme uses blue + underline for `ui.match`, matching tonsky's style as closely as Helix allows. The bracket under the cursor remains styled as regular punctuation, but is visually distinct by virtue of being under the (blue) cursor.

## Themes

This port includes the complete Alabaster family matching tonsky's original suite:

### Standard Variants (Text Color Highlighting)

**wolf-alabaster-light** - Light background with colored text
![Alabaster Light Theme](screenshots/alabaster-light.png)

**wolf-alabaster-dark** - Dark background with colored text
![Alabaster Dark Theme](screenshots/alabaster-dark.png)

### BG Variants (Background Color Highlighting)

**wolf-alabaster-light-bg** - Light background with colored backgrounds
![Alabaster Light BG Theme](screenshots/variants/alabaster-light-bg.png)

**wolf-alabaster-dark-bg** - Dark background with colored backgrounds
![Alabaster Dark BG Theme](screenshots/variants/alabaster-dark-bg.png)

### Mono Variants (Minimal Color)

**wolf-alabaster-light-mono** - Light background, mostly grayscale
![Alabaster Light Mono Theme](screenshots/variants/alabaster-light-mono.png)

**wolf-alabaster-dark-mono** - Dark background, mostly grayscale
![Alabaster Dark Mono Theme](screenshots/variants/alabaster-dark-mono.png)

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
# Standard variants (text color)
theme = "wolf-alabaster-light"
theme = "wolf-alabaster-dark"

# BG variants (background color)
theme = "wolf-alabaster-light-bg"
theme = "wolf-alabaster-dark-bg"

# Mono variants (minimal color)
theme = "wolf-alabaster-light-mono"
theme = "wolf-alabaster-dark-mono"
```

## Credits

- Based on [Alabaster](https://github.com/tonsky/sublime-scheme-alabaster) by Nikita Prokopov
- Adapted for Helix by Wolf
- Feedback on missing scopes from [@Rudxain](https://github.com/Rudxain) ([#15102](https://github.com/helix-editor/helix/pull/15102))

## License

MIT
