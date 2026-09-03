# g16.093 — Nucleus MessageCenter M1 Receipt

Status: preparation-ready
Type: Nucleus NP-5 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`; merged ToastHost `g16.091`; serial finalization follows `g16.092`
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/message-center.md`
Handoff: `../../handoffs/20260903-223100-g16-093-nucleus-message-center-m1.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus
`MessageCenter`. Pause before shared evidence, then finalize the twenty-ninth
terminal `M1` receipt against the latest merged cohort identity.

## Fixed Boundary

- Mount through production `node_compat::MessageCenter` `IntoElement` and the
  element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove production `IconButton`, `Popover`, `Button`, `Icon`, `TimeAgo`,
  `EmptyState`, `Progress`, and `StatusIndicator` composition where present.
- Cover derived unread count and trigger name, controlled open/refusal and
  focus restoration, exact labelled-dialog/list/item semantics, empty state,
  bounded scrolling, determinate/indeterminate progress, row selection,
  read-next-state, removal, mark-all-read, disabled/non-interactive paths,
  exact tokens, mounted geometry, rebuilds, and duplicate-instance identity.
- Drive mounted pointer/keyboard input. Do not call handlers directly, invent
  queue/persistence/job policy, join toast and archive streams, add public API,
  or claim A1/V1.
- Commit a biting counterexample before any bounded generalized repair.
- During preparation do not edit the manifest, receipts, generated ledger, or
  g16 front doors. Push a draft PR and pause.
- On resume, rebase onto the latest receipt/dependency merge, emit only after
  the terminal assertion, update this card and one log, then run full boards.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle fails |
| Dependencies are real | replace a composed primitive with a raw node | exact metadata/input fails |
| Host ownership is real | mutate read/open/items without rebuild | mounted state remains stale |
| Axes stay separate | selecting implicitly reads or removes | exact callback/state trace fails |
| Live rows remain inert | let progress row select/remove/read | callback trace fails |
| Progress is projection | advance value internally | rebuilt host value diverges |
| Geometry is exact | rows overlap or list escapes surface | positive order/containment fails |
| Identity is caller-scoped | reuse instance/item runtime ids | focus/callbacks cross |
| Receipt is terminal | fail final teardown/isolation assertion | no receipt emits |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Preparation: focused MessageCenter contract/render/adapter/backend and named
mounted checks plus `git diff --check`. Finalization adds native regressions,
receipt/ledger tests and check, Rust/native CI, docs check, and diff check.
Never run windowed or native-visual selectors.

## Continuation

Pause after preparation. Shared evidence and finalization wait for merged
`g16.092`. Merge and 29/29 programme closeout remain orchestrator-owned.
