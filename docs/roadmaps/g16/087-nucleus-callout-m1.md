# g16.087 — Nucleus Callout M1 Receipt

Status: complete
Type: Nucleus NP-4 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Icon receipt; serial finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`, `062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`, `parity-evidence-ledger.md`, `../../contracts/components/callout.md`
Handoff: `../../handoffs/20260903-194500-g16-087-nucleus-callout-receipt.md`

## Goal

Produce the named production-path mounted proof and one terminal `M1` receipt
for Nucleus `Callout` at a committed runtime source.

## Completed

- Runtime source `2d0e963c0b082c258213749a6d8d452339c7c01b` emits the
  terminal Callout receipt from the stable named mounted test.
- All 25 cohort receipts pin that exact runtime source. The generated Nucleus
  ledger advances only Callout from missing to mounted: 25/29 mounted. The
  full evidence ledger records 25 mounted and 150 missing GPUI behaviour cells.
- The result is M1 only. It does not infer A1 or V1.

## Fixed Boundary

- Mount through the production `node_compat::Callout` `IntoElement` path and element-backed `HeadlessDriver`; renderer-only construction is not evidence.
- Prove contract-owned tone, title/body/action/dismiss structure, production Icon composition, exact token metadata, controlled dismissal and refusal, disabled/inert paths, mounted input, geometry, and duplicate-instance isolation.
- Do not consolidate GPUI Banner/CallOut types, invent app policy, broaden public API, or claim exact pixels for allowed color-mix differences.
- Preserve both biting counterexample sequences before their bounded repairs.
  The accepted preparation head
  `b0fc557373d83dd9a1b6875f9c54c7f5cc670eed` was rebased in full onto
  `ca5b62ea97e951df3402d41a30cdbee0567b9b97`, which contains the
  ConfirmAction closeout at `019f4dd1d`. Range-diff maps `e87a85f81`
  exactly to `fe00d70e4` before repair `0a346a14b`, and maps `2e10a0048`
  exactly to `e9ccbf295` before repair `c6d751023`.
- Shared evidence contains the complete 25-receipt cohort pinned to the runtime
  source. No g16 front-door changes belong to this card.

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

Focused Callout contract, render, adapter, backend, and named mounted checks
passed after the rebase. Final validation ran `effigy regressions:native`,
receipt and ledger tests, `effigy check:parity-evidence-ledger`, `effigy
ci:rust`, `effigy ci:native`, `effigy docs:check`, and `git diff --check`. No
windowed or native-visual selector ran.

## Continuation

Pause for terminal M1 re-review. Merge and g16 front-door closeout remain with
the orchestrator. Do not start another receipt card.
