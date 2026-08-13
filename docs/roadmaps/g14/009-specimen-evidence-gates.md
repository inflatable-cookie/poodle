# g14.009 Specimen Evidence Gates

Status: planned
Owner: Poodle core
Depends on: `g14.002` (evidence state), `g14.003` (shared fixtures),
`g14.008` (specimens exist)
Governing refs: `../g13/pilot-verdict-evidence.md` §1.4,
`packages/gpui/preview/baselines/`, `test/native-visual/capture.ts`

## Objective

Convert the manual corpus sweeps into a permanent red/green. The capture,
OCR, and pixel-diff machinery exists and was proven in b036/b042. The
difference between a sweep and a gate is that a gate re-runs. Seven or
eight sweeps found the same class of hole repeatedly; a gate finds it once
and stays red until fixed.

## Deliverables

- Semantic assertions per specimen per runtime: OCR text, data-attribute
  state, layout facts. Not pixel-diff across renderers.
- Assertions wired into CI as a standing gate, failing with the
  component's name.
- Harness papercuts fixed first (all PAPERCUTS 2026-08-13): Jetstream
  `snap` overwrite, the inert `--control-size`/`--size` flag, stale GPUI
  baselines regenerated, Jetstream snap viewport or per-section capture.
- Pilot on one family before corpus-wide rollout; the cost per component
  is measured and reported.

## Acceptance

- [ ] A planted specimen regression fails the gate naming the component
  and runtime.
- [ ] Every component with a specimen has live evidence in CI, or a
  recorded reason why not.
- [ ] Per-component maintenance cost measured; if it dwarfs the catch, say
  so in the reassessment instead of hiding it.

## Next

`g14.010` reassesses the whole pinning stack.
