# g16.017 — Rating Nullable, Fractional, And Mounted Parity

Status: complete
Opened: 2026-08-27
Completed: 2026-08-27
Depends on: merged `g16.016` / PR #91 and the resolved selection in
`../../triage/20260827-222346-post-g16-016-native-lane-decision.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/rating.md`, `parity-evidence-ledger.md`

## Goal

- Make shared Rust Rating represent the web authority's nullable value,
  arbitrary fractional display, stepped input, and clear behavior without
  retaining its incompatible integer-era API.
- Implement coherent whole-step radio behavior and fractional slider behavior
  through production rendering, GPUI input dispatch, and host-owned rebuilds.
- Move exactly Rating's GPUI mounted-behavior cell from `missing` to `mounted`:
  45 → 46 mounted and 129 → 128 missing. Keep known-delta totals at
  115 present / 60 not-applicable.

## Current Evidence

- Svelte and React agree on `number | null`, default `step=0.5`, arbitrary
  fractional display, quantized user input, clear-on-reselect, whole-step
  radiogroup behavior, fractional slider behavior, disabled inertia, and
  `number | null` change payloads.
- `RatingSpec` currently stores concrete `f64` values, defaults to `step=1`,
  retains undocumented `precision` and read-only fields, and projects some
  display math directly from the spec.
- `poodle-render::rating` accepts `Fn(u32)`, always emits a radiogroup, marks
  every partly filled star selected, and can only report whole-star pointer
  changes. It has no production focus, keyboard, clear, or fractional pointer
  path.
- The node and GPUI backend already expose RadioGroup, RadioButton, and Slider
  roles; numeric accessibility values; focus/tab/ring state; Arrow, Home, End,
  Enter, and Space keys; axis-normalized scrub fractions; and host rebuilds.
  This lane does not require new generic vocabulary.
- The current GPUI specimen includes static half-star output and a legacy
  read-only example, but its live value is integer-shaped and does not prove
  nullable or fractional behavior.
- No named mounted regression currently drives Rating. Construction or direct
  callback tests do not satisfy the ledger's mounted evidence cell.

## Fixed Contract

### Clean Rust API migration

- The operator explicitly approved this breaking pre-1.0 migration on
  2026-08-27.
- Change `RatingSpec.value` and `default_value` to `Option<f64>`, both defaulting
  to `None`. Current display resolves `value.or(default_value)` like the other
  shared Rust controlled/default specs; both absent is a real empty state. The
  host still owns post-interaction state.
- Default `step` to `0.5`. Keep invalid or non-positive step fallback and the
  existing cap at `1` aligned with the web behavior machinery.
- Remove legacy `precision`, `is_readonly`, and their builders. Neither is a
  public web Rating prop. Do not add aliases, compatibility fields, silent
  conversions, or a second legacy callback.
- Change the native change payload from `u32` to `Option<f64>`. `None` means an
  accepted clear action; all selected values are clamped and user-generated
  values are snapped through the shared pure math.
- Preserve `max` and the presentation/disabled/label axes unless the paired
  contract demonstrates a direct type defect. Normalize effective item count
  safely rather than expanding the migration without evidence.
- Update the detailed contract's native notes and stale read-only claim to
  describe the active cohort honestly. Jetstream call sites receive only the
  mechanical compile migration; its backend remains program-deferred.

### Shared pure value machinery

- Put native Rating value transitions in the Rust headless substrate, parallel
  to `packages/core/src/rating.ts`: step resolution, display clamp, input snap,
  pointer value, clear-on-reselect, fill ratio, display text, and keyboard
  stepping.
- Renderer, focused Rust tests, and mounted behavior consume that machinery.
  Do not duplicate the math in the GPUI wrapper, specimen, or test driver.
- Lock matching authored vectors in TypeScript and Rust tests. These tests
  compare the two existing substrates; they do not create a generated IR,
  third runtime authority, or specimen-shaped conformance matrix.
- Incoming display values clamp but do not quantize. Only user-produced values
  snap to the effective step. `None` remains distinct from numeric zero.

### Whole-step mode

