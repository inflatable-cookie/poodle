# g15.030 — Review Screen-Clear Foundation Layout

Status: **planned** — orchestrator review required before dispatch
Parent: `027-screen-clear-human-review.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.026` (live native evidence)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Pages This Card Owns (9)

- `Box`
- `Grid`
- `Region`
- `ResizeHandle`
- `ScrollShell`
- `Separator`
- `Spacer`
- `Stack`
- `Surface`

This list is exact and exhaustive. It contains every `keep` page in the layout
audit family, no other child owns them, and this card owns no others.

## Goal

Apply the parent's human teaching review to all nine pages; keep good pages
unchanged and repair only bounded specimen defects found.

## Acceptance

Per the parent, including a recorded verdict for every page and live operator
review of every changed Svelte and React page before completion.

## Writable Scope

- the nine named specimen pages across Svelte, React, and GPUI
- their audit rows and one August batch log
