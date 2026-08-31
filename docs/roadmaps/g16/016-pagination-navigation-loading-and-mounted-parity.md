# g16.016 — Pagination Navigation, Loading, And Mounted Parity

Status: complete
Opened: 2026-08-27
Completed: 2026-08-27
Merged: PR #91
Depends on: merged `g16.015` / PR #90 and the fixed Pagination contract
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/pagination.md`, `parity-evidence-ledger.md`

## Goal

- Make native Pagination suppress every page and page-size interaction while
  loading, matching Svelte and React.
- Prove numbered, simple, and full navigation plus the wired page-size Select
  through real headless GPUI pointer and keyboard dispatch and host-owned
  rebuilds.
- Move exactly Pagination's GPUI mounted-behavior cell from `missing` to
  `mounted`: 44 → 45 mounted and 130 → 129 missing. Keep known-delta totals at
  115 present / 60 not-applicable.

## Current Evidence

- Svelte and React share the same page-window helpers, one-based page model,
  current-page inertia, boundary disabling, loading suppression, variant
  summaries, page-request payloads, and page-size callback behavior.
- `PaginationSpec` already carries current/total pages, sibling window,
  variants, item totals, page size, limit options, compact/loading state,
  presentation axes, and chrome resolution. Its helpers clamp display and
  request ranges without backend knowledge.
- `poodle-render::pagination_with_handlers` already emits live numbered,
  simple, and full controls. Page callbacks carry the destination page; the
  current page, ellipsis, unavailable boundary, and loading page buttons emit
  nothing.
- The wired page-size path composes the production shared Select with
  host-controlled open and value callbacks. It currently omits
  `SelectSpec::is_disabled = spec.is_loading`, leaving the selector live while
  every page button is disabled.
- The GPUI specimen already has one live full-variant example with host-owned
  page, page-size, and selector-open state. Preserve the curated Examples,
  Sizes, and Densities surfaces.
- No named mounted regression currently drives Pagination. Construction and
  focused renderer tests do not satisfy the ledger's mounted evidence cell.

## Fixed Contract

### Page requests

- Pagination remains controlled. Every accepted activation reports only the
  requested one-based destination; the host owns current page and rebuilds the
  spec.
- Numbered mode exposes the contract page window. A non-current page reports
  its own number; previous/next report the adjacent in-range page; the current
  page and ellipsis report nothing.
- Simple mode keeps Prev/Next plus its range summary. Full mode keeps
  first/previous/next/last only where the existing native handler capability
  makes them actionable, plus `Page X of Y`.
- First-page previous/first and last-page next/last controls remain disabled
  and inert through pointer, Enter, and Space.
- Loading makes every navigation control and the wired page-size Select
  disabled and inert. It must not report open, page, or page-size changes.

### Page-size selector

- Preserve the existing composition: `Show` + shared Select + `per page`.
- When both controlled selector handlers are present, the trigger reports the
  next open state; the host rebuilds with `limit_open`. An enabled option
  reports its parsed `usize`; the host applies page size and rebuilds.
- Apply Pagination loading to the composed Select through its existing public
  disabled field. Do not fork Select behavior or add Pagination-owned overlay
  machinery.
- Exercising this one composed path does not move or relabel Select's own
  mounted-behavior ledger cell.

### Identity and focus

- Keep enabled page buttons as ordinary sequential button stops and disabled
  boundaries outside accepted activation. Enter and Space use the same
  production activation callback as pointer input.
- The mounted fixture may assign stable test-only ids after production
  rendering so the driver can target page, boundary, and Select parts. Those
  ids are evidence plumbing, not a new Pagination identity API.
- Do not extend `NodeRole`, add navigation/current-page accessibility
  vocabulary, or claim broad native accessibility. That evidence column stays
  `manual`.

## Execution Plan

- [x] **Batch 1 — focused loading and renderer contract.** Reproduce the live
      page-size selector under loading, apply the existing disabled channel,
      and strengthen focused spec/renderer tests for page windows, variants,
      current/boundary/loading inertia, destination payloads, and limit
      callbacks.
- [x] **Batch 2 — mounted host behavior.** Add one readable named headless GPUI
      regression through production Pagination, Select, renderer, and node
      backend. Drive pointer, Enter, and Space requests; rebuild the host; open
      and choose a page-size option; prove boundaries, current page, and loading
      emit nothing.
- [x] **Batch 3 — evidence and closeout.** Preserve the human-facing specimen,
      regenerate only Pagination's mounted ledger cell, close this card/source
      decision/log/front doors, and run the required headless board.

## Specimen And Mounted Proof

- Preserve the existing Default, Middle of range, Few pages, Simple, Full,
  chrome, boundary, Sizes, and Densities examples. Do not turn the specimen
  into an exhaustive test matrix.
- The named mounted regression proves:
  - a numbered non-current page reports its authored destination and the host
    rebuild makes it current;
  - pointer, Enter, and Space share the destination callback path;
  - previous/next work in range while current, ellipsis, and boundary controls
    report nothing;
  - simple and full variants report the correct adjacent and first/last
    destinations and render their existing summaries;
  - the wired page-size Select opens through real activation, an enabled option
    reports its numeric limit, and the host rebuilds closed with that limit;
  - loading exposes disabled navigation and Select controls and produces no
    page, open, or page-size event through pointer or keyboard; and
  - unrelated Select behavior is not claimed.
- Direct callback invocation, renderer inspection alone, specimen state
  mutation, or a fixture-only fake control does not satisfy mounted proof.

## Explicit Non-Claims

- This card does not change Svelte/React props, controller behavior,
  implementations, or public tests except focused preservation evidence.
- It does not change the public Rust spec or handler shapes, remove `page` /
  `currentPage` or chrome/standalone compatibility fields, or add aliases.
- It does not redesign Select, close Select's ledger row, change generic
  node/backend vocabulary, or build a new overlay/focus system.
- It does not claim GPUI navigation-landmark/current-page accessibility,
  assistive-technology parity, or visual comparison.
- It does not change PaginationSummary or other navigation/composite families.
- It does not admit Jetstream or touch releases, versions, workflows,
  downstream repositories, or sibling repositories.

## Acceptance Criteria

- [x] Loading disables the wired page-size Select as well as every page button.
- [x] Numbered, simple, and full variants preserve their page-request payloads,
      summaries, page windows, and host-owned state model.
- [x] Current page, ellipsis, boundary, and loading activations emit nothing.
- [x] Enabled pointer, Enter, and Space activation report the same destination
      and the mounted host rebuilds the spec.
- [x] The wired limit Select reports controlled open and numeric page-size
      changes through the production mounted path; loading suppresses both.
- [x] Focused Svelte and React Pagination tests remain green without web
      implementation changes.
- [x] The curated GPUI specimen remains human-centred and keeps its current
      axes and live full example.
- [x] One named mounted regression proves the production Pagination path.
- [x] The generated ledger changes only Pagination to 45 mounted / 129 missing;
      Select stays missing, known-delta totals stay 115 / 60, and visual /
      accessibility cells remain unchanged.
- [x] One August log records the loading repair, behavior evidence, validation,
      non-claims, and next checkpoint.

## Writable Scope

- `packages/render/src/pagination.rs` and focused tests
- the smallest Pagination-only compatibility/specimen change under
  `packages/gpui/preview/src/` if mounted plumbing requires it
- the smallest Pagination mounted regression change in
  `packages/gpui/preview/tests/headless_regressions.rs`
- focused Pagination spec tests in
  `packages/contracts/components/src/pagination.rs` only when needed to lock
  existing helpers; do not change the public spec shape
- focused Svelte/React Pagination tests only for preservation evidence; do not
  change web implementations
- `scripts/parity-evidence-ledger.ts`, its focused test, and generated
  `parity-evidence-ledger.md` for the one mounted cell
- this card, its source decision, one August log, g16/front-door status, and
  `PAPERCUTS.md` only for new execution friction

Do not edit generic node/backend APIs, Select semantics or public APIs, other
component contracts or implementations, theme/token definitions, visual
fixtures, accessibility reports, package versions, workflows, releases,
downstream repositories, or sibling repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-specs` and `poodle-render` Pagination tests;
- focused Svelte and React Pagination tests;
- the named mounted Pagination regression;
- `effigy regressions:native` and `effigy probe:gpui-specimens`;
- relevant handler/event and contract/spec drift selectors;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:rust`, `effigy ci:native`, and `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch; and
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- The paired web runtimes or detailed contract disagree on page numbering,
  current/boundary behavior, variant payloads, loading, or limit changes.
- Correct behavior requires a public API break, removal of existing aliases,
  a generic node/backend change, or a Select semantic redesign.
- Mounted proof cannot drive production page and limit interactions through
  real pointer/keyboard dispatch plus host rebuild.
- The ledger generator changes another row/evidence column or validation
  requires windowed execution, workflow/release mutation, Jetstream admission,
  or another component family.

## Continuation

Return the loading repair, focused renderer/spec test names, mounted regression
name, host-rebuild and limit-selector proof, exact ledger totals, validation,
and execution log to the orchestrator. Do not compile or implement `g16.017`.
After operator merge, the orchestrator returns to the measured 45 mounted / 129
missing ledger and chooses the next bounded parity lane.
