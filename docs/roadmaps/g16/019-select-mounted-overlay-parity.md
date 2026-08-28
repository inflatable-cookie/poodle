# g16.019 — Select Mounted Overlay Parity

Status: ready
Opened: 2026-08-28
Recompiled: 2026-08-28
Depends on: completed `g16.018` / merged PR #93
Worker handoff:
`../../handoffs/20260828-181625-g16-019-select-overlay-worker.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/select.md`, `parity-evidence-ledger.md`

## Goal

- Complete Select's production GPUI interaction on the semantic substrate from
  `g16.018`.
- Replace the static native search row with real host-owned editing, keyboard,
  focus, dismissal, and overlay pointer behavior.
- Move exactly Select from missing to mounted: 46 → 47 mounted and
  128 → 127 missing. Keep known-delta totals at 115 present /
  60 not-applicable.

## Cleared Entry Gate

- PR #93 landed the exact `SelectContext`, `SelectEvent`, `SelectEffect`, and
  `SelectTransitionResult` model in TypeScript core and `poodle-headless`.
  Hosts apply the complete next context, then dispatch ordered open, query,
  and value effects.
- Svelte and React consume the shared machine. Typing reports query only;
  option selection, clear, or explicit freeform Enter/control-blur commit
  reports value. Paired vectors cover open/close, query/highlight movement,
  commit, clear, disabled inertia, and option reconciliation.
- Shared Rust now uses
  `SelectHandlers::new(instance_scope).on_transition(...)`. Select and the
  five composed handler surfaces reject empty or blank lifetime-stable scopes.
  Trigger, clear, search, listbox, and option nodes already receive scoped
  runtime ids.
- Existing Node/GPUI editing vocabulary is sufficient: replacement-text,
  edit-key, insertion, selection, submit, cancel, focus-change, caret,
  navigation-key, focus-request, and dismissal channels already power
  TextInput and other mounted controls. Select may compose those channels in
  its own search row; no new generic input capability is planned.
- Deferred-overlay pointer targeting is still a current defect. The mounted
  Pagination regression must start the Select open and stamp a test-only focus
  ring so keyboard activation can bypass the missed deferred option rows.
  `g16.018` migrated runtime identity but did not change layer hit-testing.

## Fixed Native Contract

### Host ownership

- `SelectSpec` remains the complete authored value/open/query/highlight state.
  Interaction emits one `SelectTransitionResult`; the host applies it and
  rebuilds. Do not add renderer-owned selection, query, open, or focus state.
- Keep `SelectHandlers` and the composed required instance scopes exactly as
  landed. Do not restore legacy callbacks, `Default`, optional ids, aliases,
  or fallback identity.
- Do not change the public Svelte or React props or callback timing. Focused
  web tests are preservation evidence only.

### Editable search and focus

- When `searchable=true`, the open panel renders a real editable field from the
  existing Node input/caret/edit channels, styled as the current compact search
  row rather than nesting a second full TextInput shell.
- Query replacement dispatches `SelectEvent::Query`. Enter dispatches
  `CommitHighlighted`, falling through to freeform commit only when the landed
  machine permits it. Escape closes without a value change. Tab traverses out;
  the resulting control blur closes and commits freeform only under the
  detailed contract's explicit control-blur rule.
- Arrow Up/Down while closed opens without a double move. While open, Arrow
  Up/Down and Home/End dispatch the matching highlight events. Highlight stays
  host-authored and keyed by option value.
- Searchable Select keeps real keyboard focus on the editor while highlight
  moves. Non-searchable Select keeps it on the trigger. Option rows are pointer
  targets, not independent tab stops. Closing by commit, Escape, outside
  interact, or Tab returns or advances focus according to the detailed
  contract without hidden renderer state.
- Disabled Select and disabled options have no edit, navigation, activation,
  focus, or callback path.

### Overlay interaction

- Reproduce the deferred-row pointer miss in one focused backend regression
  before changing it.
- Repair the smallest reusable layer/hit-test seam so pointer press and release
  reach the real option row after an open-state host rebuild. Preserve overlay
  paint order, containment, outside-dismiss ordering, and nested-overlay rules.
- Do not special-case Select coordinates, duplicate option handlers outside
  the node tree, keep the panel permanently in-flow, or retain the
  Pagination-era test-only focus-ring/id workaround.
- Trigger plus panel share one dismiss layer. Outside pointer interaction
  closes only when `dismissOnOutsideInteract` permits it; Escape closes the
  innermost layer. A click inside the panel must not first dispatch outside
  dismissal.

### Pointer and keyboard parity

- Pointer hover may report `Highlight`; pointer activation reports
  `CommitOption`. Disabled and filtered-out options remain inert.
- Trigger Enter/Space opens. Open-state navigation supports Arrow Up/Down,
  Home/End, Enter, Escape, and Tab through production input dispatch.
- Clear runs through `SelectEvent::Clear`, does not bubble into trigger toggle,
  and rebuilds query/value/highlight coherently.
- Group headers remain inert. Filtering preserves authored group order and
  option identity.

## Execution Plan

- [ ] **Batch 1 — editable native Select.** Replace the static search row with
      existing Node editing channels; route query, navigation, commit, clear,
      dismissal, and focus through the landed machine and host rebuilds.
- [ ] **Batch 2 — deferred overlay pointer repair.** Add a focused reproducer,
      repair the smallest generic layer/hit-test seam, and remove the
      Pagination-era Select workaround.