- `step >= 1` renders a RadioGroup root with one RadioButton per value. Only
  the item equal to the current value is selected; partly or fully filled
  earlier stars are not selected radios.
- Enabled items use instance-scoped, host-owned stable focus ids and roving
  tab behavior. The selected item is the entry stop; otherwise the first item
  is. Arrow keys and Home/End move focus without selecting, matching the web
  authority. Enter/Space select the focused value.
- Selecting the current value reports `None` only when `allow_clear=true`.
  Otherwise it reports no semantic change. The host applies accepted output
  and rebuilds the spec.
- Disabled Rating has no focus stop, pointer handler, key handler, or callback.

### Fractional mode

- `step < 1` renders one focusable Slider root with min `0`, max `max`, current
  numeric value (zero only for the accessibility fallback when state is
  `None`), and readable value text. Star targets are accessibility-hidden and
  are not separate focus stops.
- Pointer position within each star resolves through the shared snap-up math,
  including minimum one-step selection. Press/drag/release may use the existing
  scrub channel; emit a coherent semantic change and let the host rebuild.
- Arrow keys step from the current value, Home chooses zero when clearable or
  the minimum selectable step otherwise, and End chooses `max`. Enter/Space
  reports `None` only when clearable and a value is present.
- Disabled fractional mode remains inert and outside keyboard focus.

### Identity and host ownership

- Add the smallest Rating-specific handler bundle needed for a stable
  `instance_id` plus `Option<f64>` change callback, following current
  RadioGroup/ToggleGroup patterns. Renderer identity is not a web public prop
  and must not be inferred from render order, label text, or selected value.
- Rating stays controlled after rendering. Callback emission never mutates
  hidden renderer state; the host stores the result and rebuilds.
- Test-only ids may be stamped after production rendering for mounted driver
  targeting. They are evidence plumbing, not public component identity.

## Execution Plan

- [x] **Batch 1 — contract and pure machinery.** Apply the approved Rust API
      break, migrate in-repo call sites mechanically, add native pure Rating
      math with paired focused vectors, and update stale native contract notes.
- [x] **Batch 2 — renderer and mounted behavior.** Add Rating-specific handlers,
      implement whole-step radio and fractional slider paths through existing
      node vocabulary, update the curated GPUI specimen, and add one readable
      named mounted regression with real pointer/keyboard dispatch and host
      rebuilds.
- [x] **Batch 3 — evidence and closeout.** Regenerate only Rating's mounted
      ledger cell, close this card/source decision/log/front doors, and run the
      required headless board.

## Specimen And Mounted Proof

- Keep the specimen human-centred with Examples, Sizes, and Densities. Show a
  live default half-step example with a compact nullable numeric readout, a
  whole-step example, arbitrary fractional display, clearable, disabled, and
  the existing useful scale/presentation examples. Remove the non-contract
  read-only example. Do not list every state/step/size combination on Examples.
- The named mounted regression proves:
  - default half-step pointer input produces a fractional `Option<f64>` value,
    host rebuild, fill output, and slider accessibility value/text;
  - fractional Arrow keys, Home, End, and clear-on-Space/Enter use the same
    pure transition path and disabled mode emits nothing;
  - a `step=1` Rating exposes one selected radio, one roving tab stop, and
    Arrow/Home/End focus movement without selection;
  - whole-step Enter/Space and pointer activation report the same selected
    value, clear only when allowed, and host rebuild the control;
  - empty state stays `None`, arbitrary incoming fractions display without
    quantization, and user input remains quantized; and
  - separate Rating instances do not collide in native focus identity.
- Direct callback invocation, renderer inspection alone, specimen-only state
  mutation, or a fixture-only fake control does not satisfy mounted proof.

## Explicit Non-Claims

- This card does not change the public Svelte or React API or implementation,
  except focused preservation tests if current evidence is insufficient.
- It does not redesign generic Node or GPUI backend input/focus vocabulary. If
  the existing scrub/key/focus channels cannot express the contract, stop.
- It does not close broad native accessibility or visual-comparison evidence.
- It does not redesign Select, NumberInput, EditableLabel, SplitButton, menus,
  or any other component family.
