# g16 Front-Door Compaction — 2026-09-05

Northstar docs compaction (operator-requested, Chatterbox-executed). The
narrative below was moved verbatim out of the active roadmap front doors so
they read as status surfaces. Nothing here is current authority; cards, logs,
receipts, and `dispatch.md` own the live state.


## From `docs/roadmaps/g16/README.md`

### Former Current State


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


### Former Parallel Continuation


The [canonical continuation map](component-continuation-runway.md) now owns the
ready, serial, gated, and held frontier. `g16.045`–`g16.050`,
`g16.053`–`g16.086` are complete. The mounted Nucleus cohort is 23/29.
`g16.087` and `g16.088` are complete. The mounted Nucleus cohort is 25/29.
`g16.089`–`g16.094` are complete. The Nucleus mounted cohort is 29/29 with one
validated terminal M1 receipt per component. That closes the mounted receipt
phase only; accessibility, visual-lab, consumer journey, and adoption evidence
remain separately gated.
Release mutation remains
separately gated.
`g16.051` and `g16.052` retain their
explicit serial or external gates. Completed `g16.054` does not inherit release
authority from the distribution programme. Citations,
nested-menu pointer intent, the dedicated lab,
its closed icon and Nucleus extensions, GPUI accessibility, public
IconMorph admission, release/adoption mutations, and Jetstream remain gated or
held rather than becoming implicit worker work. The canonical map also holds
the remaining approved holistic-assessment directions for later promotion.

`g16.055` is a separately completed Papercuts repair in PR #151. `g16.060` and
its `g16.061` validation prerequisite are complete; neither is part of the
post-triage continuation map.

The earlier continuation programmes remain provenance:

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
- **Portfolio papercut follow-on:** `g16.033` is implemented after merged
  `g16.028`. HistoryCenter now carries five distinct Poodle-owned refusal
  meanings — a stale, protected, or unavailable deletion no longer reads as a
  missing entry — and `effigy test:svelte-pack-install` proves the packed v3
  `HistoryEntry` export on both public Svelte import paths. Keyboard vertical
  geometry remains design-deferred. Longhorn's `AlreadyAtTarget` wire code and
  Loophole's wire adoption are complete; package pin/rejection adoption remains
  gated behind later publication. CS20 `groupId` stays Loophole/Pulse
  policy-owned. None of these holds belongs to the closed drag programme.
- **Shared motion continuation:** planning PR #121 is accepted and merged.
  Architecture 012 now fixes explicit full/reduced/frozen host policy,
  restriction-only inheritance, lifecycle, native approximation, and layered
  evidence. `g16.034` is complete in merged PR #124. The additive
  block-slider lane remains next after its merge; icon feasibility and
  AgentSubagent shimmer stay downstream evidence gates.
- **Independent MarkdownEditor issue fix:** `g16.035` is complete in PR #123.
  Shared web CSS now supplies the shrink chain, and the native preview declares
  and proves the same bounded scroll ownership through mounted GPUI wheel input.
  No public sizing API changed and no parity-ledger cell moved.
- **Tree consumer-authority follow-on:** PR #125 is merged and remains the
  interaction/geometry baseline. `g16.036` is complete in PR #127 as a paired-web
  public seam: one latched moving set, synchronous external eligibility and
  destination rewrite before accepted presentation, and the substrate's real
  commit result. It adds no consumer vocabulary or second machine. Rust/GPUI
  stays on the current synchronous single-row route because honest native
  parity requires broader Node substrate work.
- **DesEngs research wave:** PR #126's eight-family intake is fully researched
  in merged PRs #128–#135. Consumed decision packets are removed; fresh triage
  notes retain only unresolved composition, menu, lab, visual, portfolio, and
  Jetstream gates. The
  dossiers reject generic approval, task-list, live-series, numeric-motion,
  cue-policy, and toast-lifecycle APIs; citations remain composition-gated;
  nested-menu pointer intent is conditional compose-and-extend work. No public
  API shape is promoted by research intake alone.

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
EditableLabel's operator-owned editing decision is accepted and compiled as
ready `g16.045`. Fader, Knob, and
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
- execute the accepted post-triage ready frontier without crossing the serial,
  external-lab, release, sibling-repository, or Jetstream gates;
