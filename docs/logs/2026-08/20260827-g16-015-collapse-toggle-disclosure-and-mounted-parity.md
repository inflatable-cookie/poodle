# g16.015 — CollapseToggle Disclosure And Mounted Parity

Date: 2026-08-27
Status: complete — merged in PR #90
Branch: `t3code/collapse-toggle-worker`
Card: `docs/roadmaps/g16/015-collapse-toggle-disclosure-and-mounted-parity.md`
Source triage: `docs/triage/20260827-195632-post-g16-014-native-lane-decision.md`

## Outcome

Shared Rust CollapseToggle hardcoded the default label as `Toggle section`,
never projected expanded state, left disabled controls focusable, and declared
no enabled tab index or structured focus ring. The renderer now uses
`CollapseToggleSpec::effective_aria_label()`, projects `expanded = !is_collapsed`,
gives enabled controls a sequential Button tab stop plus the contracted focus
ring, and removes disabled controls from focus and activation. The callback
still reports `!is_collapsed` with no hidden renderer state.

The generated ledger moves only CollapseToggle's GPUI mounted-behaviour cell:
`missing` → `mounted` (43 → 44 mounted, 131 → 130 missing). Known-delta totals
stay 115 present / 60 not-applicable. GPUI accessibility stays `manual`. GPUI
visual stays `missing`. Jetstream stays deferred.

## Repair

- Default labels are `Collapse` when expanded and `Expand` when collapsed. An
  explicit `aria_label` overrides both.
- Expanded state is always the inverse of `is_collapsed`.
- Enabled controls declare `tab_index=0` and a structured focus ring using the
  spec colour/width tokens and `0.0625rem` offset.
- Disabled controls set the backend disabled channel, have no activation
  handler, no focus handle, no tab stop, and no ring. Opacity and default
  cursor stay contract-owned.
- Activation remains prop-driven: every accepted action reports
  `!spec.is_collapsed`; the host rebuilds. Repeated activation without rebuild
  may repeat the same next value.
- `effective_icon_name()` is unchanged: expanded paints the authored direction,
  collapsed paints the exact opposite.

## Mounted evidence

`packages/gpui/preview/tests/headless_regressions.rs#collapse_toggle_disclosure_focus_and_disabled_through_mounted_pointer_and_keyboard`
proves, through production hit testing, focus, and key dispatch:

- expanded default announces `Collapse`, expanded true, and the authored left
  chevron
- pointer activation reports collapsed true once, the host rebuilds, the label
  becomes `Expand`, expanded becomes false, and the chevron flips
- Enter and Space travel the same production activation path and host rebuild
- an explicit label survives both collapsed states
- enabled focus is a real sequential stop and the node declares the contracted
  ring
- disabled controls expose disabled state, cannot receive sequential focus, and
  emit nothing through real pointer dispatch against painted bounds

All four direction pairs are covered by focused spec/renderer tests. Fixture
ids are test targeting aids only.

## Explicit non-claims

- no Svelte/React public prop or behavior change
- no Rust spec shape change, hidden renderer state, region ids, or `controls`
- no DockRegion or SplitView semantic rewrite
- no visual comparison, GPUI visual fixture, or broad native accessibility proof
- no Select, EditableLabel, NumberInput, Rating, overlay, or sibling work
- no Jetstream admission, release, version, workflow, or downstream change

## Validation

Ran in the worker worktree after `bun install`:

- focused `poodle-specs` CollapseToggle tests (2)
- focused `poodle-render` CollapseToggle tests (7)
- focused Svelte and React CollapseToggle tests (4 each, unchanged web implementations)
- named mounted CollapseToggle regression
- `effigy regressions:native` (90/90)
- `effigy probe:gpui-specimens`
- `effigy drift:handlers`, `effigy drift:events`
- `effigy docs:spec-drift`, `effigy docs:contract-drift`
- `effigy test:parity-evidence-ledger` and `effigy check:parity-evidence-ledger`
- `effigy ci:rust`, `effigy ci:native`, `effigy ci:web`
- `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

Not run / blocked:

- `effigy drift:roles` — deferred Jetstream sibling absent (`PAPERCUTS.md`)

`effigy doctor` baseline (generated-in-src, god-files, stale-suppressions)
unchanged. Northstar rust-quality activation is not installed in this
repository and was not absorbed.

## Remaining gaps

- native accessibility, visual comparison, and Jetstream admission unchanged
- generation returns to an orchestrator checkpoint at 44 mounted / 130 missing
