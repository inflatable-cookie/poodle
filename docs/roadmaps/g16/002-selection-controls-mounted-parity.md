# g16.002 — Selection Controls Mounted Parity

Status: complete for four proofs — ToggleGroup planning stop; awaiting review
Opened: 2026-08-26
Completed: 2026-08-26
Depends on: completed `g16.001`
Governing refs: `../../contracts/001-working-rules.md`,
`parity-evidence-ledger.md`,
`../../contracts/components/checkbox.md`,
`../../contracts/components/switch.md`,
`../../contracts/components/radio-group.md`,
`../../contracts/components/segmented-control.md`,
`../../contracts/components/toggle-group.md`

## Outcome

Four named mounted GPUI regressions now drive Checkbox, Switch, RadioGroup, and
SegmentedControl through the real backend/input path and host rebuild. The
generated ledger moves only those four GPUI mounted-behaviour cells from
`missing` to `mounted` (29 → 33 mounted; 145 → 141 missing).

ToggleGroup remains `missing`. The native renderer still emits the activated
option as `Fn(&str)`, while the contract requires the resulting selection as
`string | string[] | null`. Honest mounted proof would also need contracted
single-mode Arrow Left/Right roving, which Svelte, React, and the shared
machine do not implement. Both are planning stops, not repairs this card may
make. Do not invent a GPUI-only callback or keyboard rule here.

Contract-backed GPUI/shared-Rust repairs on the four closed controls: focus
patches so GPUI tracks handles; Checkbox mixed and Switch checked projection
through `current_state` / `current_checked`; RadioGroup option identity,
RadioButton a11y, and wrap/skip-disabled roving; same-value inertia for
RadioGroup and SegmentedControl. ToggleGroup also gained the same enabled-item
focus patch so GPUI can track handles; that does not close its mounted cell.
No contract or public API change. GPUI accessibility stays `manual`. No
`g16.003`.

## Goal

Prove the real mounted GPUI behaviour of one coherent, high-leverage selection
family:

- `Checkbox`
- `Switch`
- `RadioGroup`
- `SegmentedControl`
- `ToggleGroup`

All five already have contracts, Svelte and React focused evidence, shared Rust
composition, and headless GPUI construction. The `g16.001` ledger records all
five as missing mounted GPUI behaviour. Close that exact gap through the real
node/backend/input path, then update the ledger from named tests.

This is a headless behaviour batch. It does not create a shared case corpus,
visual-fixture schema, generic runtime adapter, or accessibility claim.

## Fixed Behaviour Envelope

Drive mounted controls with real GPUI input and host rebuilds. Spec-only unit
tests, direct handler invocation, and specimen construction do not satisfy this
card.

### Checkbox

- interactive activation toggles once and emits the next checked value once;
- mixed resolves to checked on the first accepted activation;
- readonly stays focusable but does not change or emit;
- disabled does not accept focus or activation.

### Switch

- interactive activation toggles once and emits the next checked value once;
- readonly stays focusable but does not change or emit;
- disabled does not accept focus or activation.

### RadioGroup

- one option is selected at a time and same-value selection is inert;
- directional movement updates real focus and selection, wraps, and skips a
  disabled option;
- group-disabled and option-disabled paths emit nothing.

### SegmentedControl

- one segment is selected at a time and same-value selection is inert for the
  active cohort;
- directional movement updates real focus and selection, wraps, and skips a
  disabled segment;
- group-disabled and segment-disabled paths emit nothing;
- stable instance scope keeps two mounted controls from sharing focus identity.

The deferred Jetstream note about re-picking the selected segment is not active
cohort authority. If current Svelte, React, shared Rust, or GPUI behaviour
contradicts the contract's same-value rule, stop with exact evidence rather than
choosing a new rule in this card.

### ToggleGroup

- single mode selects one value; re-selection emits the unchanged value unless
  `allowDeactivation` is true, in which case it emits `null`;
- multiple mode adds and removes membership with the exact ordered string-array
  payload;
- group-disabled and option-disabled paths emit nothing;
- single-mode directional focus, where the current contract requires it,
  operates through the mounted tree rather than a direct callback.

## Implementation Rules

- Build each proof from the existing public Rust spec and `poodle-render` node.
- Mount through the existing headless GPUI driver and drive real pointer or
  keyboard input. Extend the test driver only for a missing GPUI input primitive
  already required by these controls.
