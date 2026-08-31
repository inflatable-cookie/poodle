# g16.014 — Accordion Result Selection And Mounted Parity

Status: complete
Opened: 2026-08-27
Depends on: merged `g16.013` / PR #87; operator-approved clean migration fixed
in the Accordion contract
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/accordion.md`,
`parity-evidence-ledger.md`

## Goal

- Give Svelte, React, shared Rust, and GPUI one resulting-selection contract for
  Accordion single and multiple modes.
- Remove duplicate legacy Rust mode state and activated-item callbacks rather
  than preserving pre-1.0 compatibility surfaces.
- Make native disclosure semantics, focusability, disabled behavior, and
  trigger/panel identity match the documented web authority.
- Move exactly Accordion's GPUI mounted-behaviour cell from `missing` to
  `mounted`: 42 → 43 mounted and 132 → 131 missing. Keep known-delta totals at
  115 present / 60 not-applicable.

## Current Evidence

- Both web runtimes use `toggleGroupTransition`. Accepted toggles report the
  resulting `string | string[] | null`; the component owns membership logic.
- `AccordionSpec` carries both `allow_multiple` and `selection_mode`, while the
  renderer ignores both and the GPUI specimen uses the legacy field.
- `AccordionSelectionValue::Single(String)` cannot represent the contracted
  collapsible empty result.
- Shared Rust emits only the activated item as `&str`; the GPUI specimen
  reconstructs single/multiple state itself.
- `Accordion::with_id(...)` is a no-op. Triggers have no stable runtime ids,
  controls relation, Button role, expanded state, structured focus ring, or
  disabled focus suppression. Panels have Region role without labelled-by.
- The root always exposes Group role even though the contract requires it only
  in multiple mode.
- The existing mounted Accordion test proves inset-shadow paint only. It does
  not prove selection, disclosure, focus, semantics, disabled behavior,
  identity, or host rebuild.
- The curated GPUI specimen advertises Arrow/Home/End header navigation that
  the detailed contract does not require.

## Fixed Contract

### Semantic Rust value and mode

- `AccordionSpec.selection_mode` is the sole mode field. Remove
  `allow_multiple` and `with_allow_multiple` completely. Do not retain an
  alias, deprecated method, fallback, migration constructor, or compatibility
  field.
- `AccordionSelectionValue` becomes:
  - `Single(Option<String>)`; `None` is the explicit collapsed result;
  - `Multiple(Vec<String>)`; the vector is the ordered open set.
- Keep the outer optional `AccordionSpec.value` / `default_value` boundary. It
  distinguishes an omitted controlled value from a supplied value/default
  seed; it is not the single-mode collapsed value.
- Migrate all Poodle-owned GPUI and deferred-Jetstream Rust call sites directly
  to the new enum shape and sole mode field.

### Resulting-selection transition

- Add `AccordionHandlers` beside the shared renderer with a required non-empty,
  lifetime-stable `instance_id` and optional
  `on_value_change: Arc<dyn Fn(AccordionSelectionValue) + Send + Sync>`.
- Replace bare `Fn(&str)` renderer callbacks and GPUI `on_toggle` adapters with
  the typed result boundary. Do not preserve the activated-item API.
- Convert the spec to the existing `poodle_headless::toggle_group_transition`
  context once per render. `selection_mode` maps to its mode and `collapsible`
  maps to `allow_deactivation`.
- Pointer, Enter, and Space activation all use the same transition. Disabled
  or unknown items are inert. Every accepted effect forwards the resulting
  typed selection; the host owns current state and rebuilds the spec.
- Do not add Accordion-specific membership math, a second transition, hidden
  mutable renderer state, or Arrow/Home/End selection behavior.

### Disclosure semantics and identity

- The root exposes Group role only in multiple mode. Its authored `aria_label`
  remains the accessible group label where present.
- Every trigger is a Button with authored item label, expanded state, ordinary
  tab order, and the established structured control focus ring.
- Disabled triggers have no activation handler, no focus handle, and no
  sequential tab stop. Disabled styling remains on the item surface.
- Every native construction supplies stable instance scope. Trigger and panel
  semantic/runtime ids derive from that scope plus the authored item value,
  never render order, open state, label text, or a process-global counter.
- Scoped triggers expose `controls`; open Region panels expose the reciprocal
  `labelled_by`. Closed panels are absent.
- Enter and Space use normal button activation. Tab traverses enabled triggers
  and then exits. No roving-focus or arrow-key scheme is introduced.

## Execution Plan

- [x] **Batch 1 — clean semantic migration.** Update the contract-backed Rust
      value and mode surface, remove legacy APIs, migrate every Rust call site,
      and add focused `poodle-specs` tests for single open/collapsed, multiple
      open/empty, value-over-default precedence, and mode consistency.
- [x] **Batch 2 — shared transition and disclosure semantics.** Introduce the
      required handler bundle, route activation through the existing headless
      transition, project roles/expanded/relations/focus/disabled state, and add
      focused `poodle-render` tests including two-instance identity.
- [x] **Batch 3 — GPUI host rebuild and evidence.** Wire the wrapper/specimen
      through typed results and stable scopes, correct the specimen keyboard
      copy, add one named mounted headless regression, regenerate only
      Accordion's ledger cell, close the card/decision/log/front doors, and run
      the full headless validation board.

## Specimen And Mounted Proof

- Preserve the two curated Examples groups and existing size/density axes. Do
  not add an exhaustive matrix or replace human-facing copy with test fixtures.
- Keep single mode initially open on Getting started and multiple mode open on
  Design tokens plus Keyboard shortcuts. Route live changes through the typed
  result and host rebuild rather than per-item toggle reconstruction.
- Correct the Keyboard shortcuts panel to describe Enter/Space/Tab only. Do not
  implement the currently advertised Arrow/Home/End behavior as specimen-led
  contract expansion.
- Add one readable named mounted regression through the production renderer,
  GPUI node backend, real hit testing, focus/key dispatch, and host rebuild. It
  proves:
  - single-mode root role absence, labelled Button triggers, expanded state,
    one open Region, controls/labelled-by association, and focus rings;
  - pointer selection reports `Single(Some(...))` once and rebuilds open state;
  - collapsible reactivation reports `Single(None)` and removes the panel;
  - non-collapsible reactivation reports the unchanged single result, matching
    the existing web machine;
  - multiple add/remove reports complete ordered `Multiple(...)` results;
  - Enter and Space use the same result path as pointer activation;
  - disabled items emit nothing and are skipped by sequential focus; and
  - two mounted accordions with identical item values keep independent trigger
    and panel runtime/focus identity through rebuilds.
- Direct handler invocation, spec inspection alone, inset-shadow painting, or
  fixture-only state changes do not satisfy mounted behavior proof.

## Explicit Non-Claims

- This card does not change public Svelte or React props, implementations, or
  controlled/uncontrolled behavior.
- It does not add Arrow/Home/End accordion navigation, roving focus, nested
  accordions, arbitrary heading levels, or a compound-item API.
- It does not implement or claim native panel-height animation, GPUI visual
  comparison, or broad native assistive-technology proof.
- It does not admit or behaviorally repair Jetstream. Deferred callers receive
  compilation-only semantic migration and stable descriptive scopes.
- It does not change Collapsible, ToggleGroup, NumberInput, EditableLabel,
  Rating, Select, Dialog, Drawer, or generic node/backend vocabulary.
- It does not touch releases, versions, workflows, downstream repositories,
  publication, or sibling repositories.

## Acceptance Criteria

- [x] `AccordionSpec` has one `selection_mode` field and no `allow_multiple` or
      `with_allow_multiple` compatibility surface.
- [x] Single mode represents both `Some(value)` and explicit `None`; multiple
      mode represents the complete ordered set, including empty.
- [x] All Rust call sites use the semantic enum and mode directly; deferred
      Jetstream changes are mechanical only.
- [x] Every native construction supplies a non-empty stable scope through
      `AccordionHandlers`; no bare activated-item callback or no-op identity
      builder remains.
- [x] Native activation uses the existing headless ToggleGroup transition and
      reports only resulting typed selections for single, collapsible single,
      non-collapsible single, and multiple modes.
- [x] Root, trigger, and panel semantics match the contract; enabled triggers
      carry focus rings and ordinary tab stops; disabled triggers do not.
- [x] Trigger/panel relations and runtime ids remain stable across host rebuilds
      and independent across two same-valued mounted instances.
- [x] The curated GPUI specimen uses typed host rebuilds, retains its useful
      examples/axes, and no longer advertises unsupported key behavior.
- [x] One named mounted regression proves the fixed behavior through real
      pointer and keyboard dispatch.
- [x] The generated ledger changes only Accordion to 43 mounted / 131 missing;
      known-delta totals stay 115 / 60 and visual/accessibility cells remain
      unchanged.
- [x] One August log records the approved break, result transition, disclosure
      repair, evidence, validation, exact non-claims, and next checkpoint.

## Writable Scope

- `docs/contracts/components/accordion.md`
- `packages/contracts/components/src/accordion.rs`, the Accordion-specific
  portion of `packages/contracts/components/src/types.rs`, exports, and focused
  tests
- `packages/render/src/accordion.rs`, its export surface, and focused tests
- the smallest Poodle-owned Rust call-site set required by the clean spec and
  handler migration, including GPUI compatibility/specimen/capture code and
  deferred Jetstream compile-only callers
- Accordion-only code under `packages/gpui/preview/src/specimens/` and
  `packages/gpui/preview/src/node_compat.rs`
- the smallest Accordion mounted regression changes in
  `packages/gpui/preview/tests/headless_regressions.rs`
- `scripts/parity-evidence-ledger.ts`, its focused test, and generated
  `parity-evidence-ledger.md` for the one mounted cell
- this card, its source decision, one August log, g16/front-door status, and
  `PAPERCUTS.md` only for new execution friction

Do not edit web component implementations, shared TS/headless transition
semantics, generic node/backend APIs, other component contracts or semantics,
theme/token definitions, visual fixtures, accessibility reports, package
versions, workflows, releases, downstream repositories, or sibling
repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-specs`, `poodle-headless`, and `poodle-render` Accordion tests;
- focused compilation/tests for every Rust crate whose call sites change;
- focused Svelte and React Accordion tests to prove the web authority stayed
  unchanged;
