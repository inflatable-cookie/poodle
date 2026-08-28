# g16.018 — Select Semantic Machine And Interface Convergence

Status: complete
Opened: 2026-08-28
Completed: 2026-08-28
Merged: PR #93
Depends on: merged `g16.017` / PR #92 and the resolved decision in
`../../triage/20260828-085200-post-g16-017-native-lane-decision.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../contracts/components/select.md`, `parity-evidence-ledger.md`

## Goal

- Give Svelte, React, shared Rust, and GPUI construction one explicit Select
  state-transition contract for value, query, open state, and highlight.
- Repair the approved pre-1.0 freeform behavior: query is draft state reported
  per edit; value changes only on option selection or explicit freeform commit.
- Replace the legacy Rust toggle/change/clear-only seam with one atomic,
  host-owned transition result and stable instance identity.
- Leave Select at 46 mounted / 128 missing. Real native text entry, overlay
  hit-testing, focus return, mounted proof, and the ledger move belong to
  dependent `g16.019`.

## Current Evidence

- Svelte and React duplicate open/query/highlight state and keyboard decisions.
  Shared core currently owns only option flattening/filtering, disabled checks,
  opening highlight, and placement helpers.
- The detailed contract says freeform query commits on blur or Enter when no
  option is highlighted. Both web adapters currently call `onValueChange` on
  every input event, conflating query draft with selected value.
- Web auto mode requires `searchable=true` for a freeform editor; `freeform`
  alone has no input surface. Rust currently treats `freeform` alone as enough
  to show search, contradicting the public mode-resolution contract.
- `SelectSpec` carries value/open/query but no highlighted option. The renderer
  filters options itself and cannot project a host-controlled highlight.
- `SelectHandlers` exposes independent toggle/change/clear callbacks. It has no
  query/highlight result and no required instance scope.
- The Rust search row is static text; option focus and deferred-overlay pointer
  behavior remain incomplete. This card records those gaps instead of claiming
  mounted parity from direct callbacks.

## Approved Semantic Contract

### State and configuration

Create equivalent pure Select machinery in TypeScript core and
`poodle-headless`. Do not generate one language from the other.

The transition state carries:

- committed `value` as a string, using the effective clear value
  (`defaultValue` or the empty string) for no selection;
- `open: boolean`;
- draft `query: string`; and
- `highlightedValue: string | null`, keyed by stable option value rather than
  list index.

Transition configuration carries the flat option list with disabled state,
effective clear value, `searchable`, `freeform`, and whole-control disabled
state. Group flattening remains pure input normalization. Async loading,
snippets/render props, placement measurement, portals, DOM focus, and native
focus requests remain adapter/runtime responsibilities.

`freeform` is effective only with `searchable=true`, matching the public web
mode-resolution contract. Do not silently make `freeform` alone force custom
mode.

### Events and rules

Use one named event/result model covering at least:

- open, close, toggle;
- user query edit;
- explicit highlight from pointer/hover;
- previous, next, first, and last highlight movement;
- highlighted-option commit and direct option commit;
- explicit freeform commit; and
- clear.

Lock these rules in paired TypeScript/Rust vectors:

- disabled Select is inert;
- opening highlights the selected enabled visible option, otherwise the first
  enabled visible option; no enabled result means no highlight;
- query edit opens the list, filters case-insensitively, and highlights the
  first enabled match;
- movement skips disabled options, clamps at the ends, and never wraps;
- Arrow Up/Down while closed opens first and does not also move twice;
- option commit reports a changed value, copies its label into query, and
  closes;
- Enter with a highlighted option commits that option;
- explicit freeform commit is accepted only when searchable + freeform, no
  option is highlighted, and the query differs from the committed value;
- Escape/Tab/outside close without changing value;
- non-freeform close restores query to the committed option label or empty;
  that internal reset is not a user `onQueryChange` effect;
