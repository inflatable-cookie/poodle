# g16.093 — Nucleus MessageCenter M1 Receipt

Status: complete
Type: Nucleus NP-5 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-04
Depends on: completed `g16.062`; merged ToastHost `g16.091`; merged `g16.092`
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/message-center.md`
Handoff: `../../handoffs/20260903-223100-g16-093-nucleus-message-center-m1.md`

## Goal

Produce the named production-path mounted proof and one terminal `M1` receipt
for Nucleus `MessageCenter` at a committed runtime source.

## Completed

- Runtime source `f0774a7d15a195cc6b8506c5da68db99807e5376` emits the
  terminal MessageCenter receipt from the stable named mounted test.
- All 29 cohort receipts pin that exact runtime source and the g16.092 preview
  lock digest `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`.
  The generated Nucleus ledger advances only MessageCenter from missing to
  mounted: 29/29 mounted. The full evidence ledger records 29 mounted and 146
  missing GPUI behaviour cells.
- The result is M1 only. It does not infer A1 or V1.

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
- Drive mounted pointer/keyboard/wheel input. Do not call handlers directly,
  invent queue/persistence/job policy, join toast and archive streams, add
  public API, or claim A1/V1.
- `MessageCenter` owns no timeout clock. Timer isolation is not a mounted
  axis; native render does not advance, estimate, or dismiss archive rows.
- Commit a biting counterexample before any bounded generalized repair.
- During preparation do not edit the manifest, receipts, generated ledger, or
  g16 front doors. Push a draft PR and pause.
- On resume, rebase onto the latest receipt/dependency merge, emit only after
  the terminal assertion, update this card and one log, then run full boards.
- Preserve the accepted preparation series. Accepted head `79d463115` was
  rebased onto `3dc0c7361`, which contains merged `g16.092` at `17534f484`.
  Range-diff maps all four preparation commits exactly: `8e8b3618c` to
  `5dd84d2b5`, `7b323cedf` to `1c64b034e`, `b4f95ce54` to `6cf53b4f4`, and
  `79d463115` to `b6330aa50`. Runtime `f0774a7d1` adds post-scroll containment
  re-fetch and terminal receipt emission.
- Shared evidence contains the complete 29-receipt cohort pinned to the runtime
  source. No g16 front-door changes belong to this card.

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
| Evidence identity is exact | emit before predecessor merge | cohort validation fails |
| Levels stay separate | claim A1/V1 | schema or claim review fails |

## Validation

Focused MessageCenter contract/render/adapter and named mounted checks passed
after the rebase. Final validation ran `effigy regressions:native`, receipt and
ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, `effigy docs:check`, and `git diff --check`. No windowed
or native-visual selector ran.

## Continuation

Merged in PR #198 as `06de812f7037eeca204d89c72fb4c586723600eb`.
The 29/29 mounted cohort is complete. Do not infer A1, V1, lab V2, Nucleus M2,
or adoption; those remain separately authorized programme phases.
