# TODO

## Theme Improvements

### Selection Colors (Dark Theme)
- [x] Fix dark theme selection colors - make primary selection more distinct from secondary selections
  - Updated: primary `#4A7BA7` vs secondary `#264F78` - much better contrast
  - Primary selection now clearly stands out

### New Theme Variants

Following tonsky's full Alabaster suite, create these additional variants:

#### BG Variants (Background Highlighting)
- [x] Create `wolf-alabaster-light-bg.toml` - Uses background colors for highlighting instead of text colors
- [x] Create `wolf-alabaster-dark-bg.toml` - Dark version with background highlighting

#### Mono Variants (Minimal Color)
- [x] Create `wolf-alabaster-light-mono.toml` - Monochromatic with minimal highlighting
- [x] Create `wolf-alabaster-dark-mono.toml` - Dark monochromatic version

## Promotion Checklist

### Completed
- [x] Submit PR to tonsky/sublime-scheme-alabaster
- [x] Add screenshots to README (all 6 variants)
- [x] Submit PR to helix-editor/helix runtime themes (PR #15102)
- [x] Add to Helix GitHub Wiki (all 6 themes with screenshots)
- [x] Post in Helix GitHub Discussion #727 "Sharing Themes"
- [x] Submit PR to npupko/awesome-helix (PR #17)
- [x] Share in Helix Matrix chat (#helix-editor:matrix.org)
- [x] Draft blog post for DEV.to (ready to publish Monday)
- [x] Draft Reddit post for r/HelixEditor (ready to post Monday)

### For Monday Morning
- [ ] Post to r/HelixEditor subreddit
- [ ] Publish DEV.to blog post
- [ ] Share DEV.to post on social media (Mastodon/LinkedIn)

### Optional Future Promotion
- [ ] Post to Hacker News (Show HN) - after DEV.to post is live
- [ ] Cross-post to r/rust (Helix is written in Rust)
- [ ] Cross-post to r/vim, r/neovim (Alabaster is popular there)
- [ ] Post to r/programming, r/coolgithubprojects

## Notes

- Theme philosophy: Only 4 semantic categories get color (strings, constants, comments, definitions)
- Based on tonsky's Alabaster: https://github.com/tonsky/sublime-scheme-alabaster
- Color palette matches original Sublime Text values exactly (modulo primary selection vs ordinary selections in the dark versions)
