# g15.033 — Review Screen-Clear Forms, Data, and Media

Status: **planned — blocked on `g15.041` landing and `g15.032` closeout**
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

Do not dispatch this child while `g15.041` is in flight. After the Popover
migration lands, the orchestrator closes `g15.032`, verifies the audit totals,
then makes this card ready. It may then run beside one independent native or
release-tooling lane with non-overlapping writable scope.
