# g16.006 — Tabs Drag, Keyboard, And Mounted Parity

Status: complete — merged in PR #80 (`9e5934af`)
Opened: 2026-08-26
Depends on: complete `g16.005`; resolved payload lifecycle fixed in the Tabs
contract
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/tabs.md`, `parity-evidence-ledger.md`

## Outcome

Tabs has one observable selection, focus, close, keyboard-reorder, and pointer-
reorder contract across Svelte, React, shared Rust, and GPUI. The existing node
payload/drop seam reports a complete semantic lifecycle on stock GPUI: one
start, hit-tested target hover/leave, a drop with the retained edge, and one end
after drop or cancellation. One named headless GPUI regression drives the real
mounted tree through controlled host rebuilds.

The generated ledger moves only Tabs' GPUI mounted-behaviour cell from
`missing` to `mounted` (35 → 36 mounted; 139 → 138 missing). GPUI
accessibility stays `manual`; visual comparison stays `missing`. Tree and
ModelCatalogueEditor remain regression consumers of the corrected payload
seam but their ledger cells do not move. Jetstream backend admission stays
deferred.

## Fixed Decisions

- Preserve the current Tabs contract. Do not narrow native Tabs to keyboard-
  only reorder and do not revise reorder semantics across the active cohort.
- Complete the existing renderer-neutral payload/drop vocabulary instead of
  combining it with the delta-only `on_drag` gesture or adding a Tabs-only
  backend channel.
- A payload source may declare semantic start and end handlers. Start fires
  once after the runtime drag threshold. End fires exactly once after a
  successful drop or cancellation, including release outside a zone and
  Escape.
- A drop zone receives hover only while its own bounds contain the pointer and
  receives a leave signal when it stops being the active target. A successful
  drop carries the last `DropEdge` computed during hover; it must not collapse
  to the default `inside` edge.
- The backend may keep one runtime-owned active payload session to preserve
  ordering and exactly-once cleanup. Components receive payload ids, target
  state, and semantic edges only — never raw coordinates or GPUI types.
- Tabs pointer drop and Alt+Arrow both run the existing Rust
  `tabs_transition` reorder authority and emit the complete next value order.
  Hosts apply that order and rebuild controlled state; they do not reproduce
  reorder math.
- Add Delete to the renderer-neutral key vocabulary for closable tabs. Keep
  Enter/Space activation on the existing activation path and ordinary arrows
  on the current orientation-aware roving-focus path.
- `onDragPrepare`'s DOM `PointerEvent` remains web-only. Native drag session
  start/end and target-change handlers carry semantic values without pretending
  to expose DOM events.
- Svelte and React public props remain unchanged. Jetstream receives only
  compile-compatible shared-vocabulary/call-site adaptation and remains
  deferred.

## Delivery

### 1. Lock the existing web and machine contract

- Extend the focused TypeScript and Rust Tabs machine cases for ordered
  `Reorder`, bounded `ReorderStep`, focus-on-moved-tab, disabled inertia, and
  close eligibility. Keep the existing machines as the only reorder authority.
- Retain or add focused Svelte and React cases for automatic/manual activation,
  orientation-aware roving focus, Delete close, Alt+Arrow reorder, pointer
  start/over/leave/drop/end, cancellation cleanup, disabled tabs, and complete
  order payloads.
- Do not change web public props, overflow measurement, history sync, tooltip
  timing, specimens, or CSS.

### 2. Complete the payload/drop node lifecycle

- Add the smallest semantic source start/end and target-leave intents beside
  `drag_payload`, `on_drop_hover`, and `on_drop`. Keep the existing
  `NodeDropEvent { payload, edge }` result.
- Correct the GPUI backend so a zone sees hover only while hit, one active
  target receives leave on exit/change, and drop reuses the last computed edge.
- Route source end exactly once after drop or cancellation. The production
  preview host and headless mount host must use the same release/Escape cleanup
  path. Use stock crates.io GPUI 0.2.2; do not patch or fork the engine.
- Add focused `poodle-node` and GPUI backend tests for threshold start,
  hit-tested hover, target change/leave, before/inside/after edge retention,
  successful drop ordering, outside-release cancellation, Escape cancellation,
  disabled/inert paths, and exactly-once end.
- Retain the existing delta-drag and scrub paths unchanged. A payload drag must
  remain type-isolated from resize and value-control gestures.
- Keep Tree and ModelCatalogueEditor compiling and run their focused/mounted
  regressions. Adapt them only where the corrected generic lifecycle requires
  it; do not redesign their public contracts or move their ledger cells.

### 3. Complete shared Rust Tabs interaction

- Extend `TabsHandlers` with semantic reorder, drag start/end, and drop-target
  change callbacks while retaining existing change, close, focus, scoped
  identity, and panel behavior. Use owned results where the host must rebuild;
  add no aliases.
- When a non-disabled reorderable tab starts a payload drag, report its value
  and allow the host to rebuild `drag_value`. Hover/leave reports the target
  value or clears it. Drop resolves source/target indices from the current
  spec, sends `TabsEvent::Reorder`, forwards `EmitReorder`, and requests focus
  for the moved tab. End clears transient host state exactly once.
- Handle Alt+orientation-arrow through `ReorderStep` before ordinary arrow
  focus movement. Map Delete through `TabsEvent::Close`. Keep automatic/manual
  selection and disabled skipping on existing transition paths.
- Preserve instance-scoped runtime ids, roving tab stops, tab/panel
  relationships, close-button semantics, variants, size/density, and transient
  drag visuals.
- Migrate GPUI specimen/facade callers to controlled state with stable instance
  scope. Deferred Jetstream callers may change only enough to compile against
  shared signatures.

### 4. Prove the mounted result and update evidence

- Add one readable named headless GPUI regression, or the smallest coherent
  pair, that mounts real Tabs nodes and drives backend pointer and keyboard
  input with host rebuilds.
- Prove automatic and manual selection, horizontal and vertical roving focus,
  disabled skipping, close button and Delete, Alt+Arrow complete-order result,
  pointer start/hover/leave/drop/end, cancellation cleanup, correct moved-tab
  focus, and two same-valued tabsets with independent runtime identity.
- Inspect the rebuilt node for tablist/tab/panel roles, orientation, selected,
  disabled, roving tab index, controls/labelled-by linkage, and focus treatment.
  Keep this node-level evidence; do not promote broad native accessibility.
- Register the exact mounted regression name in the parity-ledger generator and
  regenerate it. Only Tabs moves from `missing` to `mounted`; totals become 36
  mounted / 138 missing.
- Add one August execution log and leave the generation front doors at the next
  orchestrator review checkpoint.

## Acceptance

- [x] Shared TS/Rust machines and both web shells agree on selection, focus,
      close, reorder payloads, disabled inertia, and cancellation cleanup.
- [x] The generic payload lifecycle reports hit-tested hover/leave, retained
      drop edge, successful-drop ordering, outside/Escape cancellation, and
      exactly one source end on stock GPUI.
- [x] Shared Rust Tabs emits complete next orders for pointer drop and
      Alt+Arrow through `tabs_transition`; no host or backend reorder helper is
      a second authority.
- [x] Delete closes only closable enabled tabs; automatic/manual activation,
      orientation-aware roving focus, panels, and scoped identity remain
      correct.
- [x] Tree and ModelCatalogueEditor payload/drop regressions remain green after
      the generic backend correction; their public contracts and ledger cells
      do not move.
- [x] The mounted GPUI proof uses real backend pointer/keyboard input and
      controlled host rebuilds, not direct handler invocation or spec
      inspection.
- [x] The generated ledger changes exactly Tabs' mounted cell and derived
      totals. GPUI accessibility remains `manual`; GPUI visual remains
      `missing`; Jetstream remains deferred.
- [x] Specimens remain curated and human-centred. One August log records the
      lifecycle repair, Tabs parity, evidence, validation, and remaining gaps.

## Writable Scope

- existing Tabs machine files and focused tests under `packages/core/` and
  `packages/contracts/headless/`; do not replace the machines
- focused Svelte and React Tabs tests, with component source changes only if a
  landed test exposes a real contract defect; public props/CSS stay unchanged
- `packages/contracts/node/src/lib.rs` for the bounded payload lifecycle and
  Delete key vocabulary
- `packages/render/src/tabs.rs`, exports, and focused tests
- `packages/gpui/node-backend/` for payload-session, hit-test, edge, release,
  Escape, and focused backend proof
- the smallest GPUI preview/headless-driver/specimen facade state needed for
  real mounted Tabs input and controlled rebuilds
- mechanical deferred Jetstream/shared caller adaptations required to compile;
  no backend behavior or admission work
- focused Tree and ModelCatalogueEditor regressions/call-site adaptations only
  where required by the generic seam; no semantic redesign
- `packages/gpui/preview/tests/headless_regressions.rs`,
  `scripts/parity-evidence-ledger.ts`, its focused test, and generated ledger
- this card, `docs/roadmaps/g16/README.md`, roadmap/generation front doors, the
  promoted Tabs contract/triage record, one August execution log, and
  `PAPERCUTS.md` for new execution friction only

Do not add a new gesture architecture, combine payload drags with delta drags,
expose coordinates or GPUI types through nodes, redesign other components,
change web public props or specimens, add visual fixtures/thresholds, promote
accessibility evidence, edit workflows, change versions, publish releases, or
touch downstream repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused core and Rust headless Tabs machine tests;
- focused Svelte and React Tabs tests;
- focused `poodle-node`, `poodle-render`, and GPUI node-backend lifecycle tests;
- retained Tree and ModelCatalogueEditor focused/mounted regressions;
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

- Exactly-once start/end, target leave, retained-edge drop, or Escape/outside
  cancellation requires a GPUI fork, engine patch, raw coordinate exposure, or
  component-specific backend state.
- The payload seam cannot remain isolated from the existing delta-drag and scrub
  paths, or a correction changes their mounted behavior.
- Tabs reorder/close cannot use the existing headless transition and effects
  without a second behavior authority.
- Existing Tree or ModelCatalogueEditor behavior regresses and repair requires
  widening either component's public contract rather than correcting the shared
  backend seam.
- Svelte and React disagree after using the existing core helpers, or a focused
  test contradicts the promoted contract.
- The GPUI proof can pass only through direct handler invocation, spec
  inspection, or construction rather than mounted input and host rebuild.
- An unrelated ledger cell moves, or validation requires windowed execution,
  workflow changes, release mutation, downstream work, or Jetstream admission.

## Continuation

Return the node/backend lifecycle diff, exact event-order proof, Tabs handler
and keyboard changes, retained Tree/ModelCatalogueEditor evidence, focused web
and Rust tests, mounted regression names, regenerated ledger totals,
validation, and execution log to the orchestrator. Do not compile or implement
another card. After operator merge, the orchestrator measures the ledger and
chooses the next bounded parity lane.
