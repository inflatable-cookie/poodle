---
title: g16.110 gpui-unofficial spike worker handoff
kind: northstar-handoff
status: complete
owner: g16.110 worker
created: 2026-09-05
updated: 2026-09-05
handoff_path: /Users/tom/.paseo/worktrees/1ugbsx1t/g16-110-gpui-unofficial-feasibility-spike/docs/handoffs/20260905-g16-110-gpui-unofficial-spike.md
tags: [coordination, handoff, g16, g16.110]
---

## What This Thread Was Doing

Execute g16.110 as a throwaway two-day spike: move the native pair from
crates.io `gpui = "0.2.2"` to `gpui-unofficial = "1.19.0-pre"`, prove
licensing, measure the API delta, and report. No merge of production changes.

## Why It Matters

This is the intended route to GPUI accessibility (upstream AccessKit) instead
of a Poodle-owned adapter. The report decides adopt-at-1.19.0, adopt later, or
reject.

## Current State

- **Done:** spike compiled against `gpui-unofficial` 1.19.0-pre from crates.io.
  AccessKit projection exists. Licence graph is GPL-clean and git-free.
  Consumer-identity gate re-points. Report written.
- **Still open:** independent exact-head review of the docs PR. Coordinator
  owns review, merge of the **report only**, and reserved closeout. Spike
  branch stays unmerged.
- **Active spec lane:** none. Spike is disposable.
- **Current batch card:** `docs/roadmaps/g16/110-gpui-unofficial-feasibility-spike.md`
- **Canonical refs:** `docs/contracts/003-native-accessibility.md`,
  `deny.toml`, `test/consumer-dual-dependency/run.ts`
- **Remaining continuation envelope:** stop. Recommendation is adopt later.
- **Lane budget / pause signal:** two-day time box; finished in one day.
- **Key files:**
  - `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-110-gpui-unofficial-feasibility-spike/docs/logs/2026-09/20260905-g16-110-gpui-unofficial-spike.md`
  - `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-110-gpui-unofficial-feasibility-spike/packages/gpui/node-backend/src/a11y.rs`

Exact identities after the spike commit (fill at push): see git on
`spike/gpui-unofficial`. Dispatch base was
`da8c9c37a2a5fd43d7767434fccc5dfceceb81e6`. Workspace is this Paseo worktree.
Do not use the coordinator checkout or sibling worktrees.

## Boundaries

- **In scope:** spike branch code, the report, PAPERCUTS append, this handoff,
  a docs-only PR against `main`.
- **Out of scope:** merging the spike, `deny.toml` exceptions, vendoring,
  windowed selectors, `poodle-node` / `poodle-render` vocabulary changes.
- **Repo constraints:** `AGENTS.md`, card stop conditions, no `*-windowed`.

## Important Context

- **Planning lineage:** g16.110 ready card; operator 2026-09-05 chose the
  republish over a fork-free adapter.
- **How the plan fits:** native pair stays on one crates.io GPUI identity.
- **Decisions:** target stayed `1.19.0-pre` (no stable 1.19.x). Live preview
  stubbed rather than patching `gpui-apple`. AccessKit tree-read recorded as
  a test-platform no-op, not faked from source-only snapshots.
- **Open tensions:** `bzip2-1.0.6` vs deny allow list; `gpui-apple` registry
  layout; overlay dismiss focus restore on 1.19.

## Suggested Next Move

Review the docs PR at the exact head in this workspace. Do not merge
`spike/gpui-unofficial`. A later migration card waits on stable 1.19.x, a
crates.io-buildable `gpui-apple`, and an operator call on `bzip2-1.0.6`.

## Completion Protocol

1. Card stop: report at the time box; remaining work listed in the log.
2. Log is the authority for what ran. Roadmap closeout is coordinator-owned.
3. Continuation envelope exhausted for this spike.
4. Lane budget: one worker day used of two.
5. Blockers: licence deny on `bzip2-1.0.6`; live Application blocked on
   `gpui-apple` build.rs; AccessKit tree unread on `TestWindow`.
6. Next task: exact-head review of the docs PR, then coordinator closeout.

Disposition: delete after the report PR is reviewed/merged or the spike is
abandoned. Keep `spike/gpui-unofficial` unmerged.
