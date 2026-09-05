# g16.114 — Nucleus A1 NP-3 Agent Workflow

Status: implementation batch prepared; awaiting exact-head review
Base: `3dbabac3990fb5f3856305b7c8f971039b0a81be` (`origin/main`, manifest revision 17)

## Scope

Added shared A1 scenarios and Svelte snapshots for AgentTranscript,
AgentChatInput, AgentPlan, AgentQuestion, ModelPicker, and StatusIndicator.
Added mounted GPUI proof entrypoints for the six rows, reusing the foundation
snapshot/comparison/emission path. No component contract or runtime behavior
changed.

## Validation

- `effigy core:build` — passed.
- `effigy test:nucleus-a11y` with snapshot writing — passed, 10 tests.
- Native focused validation was started but was delayed by concurrent Cargo
  builds in sibling worktrees; no windowed selector was run.
- `git diff --check` — passed.

## Notes

The Svelte package required the existing `effigy core:build` bootstrap before
Vitest could resolve generated style exports. The friction is recorded in the
root `PAPERCUTS.md`. The native proof remains subject to exact-head review and
must not be merged from this worker.
