# g16.088 — Nucleus ConfirmAction M1 Receipt

Status: preparation-ready
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Dialog and Button receipts; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/confirm-action.md`
Handoff: `../../handoffs/20260903-194600-g16-088-nucleus-confirm-action-receipt.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus `ConfirmAction`. Pause before shared evidence. After the orchestrator supplies the latest cohort identity, finalize one terminal `M1` receipt.

## Preparation Boundary

- Mount through the production `node_compat::ConfirmAction` `IntoElement` path and element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove real Dialog/Button composition, title/body/action semantics, destructive and ordinary posture, confirm/cancel/dismiss axes, disabled/pending inertia, host-owned acceptance/refusal rebuilds, exact callback order, token metadata, geometry, focus identity, and duplicate-instance isolation within M1 scope.
- Drive mounted pointer and keyboard input. Do not call handlers directly, claim A1 focus trapping/restoration, or invent application approval policy.
- Commit a biting counterexample before any bounded generalized repair.
- During preparation, do not edit manifest, receipts, ledger, g16 front doors, or claim M1 completion. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, set the expected test, commit runtime source, emit the cohort after the terminal assertion, update this card and one log, and run full boards.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Dialog/Button dependencies are real | replace composition with raw nodes | metadata or input fails |
| Dismissal axes stay separate | couple cancel, backdrop and Escape | exact trace fails |
| Controlled ownership is real | close without accepted host rebuild | refusal proof fails |
| Input is mounted | call handler directly | mounted trace is absent |
| Disabled/pending paths are inert | let confirm commit | callback/state trace fails |
| Identity is caller-scoped | reuse one runtime id | focus/callbacks cross |
| Geometry is exact | overlap actions or escape mount | order/containment fails |
| Receipt is terminal | fail final refusal/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Preparation: focused ConfirmAction contract/render/backend and named mounted tests plus `git diff --check`. Finalization adds `effigy regressions:native`, receipt/ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`, and `effigy docs:check`. Never run windowed or native-visual selectors.

## Continuation

Pause after preparation. Shared receipt production and merge remain serial.
