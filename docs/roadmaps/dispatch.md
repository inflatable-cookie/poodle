# Worker Dispatch Ledger

Status: active
Owner: Poodle orchestrator
Updated: 2026-08-11

The orchestrator is the only writer. Workers write their scoped deliverables,
batch log, and papercuts; they do not edit this ledger or roadmap/card status.

| Batch | Card | Branch | Model | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| `g13-b001-authority-inventory` | `g13/batch-cards/001-authority-inventory-and-docs-baseline.md` | `thread/g13-001-authority-inventory` | `deepseek-v4-flash` (`xhigh`) | merged | Commit `251cc858` → merge `a0ca039d`; counts re-measured and confirmed; `svelte:surface-audit`, `docs:lint`, `docs:check`, `git diff --check` all exit 0 on merged main; review log `docs/logs/2026-08/11-g13-b001-b005-review-and-merge.md`; worktree removed |
| `g13-b002-pilot-fixture-metrics` | `g13/batch-cards/002-pilot-fixture-and-metrics-freeze.md` | `thread/g13-002-pilot-fixture-metrics` | `deepseek-v4-flash` (`xhigh`) | merged | Commit `89debbcb` → merge `2368f436`; 32 `FIX-*` fixtures, full quantitative baseline; LOC, glob totals, and verbatim drift-gate quotes spot-verified; `docs:lint`, `svelte:surface-audit`, `git diff --check` exit 0 on merged main; found the corpus §8 classification-count defect (ruled: row marks win, `EXT` = 6); worktree removed |
| `g13-b006-button-tone-parity` | `g13/batch-cards/006-button-family-tone-parity.md` | `thread/g13-006-button-tone-parity` | `deepseek-v4-flash` (`xhigh`) | merged | Commit `22337a31` → merged; Button success + IconButton warning delivered, all 7 tone blocks verified byte-identical to their danger counterparts after substitution; renderer confirmed exhaustive over variant×tone, no Rust edited; correctly stopped on the SplitButton shadow conflict (→ `008`) |
| `g13-b007-value-domain-drift` | `g13/batch-cards/007-value-domain-drift-inventory.md` | `thread/g13-007-value-domain-drift` | `deepseek-v4-flash` (`xhigh`) | merged | Commit `8090521c` → merged; 21 disagreements + 8 unresolved types across 16 components; exit-code behaviour and button-family-clean claim both re-verified by the orchestrator; `docs:check` untouched |
| `g13-b004-rust-ir-prior-art` | `g13/batch-cards/004-rust-ir-prior-art-and-failure-audit.md` | `thread/g13-rust-ir-prior-art` | `cursor-grok-4.5-medium` | dead | Provider stream stalled twice, including resume; no worktree edits or commit. Recompile into smaller cards on a different model before any redispatch; does not block `002` |
| `g13-b005-pilot-expressiveness` | `g13/batch-cards/005-pilot-contract-expressiveness-corpus.md` | `thread/g13-pilot-expressiveness-corpus` | `deepseek-v4-flash` (`xhigh`) | merged | Commit `2f8dc5db` → merge `bb3f79ef`; 129 requirement IDs verified unique, no schema recommendation, `UNKNOWN-01`/`UNKNOWN-02` preserved unresolved; review log `docs/logs/2026-08/11-g13-b001-b005-review-and-merge.md`; worktree removed |
