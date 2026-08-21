# g15.032 — Review Screen-Clear Navigation and Overlays

Status: **ready** — `g15.031` complete and operator-approved in PR #57
Parent: `027-screen-clear-human-review.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.026` (live native evidence), `g15.031` (serial predecessor)
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

The existing audit records all ten pages as A/A/A `keep`. That is mechanical
screening, not a human verdict. `ContextMenu` must be exercised by right-click
and `HoverCard` by hover; unchanged DOM after an ordinary click is not a defect.
The landed native probe proves every route constructs and every admitted axis
tab opens, but the worker must still judge the GPUI specimen's teaching value.

## Writable Scope

- the ten named specimen pages across Svelte, React, and GPUI
- focused preview/specimen tests required by a bounded repair
- their audit rows and one August batch log

## Validation

- focused preview/component evidence for changed pages
- `effigy catalogue:check`, `effigy check:svelte`, `effigy react:build`
- if GPUI specimen code changes: `effigy check:gpui` and
  `effigy regressions:native`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Never run `*-windowed`, `test:native-visual`, Jetstream, or
release selectors.

## Continuation

Finish one ten-page review/repair PR and stop for orchestrator review plus live
operator sign-off on every changed web route. Do not start `g15.033`, visual
conformance, or release certification. A component, contract, public API,
shared runtime, or native semantic defect is a routed stop condition rather
than hidden specimen work.
