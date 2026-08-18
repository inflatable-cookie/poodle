# g15.020 — Overloaded Examples: model connections and account lifecycle

Status: **planned** — orchestrator review required before dispatch
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.011`
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Scope

The densest overloaded cluster in the catalogue: every page in both families
carries ten or more captioned examples.

Catalogue families: `model-connections`, `account-lifecycle`.
The audit measured 8 pages in this group as overloaded (10+ captioned
examples) or long (7–9) from a prop cross-product. Take the current per-page
list from the audit table rather than a copy here — `g15.015` and `g15.016`
land first in some cases and change what remains.

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
