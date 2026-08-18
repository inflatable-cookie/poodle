# g15.032 — Review Screen-Clear Navigation and Overlays

Status: **planned** — orchestrator review required before dispatch
Parent: `027-screen-clear-human-review.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.026` (live native evidence)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Pages This Card Owns (10)

Navigation:

- `Breadcrumbs`
- `NavigationMenu`
- `Pagination`
- `PaginationSummary`

Overlays and disclosure:

- `Collapsible`
- `ContextMenu`
- `DebugDialog`
- `HoverCard`
- `Menubar`
- `Popover`

This list is exact and exhaustive. It contains every `keep` page in these two
audit families, no other child owns them, and this card owns no others.

## Goal

Apply the parent's human teaching review to all ten pages; keep good pages
unchanged and repair only bounded specimen defects found.

## Acceptance

Per the parent, including a recorded verdict for every page and live operator
review of every changed Svelte and React page before completion.

## Writable Scope

- the ten named specimen pages across Svelte, React, and GPUI
- their audit rows and one August batch log
