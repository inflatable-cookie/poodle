# g16.079 — Nucleus Switch M1 Receipt

Status: complete
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.078`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/switch.md`
Handoff: `../../handoffs/20260903-115100-g16-079-nucleus-switch-receipt.md`
Log: `../../logs/2026-09/20260903-g16-079-nucleus-switch-receipt.md`

## Goal

Produce one validated `M1` receipt for Nucleus `Switch` through the production
Rust adapter, renderer, Node, GPUI backend, and mounted input path. Prepare the
independent production proof in parallel, then pause before shared receipt
files until RadioGroup has merged and the orchestrator supplies its identity.

## Preparation Boundary

- Keep and strengthen
  `switch_toggle_readonly_and_disabled_rebuild_the_host_spec`.
- Mount `node_compat::Switch::from_spec(...).into_element()` through the
  element-backed HeadlessDriver; renderer-only Node construction is not adapter
  evidence.
- Prove caller-scoped duplicate instances, exact switch role/name/checked/
  readonly/disabled projection, controlled rebuilds, mounted pointer/Space/
  Enter toggle, same-value behavior, disabled inertia, read-only focus with
  reverted mutation, and focus isolation.
- Prove label/left/right-label composition, fallback naming, custom-color over
  semantic-tone precedence, exact track/thumb geometry metadata, token styling,
  positive bounds, ordering, and containment without A1 or V1 claims.
- Commit the proof/counterexample before any focused repair. Stop for public API
  change, another toggle machine, browser form behavior, or app policy.
- During parallel preparation, do not edit the Nucleus manifest, receipts,
  generated ledger, g16 front doors, or claim M1 completion. Push a draft PR
  and pause after focused proof/falsification validation.
- On orchestrator resume after g16.078 merge: rebase, revalidate, commit runtime
  source, emit all cohort receipts at that exact SHA, update this card and one
  log, then run the full receipt boards. Merge remains orchestrator-owned.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter-path or lifecycle fails |
| Identity is caller-scoped | reuse one runtime id | focus/callbacks cross instances |
| Switch semantics are exact | omit checked/readonly/disabled/name | Node assertion fails |
| Input is mounted | invoke handler directly | mounted observation is absent |
| Controlled ownership is real | callback without host rebuild | painted state stays stale |
| Disabled/read-only differ | let disabled focus or read-only commit | focus/callback trace fails |
| Pointer and keys agree | Space/Enter diverge from pointer | controlled result differs |
| Visual metadata is exact | alter tone/color precedence or track/thumb geometry | exact metadata fails |
| Receipt is terminal | fail final isolation assertion | no receipt emitted |
| Evidence identity is exact | emit before latest predecessor merge | cohort validation fails |
| Levels stay separate | label M1 as A1/V1 | schema validation fails |

## Validation

Preparation: focused Switch spec/machine/render/backend and named mounted tests
plus `git diff --check`. Finalization: add `effigy regressions:native`, receipt/
ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, and `effigy docs:check`. Never run windowed or native-
visual selectors.

## Continuation

After merge, continue with the next dependency-ready Nucleus M1 row. Receipt
emission and merge ordering remain serial even when proof preparation overlaps.
