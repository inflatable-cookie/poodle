# g16.003 — RadioGroup Native Identity And Mounted Parity

Status: complete
Opened: 2026-08-26
Completed: 2026-08-26
Depends on: closed `g16.002`; operator decision recorded in
`../../triage/20260826-123030-selection-control-stops.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/radio-group.md`,
`parity-evidence-ledger.md`
execution log: `../../logs/2026-08/20260826-g16-003-radio-group-native-identity-and-mounted-parity.md`

## Outcome

RadioGroup's GPUI mounted-behaviour cell is closed. Shared native rendering
takes `RadioGroupHandlers::new(instance_id)`; option runtime ids derive from
that host-owned scope. The named mounted regression drives pointer selection,
same-value inertia, both orientation axes, wrap, disabled-option skip,
disabled-group inertia, and two same-valued instances with independent focus
through the real GPUI backend/input path and host rebuilds.

The generated ledger moves only RadioGroup's GPUI mounted-behaviour cell from
`missing` to `mounted` (32 → 33 mounted; 142 → 141 missing). GPUI accessibility
stays `manual`. Svelte and React are unchanged. ToggleGroup is not in this
lane.

## Fixed Decisions

- `name` remains the optional web form-group name. Web runtimes continue to
  generate a unique mounted-instance name when it is absent.
- Native focus identity is runtime construction data, not a form name or
  semantic option value.
- `poodle_render::radio_group` receives a required
  `RadioGroupHandlers::new(instance_id)` bundle. The stable scope is mandatory
  for every native construction; the callback remains optional.
- No fallback may derive native identity from render order, a constant group
  label, the selected value, or option values alone.
- RadioGroup keeps exclusive radio semantics: one tab stop, selected-or-first
  enabled focus entry, orientation-aware arrow movement, wrapping, disabled
  skipping, and same-value inertia.
- The host owns selected state. Accepted pointer or arrow selection emits the
  chosen value; the host rebuild supplies the next spec.

## Delivery

### 1. Introduce the explicit native handler boundary

- Add `RadioGroupHandlers` beside the shared renderer with:
  - required `instance_id: String` supplied through `new(instance_id)`;
  - optional `on_change: Arc<dyn Fn(&str) + Send + Sync>` through a clear
    builder or field surface.
- Change `poodle_render::radio_group` to accept the bundle instead of a bare
  optional callback.
- Migrate every in-repo call site mechanically. Static specimens still provide
  a stable descriptive scope and leave `on_change` absent.
- Deferred Jetstream call sites may receive compile-only scopes; do not change
  its backend behavior, admission status, evidence, or preview contract.

### 2. Emit instance-safe radio nodes

- Give every enabled option a backend runtime id derived from the required
  group scope and option value.
- Keep readable semantic ids separate from backend runtime ids.
- Emit per-option `RadioButton` role, selected/toggled state, disabled state,
  and roving `tab_index` projection.
- Add the contracted focus treatment so GPUI creates and paints real focus
  handles.
- The selected enabled option is the tab stop. If none is selected, or the
  selected option is disabled or unknown, the first enabled option is the tab
  stop. A disabled group has no tab stop.

### 3. Implement contracted navigation

- Vertical groups respond to Arrow Up/Down; horizontal groups respond to Arrow
  Left/Right.
- Movement wraps and skips disabled options.
- A successful move emits the target value once and returns that option's
  scoped focus target. Unrelated-axis arrows are inert.
- Pointer/activation selection emits once for a different enabled value.
  Same-value selection, disabled options, disabled groups, and unknown values
  emit nothing.
- Use existing node keys and backend focus requests. Do not add a generic
  action language, shared fixture schema, or new node vocabulary.

### 4. Prove the mounted result and update evidence

- Add one readable named headless GPUI regression, or the smallest coherent
  pair, that mounts the real node tree and drives backend pointer/keyboard
  input with host rebuilds.
- Prove pointer selection, same-value inertia, both orientation axes, wrap,
  disabled-option skip, disabled-group inertia, and selected state after
  rebuild.
- Mount two groups with identical option values and prove their focus requests
  and handles stay independent.
- Register the exact landed regression name in the parity-ledger generator and
  regenerate the ledger. Only RadioGroup's GPUI mounted-behaviour cell moves
  from `missing` to `mounted`; totals move 32 → 33 mounted and 142 → 141
  missing. GPUI accessibility remains `manual`.

## Acceptance

- [x] Every native RadioGroup construction provides a non-empty stable
      interaction scope through `RadioGroupHandlers`; no collision-prone
      fallback remains.
- [x] Two mounted same-valued groups retain independent backend focus identity.
- [x] The real mounted GPUI path proves exclusive selection, same-value
      inertia, orientation-aware arrow movement, wrap, disabled-option skip,
      and disabled-group inertia through host rebuilds.
- [x] Each enabled option exposes real focus tracking, one roving tab stop, and
      its contracted radio selection state; disabled paths are not focusable.
- [x] Svelte and React APIs and behavior remain unchanged.
- [x] The generated ledger changes exactly the RadioGroup GPUI
      mounted-behaviour cell and derived totals; accessibility and visual cells
      do not move.
- [x] One August execution log records the API migration, mounted evidence,
      validation, and any remaining gap.

## Writable Scope

- `packages/render/src/radio_group.rs` and its focused tests
- the smallest in-repo Rust call-site set required by the handler signature,
  including preview compatibility wrappers and static deferred-Jetstream call
  sites for compilation only
- `packages/gpui/preview/tests/headless_regressions.rs` and the smallest
  existing headless-driver support required by the named inputs
- RadioGroup preview modules only where stable native scopes must be supplied
- `scripts/parity-evidence-ledger.ts`, its focused test, and generated
  `parity-evidence-ledger.md`
- this card, `docs/roadmaps/g16/README.md`, roadmap/generation front doors, and
  one August execution log
- `PAPERCUTS.md` for new execution friction only

Do not edit RadioGroup's web implementations or public web props, ToggleGroup,
other component semantics, generic `RenderContext`, node vocabulary, specimens
beyond mechanical native-scope plumbing, visual fixtures or thresholds,
accessibility reports, Jetstream backend behavior, workflows, versions,
releases, or downstream repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-render` RadioGroup tests;
- focused compilation/tests for each Rust crate whose call sites change;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy test:parity-evidence-ledger`;
- `effigy check:parity-evidence-ledger`;
- `effigy ci:native`;
- `effigy ci:web` to prove the reference runtimes stayed unchanged;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything remains headless. Do not run `*-windowed`, native visual,
Jetstream preview/QA, release, tag, or publication selectors.

## Stop Conditions

- The handler bundle cannot carry stable identity without changing the web API
  or generic render context.
- The mounted backend cannot distinguish semantic node ids from focus runtime
  ids with existing vocabulary.
- Contracted movement requires a new cross-runtime action language, shared case
  corpus, generated adapter, or normalized observation model.
- Svelte and React disagree on selection semantics, or a requested repair
  contradicts the RadioGroup contract.
- The proof can pass only through direct handler invocation, spec inspection,
  or specimen construction rather than mounted input and host rebuild.
- An unrelated evidence cell moves, or validation requires windowed execution,
  workflow changes, release mutation, or Jetstream admission.

## Continuation

Return the handler/API diff, mounted regression names, two-instance identity
proof, regenerated ledger totals, validation, and execution log to the
orchestrator. Do not implement ToggleGroup or compile another card. After
orchestrator review and operator merge, the orchestrator promotes the accepted
ToggleGroup decision and compiles its separate semantic/API/mounted-parity
lane.
