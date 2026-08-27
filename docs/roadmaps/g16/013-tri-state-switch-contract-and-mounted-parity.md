# g16.013 — TriStateSwitch Contract And Mounted Parity

Status: ready
Opened: 2026-08-27
Depends on: merged `g16.012` / PR #86; operator-approved breaking migration in
`../../triage/20260827-160028-post-g16-012-native-lane-decision.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/tri-state-switch.md`,
`parity-evidence-ledger.md`

## Goal

- [ ] Give Svelte, React, shared Rust, and GPUI one ternary
      `excluded | default | included` value contract.
- [ ] Remove the legacy checkbox-shaped Rust compatibility surface rather than
      preserving aliases or fallbacks before v1.0.
- [ ] Make native radio selection, roving focus, disabled behavior, and
      instance identity match the documented web authority.
- [ ] Move exactly TriStateSwitch's GPUI mounted-behaviour cell from `missing`
      to `mounted`: 41 → 42 mounted and 133 → 132 missing. Keep known-delta
      totals at 115 present / 60 not-applicable.

## Current Evidence

- The contract and both web runtimes use `TriStateValue` in fixed order:
  excluded, default, included. Their default is `default`.
- Both web runtimes route accepted changes through `singleSelectTransition`.
  Selecting the current value, a disabled group, or an unknown value is inert.
- Native radio behavior requires one checked tab stop, Left/Right movement of
  selection and focus, Space selection, and disabled focus suppression.
- `TriStateSwitchSpec` instead stores `CheckState`, exposes `with_state`, and
  translates unchecked/mixed/checked into excluded/default/included. Its
  default therefore resolves to excluded.
- The Rust spec carries an undocumented general `label` field and legacy
  `TriStateValue::{from_check_state,to_check_state}` conversion helpers.
- The shared renderer makes all three segments focusable, leaves disabled
  segments focusable, gives no segment a stable runtime identity or structured
  focus ring, and emits when the already-selected segment is activated.
- The GPUI compatibility wrapper stamps every instance with the same root id
  and has no host-owned instance scope.
- The evidence ledger records focused web behavior and GPUI construction, but
  no named mounted TriStateSwitch behavior regression.

## Fixed Contract

### Semantic Rust value

- `TriStateSwitchSpec` stores `pub value: TriStateValue` and defaults to
  `TriStateValue::Default`.
- Replace `with_state(CheckState)` with `with_value(TriStateValue)`. Do not
  retain an alias, deprecated method, alternate constructor, silent
  conversion, or compatibility field.
- Remove the undocumented `label` field and `with_label`. The required group
  name remains `aria_label`; segment names remain the three option labels.
- Remove TriStateSwitch-only CheckState conversion prose and helpers when no
  other consumer needs them. `CheckState` itself remains available for
  checkbox semantics.
- Migrate Poodle-owned GPUI and deferred-Jetstream call sites directly to the
  semantic enum. Adapter identity strings and public component names remain
  unchanged.

### Native interaction boundary

- Add a `TriStateSwitchHandlers` boundary beside the shared renderer with a
  required non-empty, lifetime-stable `instance_id` and optional
  `on_value_change: Arc<dyn Fn(TriStateValue) + Send + Sync>`.
- Every native construction supplies that scope. Never derive it from render
  order, the selected value, labels, or a process-global counter.
- Semantic ids and backend runtime focus ids remain separate. Segment runtime
  ids derive from the authored instance scope plus fixed semantic value.
- The host owns selected state. Accepted pointer, Space, or Left/Right input
  reports the resulting `TriStateValue`; the host rebuilds the spec.

### Radio behavior

- The root exposes `RadioGroup` role and its required accessible label. Each
  segment exposes its authored label, `RadioButton` role, and selected state.
- Exactly the selected enabled segment has `tab_index=0`; other segments have
  `-1`. A disabled group has no focusable segment and no sequential tab stop.
- Left/Right wraps through excluded/default/included, reports the target once,
  and requests focus for that instance-scoped target. Space uses normal
  activation for the focused segment.
- Selecting the current value is inert. Disabled input is inert. No handler is
  installed on a disabled target.
- Each enabled segment uses the established structured control focus ring.
  Reuse existing node/backend interaction, key, runtime-id, and focus-request
  channels. A new generic backend capability is a stop.

## Execution Plan

- [ ] **Batch 1 — migrate the semantic Rust contract.** Replace legacy spec
      storage/builders/defaults and conversion helpers, correct the component
      contract's conflicting Jetstream same-value note, migrate all in-repo
      Rust call sites mechanically, and add focused `poodle-specs` tests.
- [ ] **Batch 2 — repair native selection and focus.** Introduce the required
      scoped handler boundary, project radio semantics and one roving tab stop,
      implement same-value/disabled inertia and Left/Right focus movement, and
      add focused renderer tests including two-instance identity.
- [ ] **Batch 3 — prove production dispatch and close evidence.** Wire the GPUI
      wrapper/specimen through stable scopes and host rebuilds, add one named
      mounted headless regression, regenerate only the TriStateSwitch ledger
      cell, close the decision/card/log/front doors, and run the full headless
      validation board.

## Specimen And Mounted Proof

- Keep the existing human-centred specimen groups and axes. Replace semantic
  state translation with `TriStateValue` and give every native instance a
  stable descriptive scope. Do not add an exhaustive conformance matrix.
- Keep the compact live value readout for the interactive example. The initial
  state must now truthfully demonstrate the contracted `default` value.