- repair composite selection/disclosure APIs such as Accordion only after
  their callback and state ownership are explicit;
- choose a separate visual-comparison or native-accessibility programme only
  from measured ledger leverage, without using specimen pages as exhaustive
  conformance fixtures; and
- keep Jetstream deferred until the active cohort's shared Rust/GPUI boundary
  is dependable enough for backend admission.


### Former Measured Selection


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


### Former Next Task


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
spec 069. At that point `g16.033` was still reserved pending the operator's
choice between structured rejection codes and a host-owned message; that
decision has since been made and the lane promoted. EditableLabel stays
decision-blocked.

`g16.027` merged in PR #115 after three Northstar repair rounds. Both external-file boundaries are paired and wired end to end: an
export prepares an opaque receipt on the pre-drag gesture and the host runs
the operating system's drag, while inbound batches become ordinary sessions
under one subject kind, validated before eligibility and again at the drop.
There is deliberately no committed export terminal — a native drag ending does
not prove a destination consumed anything — so the export carries its own
visible state beside the session phase. Wiring found the GPUI first-frame
sweep cancelling inbound sessions, the same shape the cross-window projection
was already exempted for. Review then closed exact terminal ownership, replay
resurrection, and the bounded-tombstone false negative. Twenty-seven claims
were falsified by planting the pre-fix behaviour back. No ledger cell moved. The log is
`../../logs/2026-08/20260831-g16-027-drag-drop-inbound-files-and-drag-out.md`.
`g16.028` closes the programme. ModelCatalogueEditor, OrderBy, and BlockEditor
moved onto the common substrate in Svelte and React; EditableList, OrderBy, and
BlockEditor gained their renderer-neutral reorder registrations and complete
result callbacks, so no native grip or move control is drawn that cannot
produce the order it promises. The band arithmetic three surfaces were about to
restate moved into `crate::drag_drop`. The certification claim is executable:
`effigy drift:drag-inventory` reads all seven programme surfaces in three
languages and fails on one planted `draggable`, `DataTransfer`, or local drag
index. Mounted Svelte *and* React component fixtures joined the headless
Chromium/WebKit probe, and three named mounted GPUI regressions moved four
ledger cells (52 → 56 mounted, 122 → 118 missing). The log is
`../../logs/2026-09/20260901-g16-028-drag-drop-migration-and-certification-closeout.md`.


## From `docs/roadmaps/README.md`

### Former Current State


- [Generation index](generation-index.md) is the canonical status summary.
- `g16.036` is complete in PR #127. The 2026-09-01 triage decisions are
  consolidated in the [canonical continuation map](g16/component-continuation-runway.md):
  `g16.045`–`g16.050`, `g16.053`, and `g16.056`–`g16.057` are complete.
  `g16.058` is ready; `g16.059` remains serially blocked behind it.
  `g16.051` and `g16.052` keep explicit serial, ownership, or external gates.
  `g16.054` stays blocked on completed `g16.059`; its security-audit dependency
  is complete and architecture 014 owns the compiled-distribution mechanics.
  Citations, nested menus, the lab-backed visual tranche, public IconMorph,
  release and adoption mutations, GPUI accessibility, Jetstream, and the
  separate holistic promotion batch remain gated or held.
  Independent `g16.055` is complete in PR #151 and remains outside the
  post-triage continuation map.
