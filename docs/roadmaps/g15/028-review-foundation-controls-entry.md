# g15.028 — Review Screen-Clear Foundation Controls and Entry

Status: **planned** — orchestrator review required before dispatch
Parent: `027-screen-clear-human-review.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.026` (live native evidence)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Pages This Card Owns (14)

Actions and selection:

- `Checkbox`
- `CollapseToggle`
- `Radio`
- `RadioGroup`
- `SegmentedControl`
- `Switch`
- `ToggleGroup`

Text and value entry:

- `CodeInput`
- `ColorPicker`
- `EditableLabel`
- `NumberInput`
- `Rating`
- `Slider`
- `ThemeSelect`

This list is exact and exhaustive. It contains every `keep` page in these two
audit families, no other child owns them, and this card owns no others.

## Goal

Apply the parent's human teaching review to all 14 pages; keep good pages
unchanged and repair only bounded specimen defects found.

## Acceptance

Per the parent, including a recorded verdict for every page and live operator
review of every changed Svelte and React page before completion.

## Writable Scope

- the 14 named specimen pages across Svelte, React, and GPUI
- their audit rows and one August batch log
