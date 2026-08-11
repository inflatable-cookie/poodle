# Worker Dispatch Ledger

Status: active
Owner: Poodle orchestrator
Updated: 2026-08-11

The orchestrator is the only writer. Workers write their scoped deliverables,
batch log, and papercuts; they do not edit this ledger or roadmap/card status.

| Batch | Card | Branch | Model | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| `g13-b001-authority-inventory` | `g13/batch-cards/001-authority-inventory-and-docs-baseline.md` | `thread/g13-001-authority-inventory` | `deepseek-v4-flash` (`xhigh`) | dispatched | Initial stream closed mid-task; resumed as OMP session `27571`; worktree `poodle-wt/g13-001-authority-inventory` |
| `g13-b004-rust-ir-prior-art` | `g13/batch-cards/004-rust-ir-prior-art-and-failure-audit.md` | `thread/g13-rust-ir-prior-art` | `cursor-grok-4.5-medium` | dead | Provider stream stalled twice, including resume; no worktree edits or commit |
| `g13-b005-pilot-expressiveness` | `g13/batch-cards/005-pilot-contract-expressiveness-corpus.md` | `thread/g13-pilot-expressiveness-corpus` | `deepseek-v4-flash` (`xhigh`) | dispatched | OMP persistent session `22910`; worktree `poodle-wt/g13-pilot-expressiveness-corpus` |
