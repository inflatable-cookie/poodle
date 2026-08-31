# g16.004 — ToggleGroup Semantic API And Mounted Parity

Status: complete — merged in PR #78
Opened: 2026-08-26
Completed: 2026-08-26
Depends on: complete `g16.003`; accepted decision fixed in the ToggleGroup
contract
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/toggle-group.md`,
`parity-evidence-ledger.md`
execution log: `../../logs/2026-08/20260826-g16-004-toggle-group-semantic-api-and-mounted-parity.md`

## Outcome

ToggleGroup has one observable selection and focus contract across Svelte,
React, shared Rust, and GPUI. Every accepted activation reports the resulting
typed selection. Single mode behaves as a horizontal radiogroup with one
roving tab stop; multiple mode behaves as ordinary pressed toggle buttons.
Native focus identity is scoped per mounted instance, and one named headless
GPUI regression proves the full controlled host-rebuild path.

The generated ledger moves only ToggleGroup's GPUI mounted-behaviour cell from
`missing` to `mounted` (33 → 34 mounted; 141 → 140 missing). GPUI
accessibility stays `manual`. Jetstream backend admission stays deferred.

## Fixed Decisions

- Resulting selection is the public callback payload in every runtime:
  `string | null` / `ToggleGroupValue::Single` in single mode and `string[]` /
  `ToggleGroupValue::Multiple` in multiple mode.
- Shared transitions own selection membership. Hosts receive the result and
  rebuild controlled state; they never reconstruct a set from an activated
  option.
- An accepted same-value activation in non-deactivating single mode still
  emits the same resulting value, preserving the existing machine contract.
- Single mode is a horizontal radiogroup. It has one selected-or-first-enabled
  tab stop. Left/Right wraps, skips disabled options, selects the target, and
  moves focus. Space/Enter and pointer activation use the same transition.
- Multiple mode is a group of pressed buttons. Every enabled item remains in
  ordinary tab order; Left/Right is not intercepted.
- Native construction requires a lifetime-stable host scope through
  `ToggleGroupHandlers::new(instance_id)`. Semantic ids and backend runtime
  focus ids remain separate.
- The existing headless `ToggleGroupValue` and `toggle_group_transition` are
  the Rust result authority. Do not add a second selection enum or transition.
- Jetstream receives compile-only call-site migration. Do not admit, exercise,
  or claim its backend.

## Delivery

### 1. Complete shared web focus machinery

- Extend the existing ToggleGroup core surface with the smallest pure helpers
  needed to derive enabled option order, the single-mode tab stop, and a
  Left/Right target. Reuse `packages/core/src/nav.ts` where it fits; do not add
  a component schema, fixture language, or new behavior authority.
- Keep `toggleGroupTransition` as the only selection transition. Focus helpers
  choose targets; selection still passes through `TOGGLE` and its
  `emitValueChange` effect.
- Add focused core tests for selected/first entry, unknown or disabled
  selection fallback, wrapping, disabled skipping, one-enabled inertia,
  disabled-group inertia, and multiple-mode non-roving behavior.

### 2. Align Svelte and React

- Project exactly one `tabindex=0` in enabled single mode. Use the selected
  enabled item, otherwise the first enabled item. Disabled groups have no tab
  stop.
- Handle Left/Right only in single mode. Prevent default, derive the target
  through shared core machinery, run the normal toggle transition, then focus
  the target inside that mounted component instance.
- Keep multiple mode as normal enabled buttons with no arrow interception.
- Keep the existing public web props and controlled/uncontrolled behavior.
  Do not add an instance-id prop.
- Use instance-local element references or root-scoped lookup. Never use a
  document-global option-value selector that can cross two same-valued groups.
- Extend focused Svelte and React tests for pointer and keyboard resulting
  payloads, selected/first tab entry, wrap, disabled skipping, deactivation,
  multiple toggling, disabled paths, and two same-valued mounted instances.

### 3. Repair the shared Rust handler boundary

- Add `ToggleGroupHandlers` beside the renderer with:
  - required `instance_id: String` supplied through `new(instance_id)`;
  - optional `on_value_change: Arc<dyn Fn(ToggleGroupValue) + Send + Sync>`.
- Change `poodle_render::toggle_group` to accept the bundle instead of the bare
  `Fn(&str)` callback. Migrate every in-repo Rust call site mechanically.
- Convert `ToggleGroupSpec` state to the existing headless context once per
  render. Every pointer or single-mode arrow activation must call
  `toggle_group_transition` and forward its owned `ToggleGroupValue` effect.
  No host-owned or test-only membership helper remains.
- Emit readable semantic item ids separately from instance-scoped runtime ids.
  Every enabled native item gets the contracted focus ring and stable focus
  handle.
- Single mode exposes one selected-or-first-enabled tab stop and Left/Right
  wrap with disabled skipping. A successful arrow activation emits the target
  result and returns its scoped focus id. Multiple mode leaves every enabled
  item focusable and installs no arrow handler.
- Static GPUI and deferred-Jetstream callers still provide stable descriptive
  scopes with no callback. Update the GPUI specimen's controlled state adapter
  to consume the typed result; delete its activated-option membership helper.

### 4. Prove the mounted result and update evidence

- Add one readable named headless GPUI regression, or the smallest coherent
  pair, that mounts the real node tree and drives backend pointer/keyboard
  input with host rebuilds.
- Prove non-deactivating single selection and same-value emission,
  `allow_deactivation` to `Single(None)`, Left/Right wrap, disabled-option
  skip, disabled-group inertia, multiple add/remove result arrays, and selected
  state after rebuild.
- Mount two single groups with identical option values and prove their focus
  requests and handles remain independent.
- Register the exact landed regression name in the parity-ledger generator and
  regenerate the ledger. Only ToggleGroup's GPUI mounted-behaviour cell moves
  from `missing` to `mounted`; totals move 33 → 34 mounted and 141 → 140
  missing. Accessibility and visual cells do not move.

## Acceptance

- [x] Core, Svelte, React, Rust headless, and shared Rust rendering agree on
      resulting-selection payloads for single, deactivating single, and
      multiple modes.
- [x] Single mode has one selected-or-first-enabled tab stop and Left/Right
      wrap with disabled skipping in Svelte, React, and GPUI.
- [x] Multiple mode keeps ordinary button tab order and ignores Left/Right in
      every active runtime.
- [x] Every native ToggleGroup construction provides a non-empty stable
      interaction scope through `ToggleGroupHandlers`; no option-only focus
      identity or activated-option callback remains.
- [x] Two same-valued mounted web groups and two same-valued mounted GPUI
      groups keep independent focus identity.
- [x] The real mounted GPUI path proves pointer and keyboard payloads,
      deactivation, multiple add/remove, disabled inertia, and host rebuilds.
- [x] Svelte and React public props remain unchanged; specimens remain curated
      and human-centred.
- [x] The generated ledger changes exactly the ToggleGroup GPUI
      mounted-behaviour cell and derived totals. GPUI accessibility remains
      `manual`; Jetstream remains deferred.
- [x] One August execution log records the semantic/API migration, mounted
      evidence, validation, and remaining gaps.

## Writable Scope

- `packages/core/src/toggle-group.ts`, its export surface, and focused tests
- `packages/svelte/components/src/ToggleGroup.svelte` and its focused test
- `packages/react/components/src/ToggleGroup.tsx` and its focused test
- `packages/contracts/headless/src/toggle_group.rs` only if a focused helper or
  test is required; do not replace the existing value/transition authority
- `packages/render/src/toggle_group.rs` and its focused tests
- the smallest Rust call-site set required by the handler signature, including
  GPUI specimen/facade code and deferred Jetstream compatibility callers for
  compilation only
- `packages/gpui/preview/tests/headless_regressions.rs` and the smallest
  existing headless-driver support required by the named inputs
- `scripts/parity-evidence-ledger.ts`, its focused test, and generated
  `parity-evidence-ledger.md`
- this card, `docs/roadmaps/g16/README.md`, roadmap/generation front doors, and
  one August execution log
- `PAPERCUTS.md` for new execution friction only

Do not edit other component contracts or semantics, generic `RenderContext`,
node vocabulary, specimens beyond mechanical typed-result/scope plumbing,
visual fixtures or thresholds, accessibility reports, Jetstream backend
behavior/admission, workflows, versions, releases, or downstream repositories.
Do not add compatibility aliases or retain the activated-option native API.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused core ToggleGroup tests and shared TS/Rust machine conformance;
- focused Svelte and React ToggleGroup tests;
- focused `poodle-headless`, `poodle-specs`, and `poodle-render` ToggleGroup
  tests plus compilation/tests for each Rust crate whose call sites change;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy test:parity-evidence-ledger`;
- `effigy check:parity-evidence-ledger`;
- `effigy ci:native`;
- `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything remains headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, or publication selectors.

## Stop Conditions

- Resulting-selection callbacks cannot use the existing headless value and
  transition without adding a second public behavior authority.
- Single-mode web focus requires a public instance-id prop or a
  document-global selector.
- Existing node keys, runtime ids, focus requests, and focus-ring vocabulary
  cannot represent the contracted native result.
- The typed handler migration requires a semantic change outside ToggleGroup
  rather than a mechanical call-site adaptation.
- Svelte and React disagree after using the same core transition/helpers, or
  an existing conformance vector contradicts the promoted contract.
- The GPUI proof can pass only through direct handler invocation, spec
  inspection, or specimen construction rather than mounted input and host
  rebuild.
- An unrelated ledger cell moves, or validation requires windowed execution,
  workflow changes, release mutation, or Jetstream admission.

## Continuation

Return the core/web/Rust API diff, focused test names, mounted regression
names, two-instance identity proof, regenerated ledger totals, validation, and
execution log to the orchestrator. Do not compile or implement another card.
After operator merge, the orchestrator reviews the measured ledger and chooses
the next bounded parity lane; no broad conformance programme is implied.