- Add one readable named mounted regression through the production renderer,
  GPUI node backend, hit testing, focus chain, key dispatch, and host rebuild.
  It proves:
  - initial Default selection, root/segment roles, labels, selected state, and
    exactly one selected tab stop;
  - pointer Excluded and Included selection with one callback and rebuilt
    selected state;
  - selected-value activation is inert;
  - Left/Right movement, wrap, callback payload, and requested focus;
  - Space activation through the focused production target;
  - two same-valued instances keep independent runtime/focus identity; and
  - disabled controls emit nothing and are skipped by sequential focus.
- Direct handler invocation, spec inspection alone, or fixture-only state
  changes do not satisfy mounted proof.

## Explicit Non-Claims

- This card does not change public Svelte or React props or behavior.
- It does not add uncontrolled native state, form submission/name behavior,
  arbitrary option counts, option reordering, or a general segmented-control
  abstraction.
- It does not preserve legacy `CheckState` storage/builders/conversions for
  TriStateSwitch. The approved pre-1.0 migration is deliberately clean.
- It does not admit or behaviorally repair Jetstream. Deferred Jetstream Rust
  callers receive compilation-only semantic migration and stable descriptive
  scopes where required.
- It does not claim broad native assistive-technology proof or GPUI visual
  comparison. Existing accessibility and visual ledger cells do not move.
- It does not change NumberInput, EditableLabel, Accordion, RadioGroup,
  ToggleGroup, SegmentedControl, or shared web selection machinery.
- It does not touch releases, versions, workflows, downstream repositories,
  publication, or sibling runtime repositories.

## Acceptance Criteria

- [ ] `TriStateSwitchSpec` uses `TriStateValue`, defaults to Default, exposes
      `with_value`, and contains no legacy `state`, `with_state`, undocumented
      general `label`, or TriStateSwitch-only CheckState compatibility path.
- [ ] All Poodle-owned Rust call sites use the semantic enum directly and the
      active cohort still compiles; Jetstream changes are mechanical only.
- [ ] Every native construction provides non-empty stable interaction scope
      through `TriStateSwitchHandlers`; no collision-prone fallback remains.
- [ ] Native semantics expose one labelled radiogroup, three labelled radios,
      correct selected state, one selected tab stop, and structured focus rings.
- [ ] Pointer, Space, and Left/Right report only changed resulting values;
      arrows wrap and move focus inside the originating instance.
- [ ] Same-value and disabled paths emit nothing; disabled segments are not
      focusable or sequential tab stops.
- [ ] Two mounted instances with the same current value retain independent
      runtime and focus identity through host rebuilds.
- [ ] The curated GPUI specimen starts at Default and uses production scoped
      handlers without adding an exhaustive example matrix.
- [ ] One named mounted regression proves the fixed behavior through real
      backend dispatch.
- [ ] The generated ledger changes only TriStateSwitch to 42 mounted / 132
      missing; known-delta totals stay 115 / 60 and visual/accessibility cells
      remain unchanged.
- [ ] One August log records the approved break, behavior repair, evidence,
      validation, exact non-claims, and next orchestrator checkpoint.

## Writable Scope

- `docs/contracts/components/tri-state-switch.md`
- `packages/contracts/components/src/tri_state_switch.rs`, the
  TriStateSwitch-specific portion of `packages/contracts/components/src/types.rs`,
  exports, and focused tests
- `packages/render/src/tri_state_switch.rs`, its export surface, and focused
  tests
- the smallest Poodle-owned Rust call-site set required by the clean spec and
  handler migration, including GPUI specimen/compatibility code and deferred
  Jetstream compile-only callers
- TriStateSwitch-only code under `packages/gpui/preview/src/specimens/` and
  `packages/gpui/preview/src/node_compat.rs`
- the smallest TriStateSwitch mounted regression changes in
  `packages/gpui/preview/tests/headless_regressions.rs`
- `scripts/parity-evidence-ledger.ts`, its focused test, and generated
  `parity-evidence-ledger.md` for the one mounted cell
- this card, its source decision, one August log, g16/front-door status, and
  `PAPERCUTS.md` only for new execution friction

Do not edit web component implementations, shared TS selection machinery,
generic node/backend APIs, other component contracts or semantics, theme/token
definitions, visual fixtures, accessibility reports, package versions,
workflows, releases, downstream repositories, or sibling repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-specs` and `poodle-render` TriStateSwitch tests;
- focused compilation/tests for every Rust crate whose call sites change;
- focused Svelte and React TriStateSwitch tests to prove the reference behavior
  stayed unchanged;
- the named mounted TriStateSwitch regression;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy drift:handlers`, `effigy drift:events`, `effigy drift:roles`, and
  relevant contract/spec drift selectors;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:rust`, `effigy ci:native`, and `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, or publication selectors.

## Stop Conditions

- The web runtimes or detailed contract disagree on value, default, ordering,
  same-value inertia, disabled behavior, or callback payload.
- The clean Rust migration requires a compatibility alias, fallback, or a
  semantic change outside TriStateSwitch rather than mechanical call-site work.
- Contracted radio behavior cannot reuse existing node/backend key, focus,
  runtime-id, role, selected-state, and focus-ring channels.
- Stable identity cannot be supplied by the host without deriving it from
  render order, state, labels, or a process-global counter.
- Mounted proof requires direct handler invocation, fixture-only behavior, or
  bypassing production hit testing, focus, key dispatch, or host rebuild.
- The ledger generator changes another row or promotes accessibility/visual
  evidence not proved by this card.
- Validation exposes NumberInput, EditableLabel, Accordion, Jetstream
  admission, release, downstream, workflow, or publication work outside this
  runway.

## Continuation

Return the semantic API diff, handler boundary, focused test names, mounted
regression name, two-instance identity proof, regenerated ledger totals,
validation, and execution log to the orchestrator. Do not compile or implement
`g16.014`. After operator merge, the orchestrator returns to the measured
ledger and chooses the next bounded parity lane.
