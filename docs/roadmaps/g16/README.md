# g16 — Next Work

Status: active — g16.018 complete; g16.019 planned
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

## Generation Runway

1. [001 — Active-cohort parity evidence ledger](001-active-cohort-parity-evidence-ledger.md) — complete; operator-reviewed in PR #75
2. [002 — Selection controls mounted parity](002-selection-controls-mounted-parity.md) — closed — partial outcome
3. [003 — RadioGroup native identity and mounted parity](003-radio-group-native-identity-and-mounted-parity.md) — complete
4. [004 — ToggleGroup semantic API and mounted parity](004-toggle-group-semantic-api-and-mounted-parity.md) — complete; merged in PR #78
5. [005 — Slider axis, keyboard, and mounted parity](005-slider-axis-keyboard-and-mounted-parity.md) — complete; merged in PR #79
6. [006 — Tabs drag, keyboard, and mounted parity](006-tabs-drag-keyboard-and-mounted-parity.md) — complete; merged in PR #80
7. [007 — TextInput controlled editing and mounted evidence](007-text-input-controlled-editing-and-mounted-evidence.md) — complete; merged in PR #81
8. [008 — Native text event routing cleanup](008-native-text-event-routing-cleanup.md) — complete; merged in PR #82
9. [009 — DurationInput single source and mounted behaviour](009-duration-input-single-source-and-mounted-behaviour.md) — complete; merged in PR #83
10. [010 — Breadcrumbs callback semantics and mounted parity](010-breadcrumbs-callback-semantics-and-mounted-parity.md) — complete
11. [011 — IconButton activation, toggle, and mounted parity](011-icon-button-activation-toggle-and-mounted-parity.md) — complete
12. [012 — Collapsible disclosure and mounted parity](012-collapsible-disclosure-and-mounted-parity.md) — complete; merged in PR #86
13. [013 — TriStateSwitch contract and mounted parity](013-tri-state-switch-contract-and-mounted-parity.md) — complete; merged in PR #87
14. [014 — Accordion result selection and mounted parity](014-accordion-result-selection-and-mounted-parity.md) — complete; merged in PR #88
15. [015 — CollapseToggle disclosure and mounted parity](015-collapse-toggle-disclosure-and-mounted-parity.md) — complete; merged in PR #90
16. [016 — Pagination navigation, loading, and mounted parity](016-pagination-navigation-loading-and-mounted-parity.md) — complete; merged in PR #91
17. [017 — Rating nullable, fractional, and mounted parity](017-rating-nullable-fractional-and-mounted-parity.md) — complete; merged in PR #92
18. [018 — Select semantic machine and interface convergence](018-select-semantic-machine-and-interface-convergence.md) — complete
19. [019 — Select mounted overlay parity](019-select-mounted-overlay-parity.md) — planned; blocked on merged g16.018

The longer generation direction stays evidence-led rather than becoming a
component-order checklist:

- close bounded foundation behavior gaps where the web authority, Rust
  contract, and mounted proof can be made coherent in one card;
- resolve editing value-model gates before implementing NumberInput or
  EditableLabel parity;
- repair composite selection/disclosure APIs such as Accordion only after
  their callback and state ownership are explicit;
- choose a separate visual-comparison or native-accessibility programme only
  from measured ledger leverage, without using specimen pages as exhaustive
  conformance fixtures; and
- keep Jetstream deferred until the active cohort's shared Rust/GPUI boundary
  is dependable enough for backend admission.

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

The post-`g16.009` checkpoint selected Breadcrumbs because it exposed a
measured callback reversal: shared Rust activated `href` crumbs and sent URLs,
while both web references activate linkless crumbs and send the authored
value. `g16.010` repaired that seam. Linkless crumbs now call `on_navigate`
with `BreadcrumbItem.value`; `href`, current, and ellipsis crumbs stay inert;
the Basic GPUI specimen shows one compact readout; one named mounted
regression proves pointer and keyboard dispatch. Ledger: 38 → 39 mounted,
136 → 135 missing; Breadcrumbs known-delta `not-applicable` → `present`
(114 → 115 present, 61 → 60 not-applicable). Native URL routing, broad
accessibility, visual comparison, and Jetstream stay out.

