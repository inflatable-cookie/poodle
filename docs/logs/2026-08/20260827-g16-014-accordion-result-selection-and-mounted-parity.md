# g16.014 — Accordion Result Selection And Mounted Parity

Date: 2026-08-27
Status: complete — merged in PR #88
Card: `docs/roadmaps/g16/014-accordion-result-selection-and-mounted-parity.md`
Source triage: `docs/triage/20260827-173242-post-g16-013-native-lane-decision.md`

## Outcome

Shared Rust Accordion carried duplicate `allow_multiple` mode state, could not
represent a collapsed single result, reported only the activated item, and left
native triggers without Button disclosure semantics or stable instance scope.
The contract now stores `AccordionSelectionValue::Single(Option<String>)` and
`Multiple(Vec<String>)`, requires a non-empty `AccordionHandlers` instance scope,
routes activation through the existing headless ToggleGroup transition, and
projects mode-correct root roles, expanded state, controls/labelled-by relations,
structured focus rings, and disabled focus suppression.

The generated ledger moves only Accordion's GPUI mounted-behaviour cell:
`missing` → `mounted` (42 → 43 mounted, 132 → 131 missing). Known-delta totals
stay 115 present / 60 not-applicable.

## Approved break

- removed `AccordionSpec.allow_multiple` and `with_allow_multiple`
- `AccordionSelectionValue::Single(String)` → `Single(Option<String>)`
- bare `Fn(&str)` renderer callbacks → typed `AccordionHandlers::on_value_change`
- GPUI `Accordion::on_toggle` → `on_value_change`; `from_spec(..., instance_id)`
  now requires an explicit non-empty scope (no fallback)
- Poodle GPUI and deferred Jetstream call sites migrated directly

## Mounted evidence

`packages/gpui/preview/tests/headless_regressions.rs#accordion_result_disclosure_focus_identity_and_disabled_paths`
proves, through production hit testing, focus, and key dispatch:

- single-mode root role absence, labelled Button triggers, expanded state, one
  open Region, controls/labelled-by association, and focus rings
- pointer selection reports `Single(Some(...))` once and rebuilds open state
- collapsible reactivation reports `Single(None)` and removes the panel
- non-collapsible reactivation reports the unchanged single result
- multiple add and remove report complete ordered `Multiple(...)` results,
  including add after remove through the mounted rebuild path
- Enter and Space use the same result path as pointer activation
- disabled items emit nothing and are skipped by sequential focus
- two simultaneously open same-valued mounted accordions keep independent trigger
  and panel runtime ids/relations through rebuilds while the sibling instance
  stays mounted

## Explicit non-claims

- no Svelte/React public prop or behavior change
- no Arrow/Home/End accordion navigation, roving focus, or panel animation
- no Jetstream admission or behavioral repair
- no visual comparison or broad native accessibility proof
- no sibling component work

## Validation

Ran in the worker worktree after `bun install`:

- focused `poodle-specs` and `poodle-render` Accordion tests (including empty-scope rejection)
- focused Svelte and React Accordion tests via `effigy test poodle-svelte -- Accordion`
  and `effigy test poodle-react -- Accordion` (10 tests each)
- `effigy regressions:native` (89/89, including the named mounted regression)
- `effigy probe:gpui-specimens`
- `effigy ci:rust`, `effigy ci:native`, `effigy ci:web`
- `effigy drift:handlers`, `effigy drift:events`
- `effigy check:parity-evidence-ledger`
- `git diff --check origin/main...HEAD`

Not run / blocked:

- final `effigy qa` — known `gate-tree-guard --compare` board issue recorded in
  `PAPERCUTS.md` (reproduces from a clean committed worktree; preceding web/docs
  steps pass). Not Accordion-specific.
- `effigy drift:roles` — deferred Jetstream sibling absent (`PAPERCUTS.md`)

`effigy doctor` baseline (generated-in-src, god-files, stale-suppressions)
unchanged.

## Remaining gaps

- native panel-height animation, visual comparison, accessibility, and Jetstream
  admission unchanged
- generation returns to an orchestrator checkpoint at 43 mounted / 131 missing