- clear reports the effective clear value, clears/reset query coherently, and
  does not bubble into a second toggle; and
- repeated open/value transitions with no semantic change do not emit duplicate
  effects.

The result contains the complete next state plus ordered semantic effects:
`openChanged(boolean)`, `queryChanged(string)`, and `valueChanged(string)`.
Adapters update their owned state from the result, then dispatch existing public
callbacks from those effects. One input event produces one atomic result.

### Web pair

- Put the transition model in `packages/core/src/select.ts`; extend its focused
  tests with paired vectors readable enough to mirror in Rust.
- Make Svelte and React consume the shared transition for open/query/highlight,
  navigation, option selection, clear, dismissal, and freeform commit. Keep
  async loading, placement, portals, snippets/render props, and DOM focus thin
  and adapter-owned.
- Preserve the public prop and callback names. This approved pre-1.0 correction
  changes callback timing only: `onQueryChange` reports edits; `onValueChange`
  no longer reports uncommitted freeform keystrokes.
- Freeform blur means focus leaves the Select control, not movement between its
  trigger/input, indicator, and portalled listbox. Prevent blur-before-option-
  click from producing a spurious freeform value before the option commit.
- Add focused paired tests for every transition rule and for exact callback
  ordering/counts. Do not use specimens as the behavior corpus.

### Rust pair

- Add the equivalent pure state/event/effect/result machinery in the
  `poodle-headless` Select module with the same named vectors.
- Extend `SelectSpec` with host-authored highlighted value and helpers needed to
  construct the transition state. Keep value/open/query host-owned. Remove or
  correct helpers that make `freeform` alone imply search/custom mode.
- Replace the public legacy `SelectHandlers { toggle, change, clear }` shape.
  The new handler bundle requires an explicit stable `instance_scope` and has
  one optional transition-result callback. Do not retain `Default`, aliases,
  legacy callback fields, or silent adapters.
- Trigger, clear, and option activation in `poodle-render` must run through the
  pure transition and report one atomic result. Renderer filtering/highlight
  projection must consume shared machinery rather than reimplement it.
- Give trigger, clear, listbox, and option nodes stable instance-scoped runtime
  identities needed by `g16.019`. Project highlighted state visibly and
  semantically without adding mounted claims.
- Migrate all in-repo Select call sites, wrappers, composed renderers, GPUI
  specimens, and Jetstream preview construction mechanically. Preserve each
  host's existing observable behavior; do not close another component's ledger
  row or admit Jetstream.

## Execution Plan

- [x] **Batch 1 — contract and shared machine.** Clarify Select callback/mode
      rules in the detailed contract; implement paired TypeScript/Rust
      transitions and focused vectors.
- [x] **Batch 2 — web adapters.** Move Svelte/React semantic decisions onto the
      shared machine, repair freeform callback timing, and prove callback/order
      behavior without changing the public prop surface.
- [x] **Batch 3 — Rust interface and renderer.** Apply the approved breaking
      handler/state migration, project shared result/highlight/identity, and
      migrate every in-repo call site mechanically.
- [x] **Batch 4 — evidence and closeout.** Keep the ledger totals unchanged,
      write one August execution log, mark this card complete, and leave
      `g16.019` as the single next task.

## Explicit Non-Claims

- No real GPUI search text editor, native keyboard lifecycle, focus return,
  deferred-overlay pointer repair, outside-dismiss backend repair, or mounted
  Select evidence. Those belong to `g16.019`.
- No Select ledger-cell move; totals remain 46 mounted / 128 missing and known
  deltas remain 115 present / 60 not-applicable.
- No new public Svelte/React props, open/query control props, or snippet shape.
- No generic Node/GPUI backend change, visual comparison, broad accessibility
  claim, or specimen-as-conformance matrix.
- No behavioral closure for Pagination, FilterBuilder, OrderBy, RelationPicker,
  TimeZoneSelect, or another composite that constructs Select.
