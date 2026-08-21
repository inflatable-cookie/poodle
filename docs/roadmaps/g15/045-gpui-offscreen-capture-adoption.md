# g15.045 — GPUI Offscreen Capture Adoption

Status: **complete** — PR #62 merged as `4e420a7b`
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
- Keep GPUI's `test-support` feature on capture/test tooling only; do not pull
  it into the shipping preview binary. `gpui_platform` itself is the one new
  normal preview dependency required by upstream's
  `gpui_platform::application()` replacement.
- Use a bounded one-shot capture command for adoption. Process pooling and a
  long-running sidecar belong to the later lab, not this card.
- Do not add named component fixtures or comparison thresholds yet.

## Acceptance Envelope

- [x] A real Poodle primitive renders repeatedly to identical 2× offscreen
      pixels without opening or focusing a window.
- [x] Existing GPUI construction and interaction selectors remain green.
- [x] The new selector can run from an ordinary worktree without sibling
      repositories or desktop permissions.
- [x] The adopted graph, headless regressions, and Metal capture stay green on
      Rust 1.95; public `rust-version = "1.95"` claims do not drift.
- [x] Dependency and public-intent package changes are documented under spec
      022 before merge.

## Stop Conditions

- `g15.044` does not produce a `go` verdict with an exact pin.
- The upgrade introduces broad component behavior changes or an unbounded API
  migration.
- Headless output differs nondeterministically under fixed inputs.
- The adopted graph cannot preserve Poodle's declared Rust 1.95 floor. Stop
  for an operator decision rather than raising public package MSRV metadata.

## Pre-Dispatch Evidence

The orchestrator reran the complete retained `g15.044` verifier from a clean
detached checkout with `RUSTUP_TOOLCHAIN=1.95.0`. The production baseline,
17-error migration, migrated preview and tests, 56/56 headless regressions,
Metal capture, 10-capture hash, PNG dimensions, and viewport checks all passed.
Upstream's own 1.97.1 toolchain pin therefore does not require Poodle to raise
its declared Rust floor for this adoption.

## Continuation

The accepted proof and exact migration recipe live in
`../../research/gpui-offscreen-capture-feasibility.md` and
`../../logs/2026-08/assets/g15-044/reproduce.sh`. Preserve its bounded
8 + 6 + 3 mechanical migration shape; stop if adoption expands into renderer
or component redesign.
