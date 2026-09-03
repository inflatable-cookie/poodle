# g16.078 — Nucleus RadioGroup M1 Receipt

Status: preparation-ready; receipt finalization blocked on merged `g16.077`
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`; receipt finalization depends on merged `g16.077`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `nucleus-parity-manifest.json`,
`parity-evidence-ledger.md`, `../../contracts/components/radio-group.md`
Handoff: `../../handoffs/20260903-115000-g16-078-nucleus-radio-group-receipt.md`

## Goal

Produce one validated `M1` receipt for Nucleus `RadioGroup` through the
production Rust adapter, renderer, Node, GPUI backend, and mounted input path.
Prepare the independent production proof in parallel, then pause before shared
receipt files until the orchestrator supplies the latest merged cohort base.

## Preparation Boundary

- Keep and strengthen
  `radio_group_exclusive_focus_identity_and_disabled_paths`.
- Mount `node_compat::RadioGroup::from_spec(...).into_element()` through the
  element-backed HeadlessDriver; renderer-only Node construction is not adapter
  evidence.
- Prove caller-scoped duplicate-valued instances, exact radiogroup/radio names
  and selected/disabled semantics, single-entry roving tab posture, controlled
  host rebuild, mounted pointer/Space/Arrow/Home/End navigation, wrapping,
  disabled skipping, orientation axes, same-value/group-disabled inertia, and
  focus retention.
- Prove exact indicator/dot production structure, selected-color precedence,
  typography, token styling, orientation layout, positive bounds, ordering,
  and containment without A1 or V1 claims.
- Commit the proof/counterexample before any focused repair. Stop for public API
  change, another selection machine, browser form-name behavior, or app policy.
- During parallel preparation, do not edit the Nucleus manifest, receipts,
  generated ledger, g16 front doors, or claim M1 completion. Push a draft PR
  and pause after focused proof/falsification validation.
- On orchestrator resume after g16.077 merge: rebase, revalidate, commit runtime
  source, emit all cohort receipts at that exact SHA, update this card and one
  log, then run the full receipt boards. Merge remains orchestrator-owned.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter-path or mounted lifecycle fails |
| Identity is caller-scoped | derive option ids from value alone | two groups cross focus/callbacks |
| Exclusive semantics are exact | omit selected radio state or accessible label | Node assertion fails |
| Roving input is exact | focus disabled row or stop at edge | mounted navigation trace fails |
| Controlled ownership is real | callback without host rebuild | painted selection stays stale |
| Disabled/same-value are inert | emit on disabled or current option | callback count changes |
| Axis is exact | accept cross-axis key | focus/selection trace changes |
| Visual metadata is exact | alter selected color, indicator, or layout tokens | exact metadata fails |
| Receipt is terminal | fail final isolation assertion | no receipt emitted |
| Evidence identity is exact | emit before latest predecessor merge | cohort validation fails |
| Levels stay separate | label M1 as A1/V1 | schema validation fails |

## Validation

Preparation: focused RadioGroup spec/machine/render/backend and named mounted
tests plus `git diff --check`. Finalization: add `effigy regressions:native`,
receipt/ledger tests, `effigy check:parity-evidence-ledger`, `effigy ci:rust`,
`effigy ci:native`, and `effigy docs:check`. Never run windowed or native-
visual selectors.

## Continuation

After orchestrator merge, g16.079 may rebase and finalize its prepared Switch
proof against this exact cohort identity.
