# Release Visual Gate Reconciliation

Date: 2026-08-10

## Scope

Run the remaining release visual gates and reconcile their operator contract.

## Findings

- The 360-capture web axis gate had no cross-framework failures.
- The GPUI gate found 55 stale local baselines. The repeated small deltas were
  the public package-scope rename inside preview snippets. Larger deltas were
  the shared Lucide catalogue expansion and new AgentChatInput questioning and
  plan-review specimens.
- The native visual README and source comments still called gitignored,
  machine-local references "committed baselines".

## Changes

- Refreshed only the 55 affected local GPUI baselines after inspecting the
  representative package-scope, icon-catalogue, and AgentChatInput diffs.
- Reworded the native visual operator contract around local references,
  inspection before refresh, and durable reason recording.

## Validation

- `effigy ci:visual`
- `bun test/native-visual/run.ts --update --slug=<55 affected slugs>`
- `bun test/native-visual/run.ts --slug=<55 affected slugs>`
- `effigy test:jetstream-visual`
- `git diff --check`
