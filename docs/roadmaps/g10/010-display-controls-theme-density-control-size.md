# g10.010 — Display Controls: Theme, Density, Control Size

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.009
Primary repos: `pug`

## Goals

- [ ] implement display controls matching the Svelte and GPUI preview apps
- [ ] theme/density/size changes propagate to all specimens

## Execution Checklist

- [ ] implement theme selector — cycle through registered themes, re-resolve
  all Pug tokens through the Jetstream `PugTheme` → `Theme` bridge
- [ ] implement density toggle — compact, normal, spacious — adjust spacing
  tokens and re-render
- [ ] implement control size selector — sm, md, lg — adjust control height
  and font size tokens
- [ ] implement state toggles — disabled, invalid, busy — applied globally
  to all interactive specimens in current view
- [ ] render display controls in a sidebar panel below the component
  catalogue
- [ ] store display state in app state and preserve across navigation
- [ ] verify theme changes propagate to all visible nodes (background, text,
  border, accent colors all update)
- [ ] verify density changes adjust spacing between all elements
- [ ] verify control size changes adjust interactive element dimensions

## Acceptance Criteria

- [ ] theme selector switches between themes with immediate visual effect
- [ ] density toggle visibly changes spacing
- [ ] control size selector changes interactive element sizes
- [ ] state toggles affect all interactive specimens
- [ ] display state is preserved when switching between sections
- [ ] `cargo check` passes

## Next Task

Open `g10.011` and expand the demo scene to 6 screens.
