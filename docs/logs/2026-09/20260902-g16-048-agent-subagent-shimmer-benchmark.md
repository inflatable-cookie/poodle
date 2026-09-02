# g16.048 — AgentSubagent Ownership And Shimmer Benchmark

Status: complete — static fallback; no candidate effect promoted
Date: 2026-09-02
PR: https://github.com/inflatable-cookie/poodle/pull/153
Card: `docs/roadmaps/g16/048-agent-subagent-ownership-and-shimmer-benchmark.md`
Handoff: `docs/handoffs/20260902-004203-g16-048-agent-subagent-benchmark.md`
Branch: `research/g16-048-agent-subagent-benchmark`
Base: `origin/main` at `c1a527898e7425853359bd72b7113a8cf38b8d97`

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
component or export changed.

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
| Chromium `20260902001252` | 8 | 2 | 8 | 0 | background-clip `source-readable` in 2 cells |
| Firefox `20260902001908` | 8 | 5 | 8 | 0 | `frame-p95` in 5 cells; background-clip `source-readable` in 2 |
| WebKit `20260902002022` | 8 | 8 | 8 | 0 | `frame-p95` in 8 cells; background-clip `source-readable` in 2 |

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
static. A combined multi-engine launch stalled at Firefox after the Chromium
receipt; the independent Firefox and WebKit receipts above are the evidence
used for this result.

## Immutable receipts

The receipt files retain the compact manifest and results digests. Raw DOM,
selection, accessibility, geometry, frame, trace, layer/memory, screenshots,
and lifecycle files remain outside canonical source through the verdict plus
90 days.

| Run | Receipt | Manifest SHA-256 | Results SHA-256 |
| --- | --- | --- | --- |
| Chromium `20260902001252` | `.artifacts/agent-subagent-shimmer/runs/20260902001252/receipt.json` | `5e33e325857dedcfcf4c1f842118be1893995fb813494ef72e25c47af303f984` | `aac1b9c52e705ed88e1b997f46410dee4ab9e6cf42d4e18d489820d2df56a0a3` |
| Firefox `20260902001908` | `.artifacts/agent-subagent-shimmer/runs/20260902001908/receipt.json` | `718d2c7ea6c4cab9b73185430b81f369d82f6f9479233d7625247e8d6edfebf6` | `c531353d90aeefea93db0b272312c86ccf4550ad8d21d932e28893b98fb16b51` |
| WebKit `20260902002022` | `.artifacts/agent-subagent-shimmer/runs/20260902002022/receipt.json` | `f35e100f3b8f415654088fa35213b4d00ce92904a2fea011e953eb09fdf8df17` | `96b4295949cbb4012eecb3085293dc47cb0d36ab02aca587bc5e9e9d798cda6a` |

## Validation

- Focused Svelte, React, shared-render, and headless-GPUI AgentSubagent checks:
  passed.
- Benchmark-local eligibility invariants: passed.
- Benchmark-local smoke runner: three independent engine receipts; 24 cells,
  24 lifecycle runs, 15 failed cells, mechanical `static-fallback`.
- `effigy docs:lint`: passed.
- `effigy docs:check`: passed; generated reports/build completed with the
  repository's existing warnings.
- `git diff --check`: passed before final documentation changes.

No parity-ledger, roadmap front-door, package, workflow, release, or
production-runtime surface changed.
