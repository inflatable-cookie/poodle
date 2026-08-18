# g15.022 — Overloaded Examples: audio and music

Status: **planned** — orchestrator review required before dispatch
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Depends on: `g15.011`
Sequenced after `g15.017`, which splits `poodle_render::audio_specimens`'s
axis groups from its example groups — curating before that would re-count the
matrices as examples.
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Scope

The audio family's pages, once their axis matrices have moved out of the
page body.

Catalogue families: `audio-music`.
The audit measured 7 pages in this group as overloaded (10+ captioned
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
