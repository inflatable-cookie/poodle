# g16.082 — Nucleus AgentChatInput M1 Receipt

Status: complete
Type: Nucleus NP-3 mounted receipt child
Opened: 2026-09-03
Closed: 2026-09-03
Depends on: completed `g16.062`, completed Button and TextInput receipts; serial
finalization follows the latest merged Nucleus receipt
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`, `077-nucleus-text-input-m1.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/agent-chat-input.md`
Handoff: `../../handoffs/20260903-133100-g16-082-nucleus-agent-chat-input-receipt.md`

## Goal

Produce the first named production-path mounted proof for Nucleus
`AgentChatInput`, then emit one terminal `M1` receipt at the exact committed
runtime source.

Completed at runtime source
`a00be5c662a034bd8ca1a4278cdd7ee2be5e3c12`. The 19-receipt cohort shares that
identity, and the generated ledger advances only AgentChatInput from missing to
mounted. M1 does not infer A1 or V1.

## Fixed Boundary

- The manifest names only the retained
  `agent_chat_input_mounted_input_and_action_follow_host_state` fixture for this
  card.
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
- Preparation did not edit the Nucleus manifest, receipts, generated ledger,
  or g16 front doors and stopped at an independently accepted draft PR.
- Finalization rebased onto the latest receipt merge and orchestration-doc head,
  revalidated, committed the terminal emitter, and emitted the 19-receipt
  cohort. Merge and g16 front-door closeout remain orchestrator-owned.

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

Focused AgentChatInput spec/machine/render/backend and named mounted tests plus
`git diff --check`. Finalization adds
`effigy regressions:native`, receipt/ledger tests,
`effigy check:parity-evidence-ledger`, `effigy ci:rust`, `effigy ci:native`,
and `effigy docs:check`. Never run windowed or native-visual selectors.

## Continuation

Pause for preparation-to-M1 re-review. Merge and g16 front-door closeout remain
with the orchestrator; this card does not start another receipt.
