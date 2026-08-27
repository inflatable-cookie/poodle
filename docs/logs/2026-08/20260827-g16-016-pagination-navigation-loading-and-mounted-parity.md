# g16.016 — Pagination Navigation, Loading, And Mounted Parity

Date: 2026-08-27
Status: complete — merged in PR #91
Branch: `t3code/review-pagination-worker-handoff`
Card: `docs/roadmaps/g16/016-pagination-navigation-loading-and-mounted-parity.md`
Source triage: `docs/triage/20260827-210417-post-g16-015-native-lane-decision.md`

## Outcome

Shared Rust Pagination disabled every page button while `is_loading`, but the
wired page-size Select stayed live: hosts could still open it and report a
limit change. Loading now sets `SelectSpec::is_disabled` and keeps the composed
Select closed. Page buttons also declare the contracted focus ring and
sequential tab stops so Enter/Space share the production activation path.

The generated ledger moves only Pagination's GPUI mounted-behaviour cell:
`missing` → `mounted` (44 → 45 mounted, 130 → 129 missing). Known-delta totals
stay 115 present / 60 not-applicable. Select stays `missing`. GPUI accessibility
stays `manual`. GPUI visual stays `missing`. Jetstream stays deferred.

## Repair

- Loading propagates into the composed Select through its public `is_disabled`
  field and forces the panel closed so open host state cannot leave live
  options.
- Enabled page and arrow controls declare `tab_index=0` and a structured focus
  ring; disabled/loading controls leave the sequential focus set and carry no
  activation handler.
- Numbered, simple, and full destination payloads, summaries, current-page
  inertia, ellipsis, and boundary disabling stay host-owned and controlled.

## Mounted evidence

`packages/gpui/preview/tests/headless_regressions.rs#pagination_navigation_limit_and_loading_through_mounted_pointer_and_keyboard`
proves, through production hit testing, focus, and key dispatch:

- numbered non-current pages report authored destinations; the host rebuilds
- pointer, Enter, and Space share the destination callback path
- previous/next work in range while current and boundary controls emit nothing
- simple and full variants report adjacent / first / last destinations and keep
  their summaries
- the wired page-size Select opens through real pointer activation; an enabled
  option reports its numeric limit through the production `on_activate` path
  (Enter after a test-only focus-ring stamp — Select options declare no ring,
  and deferred overlay pointer hit-testing misses those rows in headless)
- loading exposes disabled navigation and Select controls and produces no page,
  open, or page-size event

Fixture ids and the option focus-ring stamp are test targeting aids only. Select's
own ledger cell is not claimed.

## Explicit non-claims

- no Svelte/React public prop or implementation change beyond preservation tests
- no Rust public spec/handler shape change, alias removal, or hidden renderer state
- no Select redesign, Select ledger move, or Pagination-owned overlay machinery
- no navigation-landmark / current-page accessibility claim
- no visual comparison, PaginationSummary, or sibling component work
- no Jetstream admission, release, version, workflow, or downstream change

## Validation

Ran in the worker worktree after `effigy bootstrap:deps`:

- focused `poodle-specs` Pagination tests
- focused `poodle-render` Pagination tests (6)
- focused Svelte and React Pagination tests (loading preservation added)
- named mounted Pagination regression
- `effigy regressions:native` (91/91)
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
- generation returns to an orchestrator checkpoint at 45 mounted / 129 missing
