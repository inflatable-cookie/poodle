# g16.090 — Nucleus CommandPalette M1 Receipt

Status: complete
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Dialog and TextInput receipts; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/command-palette.md`
Handoff: `../../handoffs/20260903-210100-g16-090-nucleus-command-palette-receipt.md`

## Goal

Produce the named production-path mounted proof and one terminal `M1` receipt
for Nucleus `CommandPalette` at a committed runtime source.

## Completed

- Runtime source `cd2f2e13888dba5dcd507cb65e7f63a03e840f03` emits the
  terminal CommandPalette receipt from the stable named mounted test.
- All 26 cohort receipts pin that exact runtime source. The generated Nucleus
  ledger advances only CommandPalette from missing to mounted: 26/29 mounted.
  The full evidence ledger records 26 mounted and 149 missing GPUI behaviour
  cells.
- The result is M1 only. It does not infer A1 or V1.

## Fixed Boundary

- Mount through production `node_compat::CommandPalette` `IntoElement` and the element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove real Dialog/TextInput composition, query/results/empty/loading structure, filtering and roving selection, disabled-item inertia, activation/dismissal axes, controlled host rebuilds and refusal, exact tokens, geometry, and duplicate-instance identity.
- Drive mounted pointer and keyboard input. Do not call handlers directly, invent command routing, persistence, application policy, or claim A1 focus trapping.
- Commit a biting counterexample before any bounded generalized repair.
- Do not edit manifest, receipts, ledger, g16 front doors, or claim M1 during preparation. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, set the expected test, commit runtime source, emit only after the terminal assertion, update this card and one log, and run full boards.
- Preserve both biting counterexample sequences before their bounded repairs.
  The accepted preparation head
  `4ef7b81d890fe3f142153f8c6a215067f74fcf3f` was rebased in full onto
  `4a8dd5018da6feb30f5905a5da195cda03640fc5`, which contains the Callout
  closeout at `52409b89c`. Range-diff maps `39ca354d5` exactly to
  `9c73d897a` before repair `40f1754bb`, now `fe1bd8278`, and maps
  `586801be9` exactly to `52566cbda` before repair `4ef7b81d8`, now
  `e0a17acf2`. The first repair differs only where upstream context relocated
  removal of its temporary red-driver line; its final tree remains intact.
- Shared evidence contains the complete 26-receipt cohort pinned to the runtime
  source. No g16 front-door changes belong to this card.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Dialog/TextInput dependencies are real | replace composition with raw nodes | metadata/input fails |
| Controlled ownership is real | paint query/selection without rebuild | mounted state stays stale |
| Navigation is exact | land on disabled item or lose wrap | focus/activation trace fails |
| Dismissal axes stay separate | couple Escape and activation | exact trace fails |
| Input is mounted | call handler directly | mounted trace is absent |
| Identity is caller-scoped | reuse one runtime id | focus/callbacks cross |
| Geometry is exact | overlap results or escape mount | order/containment fails |
| Receipt is terminal | fail final refusal/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Focused CommandPalette contract, machine, render, adapter/backend, and named
mounted checks passed after the rebase. Final validation ran `effigy
regressions:native`, receipt and ledger tests, `effigy
check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`, `effigy
docs:check`, and `git diff --check`. No windowed or native-visual selector ran.

## Continuation

Pause for terminal M1 re-review. Merge and g16 front-door closeout remain with
the orchestrator. Do not start another receipt card.
