# g04.009 Navigation Cards And List Card Patterns

Status: planned
Owner: Pug Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `pug`

## Goals

- [ ] implement NavCard as a composite for navigation-oriented card links
- [ ] implement NavCardGrid as a composite for grid layouts of NavCards
- [ ] implement ListCard as a composite for structured list item cards

## Execution Checklist

- [ ] write contract for NavCard: title, description, icon, href/onClick, badge,
  disabled state, hover/focus styling
- [ ] implement NavCard composite in `@pug/svelte-composites`
- [ ] write contract for NavCardGrid: items array, column config, responsive
  breakpoints, gap
- [ ] implement NavCardGrid composite in `@pug/svelte-composites`
- [ ] write contract for ListCard: title, subtitle, metadata slots, leading
  icon/avatar, trailing action, click handler
- [ ] implement ListCard composite in `@pug/svelte-composites`
- [ ] create specimens for all three components
- [ ] register in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] NavCard renders as a clickable card with title, description, and optional
  icon
- [ ] NavCard supports href for navigation and onClick for actions
- [ ] NavCardGrid renders NavCards in a responsive grid layout
- [ ] ListCard renders a structured list item with leading content, text, and
  trailing action
- [ ] ListCard supports click interaction and hover/focus states
- [ ] all components pass build and render in the preview catalogue

## Next Task

Open `g04.010` and implement loading, skeleton presets, and progress patterns.
