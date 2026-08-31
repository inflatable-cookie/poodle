# g16 — Next Work

Status: active — g16.026 complete and merged; g16.027 delivered and under
review; g16.033 reserved at its public API decision gate
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
18. [018 — Select semantic machine and interface convergence](018-select-semantic-machine-and-interface-convergence.md) — complete; merged in PR #93
19. [019 — Select mounted overlay parity](019-select-mounted-overlay-parity.md) — complete; merged in PR #94
20. [020 — Component continuation audit](020-component-continuation-audit.md) — complete; merged in PR #95
21. [021 — Drag-and-drop semantic kernel](021-drag-drop-semantic-kernel.md) — complete; merged in PR #96
22. [022 — Drag-and-drop web custom-surface substrate](022-drag-drop-web-custom-surface-substrate.md) — complete; merged in PR #101
23. [023 — EditableList simple reorder migration](023-drag-drop-simple-reorder-migrations.md) — complete; merged in PR #104
24. [024 — Drag-and-drop Tree nested intent and auto-scroll](024-drag-drop-tree-nested-intent-and-auto-scroll.md) — complete; merged in PR #107
25. [025 — Drag-and-drop Rust and GPUI substrate](025-drag-drop-rust-gpui-substrate.md) — complete; merged in PR #108 after four orchestrator review rounds; ledger unchanged at 52 mounted / 122 missing
26. [026 — Drag-and-drop cross-window bridge, Tabs, and DockRegion](026-drag-drop-cross-window-bridge-and-dock-region.md) — complete; merged in PR #113 after two Northstar review rounds; split source/window bridge, bounded opaque receipt, clean public migration, Tabs subject-family seam, and window-owned GPUI provider census
27. [027 — Drag-and-drop inbound files and drag-out](027-drag-drop-inbound-files-and-drag-out.md) — delivered in PR #115, under review; paired export/inbound contracts, the web and GPUI boundaries, curated specimens, and a new engine probe leg
28. [028 — Drag-and-drop migration and certification closeout](028-drag-drop-migration-and-certification-closeout.md) — planned; depends on 027
29. [029 — TimeInput semantic model and native parity](029-time-input-semantic-model-and-native-parity.md) — complete; merged in PR #97; ledger 48 mounted / 126 missing
30. [030 — NumberInput value, draft, and mounted parity](030-number-input-value-draft-and-mounted-parity.md) — complete; merged in PR #98; ledger 49 mounted / 125 missing
31. [031 — Continuous audio machine and web lifecycle](031-continuous-audio-machine-and-web-lifecycle.md) — complete; merged in PR #99; no ledger cell moved
32. [032 — Continuous audio native mounted parity](032-continuous-audio-native-mounted-parity.md) — complete; merged in PR #100; ledger 52 mounted / 122 missing
33. [033 — HistoryCenter rejection surface](033-history-center-rejection-surface.md) — reserved after 026; public API decision required before dispatch; package-type source is already fixed, publication/pin movement stays separate

## Parallel Continuation

Two continuation programmes are now explicit:

- **Component continuation:** merged `g16.020` accounts for all 175 components
  and returns seven bounded candidate lanes without mutating components or the
  ledger. Its [runway map](component-continuation-runway.md) separates decision,
  audit, programme, and no-current-work gates. The register contains 93 closed,
  69 evidence-only, 3 decision-blocked, 7 programme-owned, and 3 unknown rows
  at audit closeout. Subsequent decisions and the bounded audio audit promote
  five known repairs, leave one decision-blocked row, and clear the unknown
  class without changing the 175-row denominator.
- **Dependable drag-and-drop:** architecture 011 and spec 069 govern a separate
  cross-runtime programme covering touch, nested targets, cross-window
  transfer, inbound files, and drag-out. `g16.021`–`g16.028` compile that
  programme without scattering the seven dependent components into unrelated
  repairs. `g16.021` landed the paired semantic kernel and its shared
  `dragDrop` vector corpus. `g16.022` landed the same-document web custom-surface
  controller and Svelte/React bindings. The clean Tabs and DockRegion public
  migrations are approved. EditableList proves simple reorder in `g16.023`;
  Tree proves nested intent and auto-scroll in `g16.024`. `g16.025` projected the
  same kernel through renderer-neutral Node registrations and a public GPUI
  `DragDropController`, deletes the backend-global payload session, and
  publishes an immutable stock-GPUI capability matrix — mouse, keyboard, and
  in-window capture certified; pen, touch, and device-originated cancel
  unsupported, and merged in PR #108 after the unsafe provider-unmount sweep
  was removed. `g16.026` now fixes separate per-source and per-window bridge
  roles, the bounded opaque receipt codec, and one `DragDropWindowHost` per
  GPUI window. Tabs migrates with its real DockRegion consumer. Later cards
  remain gated by landed dependencies.
- **Portfolio papercut follow-on:** `g16.033` is reserved after the merged
  `g16.026` shared-file tranche. It gives HistoryCenter distinct Poodle-owned
  deletion-refusal semantics and adds packed-package proof for the already-fixed
  v3 `HistoryEntry` export. Keyboard vertical geometry remains design-deferred;
  Longhorn's `AlreadyAtTarget` wire code remains an undispatched API decision.
  The card does not displace ordered drag card `g16.027`.

PR #94 merged the `g16.019` closeout. Its 47 mounted / 127 missing ledger is
now current on `main`; PR #95 merged the independent audit without reopening
the Select lane or changing those totals.

