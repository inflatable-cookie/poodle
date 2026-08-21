# g15.033 — Review Screen-Clear Forms, Data, and Media

Status: **ready** — `g15.041` and serial predecessor `g15.032` are complete
Parent: `027-screen-clear-human-review.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.026` (live native evidence), `g15.032` (serial predecessor)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Pages This Card Owns (7)

Forms and validation:

- `FieldSet`
- `ValidationSummary`

Data and collections:

- `CardRadioGroup`
- `ListContainer`
- `OrderBy`
- `SelectionSummary`

Media:

- `MediaThumbnail`

This list is exact and exhaustive. It contains every `keep` page in these
three audit families, no other child owns them, and this card owns no others.

## Goal

Apply the parent's human teaching review to all seven pages; keep good pages
unchanged and repair only bounded specimen defects found.

## Acceptance

Per the parent, including a recorded verdict for every page and live operator
review of every changed Svelte and React page before completion.

## Writable Scope

- the seven named specimen pages across Svelte, React, and GPUI
- their audit rows and one August batch log

## Continuation

Dispatch this final screen-clear child from current `origin/main`. It may run
beside one independent native, capture, or release-tooling lane with
non-overlapping writable scope.
