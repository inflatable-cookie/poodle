# g16.091 — Nucleus ToastHost M1 Receipt

Status: complete
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

- Runtime source `740f3cb16632fc34c93f8492198fd968a348964f` emits the
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

Focused ToastHost contract/machine/render/backend and named mounted checks
passed after the rebase. Final validation ran the native regressions, receipt
and ledger tests, parity ledger check, Rust/native CI, docs check, and diff
check. No windowed or native-visual selector ran.

## Continuation

Pause for terminal M1 re-review. Merge and g16 front-door closeout remain with
the orchestrator. Do not start MessageCenter or another receipt card.
