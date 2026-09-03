# g16.090 — Nucleus CommandPalette M1 Receipt

Status: preparation-ready
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Dialog and TextInput receipts; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/command-palette.md`
Handoff: `../../handoffs/20260903-210100-g16-090-nucleus-command-palette-receipt.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus `CommandPalette`. Pause before shared evidence, then finalize one terminal `M1` receipt against the latest cohort identity.

## Preparation Boundary

- Mount through production `node_compat::CommandPalette` `IntoElement` and the element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove real Dialog/TextInput composition, query/results/empty/loading structure, filtering and roving selection, disabled-item inertia, activation/dismissal axes, controlled host rebuilds and refusal, exact tokens, geometry, and duplicate-instance identity.
- Drive mounted pointer and keyboard input. Do not call handlers directly, invent command routing, persistence, application policy, or claim A1 focus trapping.
- Commit a biting counterexample before any bounded generalized repair.
- Do not edit manifest, receipts, ledger, g16 front doors, or claim M1 during preparation. Push a draft PR and pause.
- On resume, rebase onto the latest receipt merge, set the expected test, commit runtime source, emit only after the terminal assertion, update this card and one log, and run full boards.

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

Preparation: focused CommandPalette contract/machine/render/backend and named mounted tests plus `git diff --check`. Finalization adds native regressions, receipt/ledger tests, ledger check, Rust/native CI, docs check, and diff check. Never run windowed or native-visual selectors.

## Continuation

Pause after preparation. Shared receipt production and merge remain serial.