- [ ] **Batch 3 — mounted proof and evidence.** Update the curated GPUI
      specimen, add one named mounted Select regression covering two instances,
      move only the Select ledger cell, write one August execution log, and
      close the card/front doors.

## Specimen And Mounted Proof

- Keep Examples curated: one ordinary custom Select, one searchable example,
  one freeform example with compact value/query/open readout, useful grouped,
  clearable, loading/empty, and disabled examples. Preserve Sizes and Densities
  without copying their matrices into Examples.
- One readable named mounted regression must prove through real GPUI dispatch:
  - two independently scoped Selects open, navigate, choose, clear, dismiss,
    and rebuild without identity collision;
  - searchable typing updates query, filters options, and moves host-authored
    highlight while focus stays on the editor;
  - Arrow Up/Down, Home/End, Enter, Escape, and Tab follow the shared machine;
  - freeform Enter and control blur commit exactly once only when no option is
    highlighted;
  - grouped and disabled options preserve order and inertia;
  - a real pointer opens the trigger and selects a deferred option row after
    the host rebuild, with no test-only option ring/id override; and
  - outside interaction closes once while an inside option click commits once.
- Direct closure invocation, renderer inspection, a permanently-open fixture,
  specimen-only mutation, or a stamped keyboard target does not satisfy the
  mounted claim.

## Explicit Non-Claims

- No new Select semantic machine, web public API, or composed-component
  behavioral closure.
- No generic input vocabulary expansion. If the existing edit/focus channels
  cannot express the contract, stop.
- No broad overlay rewrite, menu/popover/dialog migration, visual-comparison
  programme, or broad native-accessibility claim.
- No Jetstream admission or preview/QA. Its in-repo construction may receive
  only mechanical compile maintenance if the shared renderer signature moves.
- No NumberInput, EditableLabel, audio family, motion research, Longhorn lab,
  release/version/workflow, downstream, or sibling-repository work.

## Acceptance Criteria

- [ ] Real editable query, pointer selection, keyboard navigation/dismissal,
      clear, and freeform commit rebuild host-owned Select state.
- [ ] Searchable focus stays on the editor while highlight moves;
      non-searchable focus stays on the trigger; closing follows the detailed
      focus-return/traversal contract.
- [ ] Disabled control/options are inert; groups and stable instance identity
      remain coherent across two mounted instances.
- [ ] Deferred option rows receive real pointer input after a bounded generic
      backend repair; outside dismissal and nested-layer behavior do not
      regress.
- [ ] The Pagination test-only Select ring/id workaround is removed and its
      mounted regression remains green.
- [ ] Curated specimens remain human-facing.
- [ ] The generated ledger changes only Select to 47 mounted / 127 missing;
      known-delta, visual, and broad accessibility totals do not move.
- [ ] One August log records the exact native path, backend repair, mounted
      proof, validation, and non-claims.

## Writable Scope

- `packages/render/src/select.rs` and focused Select tests
- the smallest Select-focused preservation tests under
  `packages/contracts/headless/` and `packages/contracts/components/`; change
  the landed machine/spec only when a proven implementation defect contradicts
  the governing contract
- the smallest relevant files under `packages/gpui/node-backend/src/` for the
  reproduced deferred-layer hit-test repair and focused backend tests
- Select-only compatibility/specimen state under `packages/gpui/preview/src/`
- the named Select regression and removal of the Pagination Select workaround
  in `packages/gpui/preview/tests/headless_regressions.rs`
- focused Svelte/React Select tests only for preservation; do not change their
  public implementation unless current behavior contradicts the landed
  contract and the worker stops for orchestrator review first
- `scripts/parity-evidence-ledger.ts`, its focused test, and generated
  `parity-evidence-ledger.md` for the single Select mounted-cell move
- this card, its source triage note, one August log, g16/front-door status, and
  `PAPERCUTS.md` only for new execution friction

Do not edit other component contracts/implementations, generic node vocabulary,
tokens/themes, visual fixtures, accessibility reports, package versions,
workflows, releases, downstream repositories, or sibling repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-headless`, `poodle-specs`, and `poodle-render` Select tests;
- focused Svelte and React Select preservation tests;
- focused GPUI node-backend layer/hit-test tests and the named mounted Select
  regression;
- the Pagination mounted regression after removing its Select workaround;
- `effigy regressions:native` and `effigy probe:gpui-specimens`;
- relevant handler/event, contract/spec, machine-shape, and role drift selectors
  when their prerequisites are available without admitting Jetstream;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:rust`, `effigy ci:native`, and `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch; and
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- The landed Svelte/React callback timing, shared transition vectors, Rust
  result surface, or detailed contract must change rather than be implemented.
- Correct editable behavior needs new generic Node vocabulary, hidden renderer
  state, a compatibility shim, or a public web/composite API change.
- Deferred pointer repair expands beyond a small layer/hit-test seam, changes
  overlay geometry/ordering broadly, or regresses dismissal/nested overlays.
- Mounted proof cannot drive real pointer and keyboard dispatch through host
  rebuilds without test-only focus/id stamps.
- The ledger generator changes another row/evidence column or validation
  requires windowed execution, workflow/release mutation, Jetstream admission,
  another component family, or a sibling repository.

## Continuation

Dispatch the linked worker handoff as the single active lane. The worker returns
the exact editable-search path, key/focus/dismiss mapping, deferred-overlay
reproducer and repair, removed Pagination workaround, named mounted regression,
exact ledger totals, validation, and execution log. The worker does not choose
the next lane or merge its PR.
