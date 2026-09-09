# 20260909-g18-001-gpui-functionality-census — execution log

Task `g18.001`, planning `d8e174fb40b2634b7d00018c721816ffc037d712`.
Worker branch `ns-d5d083f8-3bec-438f-8c5c-efeffd13b91c`. Nucleus receipts untouched.

## Execution identity

- Ran all 65 retained expected tests via `effigy regressions:native` (cargo
  `headless_regressions` with 65 name filters) at source commit
  `d8e174fb40b2634b7d00018c721816ffc037d712` with GPUI lockfile sha256
  `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`.
- Result: 65 passed, 0 failed, 0 ignored (168 other tests filtered out).
- Recorded in `docs/evidence/gpui/expected-test-execution.json` with a sha256
  of each test body. The checker refuses admission when a body changes, the
  lockfile changes, a test is ignored, or a test goes stale.

## Admission rule actually applied

A retained test admits a capability only when, in the current tree, it (1) runs
`run_headless`, (2) mounts through `HeadlessDriver` directly or through a
helper that does, (3) reaches the production renderer directly, through a
`poodle_render` import, through `node_compat`, or through a helper that does,
and (4) shows the claimed axis signals next to a real assertion. Renderer unit
tests and direct handler calls admit nothing. Nucleus M1 prose admits only the
axes its recorded actions and assertions name; A1 admits node-level
accessibility; V1 admits visual. Accessibility admission is node-level
(role/label/state in the mounted tree) everywhere; the A2 assistive-technology
projection stays a narrow hold on all 175 portable rows.

## Counts (regenerable from `docs/evidence/gpui/gpui-functionality-census.json`)

- Denominator: 176 public / 175 portable; MeterSurface sole non-portable row.
- Rows with at least one admitted capability: 73/175. Fully admitted: 24/175.
- Missing by axis: semantic 102; events 101; pointer 112; keyboard_focus 97;
  accessibility 123; visual 144.
- New mounted receipts: 65 (one per admitted expected test) under
  `docs/evidence/gpui/mounted-receipts/`, schema
  `poodle.g18-gpui-mounted-receipt.v1`.
- Refusals recorded in-census: 224 (per-test missing-axis notes, backend
  bypasses, nucleus gaps, no-evidence rows).
- Capability manifest: 175 entries; 27 carry contract-declared
  not-applicable axes with exact section references (e.g. Box, Grid, Spacer,
  Surface). No not-applicable reason cites platform state; the checker rejects
  A2, platform-tree, AccessKit, gpui-apple, upstream, 0.2.2, publication-hold,
  and assistive-technology phrasing there.
- Missing-capability groups (grouping only, no tranche planned): 11
  substrates, largest overlay-dismissal 44, selection-navigation 24,
  feedback-status 21, drag-resize-reorder 16, form-editing 13.

## Refusals worth naming

- 102 portable rows have no validated receipt and no retained expected test;
  every required capability stays missing for them.
- No expected test was renamed, ignored, or failing, so no stale-test refusal
  fired; the oracles proving those paths fail closed live in
  `scripts/gpui-functionality-census.test.ts` (14 tests: denominator,
  stale/ignored, claim-overreach, backend-bypass, widened-A2, overclaim).
- No component behaviour was repaired and no public API changed. Failing rows
  are reported as missing capabilities for Chatterbox tranche compilation.

## Validation run

- `bun test scripts/gpui-functionality-census.test.ts`: 14 pass.
- `bun scripts/gpui-functionality-census.ts` (generate) then `--check`: match,
  all oracles hold.
- `effigy check:gpui-census`, `effigy test:gpui-census`: pass.
- `effigy check:parity-evidence-ledger`, `effigy test:parity-evidence-ledger`,
  `effigy test:nucleus-parity-receipts`: pass (Nucleus rows byte-identical).
- `effigy docs:lint`: pass (cross-runtime report assertions included).
- `git diff --check`: clean.

## Limitations

- Receipts are point-in-time execution evidence pinned to the recorded commit
  and test-body hashes; editing a test or the GPUI lockfile fails the check
  until the tests are re-run and the census regenerated.
- Axis signals are keyword-shaped over test bodies and receipt prose. They are
  deliberately conservative (visual admits almost nothing outside V1), but a
  future test could in principle show a signal without proving the claim; the
  per-test signal lists are stored in each receipt for review.
- Substrate groups are contract-purpose keyword buckets for compilation, not a
  repair plan.
- No windowed selector was run. No Jetstream claim is admitted.
