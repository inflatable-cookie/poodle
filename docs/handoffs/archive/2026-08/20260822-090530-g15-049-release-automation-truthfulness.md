---
title: g15.049 release automation truthfulness worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260822-090530-g15-049-release-automation-truthfulness.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, release, ci, effigy, security]
---

## What This Thread Was Doing

Poodle is closing the release-automation gap before the v0.2.0 candidate.
Several manual workflows still hand-transcribe commands, the native workflow
names a deleted crate path, the retained conformance workflow describes the
removed g14 pilot, and `effigy release gates` currently succeeds with zero
configured gates.

Execute `g15.049` only. Make the retained manual workflows thin, immutable
launchers for current Effigy selectors; delete the stale conformance workflow
and alias; bind `effigy release gates` to the complete self-contained headless
QA board; harden the human-dispatched release workflow without publishing,
tagging, or changing the package set.

This is one worker handoff. You do not need the originating transcript or a
second prompt.

## Why It Matters

A green release command that executes no gates is worse than no command: it
creates false confidence at the exact point where Poodle's consumers are
waiting on v0.2.0. The repository already has truthful named selectors. This
card removes the duplicate command copies and makes one reviewed headless board
the release authority while retaining a deliberate human publication act.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `8d8276df2a0fd29622201aa23ddb028bed55c328`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Worker branch:** `t3code/g15-049-release-automation-truthfulness`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of its generated path or
  branch name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Ready card:**
  `docs/roadmaps/g15/049-release-automation-truthfulness.md`.
- **Allowed runway:** `g15.049` only.
- **Remaining budget:** one workflow/config batch, one August log, one PR,
  then stop.