The post-`g16.010` checkpoint selected IconButton rather than an inert
presentation component or a breaking editing migration. `g16.011` closed that
seam: command activation, controlled and seeded toggle reporting, `Node.tooltip`
projection, semantic target state, and one named mounted GPUI regression.
Ledger: 39 → 40 mounted, 135 → 134 missing. Known-delta totals stay 115 present /
60 not-applicable. NumberInput's raw-draft/value-model decision and broader
visual/accessibility programmes remain separate.

The post-`g16.011` checkpoint selected Collapsible as the next bounded
foundation lane. `g16.012` closed that seam: effective open state,
trigger/content disclosure ownership, disabled focus suppression, host-supplied
instance identity, an honest default-open specimen, and one named mounted GPUI
regression. Ledger: 40 → 41 mounted, 134 → 133 missing; known-delta totals
stay 115 / 60. TriStateSwitch, NumberInput, EditableLabel, Accordion, visual
comparison, accessibility, and Jetstream stay outside the card.

The post-`g16.012` checkpoint selected TriStateSwitch after explicit operator
approval of its clean pre-1.0 Rust migration. PR #87 closed `g16.013`: it replaces legacy
checkbox-shaped `CheckState` storage with `TriStateValue`, makes Default the
real default, removes the undocumented general label and compatibility
surface, and closes radio semantics, roving focus, stable identity, and one
mounted GPUI behavior cell. The ledger moved 41 → 42 mounted and
133 → 132 missing; known-delta totals stay 115 / 60. NumberInput,
EditableLabel, Accordion, visual comparison, accessibility, and Jetstream stay
outside the card.

The post-`g16.013` checkpoint selected Accordion after explicit operator
approval of its clean pre-1.0 Rust migration. Merged `g16.014` removes the duplicate
`allow_multiple` mode and activated-item callback, gives single mode an
explicit collapsed result, reuses the existing headless ToggleGroup transition,
and repairs disclosure semantics plus stable native identity. Ledger moved
42 → 43 mounted and 132 → 131 missing; known-delta totals stay
115 / 60. Web APIs, panel animation, visual comparison, broad accessibility,
and Jetstream admission stay outside the card.

The post-`g16.014` checkpoint selected CollapseToggle. `g16.015` repaired that
bounded disclosure seam without a public API change: native default/explicit
labels, expanded state, enabled focus/tab/ring, disabled suppression, next-state
callback, and directional chevrons now match the web authority, with one named
mounted GPUI regression. Ledger: 43 → 44 mounted and 131 → 130 missing.
Known-delta totals stay 115 / 60. Select, EditableLabel, NumberInput, Rating,
visual comparison, broad accessibility, and Jetstream stay outside the card.

The post-`g16.015` checkpoint selected Pagination rather than an unresolved
editing/API migration or a full overlay lane. `g16.016` closed one measured
loading leak — the wired page-size Select stayed live while page buttons were
disabled — and proved numbered, simple, full, boundary, loading, and limit
changes through mounted GPUI host rebuilds. Ledger: 44 → 45 mounted and
130 → 129 missing. Select's own row, native accessibility, visual comparison,
and Jetstream stay unchanged.

The post-`g16.016` checkpoint found that Select still needs a larger query,
highlight, keyboard, freeform, overlay, and focus-return planning lane. The
operator instead approved Rating's clean pre-1.0 Rust migration for `g16.017`:
nullable values, default half-step input, removal of legacy precision/read-only
fields, `Option<f64>` change payloads, shared pure math, and coherent whole-step
radio plus fractional slider behavior. `g16.017` closed that lane and moved only
Rating from 45 → 46 mounted and 129 → 128 missing. Jetstream received mechanical
compile maintenance only and remains deferred.

The post-`g16.017` checkpoint rejected another one-component/one-cell sequence.
The operator approved Select as a deliberate two-card prerequisite lane and
approved the pre-1.0 callback correction: query reports every edit, while value
changes only on option selection or explicit freeform Enter/control-blur commit.
`g16.018` converges the shared semantic machine and interfaces without moving
the ledger. `g16.019` remains blocked until the landed substrate can be checked
against real native text entry, deferred-overlay input, and focus behavior.

## Next Task

Do not start planned `g16.019` until `g16.018` is merged and the orchestrator
recompiles its exact mounted-overlay scope against the landed Select API.
