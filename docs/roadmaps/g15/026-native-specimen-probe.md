# g15.026 — Headless Native Specimen Probe

Status: **complete** — PR #50 merged as `32df3667` on 2026-08-20
Depends on: `g15.011`, `g15.019`, `g15.034`
Role: **native completion lane for `g15.011`.** The separate `g15.027` lane
owns human teaching judgment for the 56 mechanically clear pages.
Governing refs: `specimen-catalogue-audit.md`,
`../g14/026-human-centred-specimen-catalogue-audit.md` (audit rubric),
`../../contracts/001-working-rules.md`

## Outcome

Every canonical GPUI specimen route is mounted through the real preview root on
GPUI's in-memory test platform. The probe proves that all 174 portable catalogue
entries reach a real specimen rather than the fallback, and that every admitted
axis tab can be opened through the real event tree.

This is construction evidence, not visual or behavioural parity. Existing
focused native regressions continue to own component interactions. The six
`g15.028`–`g15.033` review children own human teaching and layout judgment.

## Readiness Decision

The first draft assumed `packages/gpui/preview` needed a `lib.rs`. Current
source disproves that assumption:

- `main.rs` already has an in-binary `#[cfg(test)]` module that mounts
  `PreviewRoot` with `TestAppContext::single()` and `add_window_view`.
- the canonical 174-entry Rust catalogue is available in the binary through
  `component_registry::CANONICAL_COMPONENTS`;
- GPUI's `test-support` feature exposes test-only `debug_selector` bounds and
  real pointer dispatch;
- `specimen_layout` already owns explicit axis admission and stable tab state.

Keep the preview a binary crate. Add no public library seam and no second
observation architecture.

## Delivery

### 1. Add test-only page observation

- Add stable, test-only debug selectors to the real specimen-card root, the
  `missing_specimen` fallback, admitted axis tabs, and their rendered pane
  bodies. Use GPUI's `debug_selector`; it is a no-op outside tests.
- Do not add public preview APIs, generated metadata, snapshots, accessibility
  dumps, or a cross-runtime result schema.
- Keep the production dispatcher and catalogue identity authoritative. Do not
  duplicate the 174 slugs in a fixture list.

### 2. Mount the canonical catalogue

- Add an in-binary `specimen_probe` test module. It may live in `main.rs` or a
  private `src/specimen_probe.rs` included under `#[cfg(test)]`.
- Mount the real `PreviewRoot` on `TestAppContext`; select each entry from
  `CANONICAL_COMPONENTS`; render at a fixed 768px-wide viewport; and record the
  slug in every failure.
- For each route, assert that the real specimen-card selector is painted and
  the fallback selector is absent.
- Assert the denominator is exactly 174 and that the web-only `MeterSurface`
  is not silently counted as a native route.
- Reuse one test context where safe. Reset route-owned state between entries so
  one specimen cannot make another pass.

### 3. Exercise admitted axes

- Discover `Sizes` and `Densities` from the rendered page. When a tab is
  present, click it through GPUI's real pointer event path and assert its pane
  paints. Return to `Examples` before advancing.
- Do not recreate the web axis census. `g15.019` and `g15.034` already own
  axis eligibility and exact value-domain evidence; this probe proves the
  admitted native panes construct and navigate.
- Do not drive arbitrary specimen controls. `effigy regressions:native` and
  focused owner-local tests remain the interaction authority.

### 4. Make the proof durable

- Add `probe:gpui-specimens` to `tasks/effigy.tasks.toml` as the exact
  headless binary-test selector.
- Compose it into `ci:conformance` and `ci:native`. Keep the legacy
  `ci:conformance` name but correct its comment: it now contains the focused
  mounted regressions plus the native specimen construction probe, still with
  no cross-runtime parity claim.
- Do not edit `.github/workflows/`. The existing workflow already calls
  `effigy ci:conformance`.
- Record the probe wall time. Stop and return to planning if the test body
  exceeds two minutes after compilation; do not hide a slow sweep inside QA.

### 5. Correct the audit claim

- Update `specimen-catalogue-audit.md` from “structural, provisional” to an
  exact “headless render + structural” GPUI measurement.
- Preserve the existing grades unless the live construction result exposes a
  real defect. Record 174/174 construction results, axis-navigation totals,
  failures, and the one fixed `n/a` (`MeterSurface`).
- State the remaining limit plainly: the probe does not judge copy, visual
  quality, arbitrary component interactions, or horizontal overflow.

## Acceptance

- [x] The named selector mounts exactly 174 canonical GPUI pages at 768px.
- [x] Every route paints a real specimen card; none reaches
      `missing_specimen` or panics.
- [x] Every rendered `Sizes` or `Densities` tab opens its real pane through
      pointer input.
- [x] `MeterSurface` remains the single explicit native `n/a`.
- [x] `probe:gpui-specimens` is composed into `ci:conformance` and
      `ci:native`; both remain fully headless.
- [x] The audit records the exact live-native result and no longer calls the
      GPUI render result provisional.
- [x] One batch log records selectors, denominator, axis-tab totals, runtime,
      changed claims, and any failures found.

## Writable Scope

- `packages/gpui/preview/src/main.rs`
- `packages/gpui/preview/src/specimens/mod.rs`
- `packages/gpui/preview/src/specimens/specimen_layout.rs`
- one private in-binary probe module if split from `main.rs`
- `tasks/effigy.tasks.toml`
- `docs/roadmaps/g15/specimen-catalogue-audit.md`
- one August batch log
- append-only `PAPERCUTS.md` for new execution friction only

Do not add `lib.rs`, change public packages or component contracts, edit the
generated catalogue, edit generation/dispatch state, or touch Jetstream.

## Validation

Run one coherent headless round:

- `effigy probe:gpui-specimens`
- `effigy ci:conformance`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy catalogue:check`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

No `*-windowed`, `test:native-visual`, Jetstream, browser, or release selector.

## Stop Conditions

- A public `lib.rs`, shared observation schema, fixture corpus, or generated
  per-page test inventory appears necessary.
- The probe cannot mount the production `PreviewRoot` without changing a
  component or public preview API.
- A canonical route requires OS prompts, network access, or another external
  side effect merely to construct.
- Correct axis admission cannot be consumed from `g15.019` / `g15.034` and
  would require a second cross-runtime authority.
- The post-compilation test body exceeds two minutes.
- A failure exposes component behaviour debt outside construction or axis-tab
  navigation. Record the slug and return it for a bounded follow-up; do not
  absorb that repair into this probe.

## Continuation

After merge and closeout, dispatch the six exact screen-clear review children
`g15.028`–`g15.033`. `g15.011` completes only after those six land. Then
readiness-review `g15.012`. `g15.013` remains the final certification gate.
