# Roadmap Backlog Retirement

Date: 2026-09-09
Repository: `inflatable-cookie/poodle`
Integration branch: `main`
Retirement base: `b8b672cdbbd0c8e6d7f7dbfd2d7a8ad27f18e703`
Status: complete
Authority: operator-prompted one-time Northstar roadmap-backlog retirement
(`retire-roadmap-backlog-prompt.md`), queued as
`docs/handoffs/20260909-160254-roadmap-backlog-retirement.md`

## Preflight

The integration checkout was clean and synchronized at the retirement base,
which contains the committed queue handoff. Active generation is `g17`;
the approved frontier is empty after the `g17.001` closeout. No unfinished
Poodle or Poodle Lab queue task owns `docs/roadmaps/`, `docs/triage/`, or
the live doctrine/checkers this cleanup changes, and the GitHub PR list is
empty. Pre-existing manual worktrees under `/Users/tom/.t3/worktrees/poodle/`
are outside this cleanup and were not modified, archived, detached, stopped,
or inspected.

## Inventory

The backlog surface is exactly two files:

- `docs/roadmaps/backlog/README.md` — deferred-work scaffolding
- `docs/roadmaps/backlog/svelte-semantic-sizing-rollout.md` — semantic
  sizing and density rollout, every phase marked complete

No other `backlog` directory exists under `docs/`. No backlog-item template
exists under `docs/roadmaps/templates/` (only `task-template.md`). The only
live inbound backlog-doctrine link is in `docs/roadmaps/README.md`
("Backlog items live in [`backlog/`](backlog/)"). Remaining `backlog` word
matches are historical evidence (closed handoffs, archived roll-ups, dated
logs, queue records, retired-token commentary) and stay unchanged.

## Disposition manifest

| Removed item | Disposition | Reason |
| --- | --- | --- |
| `docs/roadmaps/backlog/svelte-semantic-sizing-rollout.md` | removed as implemented, no placeholder created | Every phase is marked complete in the file; its durable behavior is already carried by current Svelte source, component contracts, specimens, component-docs entries, and the presentation architecture. It holds no unresolved commitment to move to triage. |
| `docs/roadmaps/backlog/README.md` | removed as superseded scaffolding | Deferred work no longer has a backlog intake layer; unresolved or deferred candidates live in `docs/triage/` until promoted. |
| `docs/roadmaps/backlog/` directory | deleted completely | No backlog surface remains; no alias, stub, compatibility folder, or empty directory is left behind. |

No triage note was created: there is no unresolved or deferred candidate in
the removed item. No task was created or merged, no generation was opened,
and the frontier is unchanged and empty.

## Doctrine and checker

- `docs/roadmaps/README.md` no longer teaches a live backlog. Its rules now
  state that roadmaps hold promoted executable Northstar tasks while
  unresolved or deferred candidates live in `docs/triage/` without execution
  authority.
- `packages/svelte/preview/scripts/lint-docs.ts` (run by `docs:lint`,
  inside `docs:check`) gains `validateRoadmapBacklogRetired`: a future
  `docs/roadmaps/backlog/` directory fails validation, as does any live
  `backlog/` doctrine reintroduced into `docs/roadmaps/README.md`.

## Validation

- `find docs -type d -name backlog -print` returns nothing.
- Inbound-link search finds no live backlog-doctrine link; only this log,
  the queue handoff (closed separately by the plugin), and historical
  evidence name the former path as provenance.
- `docs:lint` (`bun packages/svelte/preview/scripts/lint-docs.ts`) passes.
- `git diff --check` passes.
