# g16.089 — Nucleus DetailItem M1 Receipt

Status: complete
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Text receipt; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/detail-item.md`
Handoff: `../../handoffs/20260903-210000-g16-089-nucleus-detail-item-receipt.md`

## Goal

Produce the named production-path mounted proof and one terminal `M1` receipt
for Nucleus `DetailItem` at a committed runtime source.

## Completed

- Runtime source `91bcb4367068d31d9be5a6844baf2b2b7f51baa7` emits the
  terminal DetailItem receipt from the stable named mounted test.
- All 27 cohort receipts pin that exact runtime source. The generated Nucleus
  ledger advances only DetailItem from missing to mounted: 27/29 mounted. The
  full evidence ledger records 27 mounted and 148 missing GPUI behaviour cells.
- The result is M1 only. It does not infer A1 or V1.

## Fixed Boundary

- Mount through production `node_compat::DetailItem` `IntoElement` and the element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove real Text composition, label/value/supporting/action structure, contract-owned layouts and states, exact tokens, inert and interactive paths where exposed, host rebuilds, geometry, and duplicate-instance identity.
- Drive mounted input only through contract-owned seams. Do not invent editing, persistence, application policy, or public API.
- Commit a biting counterexample before any bounded generalized repair.
- Do not edit manifest, receipts, ledger, g16 front doors, or claim M1 during preparation. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, commit runtime source, emit only after the terminal assertion, update this card and one log, and run full boards.
- Preserve the full preparation series. Accepted head
  `03d2a6ccbe91fcca3f63ffb2f78dbc3f5fd6ca84` was rebased onto
  `1add36a7a4e9d77831b978b833e8ffe28c3a33dc`, which contains the merged
  CommandPalette 26/29 cohort and closeout. Range-diff maps all six commits
  exactly: `0003edf11` to `2e420b7f3`, `736e2cfb5` to `e5377e40e`,
  `faabf5aa0` to `9a3b9ed88`, `36c1ad16c` to `0abaf8f4b`, `614a6921e`
  to `aa186e124`, and `03d2a6ccb` to `532ad4df5`.
- The original identity sequence remains historical; the later caller-scoped
  counterexample and repair are the accepted proof. The density counterexample
  and repair remain a separate exact red/green pair.
- Shared evidence contains the complete 27-receipt cohort pinned to the runtime
  source. ToastHost remains planning-only. No g16 front-door changes belong to
  this card.

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

Focused DetailItem contract, render, adapter/backend, Jetstream mapping, and
named mounted checks passed after the rebase. Final validation ran `effigy
regressions:native`, receipt and ledger tests, `effigy
check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`, `effigy
docs:check`, and `git diff --check`. No windowed or native-visual selector ran.

## Continuation

Pause for terminal M1 re-review. Merge and g16 front-door closeout remain with
the orchestrator. Do not start another receipt card.
