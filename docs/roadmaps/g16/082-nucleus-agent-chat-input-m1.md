# g16.082 — Nucleus AgentChatInput M1 Receipt

Status: preparation-ready
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed Button and TextInput receipts; serial
finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `077-nucleus-text-input-m1.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/agent-chat-input.md`
Handoff: `../../handoffs/20260903-133100-g16-082-nucleus-agent-chat-input-receipt.md`

## Goal

Prepare the first named production-path mounted proof for Nucleus
`AgentChatInput`. Pause before shared evidence. After the orchestrator supplies
the latest merged cohort identity, finalize one terminal `M1` receipt.

## Preparation Boundary

- Add one stable expected test name to the manifest only during finalization;
  preparation may create the named test but must not edit the manifest.
- Mount `node_compat::AgentChatInput::from_spec(...).into_element()` through the
  element-backed `HeadlessDriver`; renderer-only Node construction is not
  adapter evidence.
- Prove production TextInput and Button composition, value/draft ownership,
  placeholder, disabled/read-only/loading posture, submit eligibility, mounted
  focus/input, exact callback payload/order, token metadata, ordering,
  containment, and caller-scoped duplicate-instance identity.
- Host state owns the controlled value and submission result. Submission does
  not clear or persist text unless the host rebuilds that state. Refusal and
  pending posture remain stable.
- Drive mounted pointer and keyboard input. Do not call transitions or handlers
  directly after mount. Do not infer OS IME, multiline browser geometry, or
  Nucleus agent orchestration.
- Commit a biting counterexample before any repair. A bounded native repair is
  allowed only when the production mounted proof fails.
- During preparation, do not edit the Nucleus manifest, receipts, generated
  ledger, g16 front doors, or claim M1 completion. Push a draft PR and pause.
- On orchestrator resume, rebase onto the latest receipt merge, set the expected
  test, revalidate, commit runtime source, emit the cohort, update this card and
  one log, then run full receipt boards. Merge remains orchestrator-owned.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount renderer Node directly | adapter identity/lifecycle assertion fails |
| Dependencies are real | replace TextInput or Button with raw nodes | metadata or mounted input proof fails |
| Controlled ownership is real | clear value on callback without host rebuild | painted value disagrees with host state |
| Submit eligibility is exact | submit empty/disabled/loading input | callback trace gains a forbidden payload |
| Input is mounted | invoke submit/change handler directly | mounted observation is absent |
| Callback order is exact | submit before terminal value change | ordered trace fails |
| Identity is caller-scoped | reuse one runtime id | focus, draft, or callbacks cross instances |
| Geometry is exact | overlap composer parts or escape the mount box | ordering/containment assertion fails |
| Receipt is terminal | fail final refusal/isolation assertion | no receipt is emitted |
| Evidence identity is exact | emit before the latest predecessor merge | cohort validation fails |
| Levels stay separate | label M1 as A1/V1 | schema validation fails |

## Validation

Preparation: focused AgentChatInput spec/machine/render/backend and named
mounted tests plus `git diff --check`. Finalization adds
`effigy regressions:native`, receipt/ledger tests,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
and `effigy docs:check`. Never run windowed or native-visual selectors.

## Continuation

Pause after preparation. The orchestrator selects finalization order from the
latest merged cohort; shared receipt production remains serial.
