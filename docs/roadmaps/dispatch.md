# Queue Dispatch Projection

Status: active
Owner: Chatterbox
Updated: 2026-09-09 (flattened-task switchover)
Planning authority: [`g17/README.md`](g17/README.md)

This file is the control-plane projection of the generation README's approved
frontier. It cannot make a task ready, create dependencies, or preserve a
second roadmap. The coordinator dispatches only the exact task named below and
verifies that this file and the generation README agree at the same pushed
commit.

## Ready queue task

- **Northstar task:** [`g17.001`](g17/001-nucleus-v1-visual-receipts.md)
- **State:** ready; serial; no concurrent sibling; no automatic successor.
- **Prerequisite:** poodle-lab `g01.006` complete; Lab PR #8 merged at
  `f99465f048d7c5c58603b99ae51f3209e581848e`; validated run
  `2026-09-08T14-06-48` available.
- **Completion:** immutable validated import, V1 receipts for every covered row,
  receipt-backed ledger advancement, all findings retained without adjudication.
- **Owned and reserved paths:** the task's inline dispatch manifest is exact.
- **Worker:** general implementation.
- **Review:** independent exact-head PR review required.
- **Escalation:** Chatterbox for evidence mapping/policy; operator for expansion.

## Held planning horizons

V2, M2, A2, the switch packet, GPUI keyboard-origin focus, web-pair extraction,
the contributor-guidance pilot, Jetstream, and triage holds are not queue tasks.
Their gates remain in `g17/README.md` and current triage.

## Historical queue evidence

Completed execution remains in queue history, PRs, logs, and generation
roll-ups. Historical queue records keep their original card/batch wording.
