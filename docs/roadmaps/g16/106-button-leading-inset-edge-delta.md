# g16.106 — Button Leading-Inset Edge Delta Diagnosis

Status: ready
Type: bounded diagnosis and repair-or-contract — native render path
Opened: 2026-09-05
Depends on: none
Governing refs: `packages/render/src/button.rs:99-129` (`rem_to_px`,
`size_icon_inset_rem`, `has_leading`), `packages/render/src/presentation.rs:12`
(`rem_to_px`), `test/visual/button-comparison/policy.ts` (the g15.047
tolerance table and role findings, e.g. `gpui-omits-box-shadow`),
`../../triage/20260904-155753-lab-button-run-findings.md`, poodle-lab
bundle `docs/logs/2026-09/04-145325-g01-001-button-batch-bundle`
Evidence: lab Button batch at head `fb839407b`, two independent runs agree:
`content-leading-icon` and `state-loading` render a 1.0 logical px edge
where the web renders 0.5.
Dispatch manifest: `../dispatch.md`

## Goal

Decide, with proof, whether the half-pixel difference is a Poodle rounding
defect or a GPUI rasterisation delta, and leave the repository with one of
two outcomes: the inset is emitted exactly, or the delta is contracted.

## Fixed Boundary

- Reproduce through the node inventory: for both fixtures, print the
  emitted padding/inset values from `poodle-render` and compare them with
  the CSS values the Svelte fixture computes at the same size. If they
  differ, the defect is in `rem_to_px`/`size_icon_inset_rem` or the node
  vocabulary (fractional inset lost); fix it there with a headless test that
  asserts the exact emitted value.
- If the emitted values are equal and only the raster differs, do not
  change render code. Add a role finding `gpui-snaps-subpixel-edge` to the
  g15.047 policy table with its rationale and the two fixture ids, so the
  comparator classifies it like the shadow omission. Record it as a known
  delta in the parity ledger's known-delta axis for Button.
- Do not tune tolerances, do not alter the lab, do not touch other
  components.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Diagnosis is evidenced | claim "rasteriser" without printing emitted values | log shows both runtimes' computed inset side by side |
| Repair is exact | fix changes any other fixture's emitted geometry | node inventory diff limited to the two fixtures |
| Contract is honest | role finding added while emitted values differ | reviewer rejects |
| No tolerance creep | policy numbers change | `compare.test.ts` asserts the g15.047 table unchanged |

## Validation

`cargo test -p poodle-render`, `effigy regressions:native`,
`effigy docs:check`, `git diff --check origin/main...HEAD`. Never run
windowed selectors; the lab re-captures on its own authority after merge.

## Owned Paths

`packages/render/src/button.rs`, `packages/render/src/presentation.rs`
(only if `rem_to_px` is the cause), their tests,
`test/visual/button-comparison/policy.ts` (one role finding, if contracted),
`docs/roadmaps/g16/parity-evidence-ledger.md` known-delta cell via its
generator inputs, execution log, `PAPERCUTS.md` (append only).

## Stop Conditions

Stop if the cause is in the node vocabulary and needs a new capability, or
if GPUI's paint path needs a change. Escalation owner: Chatterbox.
