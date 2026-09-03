# g16.082 — Nucleus AgentChatInput M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/082-nucleus-agent-chat-input-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-133100-g16-082-nucleus-agent-chat-input-receipt.md`
Branch: `feature/g16-082-nucleus-agent-chat-input-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-082-nucleus-agent-chat-input-receipt`
Planning base: `068bb7f3b95340954142a71b896bf70c3b8b5892` (`origin/main`)
Preparation-accepted head: `ef50f365e91f9dba749eaf69172411a0508fb969`
Runtime source: `e632700bf5768744475f91ebc6f20e3c8c35f0d9`
PR: `#188`

## Outcome

`AgentChatInput` now has one validated `M1` execution receipt through the
production Rust compat adapter, shared renderer, Node backend, and GPUI test
platform. The retained
`agent_chat_input_mounted_input_and_action_follow_host_state` fixture emits
only after its terminal eligibility, disabled, callback-order, and mounted
observation assertions.

All 19 receipts and the manifest pin runtime source
`e632700bf5768744475f91ebc6f20e3c8c35f0d9`. The generated ledger advances
only AgentChatInput's GPUI mounted-behaviour cell: 19 mounted, 156 missing. M1
does not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::AgentChatInput::from_spec(...).into_element()`
  instances compose the production TextInput and Button renderers.
- Refused and accepted controlled edit proposals both rebuild the production
  factory. Refusal rebuilds unchanged host value and selection, leaving mounted
  paint unchanged; acceptance rebuilds updated host state.
- Pointer and keyboard input prove exact change, selection, focus, submit, and
  stop ordering for idle, empty, allow-empty, read-only, disabled, and busy
  postures.
- Exact contract-owned size, density, status, disabled, field, attachment,
  editor, divider, action, footer, placeholder, opacity, and focus-ring values
  are asserted without inventing orchestration or platform channels.
- The real production mount box and actual parent bounds contain every asserted
  child. Attachments, editor, toolbar, actions, footer, and sibling instances
  are ordered with previous-bottom/next-top or previous-right/next-left checks.
- Disabled production input has no focus handle, pointer input cannot focus it,
  and its action is inert. A separate mounted adversarial Node keeps a live
  activation sink and proves backend disabled pointer suppression.

## Committed falsifications and repair

The rebased counterexample `9ba686deaff7c554814efffdffdd69d076d094ae`
failed waiting for the production editor focus handle. AgentChatInput painted
static content rather than mounting a real editor. The production compat path
was rebuilt around TextInput and Button while keeping host ownership.

The rebased review counterexample
`dc1744b42420a668f25f2ea9baf3c3ed85591ba5` committed the blocking
expectations before repair. It exposed missing root metadata and focus-within
treatment, the wrong footer token ladder, an off-mount controlled-value oracle,
incomplete parent/mount containment, and disabled evidence aimed at the prior
focus owner. The bounded repair at
`0437681bd8f64cc0f1a85b28941e38c41f4f4e76` added generalized backend-painted
text and descendant-focus observation, corrected shared renderer metadata and
tokens, used actual extents, and replaced the disabled keyboard claim with
focus suppression plus a live-sink pointer oracle.

Review at `15a104d425caea1f1d07309286d07d12816e5d82` found one remaining receipt
wording overclaim. Runtime commit
`e632700bf5768744475f91ebc6f20e3c8c35f0d9` narrows the emitted action and
assertion to the two production-factory rebuilds proved by the fixture. It does
not change the fixture or production behavior.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/agentchatinput--nucleus-agent-agent-chat-input.json` |
| Component | `AgentChatInput` |
| Scenario | `nucleus.agent.agent-chat-input` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `e632700bf5768744475f91ebc6f20e3c8c35f0d9` |
| Outcome | `passed` |

## Validation

Focused:

- AgentChatInput contract and machine helpers: 8 passed.
- AgentChatInput renderer: 2 passed.
- TextInput read-only focus seam: 1 passed.
- GPUI Node backend: 51 passed.
- Named mounted AgentChatInput fixture: 1 passed.
- Disabled live-sink routing oracle: 1 passed.
- `effigy check:gpui` — preview check plus 630 renderer and 51 backend tests.

Required boards completed before this log update:

- `effigy regressions:native` — 189 passed; all 19 receipts emitted at the
  runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed. A cold first run hit Bun's
  fixed 30-second timeout in one negative case; the unchanged warm rerun passed
  all assertions in 44.30 seconds.
- `effigy check:parity-evidence-ledger` — 176 rows validated.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.

The review repair reran the named mounted fixture (1 passed),
`effigy regressions:native` (189 passed), both receipt and ledger tests (8 and
6 passed), the 176-row ledger check, and exact-state `effigy docs:check`.
`git diff --check` is the final handoff gate after this log update.

## Limits

- AgentChatInput M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- No windowed or native-visual selector ran.
- No merge and no next card.
