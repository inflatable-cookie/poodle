# Worker Dispatch Ledger

Status: active
Owner: Poodle orchestrator
Updated: 2026-08-11

The orchestrator is the only writer. Workers write their scoped deliverables,
batch log, and papercuts; they do not edit this ledger or roadmap/card status.

| Batch | Card | Branch | Model | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| `g13-b001-authority-inventory` | `g13/batch-cards/001-authority-inventory-and-docs-baseline.md` | `thread/g13-001-authority-inventory` | `deepseek-v4-flash` (`xhigh`) | dispatched | OMP persistent session `99707`; worktree `poodle-wt/g13-001-authority-inventory` |
