# g15.026 — Headless native specimen probe

Date: 2026-08-20
Card: `docs/roadmaps/g15/026-native-specimen-probe.md`
Handoff: `docs/handoffs/20260820-103116-g15-026-native-specimen-probe.md`
PR: #50

## Outcome

Every portable catalogue route now has live native construction evidence. The
new in-binary probe (`packages/gpui/preview/src/specimen_probe.rs`, test-only)
mounts the production `PreviewRoot` on GPUI's in-memory test platform and
walks `component_registry::CANONICAL_COMPONENTS` directly at a 768px-wide
viewport — no `lib.rs`, no public API, no second observation architecture.

Measured result: **174/174 routes construct, none reach the
`missing_specimen` fallback, and every advertised axis tab opens its pane
through real pointer input — 126 `Sizes` tabs and 127 `Densities` tabs.**
`MeterSurface` is asserted out of the native denominator and remains the
single native `n/a` (web-only, spec 068). No failures; no grade in the audit
changed, because the probe exposed no construction defect.

This is construction evidence only. The probe does not judge copy, visual
quality, arbitrary component interactions, or horizontal overflow; those keep
their existing owners. The known Stepper selection/re-run and
UiPresentationProvider cascade gaps are interaction defects and remain with
the release-gap register.

## How it observes

- Test-only `debug_selector` markers on the specimen-card root
  (`specimen-card`), the fallback (`specimen-missing`), each layout tab
  (`specimen-tab-{examples,sizes,densities}`), and the active pane
  (`specimen-pane-{...}`). `debug_selector` compiles to a no-op outside
  GPUI `test-support` builds, so production carries nothing.
- The sweep asserts the registry is exactly 174 entries before walking it,
  resets nothing by hand — each route gets a fresh window and root, which is
  also the route-state reset — and includes the active slug in every failure.
- Axis tabs are discovered from the mounted page and clicked with
  `simulate_click` on their painted bounds; no handler is called directly and
  the web axis census is not duplicated (`g15.019`/`g15.034` own eligibility).

Two test-platform findings shaped the design, both recorded in
`PAPERCUTS.md`:

- gpui 0.2.2's `Frame::clear` never clears `debug_bounds`, so selector
  entries accumulate for the life of a window. A reused window reported route
  N's tabs for route N+1 (the first sweep "passed" 174/174 tabs vacuously).
  The probe opens a fresh window per route instead.
- One shared `TestAppContext` slowed superlinearly with open windows (174
  routes ≈ 90s), so the sweep runs as four parallel shards.

## Build repair (flagged scope exception)

The bin test target did not compile at all on clean `main`:
`specimens/scene_specimen.rs`'s test module used `use super::*`, which chains
the parent's `use gpui::*` and resolves `#[test]` to gpui-macros 0.2.2's
`test` proc macro — the one the repo already documents as crashing rustc —
producing a SIGBUS in `librustc_driver` on every `--test` build (reproduced
on stable 1.97.1 and 1.96.0, in both checkouts). No existing selector runs
that target, so the breakage was invisible. The fix switches the test module
to explicit imports — test-only, no production change. `scene_specimen.rs`
sits outside the card's listed writable scope; it was flagged to the operator
at the seam-proof checkpoint.

Unblocking the target exposed three pre-existing failures in
`contract_usage_docs::tests` (`parses_button_contract_usage_data`,
`parses_sidebar_nav_contract_usage_data`,
`parses_contracts_with_shifted_heading_numbers`): their expected contract
events/slots no longer match the current docs. They fail independently of
this batch, are not run by `probe:gpui-specimens` (which filters to
`specimen_probe`), and are recorded in `PAPERCUTS.md` for an owner.

## Numbers

- Routes: **174/174** construct a real specimen card; **0** reach the
  fallback; **0** panics. Denominator asserted, not inferred.
- Axis navigation: **126** `Sizes` tabs, **127** `Densities` tabs, each opened
  through pointer input with its pane asserted to paint; the probe returns to
  `Examples` before advancing.
- Fallback sentinel: an unknown slug dispatched through the production
  `render_single_specimen` arm paints `specimen-missing` and no card.
- Test-body wall time: shard bodies 5.3s / 6.1s / 7.7s / 8.3s, 8.4s wall for
  all seven probe tests — far under the two-minute stop condition.

## Changed files

- `packages/gpui/preview/src/specimen_probe.rs` — new private in-binary probe
  (three seam proofs plus the four-shard canonical sweep)
- `packages/gpui/preview/src/main.rs` — includes the probe under `#[cfg(test)]`
- `packages/gpui/preview/src/specimens/mod.rs` — `specimen-card` /
  `specimen-missing` test-only markers
- `packages/gpui/preview/src/specimens/specimen_layout.rs` — tab and pane
  test-only markers
- `packages/gpui/preview/src/specimens/scene_specimen.rs` — glob-import fix
  that unbreaks the bin test target (scope exception, flagged)
- `tasks/effigy.tasks.toml` — new `probe:gpui-specimens` selector, composed
  into `ci:conformance` (legacy name kept; comment corrected — still no
  cross-runtime parity claim) and `ci:native`; both remain fully headless
- `docs/roadmaps/g15/specimen-catalogue-audit.md` — GPUI column moved from
  "structural, provisional" to "headless render + structural": measurement
  note, totals-table label, finding 9, the unrendered-pages claim, a
  revision-5 correction entry, the completion-lane note, and the per-row
  legend. Grades preserved.
- `PAPERCUTS.md` — the `debug_bounds` accumulation and bin-test-target
  entries above

No workflow file changed; the existing workflow already calls
`effigy ci:conformance`. The tracked preview lockfile is untouched.

## Changed audit claims

- "GPUI is not measured live, and its grades are provisional" → GPUI
  construction is live-measured headlessly; grades now combine source
  structure with live construction and axis-pane navigation.
- "No unrendered pages in the web catalogue" now extends to GPUI: no route in
  any active runtime falls through to the placeholder.
- The GPUI totals row reads "headless render + structural" with unchanged
  counts (100 A / 68 B / 6 C / 0 D / 1 n/a).
- Plainly stated limit: interaction-liveness and narrow-layout claims still
  cover only Svelte and React.

## Validation

- `effigy probe:gpui-specimens` — 7 passed (3 seam proofs + 4 sweep shards)
- `effigy ci:conformance` — passed (regressions board + probe)
- `effigy regressions:native` — 50 passed
- `effigy check:gpui` — passed
- `effigy catalogue:check` — passed (catalogue-ts and catalogue-rust verified)
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No `*-windowed`, `test:native-visual`, browser, Jetstream, or release
selector ran.
