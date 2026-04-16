# g10 GPUI Production Hardening

Status: active
Updated: 2026-04-17

## Context

`g10` opened with a Jetstream feasibility framing (`g10.001`) but the live queue
expanded into a full GPUI production-hardening tranche. From `g10.003` onward the
work has been Svelte overhaul closeout, component package consolidation, GPUI
parity recovery, spec correctness, and component audit remediation. Jetstream
did not progress beyond the feasibility proof.

## Scope

- GPUI component correctness and token fidelity
- GPUI parity evidence and documented deltas vs Svelte reference
- Shared spec alignment across platforms
- GPUI runtime truth and debt classification
- Component audit remediation

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
  "blockers", dashed borders, backlog for real input/select/slider work
  (`docs/roadmaps/g10/012-gpui-runtime-truth-and-deferred-work-closure.md`)
- `g10.013` complete — GPUI component correctness and token fidelity: Select fake
  search, Button danger tone formulas, Checkbox/Switch per-size geometry, Pill
  tone handling
  (`docs/roadmaps/g10/013-gpui-component-correctness-and-token-fidelity.md`)
- `g10.014` complete — GPUI overlay architecture, navigation, and slider fidelity:
  floating overlay utility (Tooltip/Popover), TabStrip tokens + keyboard, Slider/
  RangeSlider behavioral gaps, Tabs close icon + gap token, Select option groups
  (`docs/roadmaps/g10/014-gpui-overlay-navigation-and-slider-fidelity.md`)
- `g10.015` complete — GPUI Svelte parity second pass: Callout geometry,
  RadioGroup/Pill/Code/Tabs/CodeInput fixes, inline token gap sweep
  (`docs/roadmaps/g10/015-gpui-svelte-parity-second-pass.md`)

## Active Runway

- GPUI audit queue: `g10.009` to `g10.014` complete
- `g10.015` complete
- `g10.012` remains active as a living delta register

## Next Task

`g10.015` complete. Next: `g10.016` — further audit pass (calendar geometry,
dialog/list_card values, remaining formula-hardcoded values across complex
components).
