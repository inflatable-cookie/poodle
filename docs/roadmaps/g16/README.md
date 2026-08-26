# g16 — Next Work

Status: active — `g16.007` merged in PR #81; next evidence lane unchosen
Posture: strict-ready; the next measured lane is unchosen until review
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
3. [003 — RadioGroup native identity and mounted parity](003-radio-group-native-identity-and-mounted-parity.md) — complete
4. [004 — ToggleGroup semantic API and mounted parity](004-toggle-group-semantic-api-and-mounted-parity.md) — complete; merged in PR #78
5. [005 — Slider axis, keyboard, and mounted parity](005-slider-axis-keyboard-and-mounted-parity.md) — complete; merged in PR #79
6. [006 — Tabs drag, keyboard, and mounted parity](006-tabs-drag-keyboard-and-mounted-parity.md) — complete; merged in PR #80
7. [007 — TextInput controlled editing and mounted evidence](007-text-input-controlled-editing-and-mounted-evidence.md) — complete; merged in PR #81

## Measured Selection

`g16.001` separates the next gaps rather than promoting structural presence to
parity:

- semantic/interface structure is present across the active cohort, but broad
  cross-runtime behavioural proof is not;
- `g16.002` closed three selection-control cells; `g16.003` closed RadioGroup;
  `g16.004` closed ToggleGroup;
- React has no equivalent whole-roster axe sweep, while GPUI accessibility
  remains manual;
- web visual evidence is a route sweep for 169 components, manual for five,
  and a fixed comparison for Button only;
- GPUI visual comparison is missing for 173 non-Button portable components.

`g16.002` closed mounted GPUI behaviour for `Checkbox`, `Switch`, and
`SegmentedControl`. `g16.003` closed RadioGroup after required host-owned
native interaction scope landed. `g16.004` closed ToggleGroup: resulting-
selection payloads, single-mode radiogroup roving focus, and instance-scoped
native identity. Remaining measured gaps are still semantic/interface
breadth, accessibility, and visual comparison.

The operator accepted the split-lane recommendation on 2026-08-26. RadioGroup
keeps its web form-name behavior and required host-owned native interaction
scope. ToggleGroup keeps resulting-selection payloads and single-mode
radiogroup semantics, with the same required native scope pattern.

`g16.005` closed Slider's native axis, callback, keyboard, and mounted parity.
`g16.006` closed Tabs' payload lifecycle, keyboard reorder/close, and mounted
GPUI behaviour. The generated ledger now has 36 mounted GPUI behaviour cells
and 138 missing. Tree and ModelCatalogueEditor remain regression consumers of
the corrected payload seam; their ledger cells did not move. Jetstream remains
deferred.

The post-`g16.006` checkpoint selected `TextInput` as the next leverage point.
It underpins search, command, settings, model, embed, token, editable-list, and
relation-picker surfaces. `g16.007` closed its core controlled editing through
the mounted GPUI path: 36 → 37 mounted, 138 → 137 missing. It repaired two
measured defects — `maxLength` had no owner in the Rust path, and every search
field shared one clear-button element id — and, after orchestrator review, made
unchanged edit outcomes silent so a rejected edit is distinguishable from an
accepted one. It left multiline, slug lifecycle,
validation timing, OS input methods, and native accessibility/visual closure
explicitly unclaimed. Two further findings (Tab mapped to submit in the backend
key path; blur-time `forget` keyed by the wrong id) are recorded in the
execution log for the orchestrator rather than repaired inside the card.
`NumberInput` stays out: its concrete-`f64`, stepper-only native surface needs a
separate raw-draft/value-model decision recorded in
`../../triage/20260826-213343-number-input-native-value-model.md`.

## Next Task

Orchestrator evidence checkpoint. Choose the next bounded lane from the current
ledger and the TextInput backend findings in
`../../triage/20260826-224901-text-input-native-event-cleanup.md`. Keep
NumberInput's value-model decision separate from multiline/slug closure, other
component families, and visual or accessibility programmes.
