# g16 — Next Work

Status: active — `g16.001` complete; `g16.002` four proofs complete, ToggleGroup stop
Posture: strict-ready; ToggleGroup is a planning stop, not a closed mounted cell
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
2. [002 — Selection controls mounted parity](002-selection-controls-mounted-parity.md) — four proofs complete; ToggleGroup planning stop

## Measured Selection

`g16.001` separates the next gaps rather than promoting structural presence to
parity:

- semantic/interface structure is present across the active cohort, but broad
  cross-runtime behavioural proof is not;
- GPUI mounted behaviour is present for 33 components and missing for 141;
  `g16.002` closed four selection-control cells; ToggleGroup stays missing;
- React has no equivalent whole-roster axe sweep, while GPUI accessibility
  remains manual;
- web visual evidence is a route sweep for 169 components, manual for five,
  and a fixed comparison for Button only;
- GPUI visual comparison is missing for 173 non-Button portable components.

`g16.002` closed mounted GPUI behaviour for `Checkbox`, `Switch`, `RadioGroup`,
and `SegmentedControl`. ToggleGroup remains a planning stop: native option
emission is not the contracted selection payload, and contracted arrow roving
is absent from Svelte/React/GPUI. Remaining measured gaps are still
semantic/interface breadth, accessibility, and visual comparison.

## Next Task

Resolve the ToggleGroup public-callback and roving-focus stop as a separate
semantic/API lane. Do not compile `g16.003` from this PR.