- No NumberInput, EditableLabel, menu family, audio family, motion research,
  Longhorn lab, Jetstream admission, release, version, workflow, downstream, or
  sibling-repository work.

## Acceptance Criteria

- [x] Contract, TypeScript core, Rust headless, Svelte, and React agree on the
      state/event/effect rules above.
- [x] Freeform typing reports query only; explicit Enter/control-blur commit
      reports value only when no option is highlighted.
- [x] Option selection, clear, Escape/Tab/outside close, disabled inertia,
      clamp navigation, and no-op emission counts have paired focused proof.
- [x] Svelte and React retain identical public props/callbacks and consume the
      shared machine rather than parallel semantic branches.
- [x] Rust Select has explicit highlighted state, an instance-scoped atomic
      result handler, no legacy toggle/change/clear compatibility surface, and
      no hidden renderer state.
- [x] Renderer activation uses shared transitions and emits scoped identities;
      all in-repo Rust call sites compile with behavior preserved.
- [x] Curated specimens remain human-facing. Any readout added is small and
      explanatory; Examples does not become a state matrix.
- [x] Ledger generation/checks remain byte-stable at 46 / 128 and 115 / 60.
- [x] One execution log records the approved break, paired vectors, migrations,
      validation, non-claims, and `g16.019` entry state.

## Writable Scope

- `packages/core/src/select.ts` and focused Select tests
- `packages/svelte/components/src/Select.svelte` and focused Select tests
- `packages/react/components/src/Select.tsx` and focused Select tests
- `packages/contracts/headless/src/select.rs`, its export, and focused tests
- `packages/contracts/components/src/select.rs` and focused tests
- `packages/render/src/select.rs`, its export if required, and focused tests
- mechanical Select handler/state migrations in in-repo Rust renderers,
  GPUI compatibility/specimen files, and Jetstream preview construction
- `docs/contracts/components/select.md`
- this card, its source triage note, one August log, g16/front-door status, and
  `PAPERCUTS.md` only for new execution friction

Do not edit generic node/backend APIs, parity-ledger inventories/output, other
component contracts or web implementations, tokens/themes, visual fixtures,
accessibility reports, package versions, workflows, releases, downstream
repositories, or sibling repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused TypeScript core, Svelte, and React Select tests;
- focused `poodle-headless`, `poodle-specs`, and `poodle-render` Select tests;
- construction/compile tests for mechanically migrated composed Select users;
- `effigy test:components`, `effigy probe:gpui-specimens`, and
  `effigy regressions:native`;
- relevant handler/event, contract/spec, and parity-ledger drift selectors when
  their prerequisites are available without admitting Jetstream;
- `effigy check:parity-evidence-ledger` proving totals do not move;
- `effigy ci:rust`, `effigy ci:native`, `effigy ci:web`, `effigy docs:check`;
- one final `effigy qa` after the coherent batch; and
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- Correct freeform timing differs from the operator-approved rule or requires a
  new public web prop.
- The web adapters cannot consume one pure semantic result without moving DOM,
  async loading, portal, measurement, or focus mechanics into core.
- Rust atomic state/result ownership cannot preserve current composed Select
  callers without a compatibility shim or unplanned public composite break.
- A real text editor, new generic Node/backend capability, deferred-overlay
  pointer fix, or mounted proof becomes necessary to complete this substrate
  card. Record it for `g16.019` and stop at the boundary.
- Ledger output moves, another component contract/behavior changes, or
  validation requires windowed execution, Jetstream admission, release/workflow
  mutation, downstream changes, or sibling repositories.

## Continuation

Return the exact shared transition types/rules, paired vectors, web callback
correction, Rust migration, renderer/call-site preservation proof, unchanged
ledger totals, validation, and execution log to the orchestrator. Do not start
`g16.019`. After operator-authorized merge, the orchestrator rechecks the landed
Select substrate and promotes the planned mounted-overlay card to ready.