- `g16.001` is complete and operator-reviewed in PR #75. It repairs stale
  parity reporting and produces one component-level evidence ledger. `g16.002`
  closed — partial outcome: mounted GPUI behaviour for Checkbox, Switch, and
  SegmentedControl. PR #77 closed `g16.003` RadioGroup native identity and
  mounted evidence. `g16.004` closed ToggleGroup resulting-selection, single-mode
  roving focus, and instance-scoped native identity. `g16.005` closed Slider
  axis, keyboard, callback, and mounted parity in PR #79; ledger 34 → 35
  mounted, 140 → 139 missing. `g16.006` closed Tabs drag, keyboard, and mounted
  parity in PR #80; ledger 35 → 36 mounted, 139 → 138 missing. PR #81 closed
  `g16.007` core TextInput controlled editing and one honest mounted claim;
  ledger 36 → 37 mounted, 138 → 137 missing. `g16.008` then repaired generic
  native text routing — Enter is submission, Tab is real focus traversal, and
  transient text state follows the node that paints the value — deliberately
  moving no evidence cell. `g16.009` closed DurationInput's single-source Rust
  value and one named mounted GPUI behaviour cell (37 → 38 mounted, 137 → 136
  missing). `g16.010` closed Breadcrumbs' reversed Rust callback routing and
  one named mounted GPUI behaviour cell (38 → 39 mounted, 136 → 135 missing)
  plus the Breadcrumbs known-delta cell (`not-applicable` → `present`;
  114 → 115 present, 61 → 60 not-applicable). `g16.011` closed IconButton
  command, toggle, tooltip projection, and mounted GPUI evidence (39 → 40
  mounted, 135 → 134 missing). Known-delta totals stay 115 / 60. PR #86
  closed `g16.012` Collapsible disclosure and mounted parity, moving the ledger
  to 41 mounted / 133 missing. PR #87 closed the operator-approved `g16.013`
  clean migration of TriStateSwitch from legacy `CheckState` to
  `TriStateValue`, repaired native radio behavior and identity, and moved the
  ledger to 42 mounted / 132 missing. PR #88 closed `g16.014` Accordion result
  selection, disclosure semantics, identity, and mounted parity, moving the
  ledger to 43 mounted / 131 missing. PR #90 closed `g16.015` CollapseToggle
  native label, expanded state, focus, disabled behavior, and standalone
  mounted proof; ledger 44 mounted / 130 missing. PR #91 closed `g16.016`
  Pagination loading suppression and mounted navigation/limit proof; ledger
  45 mounted / 129 missing. `g16.017` closed Rating's approved nullable /
  fractional Rust migration and one mounted GPUI cell; ledger 46 mounted /
  128 missing. `g16.018` converged Select's semantic machine and interfaces
  without moving the ledger. `g16.019` closed native Select search, overlay
  pointer targeting, and one mounted GPUI cell; ledger 47 mounted /
  127 missing. Known-delta totals stay 115 / 60. No broader
  conformance programme is implied. PR #95 closed `g16.020`: its 175-row
  continuation register now separates 100 closed, 69 evidence-only, 1
  decision-blocked, and 5 programme-owned components after promoted-lane
  reconciliation. Architecture 011/spec 069 are compiled as
  `g16.021`–`g16.028`; the paired semantic kernel in `021` landed with its
  shared `dragDrop` vector corpus in PR #96.
  TimeInput's native editing decision closed and merged as `g16.029` (ledger
  48 mounted / 126 missing). NumberInput's committed-number/raw-draft and
  clean callback migration merged as `g16.030` in PR #98 (ledger 49 mounted /
  125 missing; known-delta axis 116 present / 59 not-applicable).
  A bounded Fader/Knob/XYPad audit then found real paired-machine, web gesture
  lifetime, native mounting, and accessibility-projection defects. `g16.031`
  closed paired semantics and the Svelte/React gesture and entry lifecycles
  behind one hand-authored `audioControls` vector corpus, moving no ledger
  cell; `g16.032` mounts the three controls through one bounded
  continuous-value Node event (ledger 52 mounted / 122 missing), merged in PR
  #100. This work remains separate from payload drag-and-drop. `g16.022`
  landed the web custom-surface substrate over that kernel in PR #101;
  `g16.023` merged in PR #104 with ordered logical keyboard targets preserving
  windowed reorder. `g16.024` merged in PR #107 with Tree nested intent,
  demand-driven auto-scroll, live drop revalidation, and semantic focus
  ownership on the shared web substrate. `g16.025` projected that kernel through
  renderer-neutral `poodle-node` registrations, shared `poodle-render`
  builders, and a public GPUI `DragDropController`, replacing the
  backend-global payload session so two providers own two sessions; its
  crates.io GPUI 0.2.2 capability matrix is immutable — mouse, keyboard, and
  in-window capture certified; pen, touch, and device-originated cancel remain
  unsupported debt no mouse fixture may flip. It merged in PR #108 after four
  review rounds; no ledger cell moved. PR #113 then merged `g16.026`: Tabs now
  moves with its DockRegion host-bridge consumer through split source/window
  authority and the window-owned GPUI pump. PR #115 then merged `g16.027` after
  three Northstar repair rounds: inbound files and native file drag-out, paired
  in both languages and wired through the web and GPUI controllers, with opaque
  receipts, exact per-installation inbound ownership, validation before
  eligibility, host-owned retention, and no committed export terminal because
  a native drag ending never proves a destination took the file. `g16.028`
  then closed the programme: the last three web HTML-drag owners migrated,
  EditableList/OrderBy/BlockEditor gained their native reorder result paths,
  four GPUI mounted cells moved (52 → 56 mounted, 122 → 118 missing), and the
  absence claim became executable as `effigy drift:drag-inventory`.
  `g16.033` then implemented HistoryCenter's Poodle-owned rejection surface
  and the packed v3 `HistoryEntry` proof: five distinct refusal meanings across
  TypeScript, Rust, both web shells and mounted GPUI, and an installed-tarball
  typecheck on both public Svelte import paths. It moves no ledger cell and
  claims no publication. Keyboard vertical geometry remains design-deferred;
  package publication and Loophole adoption remain separate authorized work.
  Longhorn's `AlreadyAtTarget` wire code is complete in its owning repository.
  The later Tabs/DockRegion drag migrations now have an
  approved clean public break: old DOM-shaped helpers disappear only after
  their mounted replacements pass, with no compatibility layer. Other
  component-continuation decisions remain separate; EditableLabel's accepted
  editing contract is now ready as `g16.045`.
