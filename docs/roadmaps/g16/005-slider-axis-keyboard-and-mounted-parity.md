# g16.005 — Slider Axis, Keyboard, And Mounted Parity

Status: complete
Opened: 2026-08-26
Completed: 2026-08-26
Depends on: complete `g16.004`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/slider.md`, `parity-evidence-ledger.md`
execution log: `../../logs/2026-08/20260826-g16-005-slider-axis-keyboard-and-mounted-parity.md`

## Outcome

Slider has one observable value-control contract across Svelte, React, shared
Rust, and GPUI. Native pointer input follows the rendered axis, change effects
arrive during a captured scrub, commit arrives exactly once on release,
keyboard input uses the shared normalization authority, and the focusable node
carries complete slider accessibility intent. One named headless GPUI
regression proves the controlled host-rebuild path.

The generated ledger moves only Slider's GPUI mounted-behaviour cell from
`missing` to `mounted` (34 → 35 mounted; 140 → 139 missing). GPUI
accessibility stays `manual`; visual comparison stays `missing`. Jetstream
backend admission stays deferred.

## Fixed Decisions

- `slider_transition` and `slider_control_transition` remain the only value,
  snapping, clamping, and commit authorities in both languages.
- Pointer scrub is position-based, not delta-based. Horizontal normalizes
  left → right; vertical normalizes bottom → top. Press jumps to position,
  captured moves emit live change, and release emits one commit.
- Add the smallest renderer-neutral axis channel beside `on_scrub`. Every
  wired scrub consumer must set it explicitly; the inert `Interaction`
  default may remain horizontal. Wire RangeSlider to the same axis channel and
  retain its landed behaviour.
- All four arrows remain accepted in either orientation: Left/Down decrement,
  Right/Up increment. Home selects minimum and End selects effective maximum.
  Each accepted key emits change followed by commit. Page-key amount remains
  browser-owned and outside strict parity.
- Native Slider is one focusable slider node in both standard and embedded
  variants. It carries label, value, effective min/max, optional value text,
  orientation, disabled state, and the contracted focus treatment.
- Add `NodeA11y.value_text` and the smallest percentage-height style channel if
  native vertical geometry requires it. These are additive vocabulary needed
  by the documented control; do not build a generic schema or layout layer.
- Align shared Rust handler field names to `on_change` and
  `on_value_commit`, matching the component contract and RangeSlider. Migrate
  in-repo callers mechanically; retain no aliases.
- Svelte and React public props remain unchanged. Their focused tests must
  prove the same embedded pointer, keyboard, callback, disabled, and
  accessibility semantics; do not replace native web standard-mode behavior.
- Jetstream receives compile-only call-site adaptation. Do not run or claim
  its backend.

## Delivery

### 1. Lock the shared semantic cases

- Extend focused TypeScript and Rust headless tests around the existing
  machines for step snapping, safe maximum, press/move/end sequencing,
  controlled set-value rebuild, and disabled inertia.
- Keep normalization and effect production in the existing transitions. Do
  not add component-local math or a second fixture authority.
- Add focused Svelte and React tests for embedded horizontal and vertical
  pointer normalization, all arrows plus Home/End, change-then-commit order,
  disabled inertia, and slider ARIA fields. Preserve the curated specimens.

### 2. Complete the renderer-neutral native surface

- Add an explicit scrub axis to the node interaction vocabulary and teach the
  GPUI backend to calculate the normalized fraction from the carried node's
  bounds. Vertical uses height and inverts Y so bottom is minimum.
- Keep real pointer capture. Press, captured drag, and release must remain the
  backend-observed lifecycle; direct invocation is support evidence only.
- Add `value_text` to node accessibility intent. Project Slider role, label,
  value, safe bounds, value text, orientation, and disabled state onto the
  single focusable native control node.
- If vertical fill/thumb layout cannot be expressed faithfully with existing
  flex and absolute-position vocabulary, add only the symmetric
  `NodeStyle.height_pct` channel and its GPUI/deferred-adapter mapping.
- Wire RangeSlider's existing scrub handler to its declared orientation and
  retain its focused and mounted regressions. Do not broaden this card into a
  RangeSlider redesign or move its ledger cells.

### 3. Repair shared Rust Slider behavior

- Replace the stale delta fallback and fixed-width assumption with the one
  axis-aware scrub path. A handler bundle with either callback must install
  the scrub; commit-only hosts must still receive release.
- Forward every `SliderEffect`: change to `on_change`, commit to
  `on_value_commit`. Maintain live value and pointer-active state across the
  gesture without per-event allocation or host-owned normalization.
- Attach keyboard handling to the focusable Slider node. Run INPUT then COMMIT
  through `slider_transition`; update internal live value before the next key.
- Render horizontal and vertical geometry from `SliderSpec::orientation`.
  Keep size/density recipes, standard/embedded fill geometry, polarity, law,
  and public spec shape intact.
- Give enabled sliders a visible contracted focus treatment. Disabled sliders
  expose disabled intent and install no scrub or key handler.
- Rename the shared handler fields and adapt GPUI specimens/facades plus
  deferred Jetstream callers mechanically. GPUI specimens should keep
  controlled host state and may expose commit traces without redesign.

### 4. Prove the mounted result and update evidence

- Add one readable named headless GPUI regression, or the smallest coherent
  pair, that mounts real Slider nodes and drives backend pointer and keyboard
  input with host rebuilds.
- Prove horizontal press/drag/release ordering and exact snapped result;
  vertical bottom/top normalization; Arrow, Home, and End change-then-commit;
  disabled inertia; and updated selected value after rebuild.
- Inspect the rebuilt node for role, label, value, safe min/max, value text,
  orientation, focusability, focus treatment, and disabled state. Keep this as
  node-level semantic evidence; do not promote GPUI assistive-technology proof.
- Register the exact landed regression name in the parity-ledger generator and
  regenerate the ledger. Only Slider's GPUI mounted-behaviour cell moves from
  `missing` to `mounted`; totals move 34 → 35 mounted and 140 → 139 missing.
- Add one August execution log and close this card/front doors honestly.

## Acceptance

- [x] Shared TS/Rust machines and both web shells agree on normalized values,
      callback order, disabled inertia, and controlled rebuilds.
- [x] Horizontal and vertical pointer scrubs use the rendered axis; change
      arrives during interaction and commit exactly once at release.
- [x] All arrows plus Home/End work on native Slider through shared transitions
      and emit change then commit; disabled Slider is inert.
- [x] Native standard and embedded variants expose one focusable Slider node
      with complete node-level accessibility intent and focus treatment.
- [x] `SliderHandlers` uses `on_change` / `on_value_commit`; all in-repo call
      sites compile without aliases or silent fallbacks.
- [x] RangeSlider uses the explicit scrub axis and its existing mounted
      regression remains green; no RangeSlider ledger cell moves.
- [x] The mounted GPUI proof uses real backend pointer/keyboard input and host
      rebuilds, not direct handler calls or spec inspection.
- [x] The generated ledger changes exactly Slider's mounted cell and derived
      totals. GPUI accessibility remains `manual`; visual comparison remains
      `missing`; Jetstream remains deferred.
- [x] Specimens remain curated and human-centred. One August log records the
      implementation, evidence, validation, and remaining gaps.

## Writable Scope

- `packages/core/src/slider.ts` and focused tests only where existing machine
  tests need completion; do not replace the machine
- `packages/svelte/components/src/Slider.svelte` and focused Slider tests
- `packages/react/components/src/Slider.tsx` and focused Slider tests
- `packages/contracts/headless/src/slider.rs` and focused tests
- `packages/contracts/node/src/lib.rs` for scrub axis, value text, and only if
  required percentage height
- `packages/render/src/slider.rs`, its focused tests, and the smallest
  orientation-only RangeSlider adaptation/tests
- `packages/gpui/node-backend/` for axis-aware scrub, additive node style, and
  node-intent probe preservation
- mechanical GPUI specimen/facade and deferred Jetstream call-site changes
- `packages/gpui/preview/tests/headless_regressions.rs` and the smallest
  existing headless-driver support needed by the named inputs
- `scripts/parity-evidence-ledger.ts`, its focused test, and generated
  `parity-evidence-ledger.md`
- this card, `docs/roadmaps/g16/README.md`, roadmap/generation front doors,
  Slider contract corrections already promoted by the orchestrator, and one
  August execution log
- `PAPERCUTS.md` for new execution friction only

Do not edit unrelated components or contracts, introduce a generic event or
layout architecture, redesign specimens, add visual fixtures/thresholds,
promote accessibility evidence, change Jetstream admission, edit workflows,
change versions, publish releases, or touch downstream repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused core Slider tests and shared TS/Rust machine conformance;
- focused Svelte and React Slider tests;
- focused `poodle-node`, `poodle-headless`, `poodle-specs`, and
  `poodle-render` tests plus changed backend/caller crate tests;
- existing RangeSlider focused and mounted scrub regressions;
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

- Axis-aware pointer input cannot fit one explicit scrub-axis field without
  exposing raw layout coordinates to components or creating a second gesture
  authority.
- Vertical Slider requires a broad layout rewrite beyond one symmetric
  percentage-height channel and component-local orientation geometry.
- Callback order cannot use the existing slider transitions, or commit cannot
  be delivered exactly once through the captured backend release.
- Complete node-level slider intent requires claiming unimplemented native
  assistive-technology projection. Keep the ledger cell manual and stop if the
  card would have to overclaim it.
- Svelte and React disagree after using the existing core transitions, or an
  existing conformance vector contradicts the promoted contract.
- The GPUI proof can pass only through direct handler invocation, spec
  inspection, or specimen construction rather than mounted input and host
  rebuild.
- An unrelated ledger cell moves, or validation requires windowed execution,
  workflow changes, release mutation, or Jetstream admission.

## Continuation

Return the node/backend axis diff, handler migration, focused web/Rust tests,
mounted regression names, RangeSlider retention proof, regenerated ledger
totals, validation, and execution log to the orchestrator. Do not compile or
implement another card. After operator merge, the orchestrator measures the
ledger again and chooses the next bounded lane; Tabs remains stopped on its
recorded native drag-lifecycle decision.
