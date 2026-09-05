---
title: g16.119 A1 focus and state semantics closeout
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: complete-merged
owner: Poodle Northstar orchestrator
created: 2026-09-05
updated: 2026-09-05
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.119]
---

`docs/roadmaps/g16/119-a1-focus-and-state-semantics.md` was implemented on
`worker/g16-119-a1-focus-state-semantics`, rebased onto `origin/main` at
`e4407101e5836843e124c75b4884c118f74bb1e3`. PR #223 merged as
`cceb6646a2bf7776b670fb63f586bce037d0ee6e`.

The final runtime/evidence source pin is
`ecc88ff5a37fb13fabe721ba1595fd3ad166b2f1`. The cohort contains 29 M1 and 21
A1 receipts. All five newly active overlay rows emit empty-diff receipts:
Dialog, Popover, ConfirmAction, MessageCenter, and ModelPicker. The five
g16.118 focus-only stores were consumed.

The implementation adds one `poodle-node` accessibility-record field,
`initial_focus: bool`. The five renderers set it on exactly one node per open
overlay; the backend claims it once per runtime identity and routes the normal
mount focus request. ModelPicker's A1 probe now replays the host-owned
closed-to-open transition, and its focused regression covers controlled state
whose selected model is disabled: the first enabled fallback row is both the
initial-focus target and the single sequential tab stop.

The original five-row scope is retained: AgentQuestion is repaired and emits
an empty-diff receipt; Menu, AgentTranscript, RadioGroup, and
SegmentedControl retain their documented honest divergence stores. Older NP-3
and NP-1 records remain outside this card. The ledger manifest is repinned to
the final source pin and regenerated.

Execution log: `docs/logs/2026-09/20260905-g16-119-a1-focus-and-state-semantics.md`.

Validation and hosted results are recorded in the execution log. No windowed
selector was run. The worker pushed this revision to PR #223; independent
review accepted it and the coordinator merged it. This handoff is archived
under the retention rule.
