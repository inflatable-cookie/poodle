# 20260909-g18-001-gpui-functionality-census — execution log

Task `g18.001`, planning `d8e174fb40b2634b7d00018c721816ffc037d712`.
Worker branch `ns-d5d083f8-3bec-438f-8c5c-efeffd13b91c`. Nucleus receipts untouched.
Merged in PR #235 as `8185a9758f146e901499a8a8704a41202f99ca9e` on 2026-09-09 after independent exact-head `ready_to_merge` review ([comment #5605510026](https://github.com/inflatable-cookie/poodle/pull/235#issuecomment-5605510026)) at head `e88cd75c9e2b68fd47e597180d20c6736a80bc31`.

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

  - `bun test scripts/gpui-functionality-census.test.ts`: 17 pass (denominator,
    stale/ignored-test, claim-overreach, backend-bypass, widened-A2, overclaim,
    record-state pin, evidence-text, and announce-hardening oracles).
  - `bun scripts/gpui-functionality-census.ts` (generate) then `--check`:
    byte-identical match, all oracles hold — at the planning commit and again
    at every later commit, because evidence identity is record state (the
    execution record pin), never the live checkout hash.
- `effigy check:gpui-census`, `effigy test:gpui-census`: pass.
- `effigy check:parity-evidence-ledger`, `effigy test:parity-evidence-ledger`,
  `effigy test:nucleus-parity-receipts`: pass (Nucleus rows byte-identical).
- `effigy docs:lint`: pass (cross-runtime report assertions included).
- `git diff --check`: clean.

## Limitations

  - Receipts are point-in-time execution evidence pinned to the execution
    record commit and per-test body hashes. Editing a test or the GPUI lockfile
    fails the check until the tests are re-run and both record and census are
    regenerated; the checker additionally refuses a tree that does not descend
    from the pinned commit.
  - Refusals fall into four recorded classes: no-evidence rows (no receipt and
    no retained test), stale or bypassing tests (absent body, ignored, or no
    mounted-backend production path), unproved axes (a mounted test whose body
    shows no signal for a required axis), and nucleus gaps (manifest rows with
    no validated M1 receipt). Each refusal names its row, test, and cause.
  - Axis signals are keyword-shaped over test bodies and receipt prose. They are
    deliberately conservative (visual admits almost nothing outside V1;
    announcement evidence requires the read/handler idiom, not the bare word);
    receipts store the matched body fragments plus the exact driver and
    renderer references observed, so review reads evidence instead of patterns.
- Substrate groups are contract-purpose keyword buckets for compilation, not a
  repair plan.
- No windowed selector was run. No Jetstream claim is admitted.

## Closeout

- Merge performed by the plugin as `8185a9758f146e901499a8a8704a41202f99ca9e` on 2026-09-09 (PR #235).
- Closeout re-verification on merged main `8185a9758f146e901499a8a8704a41202f99ca9e`: census `--check` match; census tests 17 pass / 0 fail; ledger tests 9 pass / 0 fail; nucleus-parity-receipts tests 17 pass / 0 fail; `effigy docs:lint` pass; `git diff --check` clean.
- Non-blocking reviewer notes (deferred, no acceptance impact): `validatePinAncestry` requires non-shallow history; the four-refusal-class taxonomy fires two shapes in this census (122 unproved-axis + 102 no-evidence).
