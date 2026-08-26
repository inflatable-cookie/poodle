# g16 — Next Work

Status: active — `g16.003` ready
Posture: strict-ready; RadioGroup identity decision promoted; ToggleGroup
semantic/API lane sequenced behind it
Opened: 2026-08-25
Governing refs: `../../../README.md`, `../../README.md`,
`../../contracts/001-working-rules.md`, `../g15/README.md`,
`../generation-index.md`

## Aim

Make active-cohort parity measurable from current evidence before choosing a
new conformance runway. Separate structural presence, focused behaviour,
mounted runtime proof, accessibility, and visual comparison. Do not build a
third cross-runtime authority to produce the inventory.

## Current State

The release, recovery, and consumer-adoption programme belongs to g15 and is
complete through `g15.079`. Its release roster proves complete structural
coverage across Svelte, React, shared Rust, and GPUI, but current parity reports
still embed older g09/g10 counts and blur evidence levels.

The earlier `g16.001`–`g16.025` numbering was a planning-boundary mistake. Those
cards now live canonically as `g15.055`–`g15.079`. Historical handoff, branch,
and log filenames retain their original g16 labels as point-in-time provenance;
they do not define this generation's runway.

Jetstream remains program-deferred. Its shared Rust and in-repo adapter surfaces
stay maintained; backend admission is not part of the first card.

## Runway

1. [001 — Active-cohort parity evidence ledger](001-active-cohort-parity-evidence-ledger.md) — complete; operator-reviewed in PR #75
2. [002 — Selection controls mounted parity](002-selection-controls-mounted-parity.md) — closed — partial outcome
3. [003 — RadioGroup native identity and mounted parity](003-radio-group-native-identity-and-mounted-parity.md) — ready

## Measured Selection

`g16.001` separates the next gaps rather than promoting structural presence to
parity:

- semantic/interface structure is present across the active cohort, but broad
  cross-runtime behavioural proof is not;
- GPUI mounted behaviour is present for 32 components and missing for 142;
  `g16.002` closed three selection-control cells; RadioGroup and ToggleGroup
  stay missing;
- React has no equivalent whole-roster axe sweep, while GPUI accessibility
  remains manual;
- web visual evidence is a route sweep for 169 components, manual for five,
  and a fixed comparison for Button only;
- GPUI visual comparison is missing for 173 non-Button portable components.

`g16.002` closed mounted GPUI behaviour for `Checkbox`, `Switch`, and
`SegmentedControl`. It stopped RadioGroup because unnamed groups had no stable
native instance identity, and stopped ToggleGroup on callback payload, roving
focus, and item identity. Remaining measured gaps are still
semantic/interface breadth, accessibility, and visual comparison.

The operator accepted the split-lane recommendation on 2026-08-26. RadioGroup
keeps its web form-name behavior and receives required host-owned native
interaction scope through a handler bundle. ToggleGroup will keep resulting-
selection payloads and single-mode radiogroup semantics, but its contract and
execution card wait until the RadioGroup identity pattern lands.

## Next Task

Dispatch `g16.003`. Do not parallelise or implement ToggleGroup in the same
worker.