- **Dispatch topology:** parallel with `g15.046` only. `g15.046` owns Button
  fixture data and focused validators and is explicitly forbidden from
  editing Effigy config, task declarations, or workflows. Do not touch its
  test-data or GPUI test surfaces.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/roadmaps/g15/049-release-automation-truthfulness.md`,
  `docs/roadmaps/g15/release-gap-register.md`,
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
  `docs/contracts/001-working-rules.md`, `effigy.toml`,
  `tasks/effigy.tasks.toml`, `packages/release-operations.json`, and the six
  current workflow files named by the card.
- **External authority:** current official GitHub immutable-action guidance at
  <https://docs.github.com/en/actions/reference/security/secure-use> and npm
  trusted-publishing guidance at <https://docs.npmjs.com/trusted-publishers/>.
  Re-check both while implementing; do not rely on a blog or action mirror.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** static workflow validation and supported
  headless selectors only. Never dispatch a workflow, run a windowed/native-
  visual or Jetstream selector, create/push a tag, publish a package, or run
  `effigy release prepare`/`execute`.
- **Required validation:** `actionlint .github/workflows/*.yml`, focused static
  assertions, retained local selectors supported by the host, one final
  `effigy release gates`, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review.
- **Merge authorisation:** absent. Push the PR and stop for review.

The motion-learning and Longhorn conformance-lab triage notes remain open and
out of scope. `g15.050` remains blocked on the other release gaps even after
this card lands.

## Fixed Decisions

Implement these decisions. Do not reopen them inside this worker lane:

1. Retained workflow-to-selector mapping:
   - `ci-web.yml` runs `effigy ci:web`;
   - `ci-rust.yml` runs `effigy ci:rust`;
   - `ci-native.yml` runs `effigy ci:native`;
   - `ci-visual.yml` maps its explicit `smoke`, `axis`, and `sweep` input to
     `effigy test:visual-smoke`, `effigy ci:visual`, and
     `effigy test:visual-sweep` respectively, rejecting any other value.
2. Delete `.github/workflows/ci-conformance.yml`. Remove the legacy
   `ci:conformance` task alias and its stale explanatory comments. Its retained
   native regressions already belong to `ci:native`.
3. Add the one release gate below and raise the manifest's minimum Effigy
   version if required by the current release-gate grammar:

   ```toml
   [release.gates.headless]
   command = "effigy qa"
   description = "Run Poodle's complete self-contained headless release board"
   ```

   The release workflow invokes `effigy release gates`; it must not maintain a
   second partial list beside that gate.
4. Keep all CI workflows manual. Keep `release.yml` human-dispatched against a
   tag and dry-run by default. Publication remains core + Svelte only. React is
   packed/certified elsewhere but remains unpublished for v0.2.0.
5. Pin every retained `uses:` reference to a reviewed full commit SHA and add
   its release tag in a comment. Pin Bun `1.3.14`, Effigy `0.11.0`, and the npm
   CLI used for trusted publishing to exact reviewed versions. Preserve
   `contents: read`, job-local `id-token: write`, OIDC, and no `NPM_TOKEN`.
6. Do not configure Effigy version mutation. Multi-manifest lockstep version
   changes belong to `g15.050`.

Starting action evidence gathered by the orchestrator on 2026-08-22 follows.
Resolve each official tag again and verify the commit belongs to the official
repository before using it:

- `actions/checkout` v7.0.1:
  `3d3c42e5aac5ba805825da76410c181273ba90b1`
- `actions/setup-node` v7.0.0:
  `820762786026740c76f36085b0efc47a31fe5020`
- `actions/cache` v6.1.0:
  `55cc8345863c7cc4c66a329aec7e433d2d1c52a9`
- `actions/upload-artifact` v7.0.1:
  `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a`
- `oven-sh/setup-bun` v2.2.0:
  `0c5077e51419868618aeaa5fe8019c62421857d6`
- `inflatable-cookie/setup-effigy` v1 current release commit:
  `987fd556617ea2c3e0ab5cef6b47b250817f50c8`
- `dtolnay/rust-toolchain` stable current commit:
  `4360b52568e2003a75bf9bc1d59f33a8e3fc893c`

If a current official release differs, use the current reviewed immutable SHA
and record the source. Do not silently use an old SHA merely because it is in
this handoff.

## Boundaries

In scope:

- the five retained workflows named by the card;
- deletion of the stale conformance workflow;
- `effigy.toml` release-gate configuration and minimum-version truth;
- removal of the `ci:conformance` alias from `tasks/effigy.tasks.toml`;
- narrow static evidence that retained workflows map to exact selectors, use
  no mutable action refs, name no deleted package paths, and introduce no
  registry token;
- release/pre-tag operator prose required to match the implemented path;
- one August `g15.049` execution log with actual action/tool versions,
  selectors, validation, and explicit no-mutation statement;
- root `PAPERCUTS.md` only for newly encountered small execution friction.

Out of scope:

- component, specimen, token, fixture, capture, comparator, baseline, Rust
  implementation, GPUI presentation-context, or Jetstream changes;
- package/crate version changes, lockfile changes, release notes, tags,
  registry publication, trusted-publisher account configuration, secrets, or
  changes to the publish set;
- automated publication triggers or broadening any workflow beyond manual
  dispatch;
- `g15.046`, `g15.047`, `g15.050`, or `g15.013` implementation;
- `effigy release prepare`, `effigy release execute`, `npm publish`, or any
  GitHub workflow dispatch;
- merging the PR.

Stop and report if an exact fixed selector cannot run on the workflow's stated
OS, if `effigy release gates` cannot call the configured headless board without
recursion or mutation, if current official guidance conflicts with the fixed
OIDC posture, if the package publish set would change, or if a workflow outside
the fixed retained/deleted set must change.

## Important Context

- `ci:native` is self-contained. It includes Poodle-owned GPUI checks, native
  regressions, specimen construction, and the in-repo Jetstream adapter. It
  does not require the sibling Jetstream engine and must remain headless.
- `ci:visual` is a paired-web browser lane and is not part of `effigy qa`.
  Keep its tier input explicit and install the browser/runtime prerequisites
  its selectors need; do not inline the visual runner command.
- `qa` is the intended complete self-contained release board: web, Rust,
  native, packed install, licences, and security. Native visual and paired
  Jetstream remain separate by design.
- The release workflow currently runs `effigy ci` plus a separate packed-
  consumer step. Replace the gate authority with `effigy release gates`; do
  not leave competing prose that calls the narrower command complete.
- Preserve the pack-and-inspect stage and no-publish dry-run. It is artifact
  evidence, not a substitute gate.
- Trusted publishing requires a sufficiently current Node/npm combination.
  The orchestrator observed npm `12.0.2` as current on 2026-08-22; verify the
  exact chosen version against npm's official requirements before pinning it.
- `actionlint` is installed locally. YAML syntax alone is insufficient: add a
  focused repository-level static check if that is the smallest durable way to
  prevent mutable refs, stale commands, or zero-gate drift from returning.
- Do not test the workflow by dispatching it. Local selectors, `actionlint`,
  static inspection, and dry-run packing are the evidence available here.

## Suggested Next Move

Read this handoff from the top, run the four-command worktree preflight, then
read the card and current Effigy/workflow surfaces. Confirm current official
action SHAs and npm trusted-publishing requirements before editing.

Land the Effigy gate/alias cleanup and static regression first. Then convert
the retained CI workflows one at a time to thin selector launchers. Harden the
release workflow last, so its single gate points at an already-proved local
configuration. Run the broad headless release gate once, after the batch is
complete.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad read, run only:
   - `git rev-parse --show-toplevel`
   - `git branch --show-current`
   - `git status --porcelain`
   - `git worktree list --porcelain`
2. Accept a clean, registered, dedicated non-`main` worktree supplied by the
   launcher. If it is dirty, `main`, or unregistered, stop. Do not clean or
   reset it.
3. A manual fallback is allowed only after reading `.agents.local.env` and
   finding the operator-selected `AGENTS_WORKTREE_CONTAINER_DIR`. Never use
   `/tmp`, `TMPDIR`, or a guessed repository-adjacent path.
4. Run `git fetch origin`; confirm `HEAD` equals current `origin/main`; confirm
   `git merge-base --is-ancestor 8d8276df2a0fd29622201aa23ddb028bed55c328 HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the ready card, release
   gap register, release/package spec, working rules, current Effigy config,
   task declarations, release operations policy, and all six current workflow
   files.
6. Run `effigy tasks` and `effigy release gates` once before editing. Record
   that the latter is currently vacuous; do not treat its exit zero as proof.

### While you work

- Keep workflows thin: setup, cache/install prerequisites, one named Effigy
  selector, and artifact upload where the workflow genuinely owns an artifact.
- Verify every full action SHA from the official repository and record the tag
  plus source in the August log.
- Keep visual tier selection explicit and fail closed on unknown input.
- Add narrow static evidence before deleting the stale workflow/alias, then
  prove the intended state passes.
- Preserve release dry-run, tag agreement, artifact inspection, OIDC, and the
  existing two-package publish loop.
- Run focused validation while editing. Save `effigy release gates` for one
  final complete pass instead of repeatedly running the full board.
- Stop on every condition named by the card or this handoff.

### When the assigned runway is complete

1. Run `actionlint .github/workflows/*.yml` and the focused static assertions.
2. Prove the retained workflows invoke only the fixed selectors, contain no
   mutable `uses:` refs, stale deleted GPUI path, `ci:conformance`,
   `NPM_TOKEN`, `npm@latest`, `bun-version: latest`, or Effigy 0.9.1.
3. Run the retained selectors supported on the current host. Run
   `effigy release gates` once as the final non-vacuous read-only release board
   and record the named gate plus result. Never substitute a zero-gate pass.
4. Run `effigy docs:check` and finish with
   `git diff --check origin/main...HEAD`.
5. Confirm no package/crate version, lockfile, tag, release note, publication,
   component, specimen, fixture, capture, native implementation, Jetstream
   engine, or unrelated workflow change exists.
6. Push the worker branch and open one reviewable PR against current `main`.
   The planning base above is the pre-handoff commit, not the commit containing
   this file.
7. Link `g15.049`, the release gap register, changed workflows/config, static
   evidence, August log, official security sources, validation, and
   continuation to `g15.050` in the PR body.
8. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will independently inspect selector mapping, release-gate
non-vacuity, immutable action evidence, OIDC/permission posture, publish-set
preservation, static regression coverage, local checks, and GitHub checks.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation after review.

- **Requested changes:** none yet.
- **Closeout refs:** the `g15.049` card, its August execution log,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/g15/release-gap-register.md`,
  `docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.

### Handoff closeout

Once the PR is merged, the orchestrator owns runway/log/dispatch closeout and
the advance decision. The worker does not mark `g15.049` complete on `main`.
