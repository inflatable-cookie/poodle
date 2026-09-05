# g16.114 — Nucleus A1 NP-3 Agent Workflow

Status: revision 18 disposition recorded; awaiting fresh exact-head review
Base: `3dbabac3990fb5f3856305b7c8f971039b0a81be` (`origin/main`, manifest revision 17)
Manifest revision 18 source commit: `c0659590b8e8abfa8bda0f97037ad4b1c5ecf78e`; lock digest unchanged at `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`.

## Scope

Added shared A1 scenarios and Svelte snapshots for AgentTranscript,
AgentChatInput, AgentPlan, AgentQuestion, ModelPicker, and StatusIndicator.
Added mounted GPUI proof entrypoints for the six rows, reusing the foundation
snapshot/comparison/emission path. StatusIndicator is the only promoted NP-3
receipt. ChatInput, AgentPlan, AgentQuestion, AgentTranscript, and ModelPicker
are recorded under `docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/`
with both snapshots, diffs, and exact changed attributes. Switch/Tabs receipts
remain coordinator-owned.

## Validation

- `effigy core:build` — passed.
- `effigy test:nucleus-a11y` with snapshot writing — passed, 10 tests.
- `effigy regressions:native` equivalent focused headless native checks — six
  NP-3 selectors passed; five recorded divergence rows emitted no receipts.
- No windowed selector was run.
- `git diff --check` — passed.

## Notes

The Svelte package required the existing `effigy core:build` bootstrap before
Vitest could resolve generated style exports. The friction is recorded in the
root `PAPERCUTS.md`. The native proof remains subject to exact-head review and
must not be merged from this worker.
