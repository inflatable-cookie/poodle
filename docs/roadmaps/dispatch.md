# Worker Dispatch Ledger

Status: active
Owner: Poodle orchestrator
Updated: 2026-08-11

The orchestrator is the only writer. Workers write their scoped deliverables,
batch log, and papercuts; they do not edit this ledger or roadmap/card status.

| Batch | Card | Branch | Model | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| `g13-b001-authority-inventory` | `g13/batch-cards/001-authority-inventory-and-docs-baseline.md` | `thread/g13-001-authority-inventory` | `deepseek-v4-flash` (`xhigh`) | merged | Commit `251cc858` → merge `a0ca039d`; counts re-measured and confirmed; `svelte:surface-audit`, `docs:lint`, `docs:check`, `git diff --check` all exit 0 on merged main; review log `docs/logs/2026-08/11-g13-b001-b005-review-and-merge.md`; worktree removed |
| `g13-b002-pilot-fixture-metrics` | `g13/batch-cards/002-pilot-fixture-and-metrics-freeze.md` | `thread/g13-002-pilot-fixture-metrics` | `deepseek-v4-flash` (`xhigh`) | dispatched | Dispatched 2026-08-11 from `6eade887`, `--max-time 90m`; worktree `poodle-wt/g13-002-pilot-fixture-metrics`; log `/tmp/omp-g13-b002.log`; event-driven watcher on pid, no polling model monitor |
| `g13-b004-rust-ir-prior-art` | `g13/batch-cards/004-rust-ir-prior-art-and-failure-audit.md` | `thread/g13-rust-ir-prior-art` | `cursor-grok-4.5-medium` | dead | Provider stream stalled twice, including resume; no worktree edits or commit. Recompile into smaller cards on a different model before any redispatch; does not block `002` |
| `g13-b005-pilot-expressiveness` | `g13/batch-cards/005-pilot-contract-expressiveness-corpus.md` | `thread/g13-pilot-expressiveness-corpus` | `deepseek-v4-flash` (`xhigh`) | merged | Commit `2f8dc5db` → merge `bb3f79ef`; 129 requirement IDs verified unique, no schema recommendation, `UNKNOWN-01`/`UNKNOWN-02` preserved unresolved; review log `docs/logs/2026-08/11-g13-b001-b005-review-and-merge.md`; worktree removed |