The component-continuation lane now has four approved implementation plans.
TimeInput's segmented native editing model is authoritative in its contract and
closed as `g16.029`. NumberInput's typed committed value, optional raw draft,
clean callback migration, and mounted editor are authoritative and compiled as
`g16.030`. NumberInput stays serial behind TimeInput's merge because both edit
shared core/headless exports and the domain-vector corpus.
Promoting NumberInput's explicit runtime deltas moved only that ledger axis from
115 / 60 to 116 present / 59 not-applicable; mounted totals are now 49 / 125
after `g16.030`.
EditableLabel still needs an operator-owned editing decision. Fader, Knob, and
XYPad now have a completed bounded audit and two serial cards: `g16.031`
aligns paired machines plus Svelte/React gesture and entry lifecycles;
`g16.032` added the missing continuous-value Node/GPUI seam and three mounted
proofs; ledger 49 → 52 mounted, 125 → 122 missing. The old “visual-state-only
Rust” description was inaccurate: Rust machines exist, but their fine
movement and component distinctions drifted until `g16.031`, and the native
render path did not mount them until `g16.032`. GPUI
accessibility, cross-runtime visual comparison, motion learning, the Longhorn
lab, and Jetstream admission remain separate programme choices rather than
component cards.

The longer generation direction stays evidence-led rather than becoming a
component-order checklist:

- close bounded foundation behavior gaps where the web authority, Rust
  contract, and mounted proof can be made coherent in one card;
- execute the approved NumberInput value-model migration only after its serial
  prerequisites, and resolve EditableLabel's separate gate before implementation;
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
`NumberInput` stayed out: its concrete-`f64`, stepper-only native surface needed
a separate raw-draft/value-model decision later fixed in its contract and
`g16.030`.

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
`g16.018` converged the shared semantic machine and interfaces without moving
the ledger and merged in PR #93. The landed review confirmed that existing Node
editing channels can express Select's search editor and that deferred overlay
pointer targeting still needs a bounded backend repair. `g16.019` now carries
the exact mounted scope.

`g16.019` closed that mounted lane: real native search editing, a bounded
deferred-overlay pointer repair, removal of Pagination's Select option-id
workaround, and one named two-instance mounted regression. Ledger: 46 → 47
mounted and 128 → 127 missing. Known-delta totals stay 115 / 60.

`g16.021` landed the drag programme's foundation: one renderer-neutral drag
session implemented once per language pair (`packages/core/src/drag-drop.ts`
and `poodle_headless::drag_drop`), proved by one hand-authored `dragDrop`
corpus — 25 session cases over 139 ordered steps plus 7 arbitration cases —
that both conformance runners execute. Lifecycle, session identity, intent,
cancellation, nested-target arbitration, exactly-once terminal effects, and the
single-use session-id rule are settled. No adapter, component, or ledger cell
moved.

`g16.031` closed the first half of the continuous-audio repair. Knob, Fader,
and XYPad now run one gesture model in both languages: one accepted begin, one
shared terminal for release and cancellation, anchored coarse/fine rebase,
Knob's vertical-versus-circular split, an inclusive detent radius with
first-declared tie resolution, and XYPad's press-position and atomic-pair
behaviour. A hand-authored `audioControls` corpus — 35 ordered cases over 171
steps plus 17 geometry cases — runs through both conformance runners with no
tolerance. All six web adapters now accept one primary pointer and close on
cancel, lost capture, or teardown; Svelte Knob and Fader gained React's
one-blur entry suppression. The only public web addition is a `DRAG_CANCEL`
event on the two core event unions. Rust gained the `knob_transition` /
`fader_transition` pair and the pointer-mapping helpers `g16.032` needs. No
ledger cell moved.

## Next Task

`g16.025` merged in PR #108. Stock GPUI 0.2.2 now certifies mouse, keyboard,
and its in-window capture-equivalent drag route; pen, touch, and
device-originated pointer cancellation remain explicit unsupported debt. No
ledger cell moved.

`g16.026` merged in PR #113 after Northstar review rounds 1-2. Round 1 closed
two blockers: the Rust cross-window bridge was declaration-only and is now
wired through the GPUI controller with four falsified host-stub proofs, and the
DockRegion contract contradicted the landed API. Round 2 closed four
exact-authority defects in that repair: a late lease returned through the wrong
host, asynchronous answers that never woke the window, a picker probe at
installation, and a bridge replacement that stranded the outgoing transaction. Every review-oracle row has named proof and six
of them were falsified — including the two-window false-cancel, which
reproduces the exact `g16.025` defect when a thread-global census is planted
back. The log is
`../../logs/2026-08/20260831-g16-026-drag-drop-cross-window-bridge.md`. One
consequence needs review attention: cross-region DockRegion transfer now
requires one common `DragDropProvider`, per the operator decision recorded in
spec 069. `g16.033` remains reserved until the operator chooses structured rejection
codes or a host-owned message; EditableLabel stays decision-blocked.

`g16.027` is delivered in PR #115 and awaiting review. Both external-file boundaries are paired and wired end to end: an
export prepares an opaque receipt on the pre-drag gesture and the host runs
the operating system's drag, while inbound batches become ordinary sessions
under one subject kind, validated before eligibility and again at the drop.
There is deliberately no committed export terminal — a native drag ending does
not prove a destination consumed anything — so the export carries its own
visible state beside the session phase. Wiring found the GPUI first-frame
sweep cancelling inbound sessions, the same shape the cross-window projection
was already exempted for. Eight claims were falsified by planting the pre-fix
behaviour back. No ledger cell moved. The log is
`../../logs/2026-08/20260831-g16-027-drag-drop-inbound-files-and-drag-out.md`.
`g16.028` becomes the next ordered drag card once this merges.
