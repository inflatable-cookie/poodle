# g09.016 — Visual Parity Audit: Systematic Svelte vs GPUI Comparison

Status: complete
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.015
Primary repos: `pug`

## Goals

- [ ] systematically compare every component specimen between Svelte and GPUI
- [ ] identify and fix visual discrepancies until both previews are visually
  identical within native-adaptation tolerances

## Execution Checklist

- [ ] run Svelte preview and GPUI preview side-by-side with the same theme
  and density settings
- [ ] compare structural primitives: background fill, border color/width,
  border radius, padding, margin, gap
- [ ] compare action primitives: button colors (all variants), hover/active
  states, disabled opacity, icon sizing
- [ ] compare input primitives: input height, border color, focus ring,
  placeholder color, label alignment
- [ ] compare selection primitives: checkbox/radio indicator size, switch
  track/thumb sizing, slider track height and fill color
- [ ] compare feedback primitives: progress bar height and colors, badge
  background and text, status indicator dot size and colors
- [ ] compare overlay primitives: dialog backdrop, border radius, shadow,
  padding; menu item height and hover background
- [ ] compare date/time primitives: calendar grid cell size, selected day
  highlight, navigation arrow sizing
- [ ] compare composite specimens: layout structure, spacing between sections,
  header/content arrangement
- [ ] compare workstation surfaces: shell proportions, header height, sidebar
  width, status bar height
- [ ] compare demo app screens: overall layout proportions, content density,
  component arrangement
- [ ] for each discrepancy found, trace to the token resolution or style
  mapping and fix:
  - [ ] incorrect token reference → fix in component source
  - [ ] missing token resolution → add resolve call
  - [ ] wrong style mapping (e.g., padding vs margin) → fix in IntoElement
  - [ ] GPUI API limitation → document as intentional delta
- [ ] re-compare after fixes to verify resolution
- [ ] document all intentional native-adaptation deltas with justification

## Acceptance Criteria

- [ ] every component specimen has been compared between Svelte and GPUI
- [ ] all fixable discrepancies have been resolved
- [ ] intentional deltas are documented with technical justification
- [ ] side-by-side comparison shows consistent visual appearance across
  themes and density modes
- [ ] token coverage is complete — no component references unresolved tokens

## Next Task

Open `g09.017` and update the cross-runtime parity report.
