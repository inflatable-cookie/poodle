# g15.045 — GPUI Offscreen Capture Adoption

Status: **ready** — `g15.044` verdict `go` accepted with PR #61
Parent: `012-visual-conformance-lane.md`
Depends on: `g15.044` (complete), operator approval of the exact GPUI
dependency migration (satisfied by accepted PR #61 verdict)
Unblocks: `g15.046`
Governing refs: the capture-platform decision promoted by `g15.044`,
`../../contracts/001-working-rules.md`, `012-visual-conformance-lane.md`

## Goal

Adopt the proved offscreen GPUI raster seam in Poodle, replacing the release
lane's dependence on a desktop-capturing native harness while preserving the
existing headless construction and interaction evidence.

## Scope Envelope

- Pin `zed-industries/zed@1ea16c1ab9dd6d36649e002dc60995634da04daf`
  and migrate only the adapter,
  node backend, preview, and test surfaces required by the accepted proof.
- Add a deterministic headless capture command with explicit viewport, theme,
  and control-size inputs. Record scale as `2.0` and reject any other requested
  scale; the accepted revision is 2×-only and this card does not add a shim.
- Emit typed capture receipts beside PNG output. A missing or stale image must
  fail, never pass as an empty comparison.
- Keep the old windowed visual tool local-only until later cleanup; do not run
  it or route release/CI through it.
- Keep `test-support` and `gpui_platform` on capture/test tooling only; do not
  pull the feature into the shipping preview binary.
- Use a bounded one-shot capture command for adoption. Process pooling and a
  long-running sidecar belong to the later lab, not this card.
- Do not add named component fixtures or comparison thresholds yet.

## Acceptance Envelope

- [ ] A real Poodle primitive renders repeatedly to identical 2× offscreen
      pixels without opening or focusing a window.
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

The accepted proof and exact migration recipe live in
`../../research/gpui-offscreen-capture-feasibility.md` and
`../../logs/2026-08/assets/g15-044/reproduce.sh`. Preserve its bounded
8 + 6 + 3 mechanical migration shape; stop if adoption expands into renderer
or component redesign.
