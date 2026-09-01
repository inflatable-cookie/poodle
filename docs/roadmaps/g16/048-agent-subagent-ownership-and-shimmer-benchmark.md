# g16.048 — AgentSubagent Ownership And Shimmer Benchmark

Status: ready — benchmark is serial behind the contract/runtime reconciliation
inside this card
Type: benchmark
Opened: 2026-09-01
Depends on: merged `g16.034` and operator acceptance recorded in
`../../handoffs/20260901-234025-post-triage-canonical-runway.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/agent-subagent.md`,
`../../contracts/components/agent-transcript.md`,
`../../architecture/012-semantic-motion-policy.md`

## Goal

First reconcile AgentSubagent's stale draft/Svelte-only contract with its live
Svelte, React, shared Rust, and GPUI implementation. Then run one disposable
web benchmark for a finite running-activity-line sweep. Produce a threshold
verdict only; do not ship shimmer or add a generic effect API.

## Phase Gate

The benchmark cannot start until the contract records only current shipped
truth: active runtime ownership, transcript live-region ownership, the
host-supplied activity line, current static rendering in every active runtime,
and Jetstream's deferred posture. The reconciliation must not add a shimmer
motion role, lifecycle, web-only effect promise, mask/fallback law, or future
implementation surface.

## Benchmark Candidate Lifecycle

The following lifecycle belongs only to the disposable benchmark card and its
execution log:

- one non-looping 2.0-second sweep after the first committed frame of an
  eligible `running` epoch with non-empty `activityLine` under `full` policy;
- same-epoch line replacement updates semantic text without restarting or
  queueing; leaving and later re-entering `running` creates one new epoch;
- every loss of eligibility, reduced/frozen policy, forced colors, print,
  unsupported path, inactivity/offscreen state, or unmount cancels immediately
  and leaves ordinary readable static text;
- AgentTranscript remains the sole live-region owner. Selection/copy and the
  accessibility tree expose the source string exactly once;
- native output stays static. No public pause/stop/hide prop, status mutation,
  `TextShimmer`, arbitrary Text/AgentMessage effect, or GPU claim is allowed.

## Benchmark

Compare ordinary static text, the current background-position repaint
baseline, a clean-room mask-plus-transform candidate, and a complete
background-clip alternative only where supported. Use the packet's fixed
content cases, N=1/10/50/100 scale, 320/640 widths, DPR and theme axes, pinned
Blink/Gecko/WebKit builds, three repetitions, static pre/post-roll, and one
finite sweep.

Retain an immutable manifest, digests, per-cell summaries, lifecycle events,
geometry, paint, layer/memory, frame, selection/copy, DOM/accessibility, and
fallback results. Raw sanitized evidence stays outside canonical source through
the verdict plus 90 days. The compact manifest, thresholds, and verdict go in
one execution log.

## Acceptance

- The contract no longer says draft or defers live React/GPUI paths. It records
  current static output and keeps Jetstream program-deferred without promising
  the benchmark candidate.
- Every candidate preserves zero effect-caused geometry change, one semantic
  text value, exact selection/copy, focus order, status ownership, and static
  fallbacks.
- One epoch produces exactly one sweep. Replacement, cancellation, teardown,
  and re-entry follow the fixed lifecycle with no retained handle.
- Performance, layer, memory, contrast, alignment, and frame results are
  checked against every hard budget in the accepted packet. No aggregate hides
  a failing engine/device/content/count cell.
- Promotion requires the packet's conjunction, including the measured win over
  background-position or zero steady-state paint. Failure returns the static
  result; it does not relax thresholds.
- The diff adds no production effect, package export, permanent benchmark
  selector/corpus, public recipe, native mask, release, or ledger claim.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| AgentSubagent owns the line | benchmark generic Text | only real AgentSubagent activity-line hosts are present |
| One epoch means one sweep | replace line twice while running | semantic text updates; sweep count stays one |
| Cancellation is terminal | full → frozen or unmount mid-sweep | zero remaining handles and readable source text |
| Re-entry is fresh | running → waiting → running | old epoch stays dead; new epoch gets one sweep |
| Source text remains singular | mask candidate duplicates paint | copy and accessibility expose source once; duplicate is inert/hidden |
| Claims match traces | property support but recurring paint | candidate stays static/paint-bound; no GPU-accelerated claim |
| Contract remains shipped truth | benchmark candidate fails every threshold | component contract still promises only current static behavior |

## Writable Scope

AgentSubagent contract; a disposable ignored benchmark harness and artifacts;
one compact execution log; this card; new papercuts. Contract reconciliation
may cite live component/spec/render/specimen evidence but must not edit
production runtime code or add candidate shimmer semantics to the contract. Do
not edit generic Text, AgentMessage, Skeleton,
Spinner, motion policy, packages, releases, workflows, native visual routes,
or Jetstream behavior.

## Validation

Run existing focused AgentSubagent Svelte, React, Rust render, and headless GPUI
checks; benchmark-local deterministic checks; `effigy docs:lint`,
`effigy docs:check`, and `git diff --check origin/main...HEAD`. Browser traces
are web-only and non-windowed. Never run native visual or `*-windowed`
selectors.

## Stop Conditions

Stop if the contract/runtime facts disagree materially, the benchmark needs a
public or permanent effect surface, text becomes transparent/unselectable,
any hard budget fails, a complete engine path cannot be identified, or native
mask support becomes necessary. Stop if candidate lifecycle or visual-effect
language enters the component contract during this card.

## Continuation

A passing verdict permits a separate bounded AgentSubagent web implementation
card. That later card amends the contract before shipping the passing effect.
A failed or inconclusive verdict closes on static text with no contract
rollback because this card never put the candidate in the contract.
