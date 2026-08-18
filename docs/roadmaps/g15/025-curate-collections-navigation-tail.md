# g15.025 — Overloaded Examples: collections, navigation and the long tail

Status: **planned** — orchestrator review required before dispatch
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.011`
Sequenced after `g15.016`.
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Scope

The remaining families, each with one to three affected pages.

Catalogue families: `data-collections`, `navigation`, `overlays-disclosure`, `layout`, `media`, `date-time`.
### Pages this card owns (10)

- `Accordion`
- `Dialog`
- `FilterBuilder`
- `ListCard`
- `ListCardCounter`
- `MediaPreview`
- `SplitView`
- `Stepper`
- `TimeAgo`
- `Tree`

This list is exact and exhaustive: it is every page in these families whose
`Examples` view the audit measured as overloaded (10+ captioned examples) or
long (7–9). No other card owns these pages, and this card owns no others. If a
prerequisite card changes a page's count before this one runs, re-measure and
record the change — do not silently widen or narrow the set.

No component, contract, or public API change.

## Goals

- [ ] Every page in the group meets the parent's method.
- [ ] Svelte and React stay identical; GPUI teaches the same set.
- [ ] Removals are named, with contract coverage checked first.

## Acceptance

Per the parent, including its operator-review checkpoint: **the changed pages
are reviewed live in the Svelte and React previews before this card is called
complete.** Unreviewed pages remain an explicit PR item.

## Writable Scope

- the specimen files for these families across Svelte, React, and GPUI
- one August batch log
