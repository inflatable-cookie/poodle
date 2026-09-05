# g16.119 — A1 Focus And State Semantics

Status: ready — concurrent with `g16.118`
Type: native behaviour repair — GPUI backend focus routing and `poodle-render`
state projection; A1 receipts for five rows
Opened: 2026-09-05
Depends on: merged `g16.111`–`g16.117`
Governing refs: `../../contracts/003-native-accessibility.md`,
`../../contracts/components/{menu,agent-question,agent-transcript,radio-group,segmented-control}.md`,
`packages/gpui/node-backend/src/{lib,interaction}.rs` (focus), `packages/render/src/*.rs`,
`nucleus-parity-receipts/a1-divergences/{menu,agent-question,agent-transcript,radio-group,segmented-control}/`
Dispatch manifest: `../dispatch.md`

## Rows and recorded causes

| Row | GPUI | Svelte (reference) |
| --- | --- | --- |
| Menu | after activation the menu is not focused; item not in focus order | focus moves into the menu; item is focus stop 3 |
| AgentQuestion | post-action focus stays; label not `labelled_by`-linked | focus moves to the answered state's control; label linked |
| AgentTranscript | post-action focus `null` | focus lands on the new entry |
| RadioGroup | `orientation` "vertical" projected where Svelte omits; second radio not in focus order | roving focus: one stop, `orientation` only when set |
| SegmentedControl | `selected` projected on segments; segments all in focus order | Svelte uses `aria-pressed`/checked semantics (`selected` null) and one roving stop |

## Fixed Boundary

- Backend: after a scenario action, focus must land where the contract says
  and the accessibility snapshot must show it (`focused`, `focus_order`),
  through production dispatch, not test shims. Roving tab stops project one
  `focus_order` entry per group.
- Render: project the state the contract names (`pressed`/`checked` for
  SegmentedControl per its contract; `orientation` only when the prop is
  set), and link `labelled_by` where a label node exists.
- Svelte unchanged unless the contract contradicts it; ruling recorded first.
- Proof: empty-diff A1 receipts for the five rows; divergence stores deleted;
  cohort repin and re-emit; ledger regenerated.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Focus is real | `focused` asserted from the record without dispatch | receipt comes from the mounted driver run |
| One roving stop | two segments in focus order | diff non-empty |
| No overlay structure edits | a change to dialog composition | lane is red (that is `118`) |

## Validation

`effigy regressions:native`, `cargo test -p poodle-render`,
`effigy check:parity-evidence-ledger`, `effigy docs:check`, `git diff --check
origin/main...HEAD`.

## Owned Paths

GPUI backend focus routing for the five rows,
`packages/render/src/{menu,agent_question,agent_transcript,radio_group,segmented_control}.rs`
and tests, the rows' A1 tests, scenarios, receipts and divergence stores,
manifest `resolution`, ledger, execution log, `PAPERCUTS.md` (append).

## Stop Conditions

Stop when a focus rule is not in the contract; return the question with the
two snapshots. Escalation owner: Chatterbox.