- the named mounted Accordion regression;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy drift:handlers`, `effigy drift:events`, and relevant contract/spec
  drift selectors;
- `effigy drift:roles` only when the deferred Jetstream sibling is already
  available; otherwise record the known `PAPERCUTS.md` blocker and do not create
  a symlink or admit Jetstream;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:rust`, `effigy ci:native`, and `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, or publication selectors.

## Stop Conditions

- The paired web runtimes or detailed contract disagree on selection mode,
  collapsible behavior, same-value emission, disabled behavior, callback
  payload, or controlled/uncontrolled ownership.
- The clean migration requires a compatibility alias/fallback or a semantic
  change outside Accordion rather than mechanical call-site work.
- The existing headless ToggleGroup transition cannot produce every contracted
  Accordion result without changing its semantics.
- Existing node/backend role, expanded, controls, labelled-by, runtime-id,
  activation, focus, and focus-ring channels cannot express the fixed result.
- Stable identity cannot be supplied by the host without render-order, state,
  labels, or process-global fallbacks.
- Mounted proof bypasses production hit testing, focus, key dispatch, or host
  rebuild, or proves only the retained inset-shadow path.
- The ledger generator changes another row/evidence column or validation
  requires windowed execution, workflow/release mutation, Jetstream admission,
  or work in another component family.

## Continuation

Return the semantic API diff, handler boundary, focused test names, mounted
regression name, two-instance identity proof, regenerated ledger totals,
validation, and execution log to the orchestrator. Do not compile or implement
`g16.015`. After operator merge, the orchestrator returns to the measured
ledger and chooses the next bounded parity lane.
