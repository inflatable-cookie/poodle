# g16.089 — Nucleus DetailItem M1 Receipt

Status: preparation-ready
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Text receipt; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/detail-item.md`
Handoff: `../../handoffs/20260903-210000-g16-089-nucleus-detail-item-receipt.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus `DetailItem`. Pause before shared evidence, then finalize one terminal `M1` receipt against the latest cohort identity.

## Preparation Boundary

- Mount through production `node_compat::DetailItem` `IntoElement` and the element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove real Text composition, label/value/supporting/action structure, contract-owned layouts and states, exact tokens, inert and interactive paths where exposed, host rebuilds, geometry, and duplicate-instance identity.
- Drive mounted input only through contract-owned seams. Do not invent editing, persistence, application policy, or public API.
- Commit a biting counterexample before any bounded generalized repair.
- Do not edit manifest, receipts, ledger, g16 front doors, or claim M1 during preparation. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, commit runtime source, emit only after the terminal assertion, update this card and one log, and run full boards.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Text dependency is real | replace Text with raw node | metadata/layout fails |
| Structure/state is exact | collapse label/value/supporting/action | Node assertions fail |
| Input is mounted | call handler directly | mounted trace is absent |
| Token posture is exact | substitute nearby size/tone | exact metadata fails |
| Identity is caller-scoped | reuse one runtime id | callbacks/focus cross |
| Geometry is exact | overlap rows or escape mount | order/containment fails |
| Receipt is terminal | fail final rebuild/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Preparation: focused DetailItem contract/render/backend and named mounted tests plus `git diff --check`. Finalization adds native regressions, receipt/ledger tests, ledger check, Rust/native CI, docs check, and diff check. Never run windowed or native-visual selectors.

## Continuation

Pause after preparation. Shared receipt production and merge remain serial.
