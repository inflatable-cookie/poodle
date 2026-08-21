# g15.045 — GPUI Offscreen Capture Adoption

Status: **planned — blocked on a `go` verdict from `g15.044`**
Parent: `012-visual-conformance-lane.md`
Depends on: `g15.044`, operator approval of any GPUI dependency migration
Unblocks: `g15.046`
Governing refs: the capture-platform decision promoted by `g15.044`,
`../../contracts/001-working-rules.md`, `012-visual-conformance-lane.md`

## Goal

Adopt the proved offscreen GPUI raster seam in Poodle, replacing the release
lane's dependence on a desktop-capturing native harness while preserving the
existing headless construction and interaction evidence.

## Scope Envelope

- Pin the exact reviewed GPUI version/revision and migrate only the adapter,
  node backend, preview, and test surfaces required by the accepted proof.
- Add a deterministic headless capture command with explicit viewport, scale,
  theme, and control-size inputs.
- Emit typed capture receipts beside PNG output. A missing or stale image must
  fail, never pass as an empty comparison.
- Keep the old windowed visual tool local-only until later cleanup; do not run
  it or route release/CI through it.
- Do not add named component fixtures or comparison thresholds yet.

## Acceptance Envelope

- [ ] A real Poodle primitive renders repeatedly to identical offscreen pixels
      without opening or focusing a window.
- [ ] Existing GPUI construction and interaction selectors remain green.
- [ ] The new selector can run from an ordinary worktree without sibling
      repositories or desktop permissions.
- [ ] Dependency and public-intent package changes are documented under spec
      022 before merge.

## Stop Conditions

- `g15.044` does not produce a `go` verdict with an exact pin.
- The upgrade introduces broad component behavior changes or an unbounded API
  migration.
- Headless output differs nondeterministically under fixed inputs.

## Continuation

The orchestrator makes this card ready only after `g15.044` records the exact
pin, migration inventory, retained selector shape, and operator decision.
