# g16.011 — IconButton Activation, Toggle, And Mounted Parity

Status: complete
Opened: 2026-08-27
Closed: 2026-08-27
Depends on: merged `g16.010` / PR #84
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/icon-button.md`,
`parity-evidence-ledger.md`,
`../../triage/20260827-125702-post-g16-010-native-lane-decision.md`

## Goal

Make shared Rust IconButton expose the contract's command and toggle outcomes,
project its native tooltip text and semantic state, and prove pointer plus
keyboard behavior through real mounted GPUI dispatch.

The generated ledger may move exactly one cell: IconButton GPUI mounted
behavior from `missing` to `mounted`, taking totals from 39 to 40 mounted and
135 to 134 missing. Known-delta totals stay 115 present / 60 not-applicable.

## Current Evidence

- Svelte and React agree: an available icon button dismisses its tooltip,
  emits the inverse pressed value when toggle mode is active, then fires the
  command callback. Enter and Space use the same activation path.
- Toggle mode is active when either `pressed` or `defaultPressed` is present.
  `pressed` is controlled; `defaultPressed` seeds the uncontrolled web state.
- `IconButtonSpec` carries `is_pressed`, `default_pressed`, `tooltip`,
  accessible label, loading/disabled state, disclosure state, and control
  identity. No new public data field is needed for mounted behavior.
- `IconButtonSpec::uses_pressed_semantics` and the renderer read only
  `is_pressed`; `default_pressed` has no effect in Rust.
- `poodle-render::icon_button` accepts only `on_click`. It cannot separately
  report the resulting pressed value. The GPUI compatibility wrapper always
  passes `None`, although the specimen bypasses it and wires a few direct
  renderer callbacks.
- The node vocabulary and GPUI backend already support `Node.tooltip`,
  activation, button roles, accessible labels, toggled state, tab position,
  and `FocusRing`. IconButton does not project those channels as one complete
  target.
- The generated evidence ledger records focused web behavior and a GPUI
  specimen route, but no named mounted GPUI regression.

## Fixed Behavior Envelope

### Command and toggle dispatch

- Keep `icon_button(spec, ctx, on_click)` as the simple command/composition
  entry point. Add a handler-bundle entry point for command plus
  `on_pressed_change`; do not rewrite every composite caller or add an alias.
- Toggle mode is active when `is_pressed` or `default_pressed` is present.
  Effective current state is `is_pressed.or(default_pressed).unwrap_or(false)`.
- Available toggle activation reports exactly one inverse boolean through
  `on_pressed_change`, then invokes `on_click` exactly once. This order matches
  Svelte and React.
- A command-only button invokes `on_click` once and never manufactures a
  pressed-change event.
- Disabled and loading buttons invoke neither callback through pointer,
  Enter, or Space.
- The Rust renderer remains stateless: the host owns current state and
  rebuilds the spec after a reported change. `default_pressed` is the seed,
  not hidden renderer state.

### Native target and tooltip projection

- The same square target owns pointer and keyboard activation, explicit
  button role, accessible label, sequential focus, optional toggled state,
  disclosure state, and the standard structured focus ring.
- Available buttons use `tab_index=0`. Disabled/loading buttons are inert and
  excluded from sequential focus.
- The rendered toggled state uses the same effective current state as the
  emitted next value.
- Project tooltip text through `Node.tooltip`: explicit `tooltip` wins;
  otherwise a non-empty `aria_label` is the fallback. Empty text projects no
  tooltip.
- GPUI's native tooltip chrome owns placement, timing, and paint. Record that
  bounded runtime mechanism in the contract; do not build the Tooltip overlay
  component or extend the backend in this card.
- Align the Rust `tooltip_placement` default with the contract even though the
  GPUI native-tooltip projection does not consume placement yet.

### Specimen and mounted proof

- Expose command and pressed-change handlers on the GPUI compatibility
  wrapper, and use the production handler-bundle path for the interactive
  specimen examples.
- Keep the existing human-centered specimen groups. Preserve the compact last
  action/toggle feedback; do not add a conformance matrix.
- Add one named mounted regression through the production renderer, node
  backend, focus chain, pointer hit testing, and keyboard dispatch. It proves:
  - pointer command activation emits once;
  - Enter and Space emit the resulting toggle value and the host rebuilds the
    toggled state;
  - `default_pressed=true` starts toggled and first activation reports false;
  - explicit and fallback tooltip text reach `Node.tooltip`;
  - role, accessible name, tab position, focus ring, disclosure state, and
    toggled state ride the same target; and
  - disabled/loading targets never emit and are skipped by sequential focus.

## Explicit Non-Claims

- This card does not redesign tooltip visuals, create a tooltip overlay,
  reproduce the web timer/Escape lifecycle, or add a new node/backend channel.
- It does not claim broad native assistive-technology coverage. The mounted
  regression proves declared node/backend semantics only.
- It does not add `aria-busy` or `aria-describedby` vocabulary to
  `poodle-node`; those remain outside this bounded mounted lane.
- It does not change Svelte or React behavior or public props. Their focused
  tests are reference evidence.
- It does not redesign variants, tones, sizes, densities, icon sizing,
  loading animation, active translation, or color recipes.
- It does not admit Jetstream, promote visual comparison, or touch release,
  package, workflow, downstream, or sibling-repository surfaces.
- It does not include EditableLabel, NumberInput, TimeInput, Pill, or
  IconButton-consuming composites except for compilation-only call-site
  repair if the additive renderer entry point exposes one.
- It does not move any ledger row except IconButton or any evidence column
  except GPUI mounted behavior.

## Delivery

### 1. Reconcile declaration and focused renderer behavior

- Add `is_toggle_mode` and `current_pressed` helpers that honor both pressed
  inputs. Align the tooltip-placement default.
- Add a focused handler-bundle renderer entry point while retaining the
  existing command helper for component composition.
- Cover command order, next pressed value, seeded state, disabled/loading
  suppression, tooltip fallback/override, and semantic target projection in
  focused Rust tests.
- Update the IconButton contract only where its native notes and Known Deltas
  need to describe the admitted native-tooltip mechanism and host-owned state
  rebuild. Do not weaken active-cohort semantic requirements.

### 2. Wire GPUI and mounted evidence

- Add wrapper builders for command and pressed-change handlers.
- Route interactive specimen examples through that wrapper/handler path and
  preserve their current concise feedback.
- Add the named mounted regression using real hit testing and keyboard
  dispatch. Fixture-local ids may be stamped after render; no public instance
  id is required.

### 3. Prove and close

- Regenerate the evidence ledger and verify only IconButton changes: 40
  mounted / 134 missing. Known-delta totals remain 115 / 60.
- Mark the source triage note resolved, add one August execution log, close
  this card, and return g16 to an orchestrator evidence checkpoint. Do not
  compile or start `g16.012` in the worker thread.

## Acceptance

- [x] Command-only activation invokes `on_click` once and emits no pressed
      change.
- [x] Controlled and seeded toggle activation report the inverse current
      state once, before the command callback, and host rebuilds project the
      new toggled state.
- [x] Disabled/loading buttons invoke neither callback and are not sequential
      focus stops.
- [x] The target projects button role, accessible label, toggled/disclosure
      state, tab position, and the standard focus ring together.
- [x] Explicit tooltip text and the accessible-label fallback reach
      `Node.tooltip`; empty tooltip text is omitted.
- [x] The existing GPUI specimen visibly demonstrates command and toggle
      outcomes through the production wrapper path without becoming
      exhaustive.
- [x] One named mounted regression proves pointer, Enter, Space, seeded toggle,
      rebuild, focus traversal, and inert-state behavior through production
      dispatch.
- [x] Focused Svelte and React IconButton tests stay green without behavior
      changes.
- [x] The ledger changes only IconButton from missing to mounted: 40 mounted /
      134 missing; known-delta totals remain 115 / 60.
- [x] One August log records the defect, repair, evidence, validation, exact
      non-claims, and next orchestrator checkpoint.

## Outcome

Complete. The full record is
`../../logs/2026-08/20260827-g16-011-icon-button-activation-toggle-and-mounted-parity.md`.

## Writable Scope

- `docs/contracts/components/icon-button.md`
- `packages/contracts/components/src/icon_button.rs`
- `packages/render/src/icon_button.rs`
- IconButton-only compatibility/specimen state under `packages/gpui/preview/`
- the smallest IconButton mounted regression changes in
  `packages/gpui/preview/tests/headless_regressions.rs`
- focused Svelte/React IconButton tests only if a test-only correction is
  required; do not change web implementation behavior
- generated parity ledger/check surfaces only for the one IconButton mounted
  cell
- this card, its source triage note, one August log, g16/front-door status,
  and `PAPERCUTS.md` only for new execution friction

Do not edit other component contracts or implementations, shared node/backend
APIs, theme/token definitions, visual fixtures, accessibility reports,
versions, release metadata, workflows, downstream repositories, or sibling
runtime repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-specs` and `poodle-render` IconButton tests;
- focused Svelte and React IconButton tests;
- the named mounted IconButton regression;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy drift:handlers`, `effigy drift:events`, and relevant contract/spec
  drift selectors;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:native`;
- `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, or publication selectors.

## Stop Conditions

- Svelte and React disagree on command order, toggle-mode detection, or the
  resulting pressed value.
- Honest behavior needs hidden renderer-owned mutable state, a breaking
  `IconButtonSpec` migration, or a new node/backend capability.
- Native tooltip projection cannot reuse `Node.tooltip` without constructing
  the Tooltip overlay or changing shared backend behavior.
- The standard focus ring or toggled semantics cannot reuse existing node
  channels without changing other components.
- Mounted proof requires direct handler invocation, fixture-only behavior, or
  bypassing production focus/key dispatch.
- The ledger generator changes another row or promotes accessibility/visual
  evidence not proved by this card.
- Validation exposes EditableLabel, NumberInput, TimeInput, Jetstream, release,
  or downstream work outside this runway.
