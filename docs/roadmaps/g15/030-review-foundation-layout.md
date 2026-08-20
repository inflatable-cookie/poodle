# g15.030 — Review Screen-Clear Foundation Layout

Status: **complete** — PR #55 merged as `120a0062`; bounded ResizeHandle
specimen repair accepted, native semantics routed to `g15.040`
Parent: `027-screen-clear-human-review.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.026` (live native evidence), `g15.039`
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

## Closeout

All nine pages received a human teaching verdict. Eight were retained unchanged.
ResizeHandle gained live pane resizing in both web specimens and drag-driven
pane state in GPUI; the web examples now carry their real ARIA bounds. Review
correctly stopped on the shared native renderer's missing focus, keyboard, and
value declaration rather than hiding it as specimen work. That gap is
`g15.040` and blocks the next review child.

Focused specimen evidence, `effigy docs:check`, GitHub `active-cohort`, and
range whitespace checks passed. The operator directed the final corrections
and merge, explicitly waiving the remaining live paired-preview checkpoint.
