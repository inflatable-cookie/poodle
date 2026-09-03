# g16.087 — Nucleus Callout M1 Receipt

Status: preparation-ready
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Icon receipt; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/callout.md`
Handoff: `../../handoffs/20260903-194500-g16-087-nucleus-callout-receipt.md`

## Goal

Prepare the named production-path mounted proof for Nucleus `Callout`. Pause before shared evidence. After the orchestrator supplies the latest cohort identity, finalize one terminal `M1` receipt.

## Preparation Boundary

- Mount through the production `node_compat::Callout` `IntoElement` path and element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove contract-owned tone, title/body/action/dismiss structure, production Icon composition, exact token metadata, controlled dismissal and refusal, disabled/inert paths, mounted input, geometry, and duplicate-instance isolation.
- Do not consolidate GPUI Banner/CallOut types, invent app policy, broaden public API, or claim exact pixels for allowed color-mix differences.
- Commit a biting counterexample before any bounded generalized repair.
- During preparation, do not edit manifest, receipts, ledger, g16 front doors, or claim M1 completion. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, commit runtime source, emit the cohort only after the terminal assertion, update this card and one log, and run full boards.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Icon dependency is real | replace Icon with raw node | metadata or layout fails |
| Controlled dismissal is real | unmount without host rebuild | refusal proof fails |
| Input is mounted | invoke dismiss handler directly | mounted trace is absent |
| Tone/token posture is exact | substitute adjacent tone | exact metadata fails |
| Identity is caller-scoped | reuse one runtime id | callbacks/focus cross |
| Geometry is exact | overlap content or escape mount | order/containment fails |
| Receipt is terminal | fail final refusal/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Preparation: focused Callout contract/render/backend and named mounted tests plus `git diff --check`. Finalization adds `effigy regressions:native`, receipt/ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`, and `effigy docs:check`. Never run windowed or native-visual selectors.

## Continuation

Pause after preparation. Shared receipt production and merge remain serial.
