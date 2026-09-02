# g16.048 — AgentSubagent Ownership And Shimmer Benchmark

Status: complete — static fallback; no candidate effect promoted
Date: 2026-09-02
PR: https://github.com/inflatable-cookie/poodle/pull/153
Card: `docs/roadmaps/g16/048-agent-subagent-ownership-and-shimmer-benchmark.md`
Handoff: `docs/handoffs/20260902-004203-g16-048-agent-subagent-benchmark.md`
Branch: `research/g16-048-agent-subagent-benchmark`
Base: `origin/main` at `f645091f17a10b0ab8af82c200d5c21107cabb47`
Rebased before repair onto the current `main` head.

## Contract gate

`docs/contracts/components/agent-subagent.md` now records the shipped static
truth across Svelte, React, shared Rust composition, and GPUI preview. It names
AgentTranscript as the web/native transcript live-region owner, the activity
line as host-supplied, and Jetstream as program-deferred. It records current
web/native handler and item-input deltas without adding candidate effect
semantics, a public control, a mask, or a fallback promise.

Focused contract/runtime checks passed before the benchmark started:

- Svelte AgentSubagent: 6 tests passed.
- React AgentSubagent: 4 tests passed.
- Core AgentSubagent conformance: 24 tests passed.
- Shared render: 1 filtered test passed.
- Headless GPUI AgentSubagent regression: 1 filtered test passed.
- `effigy docs:lint`: passed.
- `git diff --check`: passed.

## Benchmark shape

The harness is disposable and ignored at `.artifacts/agent-subagent-shimmer/`.
It mounts the real Svelte and React `AgentSubagent` components, with the
transcript's single `role="log"` / `aria-live="polite"` owner. The four
benchmark-only candidates are static, background-position, mask-transform, and
background-clip. The mask treatment was authored in the harness; no production
component or export changed. Every artifact path is phase-qualified under
`raw/baseline/`, `raw/observed/`, or `raw/lifecycle/`, and every receipt runs
the integrity gate before writing `receipt.json`.

The serial smoke gate used `C1/N1/320/Eclipse/desktop/DPR1`, one repetition,
the 0.5 s static pre-roll, one 2.0 s sweep, and the 0.5 s post-roll for both
web runtimes. Each engine also ran the lifecycle receipt across all four
candidates. The browser builds reported by the runner were Chromium
`151.0.7922.34`, Firefox `153.0`, and WebKit `26.5`. No native visual or
windowed selector ran.

The packet thresholds recorded as benchmark inputs were: zero effect geometry
change within a 0.25 CSS-pixel measurement tolerance; zero recurring layout;
zero recurring main-thread paint for a compositor-oriented claim; N=100
style+layout+paint p95 no more than static +2 ms and no more than 4 ms absolute;
static layer count + N + 4 maximum; texture-memory caps of 8/16 MiB at DPR1
and 16/32 MiB at DPR2 for N=50/100; frame p95/p99 of 16.7/33.4 ms and dropped
frames of 1% desktop / 2% low-power; alignment within 0.5 CSS px; text
contrast of 4.5:1 normal / 3:1 large; and exact one-value selection, copy, and
accessibility output. Unavailable texture memory makes no GPU or memory claim.

## Results

| Engine receipt | Cells | Failed cells | Lifecycle runs | Failed lifecycle | Hard failure |
| --- | ---: | ---: | ---: | ---: | --- |
| Chromium `20260902004348` | 8 | 2 | 8 | 0 | background-clip `source-readable` in 2 cells |
| Firefox `20260902004506` | 8 | 5 | 8 | 0 | `frame-p95` in 5 cells; background-clip `source-readable` in 2 |
| WebKit `20260902004622` | 8 | 8 | 8 | 0 | `frame-p95` in 8 cells; background-clip `source-readable` in 2 |

All three receipts passed integrity with 144 artifact refs, 144 unique paths,
zero missing paths, zero hash mismatches, and zero conflicting path digests.

Across the 24 smoke cells, geometry, source-text count, exact selection,
transcript live-region ownership, focusability, and frame-p99 gates did not
produce a failure. The six background-clip failures made the source text
transparent (`rgba(0, 0, 0, 0)` / transparent text fill), so that candidate is
rejected. The Firefox and WebKit frame-p95 failures also keep their candidate
paths on static fallback. All 24 lifecycle receipts passed same-epoch
replacement, cancellation, re-entry, reduced/frozen, offscreen, unsupported,
and unmount checks.

The mechanical verdict is `static-fallback`. The full review matrix was not
continued after these hard smoke failures, so this log makes no N=10/50/100,
low-power, DPR2, contrast-matrix, or promotion claim. Native output remains
static. The Chromium, Firefox, and WebKit receipts above were produced by
separate isolated engine runs.

## Immutable receipts

The receipt files retain the compact manifest and results digests. Raw DOM,
selection, accessibility, geometry, frame, trace, layer/memory, screenshots,
and lifecycle files remain outside canonical source through the verdict plus
90 days.

| Run | Receipt | Manifest SHA-256 | Results SHA-256 |
| --- | --- | --- | --- |
| Chromium `20260902004348` | `.artifacts/agent-subagent-shimmer/runs/20260902004348/receipt.json` | `1959fe0922a1787bf2dd7d500bffb811541de7ea114abf9f3cc60dff36795f98` | `a38541959ecd0921adb5cf2d2625b5205bc37e09c9e1ae3796c79f82657174a4` |
| Firefox `20260902004506` | `.artifacts/agent-subagent-shimmer/runs/20260902004506/receipt.json` | `f2adc475ddf06b69e5a5050c3ffcf30c172f1d883a50366137c08e1689e4efd4` | `b68f5dd8f86cc5cb5607f23a40c17c732dbd397d68d6e0e6aa530b8591299ef3` |
| WebKit `20260902004622` | `.artifacts/agent-subagent-shimmer/runs/20260902004622/receipt.json` | `2e4c668a5998a5b188069c94d01e30c1cf119d71239f1c0a5f00f20e54913db8` | `1bbe97aa4e1768eec52a097a71a1daeefa84f3567bba3bf1a253d57a6baa25c4` |

## Validation

- Focused Svelte, React, shared-render, and headless-GPUI AgentSubagent checks:
  passed.
- Benchmark-local eligibility invariants: passed.
- Benchmark-local repaired smoke runner: three independent engine receipts; 24
  cells, 24 lifecycle runs, 15 failed cells, mechanical `static-fallback`, and
  all three receipt-integrity gates passed.
- `effigy docs:lint`: passed.
- `effigy docs:check`: passed; generated reports/build completed with the
  repository's existing warnings.
- `git diff --check`: passed before final documentation changes.

No parity-ledger, roadmap front-door, package, workflow, release, or
production-runtime surface changed.
