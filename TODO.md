# TODO

## Theme Improvements

### Selection Colors (Dark Theme)
- [x] Fix dark theme selection colors - make primary selection more distinct from secondary selections
  - Updated: primary `#4A7BA7` vs secondary `#264F78` - much better contrast
  - Primary selection now clearly stands out

### New Theme Variants

Following tonsky's full Alabaster suite, create these additional variants:

#### BG Variants (Background Highlighting)
- [ ] Create `wolf-alabaster-light-bg.toml` - Uses background colors for highlighting instead of text colors
- [ ] Create `wolf-alabaster-dark-bg.toml` - Dark version with background highlighting

#### Mono Variants (Minimal Color)
- [ ] Create `wolf-alabaster-light-mono.toml` - Monochromatic with minimal highlighting
- [ ] Create `wolf-alabaster-dark-mono.toml` - Dark monochromatic version

## Promotion Checklist

### Completed
- [x] Submit PR to tonsky/sublime-scheme-alabaster
- [x] Add screenshots to README

### In Progress
- [ ] Submit PR to helix-editor/helix runtime themes

### Pending
- [ ] Add to Helix GitHub Wiki (with screenshots)
- [ ] Post in Helix GitHub Discussion #727 "Sharing Themes"
- [ ] Submit PR to npupko/awesome-helix
- [ ] Post to r/HelixEditor subreddit
- [ ] Share in Helix Matrix chat (#helix-editor:matrix.org)
- [ ] Write blog post on DEV.to
- [ ] Post to Hacker News (Show HN)
- [ ] Post to r/rust, r/programming, r/coolgithubprojects
- [ ] Consider Product Hunt launch

## Notes

- Theme philosophy: Only 4 semantic categories get color (strings, constants, comments, definitions)
- Based on tonsky's Alabaster: https://github.com/tonsky/sublime-scheme-alabaster
- Color palette matches original Sublime Text values exactly
