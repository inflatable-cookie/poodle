# g16 — Next Work

Status: active — orchestrator evidence checkpoint after merged `g16.009`
Posture: strict-ready
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
8. [008 — Native text event routing cleanup](008-native-text-event-routing-cleanup.md) — complete; merged in PR #82
9. [009 — DurationInput single source and mounted behaviour](009-duration-input-single-source-and-mounted-behaviour.md) — complete; merged in PR #83

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

The post-`g16.007` checkpoint found that both recorded backend defects share one
bounded routing seam. `g16.008` corrected generic Tab-versus-submit dispatch and
root-versus-painted-value text-state identity before another editable control is
measured, and moved no ledger cell: still 37 mounted / 137 missing. Enter is
submission, Tab is traversal through gpui's own `focus_next`/`focus_prev` bound
at the window host, a node declaring `Interaction::focusable` is now a real tab
stop, and transient text state is keyed by the node that paints the value rather
than by the focused root. EditableLabel commits on Tab through the blur its
contract names, and DurationInput traverses `H → M → S → out` after its inert
focusable root declaration was removed in review. TextInput, CodeInput, DurationInput, and EditableLabel keep
their current evidence levels — this card proved routing, not parity.
NumberInput's value-model decision remains separate from multiline/slug closure,
other component families, and visual or accessibility programmes.

The post-`g16.008` checkpoint selected DurationInput as the next bounded lane.
`g16.009` closed the clean pre-1.0 break: the three segment fields are the only
Rust value, `show_seconds` defaults to `true`, display and min/max invalid
state derive from that value, and one named mounted GPUI regression proves
carry, borrow, digit entry, max-hours swallowing, visible-segment traversal,
callback totals, and disabled inertia through production dispatch. Ledger:
37 → 38 mounted, 137 → 136 missing. Accessibility, visual comparison, IME,
and Jetstream stay unclaimed.

## Next Task

Orchestrator evidence checkpoint after `g16.009`. Do not compile or start
another card from this worker thread.
