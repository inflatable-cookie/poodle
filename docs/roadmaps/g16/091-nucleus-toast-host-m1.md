# g16.091 — Nucleus ToastHost M1 Receipt

Status: complete — merged in PR #197 as `4a615e99046fa9e6dc14801ef1e6f60760336fc2`; card-required native gate has an inherited dependency failure recorded below
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Icon receipt; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/toast-host.md`
Handoff: `../../handoffs/20260903-221500-g16-091-nucleus-toast-host-receipt.md`

## Goal

Produce the named production-path mounted proof and one terminal `M1` receipt
for Nucleus `ToastHost` at a committed runtime source.

## Completed

- Runtime source `0f04083c9dee61d01722104e7403368559f0b590` emits the
  terminal ToastHost receipt from the stable named mounted test.
- All 28 cohort receipts pin that exact runtime source. The generated Nucleus
  ledger advances only ToastHost from missing to mounted: 28/29 mounted. The
  full evidence ledger records 28 mounted and 147 missing GPUI behaviour cells.
- The result is M1 only. It does not infer A1 or V1.

## Fixed Boundary

- Mount through production `node_compat::ToastHost` `IntoElement` and the element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove real Icon and contract-owned toast composition, ordering/placement, variants and tokens, controlled add/update/remove posture, dismiss/action input, disabled/inert paths, timeout policy only where the contract and headless clock own it, geometry, and duplicate-host identity.
- Drive mounted pointer/keyboard and deterministic time input. Do not call handlers directly, invent application queue policy, add MessageCenter behavior, or broaden public API.
- Commit a biting counterexample before any bounded generalized repair.
- Do not edit manifest, receipts, ledger, g16 front doors, or claim M1 during preparation. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, set the expected test, commit runtime source, emit only after the terminal assertion, update this card and one log, and run full boards.
- Preserve the full preparation series. Accepted head
  `5b9d9fa1f8dc5cdf9010c35a0e773986720acbb8` was rebased onto
  `420b9a7b1b6ab40f32f3936b5bbc2483a180b0ae`, which contains the merged
  DetailItem 27/29 cohort and closeout. Range-diff maps all seven commits
  exactly: `7690aeb40` to `0e9fed316`, `9b493726c` to `e7a3cd43a`,
  `1118a6d89` to `b67e619a4`, `636778932` to `4784e0101`, `8d27aeb86`
  to `e2ccd643d`, `17fa8b582` to `14b4df3d1`, and `5b9d9fa1f` to
  `b2ffd421d`.
- The identity counterexample remains the genuine test-only red commit. The
  placement, token, and teardown counterexamples and their bounded repairs
  remain separate accepted history.
- Cross-window focus-sweep counterexample `f089919b8` hangs before the repair
  and is followed by bounded repair `0f04083c9`. Focus paint ownership and
  lost-host sweeping are keyed by `AnyWindowHandle`, so one ending frame cannot
  discard another live window's focus handles.
- Shared evidence contains the complete 28-receipt cohort pinned to the runtime
  source. MessageCenter remains planning-only. No g16 front-door changes belong
  to this card.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Icon/toast dependencies are real | replace composition with raw nodes | metadata/input fails |
| Controlled queue ownership is real | mutate painted queue without host rebuild | mounted state diverges |
| Ordering and placement are exact | reorder or overlap toasts | geometry/trace fails |
| Dismiss/action axes stay separate | couple callbacks | exact trace fails |
| Disabled/inert paths are exact | let unavailable action commit | callback/state fails |
| Time policy is deterministic | allow stale timeout to remove replacement | generation proof fails |
| Identity is caller-scoped | reuse host/item ids | callbacks/timers cross |
| Receipt is terminal | fail final teardown/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Focused ToastHost contract/machine/render/backend, focus, and named mounted
checks passed after the repair. `effigy regressions:native` completed with
202/202 tests and emitted all 28 receipts at the exact runtime source.
`effigy ci:rust` passed. The card-required `effigy ci:native` completed its
drift, build, adapter, 202-test regression, and 9-test specimen stages, then
failed in the fresh downstream consumer because crates.io `tinyvec 1.13.0`
does not compile its alloc-only path (`cannot find macro vec`). The failure is
outside this card and is not reported as a pass. Receipt/ledger checks, docs
check, and diff check are recorded in the execution log. No windowed or
native-visual selector ran.

## Continuation

Proceed with `g16.092` to repair the inherited fresh-consumer dependency gate
and prepare `g16.093` MessageCenter in parallel. MessageCenter shared evidence
and finalization remain serial after the dependency repair merges.