- It does not admit Jetstream behavior or run its preview/QA. Mechanical
  compilation changes do not move its evidence or erase its deferred status.
- It does not touch releases, versions, workflows, downstream repositories, or
  sibling repositories.

## Acceptance Criteria

- [x] Rust Rating uses nullable authored/default values, default half-step
      input, no legacy precision/read-only surface, and `Option<f64>` callbacks.
- [x] Native pure math matches the TypeScript authority on display clamp,
      snapping, pointer ratios, fill, clear, formatting, and keyboard vectors.
- [x] Whole-step mode has correct radiogroup/radio selection, roving focus,
      keyboard lifecycle, pointer behavior, disabled inertia, and host rebuilds.
- [x] Fractional mode has correct slider semantics, numeric value/text, stepped
      pointer and keyboard behavior, clear behavior, disabled inertia, and host
      rebuilds.
- [x] Arbitrary incoming fractions display without step quantization; accepted
      user values are quantized; `None` is not collapsed into zero.
- [x] Focus identity is stable and isolated across Rating instances without a
      new web prop or render-order-derived id.
- [x] Focused Svelte and React Rating tests remain green without public web
      implementation changes.
- [x] The GPUI specimen is interactive, human-centred, and preserves Sizes and
      Densities without becoming an exhaustive conformance page.
- [x] One named mounted regression proves the production Rating path.
- [x] The generated ledger changes only Rating to 46 mounted / 128 missing;
      known-delta totals remain 115 / 60 and visual/accessibility cells remain
      unchanged.
- [x] One August log records the migration, proof, validation, non-claims, and
      next checkpoint.

## Writable Scope

- `packages/contracts/components/src/rating.rs` and focused tests
- the smallest Rating module/export under `packages/contracts/headless/src/`
  plus focused tests
- `packages/render/src/rating.rs` and focused tests
- Rating-only compatibility/specimen changes under `packages/gpui/preview/src/`
- the smallest Rating mounted regression change in
  `packages/gpui/preview/tests/headless_regressions.rs`
- mechanical Rating compile migrations in in-repo Jetstream adapter/preview
  files and focused construction tests only; do not change behavior claims
- focused TypeScript core/Svelte/React Rating tests only to lock or preserve the
  existing authority; avoid web implementation changes
- `docs/contracts/components/rating.md`
- `scripts/parity-evidence-ledger.ts`, its focused test, and generated
  `parity-evidence-ledger.md` for the one mounted cell
- this card, its source decision, one August log, g16/front-door status, and
  `PAPERCUTS.md` only for new execution friction

Do not edit generic node/backend APIs, other component contracts or
implementations, theme/token definitions, visual fixtures, accessibility
reports, package versions, workflows, releases, downstream repositories, or
sibling repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused TypeScript core, Svelte, and React Rating tests;
- focused `poodle-headless`, `poodle-specs`, and `poodle-render` Rating tests;
- the named mounted Rating regression;
- `effigy regressions:native` and `effigy probe:gpui-specimens`;
- relevant handler/event, contract/spec, and role drift selectors when their
  existing prerequisites are available without admitting Jetstream;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:rust`, `effigy ci:native`, and `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch; and
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- Svelte, React, core machinery, or the detailed contract disagree on nullable
  ownership, default step, clear behavior, whole-step focus, or fractional
  pointer/keyboard semantics.
- Correct behavior needs a new generic Node/GPUI backend API, hidden renderer
  state, a compatibility shim, or a public web API change.
- The approved Rust break reaches a downstream or sibling repository rather
  than only in-repo mechanical call sites.
- Mounted proof cannot drive both production radio and slider paths through
  real pointer/keyboard dispatch plus host rebuild.
- The ledger generator changes another row/evidence column or validation
  requires windowed execution, workflow/release mutation, Jetstream admission,
  or another component family.

## Continuation

Return the exact Rust migration, pure-math tests, renderer tests, mounted
regression name, whole-step/fractional host-rebuild proof, exact ledger totals,
validation, and execution log to the orchestrator. Do not compile or implement
`g16.018`. After operator merge, the orchestrator returns to the measured
46 mounted / 128 missing ledger and chooses the next bounded parity lane.
