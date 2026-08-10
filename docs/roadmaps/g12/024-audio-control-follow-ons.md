# 024 Audio Control Follow-ons

Status: complete
Owner: Poodle core
Created: 2026-08-10
Depends on: `docs/architecture/008-audio-control-family.md`

## Gate

Phase 1 and the component-specific VisualState extensions were approved by the
maintainer on 2026-08-10. Keep asset skins, GPUI, and host parameter binding
out of the Phase 2 batch.

## Phase 2

- [x] Envelope/curve editor: points, segment curves, add/remove/drag, snap hooks.
- [x] XY pad: two value laws and a two-axis gesture machine.
- [x] Audio toggle, momentary, and multi-state switch: latch/momentary semantics
  and lamp state distinct from generic Button.
- [x] Gain-reduction meter: AudioMeter-derived feed and inverted scale.

## Acceptance

- [x] Framework-free machines and math/transition tests.
- [x] Serializable component-specific VisualState contracts.
- [x] Standard Svelte renderers with adapter-owned input and accessibility.
- [x] Full recipe hook coverage with zero Phase 2 candidates.
- [x] Component contracts, docs tables, registry entries, and specimens.

## Validation

- `effigy test:core`: 414 passed.
- `effigy test:components`: 752 passed, including accessibility checks.
- `effigy svelte:build`: passed.
- Docs lint, contract drift, spec drift, and `git diff --check`: passed.
- Recipe inventory: 139 components, 1,057 hooks, zero candidates for all Phase
  2 components. Three unrelated pre-existing candidates remain.
- Collaborative preview navigation passed; snapshot/text inspection timed out,
  so no pixel review is claimed.

## Phase 3 Stubs

Issue stubs only. Do not add exports or placeholder implementations.

### Audio Keyboard

Decide pointer/computer-key mapping, velocity, note ownership, chord behavior,
focus policy, and host note-event effects.

### Waveform And Graph Display

Decide immutable data/feed shape, viewport decimation, cursor hit testing,
selection semantics, and canvas/SVG/native parity.

### Mod-matrix Grid

Decide source/destination identity, sparse routing, keyboard grid navigation,
modulation depth law, and accessible grid semantics.

## Later Architecture Work

Asset skins, Tier 2 custom rendering, GPUI, and host parameter binding require
separate contracts. Architecture 008 preserves their VisualState and gesture
seams.
