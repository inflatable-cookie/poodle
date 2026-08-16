# 005 — Agent Local Paths and Manual Worktree Locations

Status: active
Owner: Poodle core
Updated: 2026-08-16
Depends on: `docs/contracts/001-working-rules.md`
Affects: `AGENTS.md`, `docs/handoffs/`, and manual worker launch procedures

## Purpose

Agents sometimes need machine-local paths that must not be committed. Poodle
keeps those values in one predictable path-only registry, separate from the
planning spine and from credentials.

## Files

- `.agents.local.env.example` is tracked and documents supported keys.
- `.agents.local.env` is local-only, ignored by Git, and created on first need.
- `AGENTS.md` is the required discovery point for this contract.

Use a harness-provided location when one exists. Do not require the local file
during normal harness-managed operation.

## Supported Keys

| Key | Required when | Meaning |
| --- | --- | --- |
| `AGENTS_WORKTREE_CONTAINER_DIR` | A manual worktree is required | Absolute directory containing agent-created worktrees. |
| `AGENTS_SCRATCH_DIR` | Shared scratch space is useful | Optional absolute directory for temporary reports and non-repository scratch files. |
| `AGENTS_ARTIFACT_DIR` | Large local outputs should stay outside the repo | Optional absolute directory for generated artifacts that are not committed. |

Only path-valued, namespaced `AGENTS_*` keys belong in the file. Never add API
keys, tokens, passwords, connection strings, credentials, or commands. Parse
entries as data; never execute or source the file.

## Manual Worktree Procedure

1. Prefer a worktree supplied by the harness or parent orchestrator. Do not
   create a second location when another owner already manages its lifecycle.
2. Before creating a manual worktree, read `.agents.local.env`.
3. If `AGENTS_WORKTREE_CONTAINER_DIR` is absent, empty, non-absolute, or points
   inside the repository, stop and ask the operator:

   > What absolute directory should this repository use as its manual worktree
   > container? I will store it in untracked `.agents.local.env` as
   > `AGENTS_WORKTREE_CONTAINER_DIR=...` and use a separate subdirectory per
   > repository and lane.

4. After the operator answers, create the ignored local file, then create or
   validate the container directory.
5. Create worktrees below it using
   `<container>/<repository-slug>-<lane-slug>`. Never guess `/tmp`, `TMPDIR`, a
   repository child, or a repository-adjacent path.
6. If the location cannot be validated or created, stop and report the
   boundary failure.

## Nested-Agent Boundary

A worker or subagent must not start another orchestrator workflow, dispatch a
new worker, or create a nested worktree unless the operator explicitly assigns
that separate lane and this path contract is satisfied.

## Handoff Requirements

Each worker handoff states whether its worktree is harness-managed or manual.
A manual fallback uses the operator-selected container and never a hard-coded
temporary path. If the worker cannot establish a clean, dedicated, non-`main`
worktree, it stops before editing.
