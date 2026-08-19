# g15.022 — Overloaded Examples: audio and music

Status: **complete** — PR #46 accepted at `eddc233e`; merge `4d736469`.
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Consumes: `g15.011` partial screening baseline
Sequenced after `g15.017`, which splits `poodle_render::audio_specimens`'s
axis groups from its example groups — curating before that would re-count the
matrices as examples.
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Scope

The audio family's pages, once their axis matrices have moved out of the
page body.

Catalogue families: `audio-music`.
### Pages this card owns (11)

- `AudioMeter`
- `AudioSwitch`
- `EnvelopeEditor`
- `Fader`
- `GainReductionMeter`
- `Keyboard`
- `Knob`
- `ModMatrixGrid`
- `ValueReadout`
- `WaveformDisplay`
- `XYPad`

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
