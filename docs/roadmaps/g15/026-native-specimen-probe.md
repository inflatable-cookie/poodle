# g15.026 — Headless Native Specimen Probe

Status: **planned** — orchestrator review required before dispatch
Role: **native completion lane for `g15.011`.** The audit left the GPUI third
unmeasured. The separate `g15.027` lane closes human teaching judgment for the
56 mechanically clear pages; `g15.011` requires both lanes.
Consumes: `g15.011` partial screening baseline
Governing refs: `specimen-catalogue-audit.md`,
`../g14/026-human-centred-specimen-catalogue-audit.md` (audit rubric),
`../../contracts/001-working-rules.md`

## Why

`g15.011` measured the Svelte and React catalogues live: every page loaded,
its captions read from the DOM, its controls clicked, and its layout checked at
a narrow viewport. **GPUI could not be measured the same way**, so its grades
in the audit are labelled provisional and structural — derived from the
dispatch table and module source, not from a rendered page.

The blocker is shape, not effort-in-principle: `packages/gpui/preview` is a
binary crate, and `render_single_specimen` depends on `PreviewRoot`,
`AppState`, and the catalogue sidebar. Nothing outside `main.rs` can construct
a specimen page, so no test can render one. The existing headless driver
(`headless_driver.rs`, retained from `g14.023`) mounts a `poodle-node` tree,
which is the component tier, not the page tier.

Until this lands, three audit claims hold for the web catalogue only: that
every page renders, that no page has dead primary interaction, and that pages
behave at a narrow width.

## Scope

- a library seam so the preview's specimen pages can be constructed outside
  `main.rs`
- a headless probe over all 174 native pages
- the audit's GPUI column, re-measured and un-provisioned

## Goals

- [ ] `packages/gpui/preview` exposes enough as a library for a test to build
      an `AppState` and render any slug. `main.rs` keeps owning the window.
- [ ] A headless probe renders every canonical slug and records: the page
      constructs, it is not the `missing_specimen` fallback, it has captioned
      sections, and its axis panes exist where the web page teaches that axis.
- [ ] Representative interaction is driven through the real event tree for the
      pages whose primary affordance is a click, reusing `headless_driver`'s
      machinery rather than calling handlers directly.
- [ ] Narrow-layout behaviour is recorded where the native page has a
      responsive rule to check.
- [ ] The audit's GPUI grades are regenerated from the probe and the
      provisional label removed.

## Acceptance

- [ ] Every one of the 174 native pages has a live-measured grade.
- [ ] The probe runs headlessly in CI-compatible time and is wired into a
      named selector.
- [ ] No `*-windowed` selector, `test:native-visual`, or Jetstream selector is
      required to run it.
- [ ] The audit records the new totals and states what changed from the
      provisional pass.

## Stop Conditions

- The seam turns into a second parity architecture. It exposes construction,
  not a cross-runtime observation plane.
- The probe grows a fixture corpus. `g14.008` rejected that and the retained
  driver's header says so.
- Native pages are graded by screenshot comparison. Specimen screenshots are
  not parity tests.

## Writable Scope

- `packages/gpui/preview` crate shape (a `lib.rs` seam and its `Cargo.toml`)
- a new headless test/probe and its selector in `tasks/effigy.tasks.toml` —
  **operator approval required** for the task definition
- the audit's GPUI column and totals
- one batch log

## Validation

- `effigy check:gpui`, `effigy regressions:native`, the new probe selector,
  `effigy docs:check`, `git diff --check`
- headless only.
