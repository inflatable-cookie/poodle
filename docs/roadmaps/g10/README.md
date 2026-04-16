# g10 Jetstream Focus

Status: active
Updated: 2026-04-13

## Context

`g10` is now the active generation after `g09`.
Its original Jetstream feasibility framing is still useful, but the live queue
has clearly broadened into a component-overhaul and renderer-parity recovery
surface that needs to be made explicit again.

## Scope

- Jetstream renderer feasibility and constraints
- Jetstream component implementation depth
- Jetstream specimen and preview coverage
- Jetstream parity evidence and documented deltas
- Jetstream-specific closeout work
- any component-overhaul recovery work that is materially part of the current
  Poodle execution queue rather than historical side work

## Working Rule

- do not use `g10` to absorb unfinished `g09` work by pretending `g09` is still
  active
- keep the live queue explicit instead of relying on handoff-only momentum
- use `g10` for the active Jetstream and component-overhaul tranche

## Current State

- `g10.001` complete — Jetstream feasibility proven
- `g10.002` complete — recovery/control lane opened, queue frozen, seams
  classified, next milestone compiled
- `g10.003` complete — Svelte Component Overhaul Closeout
- `g10.004` complete — Unified Component Package
- `g10.005` complete — GPUI preview shell, navigation, and native state parity
- `g10.006` complete — GPUI component page usage docs and shape parity
- `g10.007` complete — GPUI long-tail component parity sweep and closeout
  checkpoint
- `g10.008` complete — GPUI contract coverage gaps and list composition
  (`docs/roadmaps/g10/008-gpui-contract-coverage-gaps-and-list-composition.md`)
- `g10.009` complete — GPUI token resolution and visual fidelity
  (`docs/roadmaps/g10/009-gpui-token-resolution-and-visual-fidelity.md`)
- `g10.010` complete — GPUI behavior, adapter realization, and form remediation
  parity
  (`docs/roadmaps/g10/010-gpui-behavior-adapter-and-form-remediation-parity.md`)
- `g10.011` complete — shared spec and cross-platform button semantics
  (`docs/roadmaps/g10/011-shared-spec-and-cross-platform-button-semantics.md`)
- `g10.012` active — GPUI runtime truth, retire `g08` delta register, close false
  “blockers”, dashed borders, backlog for real input/select/slider work
  (`docs/roadmaps/g10/012-gpui-runtime-truth-and-deferred-work-closure.md`)
- `g10.013` pending — GPUI component correctness and token fidelity: Select fake
  search, Button danger tone formulas, Checkbox/Switch per-size geometry, Pill
  token resolution
  (`docs/roadmaps/g10/013-gpui-component-correctness-and-token-fidelity.md`)
- `g10.014` pending — GPUI overlay architecture, navigation, and slider fidelity:
  Tooltip/Popover floating overlay utility, TabStrip ARIA + keyboard, Slider/
  RangeSlider behavioral gaps, Tabs polish
  (`docs/roadmaps/g10/014-gpui-overlay-navigation-and-slider-fidelity.md`)

Three active seams identified:
- Seam A (Svelte Component Overhaul): complete via `g10.003`
- Seam B (Jetstream Implementation): active next lane
- Seam C (Parity/Verification): closeout checkpoint reached via `g10.005` to
  `g10.007`

## Active Runway

- default lane: Seam B Jetstream implementation **or** GPUI production hardening
  via `g10.012` (user priority)
- parity checkpoint: `g10.005` to `g10.007` complete
- GPUI / spec queue: `g10.009` to `g10.011` complete; **`g10.012` open** for
  deferred functional work (text input, select overlay, sliders, token sweep,
  adapter mount strategy)

## Next Task

Drive **`g10.012`** checklist, or resume Jetstream / rollover planning. Living GPUI
vs Svelte truth: `012-gpui-runtime-truth-and-deferred-work-closure.md` (not
`g08/delta-register.md`).