- Rebuild the host-controlled spec after callbacks. Assertions must observe the
  rebuilt mounted tree, emitted payloads, and real focus state where relevant.
- Prefer one readable named regression per component, with extra tests only
  where one test would hide distinct disabled or focus behaviour.
- Repair a measured shared-Rust or GPUI defect only when the existing contract
  decides the result. Add a regression that fails on the old behaviour.
- Update `MOUNTED_BEHAVIOUR_TESTS` in the parity-ledger generator with the exact
  landed test names, regenerate the ledger, and derive its summary. Do not hand
  edit ledger cells or totals.
- Keep `GPUI accessibility` at `manual`. Node role/state assertions support the
  mounted behaviour claim; they do not prove assistive-technology parity.

## Acceptance

- [x] Checkbox, Switch, RadioGroup, and SegmentedControl each have at least
      one resolvable mounted regression in
      `packages/gpui/preview/tests/headless_regressions.rs`.
- [x] Those four regressions drive the real mounted backend/input path and
      host rebuild; none passes by calling a handler or transition helper
      directly.
- [x] Checkbox and Switch prove accepted toggle, readonly, and disabled
      behaviour; Checkbox also proves mixed-to-checked resolution.
- [x] RadioGroup and SegmentedControl prove exclusive selection, directional
      focus/selection, wrap, disabled-option skip, and disabled-group inertia.
- [x] SegmentedControl proves two mounted instances keep independent focus
      identity.
- [x] ToggleGroup remains a planning stop: native `Fn(&str)` option emission
      is not the contracted selection payload, and contracted single-mode
      arrow roving is absent from Svelte/React/GPUI. No public API or
      GPUI-only keyboard rule was invented.
- [x] Any repaired runtime defect is contract-backed and recorded with its
      before/after evidence. No contract or public API changes.
- [x] The generated ledger changes exactly these four GPUI mounted-behaviour
      rows from `missing` to `mounted`; ToggleGroup stays `missing`; unrelated
      evidence cells do not move.
- [x] One August execution log records tests, defects, repairs, validation,
      the ToggleGroup stop, and remaining gaps.

## Writable Scope

- `packages/gpui/preview/tests/headless_regressions.rs` and the smallest existing
  headless-driver support needed for the named input paths
- the five selected modules under `packages/render/src/`
- `packages/gpui/node-backend/` only for a directly measured input, focus, or
  node-projection defect in the selected family
- selected component modules under `packages/gpui/preview/src/` only when the
  production preview seam, rather than test construction, owns the measured
  defect
- `scripts/parity-evidence-ledger.ts`, its focused test, and the generated
  `docs/roadmaps/g16/parity-evidence-ledger.md`
- this card, `docs/roadmaps/g16/README.md`, generation/front-door status, and
  one August execution log
- `PAPERCUTS.md` for new execution friction only

Do not edit Svelte or React components, component contracts, public Rust specs,
specimen content, visual fixtures or thresholds, accessibility reports,
Jetstream, workflows, package versions, releases, or downstream repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused Rust tests for every changed `poodle-render` or backend module;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy test:parity-evidence-ledger`;
- `effigy check:parity-evidence-ledger`;
- `effigy ci:native`;
- `effigy ci:web` to prove the reference runtimes stayed unchanged;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything remains headless. Do not run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, or publication selectors.

## Stop Conditions

- A named behaviour is missing from, or contradicted inside, its component
  contract.
- Svelte and React disagree on the observable rule, or the Svelte reference
  contradicts the contract.
- The fix needs a component contract, public API/spec change, new node
  vocabulary, generic action language, shared case corpus, generated adapter,
  or normalized cross-runtime observation model.
- A test can pass only through direct handler invocation, spec inspection, or
  specimen construction rather than mounted input.
- Repair crosses outside the five selected components or requires windowed
  execution, visual thresholds, accessibility promotion, Jetstream, a workflow,
  or a release mutation.
- The ledger changes an unrelated component or evidence class.

## Continuation

Return the four mounted proofs, the ToggleGroup planning stop, and the
regenerated ledger to the orchestrator. Resolve ToggleGroup as a separate
semantic/API lane before compiling a next card. Do not compile `g16.003` from
this stop.
