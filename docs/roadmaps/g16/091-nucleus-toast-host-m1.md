# g16.091 — Nucleus ToastHost M1 Receipt

Status: preparation-ready
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Icon receipt; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/toast-host.md`
Handoff: `../../handoffs/20260903-221500-g16-091-nucleus-toast-host-receipt.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus `ToastHost`. Pause before shared evidence, then finalize one terminal `M1` receipt against the latest cohort identity.

## Preparation Boundary

- Mount through production `node_compat::ToastHost` `IntoElement` and the element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove real Icon and contract-owned toast composition, ordering/placement, variants and tokens, controlled add/update/remove posture, dismiss/action input, disabled/inert paths, timeout policy only where the contract and headless clock own it, geometry, and duplicate-host identity.
- Drive mounted pointer/keyboard and deterministic time input. Do not call handlers directly, invent application queue policy, add MessageCenter behavior, or broaden public API.
- Commit a biting counterexample before any bounded generalized repair.
- Do not edit manifest, receipts, ledger, g16 front doors, or claim M1 during preparation. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, set the expected test, commit runtime source, emit only after the terminal assertion, update this card and one log, and run full boards.

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

Preparation: focused ToastHost contract/machine/render/backend and named mounted tests plus `git diff --check`. Finalization adds native regressions, receipt/ledger tests, ledger check, Rust/native CI, docs check, and diff check. Never run windowed or native-visual selectors.

## Continuation

Pause after preparation. `MessageCenter` remains gated on the accepted ToastHost boundary. Shared receipt production and merge remain serial.
