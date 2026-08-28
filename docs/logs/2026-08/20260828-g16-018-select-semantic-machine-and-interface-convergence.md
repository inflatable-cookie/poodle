# g16.018 — Select Semantic Machine And Interface Convergence

Date: 2026-08-28
Status: complete — awaiting merge
Branch: `t3code/13de1b7f`
Card: `docs/roadmaps/g16/018-select-semantic-machine-and-interface-convergence.md`
Source triage: `docs/triage/20260828-085200-post-g16-017-native-lane-decision.md`

## Outcome

Select's open, query, highlight, and committed-value transitions now share one
named machine in TypeScript core and `poodle-headless`. Svelte and React consume
that result instead of parallel adapter branches. Freeform typing reports
`onQueryChange` only; `onValueChange` fires on option selection, clear, or
explicit Enter/control-blur commit when no option is highlighted.

Shared Rust dropped the legacy toggle/change/clear handler bundle. The public
surface is `SelectHandlers::new(instance_scope).on_transition(...)`. Trigger,
clear, and option activation run through `select_transition` and report one
atomic result. `SelectSpec` carries host-authored highlighted value. Search
requires `searchable`; `freeform` alone does not show a search row or force
custom mode. Instance-scoped identities cover trigger, clear, listbox, search,
and option nodes.

The generated ledger does not move: 46 mounted / 128 missing. Known-delta
totals stay 115 present / 60 not-applicable. No mounted Select evidence is
claimed.

## Exact API break

- Svelte/React public prop names are unchanged. Callback timing is the
  approved pre-1.0 correction: query per edit, value on explicit commit.
- Rust `SelectHandlers` no longer has `toggle`, `change`, `clear`, or
  `Default`. It requires `instance_scope` and one optional transition-result
  callback.
- `SelectSpec.highlighted_value` is host-authored. `shows_search_input` is
  `searchable` only.
- No aliases, shims, or silent fallbacks.

## Shared transition

`packages/core/src/select.ts` and `packages/contracts/headless/src/select.rs`
implement the same events (`OPEN`, `CLOSE`, `TOGGLE`, `QUERY`, `HIGHLIGHT`,
`HIGHLIGHT_PREV`/`NEXT`/`FIRST`/`LAST`, `COMMIT_HIGHLIGHTED`, `COMMIT_OPTION`,
`COMMIT_FREEFORM`, `CLEAR`, `OPTIONS_CHANGED`) and ordered effects (`openChanged`, `queryChanged`,
`valueChanged`). Query highlight is first enabled match; opening still prefers
the selected option. Shared vectors live in
`packages/contracts/headless/vectors/machines.json` under `select` and run in
both conformance harnesses.

## Web adapters

Svelte and React dispatch those events, apply the complete next state, then
fire existing public callbacks from effects. Control-blur means focus left the
root including the portalled listbox. Tab, Escape, and outside dismiss close
without a freeform value. Option `mousedown` preventDefault stops
blur-before-click from committing a draft query.

## Native migration

Renderer activation uses the shared transition. In-repo callers
(Pagination, FilterBuilder, OrderBy, RelationPicker, TimeZoneSelect,
BlockEditor, GPUI specimens/node_compat, Jetstream preview construction) were
migrated mechanically. Composite public APIs are unchanged. Jetstream received
compile maintenance only.

## Explicit non-claims

- no real GPUI search text editor, native keyboard lifecycle, focus return,
  deferred-overlay pointer repair, or mounted Select evidence
- no Select ledger-cell move
- no new public Svelte/React props
- no generic Node/GPUI backend change, visual comparison, or accessibility
  claim
- no behavioral closure for Pagination, FilterBuilder, OrderBy, RelationPicker,
  TimeZoneSelect, or another composite
- no Jetstream admission, release, version, workflow, or downstream change

`g16.019` remains planned and blocked on the merged substrate.

## Review corrections

QUERY highlight is the first enabled visible match, not the selected option.
`OPTIONS_CHANGED` reconciles highlight after lazy loads without a second
`queryChanged`. Composite Select scopes are instance-unique (BlockEditor uses
`block.id`; Pagination, OrderBy, RelationPicker, and TimeZoneSelect take
optional handler `instance_id` plus authored spec labels). Paired vectors
cover `HIGHLIGHT_PREV`/`FIRST`/`LAST` and `OPTIONS_CHANGED`.

## Validation

Ran in the worker worktree
(`/Users/tom/.t3/worktrees/poodle/t3code-13de1b7f`, branch `t3code/13de1b7f`):

- focused TypeScript core Select tests and `machines.json` `select` vectors
- focused `poodle-headless` `select_conformance`
- focused Svelte/React Select tests (26)
- focused `poodle-specs` and `poodle-render` Select tests
- `effigy test:components` (3133)
- `effigy probe:gpui-specimens` (8)
- `effigy regressions:native` (92/92); Pagination's test-only Select stamp
  now also writes `runtime_id` because production identities take precedence
  in the GPUI backend
- `effigy drift:handlers`, `effigy drift:events`
- `effigy docs:spec-drift`, `effigy docs:contract-drift`
- `effigy check:parity-evidence-ledger` (46 mounted / 128 missing; 115 / 60)
- `effigy ci:rust`, `effigy ci:web`, `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

Not run / blocked:

- `effigy drift:roles` and Jetstream preview — deferred Jetstream sibling
  absent (`PAPERCUTS.md`)

`effigy doctor` baseline (generated-in-src, god-files, stale-suppressions)
unchanged. Northstar rust-quality activation is not installed in this
repository and was not absorbed. `docs:machine-shape-drift` still reports
pre-existing unpinned duplicates (rating, history-center, and others);
Select is pinned.

## Remaining gaps

- native search editing, overlay pointer targeting, focus return, and mounted
  Select proof belong to `g16.019`
- generation stays at 46 mounted / 128 missing until that card is recompiled
  after merge

## Stopped identity boundary

Making default composite construction unique requires a source-breaking
change to `OrderByHandlers`, `PaginationHandlers`, `RelationPickerHandlers`,
`TimeZoneSelectHandlers`, and `DateTimeZonePickerHandlers`: required
`instance_id`, no `Default`, and wrapper/`pagination(...)` signature
changes. That is outside the approved Select handler break. The card stop
condition and working rules require operator approval before that
migration.

The required-ID expansion was reverted. BlockEditor stays unique via
`block.id`. Query highlight, `OPTIONS_CHANGED`, and navigation vectors
stay. Default OrderBy, Pagination (including `pagination(...)` /
`js_pagination`), RelationPicker, and TimeZoneSelect still collide when
the host does not author a scope.

Operator choice: approve that composite-handler break and re-apply it
consistently (including `pagination(...)` and `js_pagination`), or leave
default-construction identity for a later card.