- `g14` tested executable component conformance across Svelte, React, and
  GPUI. `g14.008` rejected the mechanism after its cost and coverage audit;
  `g14.021` preserved the useful fixes and removed the failed authority;
  `g14.022` completed the closeout. The generation is complete.
- `g15` was the release-first runway. The 175-component Svelte and React
  implementation/evidence rosters, measured native declaration/specimen
  baseline, specimen curation and review, native specimen probe, packed roster,
  the first primitive fixture inventory, and truthful release automation are
  complete. PR #68 closed the exact Button comparison; PR #69 then closed its
  measured native focus-ring defect and Stepper keyboard-entry gap. PR #66
  closed `g15.049`; PR #67 closed the GPUI/Zed dependency-licence policy gap
  without admitting GPL code. Those lanes and the `0.2.0` candidate completed;
  its workflow then failed before publication. Card `054` produced green
  replacement candidate `3d914261`; completed gate `013` tagged and published
  it as `v0.2.1` in run `32658293188`. The broken Git tag was later retracted
  after v0.2.2 replaced its fork-sourced GPUI graph; npm retains 0.2.1 for
  install stability while `latest` is 0.2.2. React remains source-only. The
  corrected v0.2.2 candidate then restored crates.io GPUI and all 16
  authoritative consumers moved to the public boundary. The operator removed
  Loophole Legacy, so its cancelled card is historical evidence rather than a
  release obligation. The generation is complete through `g15.079`.
  Jetstream backend admission remains deferred.
- `g13` is complete. Its Rust-authored component IR pilot recorded **revise**,
  then retired and unwound component generation. It remains evidence for g14.
- The first g14 machine-pinning/scene runway was reset after five merged
  batches. Its history is archived in
  [the false-start record](archive/2026-08-14-g14-machine-pinning-false-start.md).
- `g09`–`g12` are complete. Earlier generations remain historical evidence.
- Release automation is tracked separately from roadmap implementation work.



## From `docs/roadmaps/generation-index.md`

### Former Active Track


