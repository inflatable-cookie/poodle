# g15.023 — Overloaded Examples: foundation entry, content and status

Status: **planned** — orchestrator review required before dispatch
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.011`
Sequenced after `g15.016`, so caption idioms are uniform before examples are
counted and cut.
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Scope

Foundation pages where the length is a variant or state cross-product.

Catalogue families: `text-value-entry`, `actions-selection`, `content-identity`, `status-progress`.
The audit measured 12 pages in this group as overloaded (10+ captioned
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
