# g16.083 — Nucleus AgentTranscript M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/083-nucleus-agent-transcript-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-163500-g16-083-nucleus-agent-transcript-receipt.md`
Branch: `feature/g16-083-nucleus-agent-transcript-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-083-nucleus-agent-transcript-receipt`
Planning base: `068bb7f3b95340954142a71b896bf70c3b8b5892`
Preparation-accepted head: `61c5cc7c909ea8cfce44c044fbb6559b303dc44a`
Finalization base: `dec442579f400a78d7b656f6feaa8943520b4f57`
Runtime source: `2d14d7e6afa25976cd475afb6b60542027b47216`
PR: `#190`

## Outcome

`AgentTranscript` now has one validated `M1` receipt through the production
Rust compatibility adapter, shared renderer, Node backend, and mounted GPUI
test platform. The retained
`agent_transcript_records_rebuild_through_production_mounted_input` fixture
emits only after its terminal production-dependency assertion.

All 20 cohort receipts and the manifest pin runtime source
`2d14d7e6afa25976cd475afb6b60542027b47216`. The generated ledger advances only
AgentTranscript's GPUI mounted-behaviour cell: 20 mounted, 155 missing. M1 does
not infer A1 or V1.

## Production proof

- Caller-scoped `node_compat::AgentTranscript::from_spec(...).into_element()`
  instances reach `HeadlessDriver` through the production adapter, renderer,
  Node backend, and GPUI element path.
- The empty rebuild mounts the real EmptyState. The loading rebuild replaces it
  with a contained activity row composed from the real Spinner and Text.
- User AgentMessage composition keeps the exact elevated background and surface
  radius without either Surface elevation shadow channel. Structural assertions
  fail if a raw shell replaces the production Surface composition.
- Host-owned disclosure and appended records rebuild the production factory.
  Duplicate instances keep separate runtime IDs, focus, callbacks, bounds, and
  scroll state.
- Mounted keyboard, pointer, wheel, and jump-control input proves exact callback
  order, bounded overflow, detached-reader preservation, and return to pinned
  following.
- Ordered message, tool-run, answered-question, decided-plan, and activity
  records preserve their contract-owned roles, statuses, text, containment, and
  non-overlapping geometry.

## Committed falsifications and repair

The rebased production-path counterexample
`860e4e8d1` failed on missing mounted identity and real input seams. Repair
`11d53cac1` routed AgentTranscript through production primitives with
caller-scoped identity and tracked scrolling.

The rebased dependency counterexample `a38757624` exposed raw empty/loading
text and an elevated Surface override that retained an uncontracted shadow.
Repair `de304cbbd` mounted real EmptyState, Spinner, and Text structures and
composed the exact AgentMessage Surface treatment without shadow or invented
posture metadata.

Runtime commit `2d14d7e6a` makes the stable receipt fixture execute that accepted
dependency oracle and emits only after its terminal assertion.

## Receipt identity

| Field | Value |
| --- | --- |
| File | `docs/roadmaps/g16/nucleus-parity-receipts/agenttranscript--nucleus-agent-agent-transcript.json` |
| Component | `AgentTranscript` |
| Scenario | `nucleus.agent.agent-transcript` |
| Proof level | `M1` |
| Runtime | `gpui-headless` |
| Command | `effigy regressions:native` |
| Source commit | `2d14d7e6afa25976cd475afb6b60542027b47216` |
| Outcome | `passed` |

## Validation

- Focused AgentTranscript contract, render, AgentMessage composition, backend
  scroll, and three mounted production-path tests passed after rebase.
- The stable receipt fixture and standalone dependency counterexample both
  passed after terminal emission was added.
- `effigy regressions:native` — 191 passed; all 20 receipts emitted at the
  runtime source.
- `effigy test:nucleus-parity-receipts` — 8 passed.
- `effigy test:parity-evidence-ledger` — 6 passed after the generated ledger was
  updated from the validated cohort.
- `effigy check:parity-evidence-ledger` — 176 rows validated.
- `effigy ci:rust` — clean.
- `effigy ci:native` — clean.

Exact-state `effigy docs:check` and `git diff --check` are the final handoff
gates after this log update.

## Limits

- AgentTranscript M1 only. No other Nucleus row advances.
- No A1, V1, Nucleus, web, g16 front-door, workflow, release, version, or
  native-visual change.
- No windowed or native-visual selector ran.
- No merge and no next card.