- `g16`
  - Status: active; strict-ready
  - Range: `001` closed; `002` closed — partial outcome; `003` merged in PR #77;
    `004` merged in PR #78; `005` merged in PR #79; `006` merged in PR #80;
    `007` merged in PR #81; `008` merged in PR #82; `009` merged in PR #83;
    `010` merged in PR #84; `011` merged in PR #85; `012` merged in PR #86;
    `013` merged in PR #87; `014` merged in PR #88; `015` merged in PR #90;
    `016` merged in PR #91; `017` merged in PR #92; `018` merged in PR #93;
    `019` merged in PR #94; `020` merged in PR #95; `021` merged in PR #96;
    `022` merged in PR #101 (web custom-surface substrate, no ledger movement);
    `023` merged in PR #104; `024` merged in PR #107; `025` merged in PR #108;
    `026` merged in PR #113 after two Northstar review rounds; `027` merged in
    PR #115 after three Northstar repair rounds; `028` merged in PR #118 after
    four Northstar repair rounds and closed the drag programme;
    `029` complete and merged (TimeInput native parity, ledger 48/126); `030`
    merged in PR #98 (NumberInput value/draft/mounted parity, ledger 49/125);
    `031` merged in PR #99; `032` merged in PR #100 (continuous audio native
    mounted parity, ledger 56/118 after `028`); `033` merged in PR #120;
    `034` merged in PR #124 after four exact-head review rounds and delivered
    the shared motion policy and five-family pilot; `035` merged in PR #123 after
    two native-oracle repair rounds and closed the independent MarkdownEditor
    bounded-preview issue; `036` merged in PR #127 with the paired-web Tree
    external drop-authority adapter; `037`–`044` completed the DesEngs research
    wave in merged PRs #128–#135; `045` merged in PR #155; `046` merged in PR
    #154; `047` merged in PR #152 after one exact-head repair round; `048`
    merged in PR #153 with a verified static-fallback benchmark verdict; `049`
    merged in PR #156; `050`, `053`–`061` are complete; independent `055`
    merged in PR #151; `062` merged in PR #170; `063` and `064` merged in PRs
    #167 and #168; `066` merged in PR #171; `065` merged in PR #172; `067`
    merged in PR #173; `068` merged in PR #174; `069` merged in PR #175;
    `070` merged in PR #176; `071` merged in PR #177; `072` merged in PR #178;
    `073` merged in PR #179; `074` merged in PR #180 after three exact-head
    review rounds; `075` merged in PR #181 after independent exact-head review;
    `076` merged in PR #182 after one repair/re-review round; `077` merged in
    PR #184 after one repair/re-review round; `078` merged in PR #185 after
    parallel proof preparation and independent exact-head review; `079` merged
    in PR #183 after two repair/re-review rounds; `080` merged in PR #186 after
    one repair/re-review round; `081` merged in PR #187 after a preparation
    token-recipe repair and independent finalization review; `082` merged in
    PR #188 after preparation repair and one final receipt-claim correction;
    `083` merged in PR #190 after two preparation repair rounds and exact-head
    M1 review
  - Aim: use one current active-cohort evidence ledger to select and close
    bounded semantic and mounted-behavior gaps without inventing another
    conformance authority
  - Completed: `g16.084` AgentQuestion merged in PR #189 at
    `5c8f5e44383b221c31efab59e10ec6d1312fc234`; the mounted Nucleus cohort is
    21/29.
  - Completed: `g16.085` ModelPicker merged in PR #191 at
    `7a44013c8aa3fc69fffeb7f56f5d7cbe4cf762fe`; the mounted Nucleus cohort is
    22/29.
  - Completed: `g16.086` StatusIndicator merged in PR #192 at
    `9edb437d463fb55e9b6d953c513a78441dec4bab`; the mounted Nucleus cohort is
    23/29.
  - Completed: `g16.088` ConfirmAction merged in PR #193 at
    `34fb80b40bc840a31959bf44b496f24d27c12a3f`; the mounted Nucleus cohort is
    24/29.
  - Completed: `g16.087` Callout merged in PR #194 at
    `e674faac7f0c37d742b22d7a782a87bfd4875621`; the mounted Nucleus cohort is
    25/29.
  - Completed: `g16.090` CommandPalette merged in PR #195 at
    `f8e88fb458256746f941927ca3b29ab764da52c4`; the mounted Nucleus cohort is
    26/29.
  - Completed: `g16.089` DetailItem merged in PR #196 at
    `c723ab66b89d8f9a5f71b95b1512fbf566b926a9`; the mounted Nucleus cohort is
    27/29.
  - Completed: `g16.091` ToastHost merged in PR #197 at
    `4a615e99046fa9e6dc14801ef1e6f60760336fc2`; the mounted Nucleus cohort is
    28/29.
  - Completed: `g16.094` ordinary Cargo scope classification merged in PR #200
    at `f7ae38d9f7e644de6d39de43363dd77bbf75f842`; dependency-only Cargo changes
    no longer inherit release-version treatment in ordinary installed smoke.
  - Completed: `g16.092` native fresh-consumer repair merged in PR #199 at
    `17534f484665bbbdd93e2ec70bec521318201941`; stable and Rust 1.95 consumer
    graphs now pass with a freshly emitted 28-receipt identity.
  - Completed: `g16.093` MessageCenter merged in PR #198 at
    `06de812f7037eeca204d89c72fb4c586723600eb`; the Nucleus mounted cohort is
    complete at 29/29. This closes M1 only, not A1/V1/V2/M2 or adoption.
  - Completed: `g16.111` Nucleus A1 accessibility receipt foundation merged in
    PR #215 at `3dea40372063a05b46a550f8f69648564506a949`; Switch and Tabs
    provide validated mounted A1 receipts, while Select's five-attribute
    divergence is recorded for NP-2 rather than repaired in this lane.
  - Ready frontier (`../roadmaps/dispatch.md`): `g16.103` is the urgent,
    operator-authorized release tarball verifier repair. It aligns stale
    `package/src/**` workflow assertions with the canonical compiled
    `package/dist/**` boundary and is the serial predecessor of `g16.097`.
    No remaining lane among the
    `g16.100`–`g16.102` tranche; `g16.102` Tabs fill layout is merged in PR #207
    at `a74e50955`; `g16.100` DockRegion `showTabs` portable is
    merged in PR #206 at `2db86aadd`;
    `g16.098` cold-checkout web board repair is merged in PR #203 at
    `c8636c699`, `g16.099` React prop port tranche is merged in PR #204 at
    `660b9510d`, and `g16.101` Tree accessible name is merged in PR #205 at
    `1d8e6aeab`. `g16.096` Linux board is complete in PR #201. The
    coordinator-executed `g16.097` re-certification passed local gates at
    `b4158a1b` but stopped before tag on the stale release verifier; it resumes
    only from the repaired post-`103` main tip. The first `v0.3.0` tag is
    retracted; nothing was published. No additional mounted receipt child. Later Nucleus
    evidence waits on its explicit accessibility, visual-lab, consumer, and
    operator authority gates.
    `g16.045`–`g16.050` and `g16.053`–`g16.061` are complete; the immutable
    `0.3.0` candidate merged in PR #165 without release authority.
  - Independent completed lane: `g16.060` paired web Tabs controlled-panel
    focus transfer merged in PR #164 and passed its Figmatic consumer proof.
  - Independent completed lane: `g16.055` drag-source pointer-gesture browser
    suppression merged in PR #151; it remains outside the post-triage
    continuation map.
  - Blocked frontier: `g16.051` on `050`, the operational
    Button lab, and its separately accepted icon adapter/manifest extension;
    `g16.052` on named reviewers, approvals, and orchestrator run custody;
    Publication/adoption remains gated on the merged `g16.054` candidate plus separate
    release and sibling authority; `g16.053` and `g16.059` are complete.
    Citations, nested menus, the visual tranche, public IconMorph,
    publication/adoption, CS20, keyboard geometry, GPUI accessibility,
    Jetstream, and the separate holistic promotion batch retain the gates
    recorded in `g16/component-continuation-runway.md`.
  - Next move: the coordinator dispatches `g16.103`, merges it after exact-head
    review and green main proof, then performs `g16.097` release certification.
    The visual lab has repository authority
    (`inflatable-cookie/poodle-lab`, 2026-09-04); VL-1 is planned in that
    repository. Loophole adoption follows the npm proof. Nucleus A1 and M2/V2
    remain the next Poodle programme choices after the lab bootstrap.
