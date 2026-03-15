# g09.014 — Preview App Controls: Theme, Density, Control Size, Route State

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.013
Primary repos: `pug`

## Goals

- [ ] implement display controls matching the Svelte preview's sidebar controls
- [ ] implement route-addressable state so navigation context is preserved

## Execution Checklist

- [ ] implement theme selector dropdown listing all registered themes (e.g.,
  "loophole-studio") — changing theme re-resolves all token values and
  re-renders all specimens
- [ ] implement density mode toggle: compact, normal, spacious — adjust
  spacing tokens accordingly
- [ ] implement control size selector: sm, md, lg — adjust control height
  and font size tokens
- [ ] implement appearance treatment override: system, brand-raised
- [ ] implement state toggles: disabled, invalid, busy — applied globally
  to all interactive specimens in the current view
- [ ] store current display state (theme, density, control size, appearance,
  state toggles) in `AppState`
- [ ] store current navigation state (section, selected component slug) in
  `AppState`
- [ ] implement state serialization/deserialization so that navigation state
  can be restored on app restart (persist to a config file or similar)
- [ ] ensure display control changes trigger immediate re-render of all
  visible specimens
- [ ] render display controls in a sidebar panel below the component
  catalogue, matching Svelte layout
- [ ] verify theme changes propagate to all components (background colors,
  text colors, accent colors all update)

## Acceptance Criteria

- [ ] theme selector shows available themes and switching works immediately
- [ ] density toggle changes spacing across all visible specimens
- [ ] control size selector changes control dimensions
- [ ] disabled toggle grays out all interactive elements
- [ ] navigation state (section + component) is preserved across display
  control changes
- [ ] `cargo check` passes for the preview crate

## Next Task

Open `g09.015` and implement the 6-screen shared demo app.
