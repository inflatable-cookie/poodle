---
title: g16.119 A1 focus and state semantics closeout
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: implementation-complete-awaiting-review
owner: Poodle Northstar orchestrator
created: 2026-09-05
updated: 2026-09-05
base_required: pushed-main
tags: [coordination, handoff, worker, g16, g16.119]
---

`docs/roadmaps/g16/119-a1-focus-and-state-semantics.md` is implemented on
`worker/g16-119-a1-focus-state-semantics`, rebased onto `origin/main` at
`e4407101e5836843e124c75b4884c118f74bb1e3`. Nothing is merged.

The final runtime/evidence source pin is
`1636f70bd373323128e3bf4dd5e923fed0066e45`. The cohort contains 29 M1 and 21
A1 receipts. All five newly active overlay rows emit empty-diff receipts:
Dialog, Popover, ConfirmAction, MessageCenter, and ModelPicker. The five
g16.118 focus-only stores were consumed.

The implementation adds one `poodle-node` accessibility-record field,
`initial_focus: bool`. The five renderers set it on exactly one node per open
overlay; the backend claims it once per runtime identity and routes the normal
mount focus request. ModelPicker's A1 probe now replays the host-owned
closed-to-open transition, so its selected enabled model row is proved focused.

The original five-row scope is retained: AgentQuestion is repaired and emits
an empty-diff receipt; Menu, AgentTranscript, RadioGroup, and
SegmentedControl retain their documented honest divergence stores. Older NP-3
and NP-1 records remain outside this card. The ledger manifest is repinned to
the final source pin and regenerated.

Execution log: `docs/logs/2026-09/20260905-g16-119-a1-focus-and-state-semantics.md`.

Validation and hosted results are recorded in the execution log. No windowed
selector was run. The worker pushes this revision to PR #223 and stops for
fresh exact-head review; merge remains coordinator-owned.
