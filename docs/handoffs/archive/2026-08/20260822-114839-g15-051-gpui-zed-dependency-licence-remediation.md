---
title: g15.051 GPUI/Zed dependency licence remediation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260822-114839-g15-051-gpui-zed-dependency-licence-remediation.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, gpui, zed, licence, security]
---

## What This Thread Was Doing

Poodle's truthful release board found a real native distribution blocker. The
pinned Zed graph brings GPL-3.0-or-later `zlog`, `ztracing`, and
`ztracing_macro` into GPUI's normal dependency graph. The board also rejects
the permissive bzip2 licence and every Git source because the repository has
not yet recorded the reviewed licence/source policy.

The operator accepted the orchestrator's recommendation: allow bzip2 with its
notice, refuse GPL exceptions, remove the GPL graph through one minimal
Poodle-owned Zed fork, and pin every Git source exactly. Execute `g15.051`
only. This handoff contains the complete worker boundary; no copied transcript
or second prompt is needed.

## Why It Matters

Poodle cannot call the native release permissive while its distributed GPUI
graph includes strong-copyleft tracing crates. Simply adding exceptions would
silence the gate without resolving the release claim. This batch makes the
graph and the policy agree, then proves that agreement through the same
fail-closed board that exposed the problem.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `24d6d80c8eedf92b78d23a259d94d7edfa3b5cb1`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** spec 022's native licence/source
  rule, ready card `g15.051`, release-gap ownership, and `g15.050` dependency.
- **Worker branch:** `t3code/g15-051-gpui-zed-dependency-licence-remediation`.
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of its generated path or
  branch name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Ready card:**
  `docs/roadmaps/g15/051-gpui-zed-dependency-licence-remediation.md`.
- **Allowed runway:** `g15.051` only.
- **Remaining budget:** one external fork patch, one Poodle policy/adoption
  batch, one August log, one PR, then stop.
- **Dispatch topology:** serial implementation lane. `g15.043` and `g15.047`
  are not ready and must not be pulled into this work.
- **Parallel safety check:** no parallel worker is currently authorised.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
  `docs/roadmaps/g15/051-gpui-zed-dependency-licence-remediation.md`,
  `docs/roadmaps/g15/release-gap-register.md`,
  `docs/contracts/001-working-rules.md`, `deny.toml`,
  `scripts/audit-repository-security.ts`,
  `scripts/audit-license-compliance.ts`, and the two GPUI manifests/lockfiles
  named by the card.
- **External authority:** SPDX's
  [`bzip2-1.0.6`](https://spdx.org/licenses/bzip2-1.0.6.html), the
  [GNU GPL FAQ](https://www.gnu.org/licenses/gpl-faq.en.html), and
  [Zed issue #55470](https://github.com/zed-industries/zed/issues/55470).
- **Model capability profile:** frontier coding/review model, high reasoning;
  this touches distribution policy and an external fork.
- **Tool/runtime restrictions:** headless checks only. Never run a
  `*-windowed`, native-visual, paired-Jetstream, release-prepare,
  release-execute, tag, publish, or workflow path.
- **Required validation:** focused fork check; exact dependency-graph proof;
  preview `cargo deny`; `effigy audit:licenses`; `effigy audit:security`;
  `effigy ci:native`; `effigy smoke:gpui-offscreen-capture`;
  `effigy docs:check`; one final `effigy release gates`; and
  `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review.
- **Merge authorisation:** absent. Push the PR and stop for review.

The `inflatable-cookie/zed` repository did not exist when this handoff was
written. Creating it as a fork is the one authorised external mutation. Do not
open an upstream PR, comment on or close the upstream issue, publish a release,
or alter any other external repository.

## Boundaries

In scope:

- create `inflatable-cookie/zed` as a fork of `zed-industries/zed`;
- create a branch based on exact upstream commit
  `1ea16c1ab9dd6d36649e002dc60995634da04daf` and push one minimal patch commit;
- in upstream `crates/gpui` and `crates/sum_tree`, replace normal
  `ztracing::instrument` use with standard `tracing::instrument` and remove
  their normal `ztracing` dependency declarations;
- leave test-only `zlog` alone when it does not enter Poodle's resolved graph;
  remove it only if focused fork validation requires the equally narrow
  cleanup;
- point Poodle's GPUI node-backend and preview to the exact resulting fork
  commit and update only their lockfiles;
- admit `bzip2-1.0.6`, preserve its redistribution notice, and add only the
  five fixed Git repositories to the source policy;
- make the security audit check exact manifest and lockfile URL/revision pairs;
- add focused negative evidence proving unknown URLs, mutable refs, changed
  revisions, GPL licences, and missing notice markers still fail where the
  existing test structure supports it;
- write one `docs/logs/2026-08/20260822-g15-051-*.md` execution record.

Out of scope:

- GPL licence/package exceptions, `unknown-git` relaxation, or moving Git refs;
- unrelated Zed changes, rebases to newer Zed, feature changes, renderer/API
  changes, refactors, or upstream coordination;
- Poodle components, contracts, tokens, specimens, themes, visual baselines,
  package versions, release notes, workflows, Jetstream integration, or
  unrelated dependencies;
- `g15.043`, `g15.047`, `g15.050`, or `g15.013` implementation;
- release preparation/execution, tags, publication, merging the PR.

Stop and report if the `inflatable-cookie` owner cannot create/push the fork,
the patch needs anything beyond the named tracing substitution, a GPL crate
remains in either Poodle graph, a sixth Git repository appears, notice terms
are unclear, or a required proof needs a window, focus, workflow edit, or
release mutation.

## Important Context

- Poodle currently pins upstream Zed commit
  `1ea16c1ab9dd6d36649e002dc60995634da04daf` in
  `packages/gpui/node-backend/Cargo.toml` and
  `packages/gpui/preview/Cargo.toml`. Preserve that implementation base.
- The exact upstream patch sites observed at planning time are:
  - `crates/gpui/Cargo.toml` and two annotations in
    `crates/gpui/src/svg_renderer.rs`;
  - `crates/sum_tree/Cargo.toml`, imports/annotations in
    `crates/sum_tree/src/cursor.rs`, and imports/annotations in
    `crates/sum_tree/src/sum_tree.rs`.
- `tracing` already exists in the Zed workspace. The fork is a dependency
  substitution, not new tracing machinery.
- Current reviewed transitive Git revisions are:
  - `zed-industries/font-kit` at
    `94b0f28166665e8fd2f53ff6d268a14955c82269`;
  - `zed-industries/scap` at
    `4afea48c3b002197176fb19cd0f9b180dd36eaac`;
  - `zed-industries/wasm_thread` at
    `0cf96c7708dfb97ccf3da50347e25edcf75d6937`;
  - `proptest-rs/proptest` at
    `3dca198a8fef1b32e3a66f1e1897c955b4dc5b5b`.
  Reconcile these against the regenerated lockfiles. Stop on any unplanned
  revision change instead of widening approval.
- Keep `deny.toml` fail-closed: add `bzip2-1.0.6` to the permissive allowlist,
  add the five exact Git repository URLs to `allow-git`, retain
  `unknown-git = "deny"`, and retain `required-git-spec = "rev"`.
- `cargo-deny` approves repository URLs and full-revision syntax. The security
  audit owns the stricter exact URL-plus-revision assertion across manifests
  and lockfiles.
- The root third-party notice already carries Lucide and Inter. Add bzip2 from
  the authoritative upstream licence and extend the existing compliance audit.
  If a public native package will not carry that root notice, add the smallest
  truthful native notice surface and prove its packaging path.
- A discarded PR #66 attempt contained GPL exceptions. It is not authority and
  must not be restored. Rebuild the exact source policy from this card.
- `effigy release gates` is broad. Run it once, only after focused checks pass.
  It must become green through the graph change, never through a bypass.

## Suggested Next Move

Read this handoff from the top and run the four-command worktree preflight
below before broad repository reads. Confirm the Poodle base, then inspect the
exact upstream files at the pinned Zed commit.

Create the fork and minimal patch first. Record its upstream base and resulting
commit before touching Poodle. In Poodle, update both manifests and lockfiles,
prove the GPL crates disappear, then harden `deny.toml`, notice evidence, and
the exact source/revision audit. Run focused checks before the single broad
release-gate pass.

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
   launcher, regardless of generated path or branch-name differences. Record
   its actual path and branch; do not create another worktree. If the launcher
   supplied `main`, a dirty checkout, or an unregistered checkout, stop and
   report it. Never clean, reset, stash over, or discard it.
3. A manual fallback is allowed only after reading `.agents.local.env` and
   finding the operator-selected `AGENTS_WORKTREE_CONTAINER_DIR`. Never use
   `/tmp`, `TMPDIR`, or a guessed repository-adjacent path.
4. Run `git fetch origin`; confirm `HEAD` equals current `origin/main`; confirm
   `git merge-base --is-ancestor 24d6d80c8eedf92b78d23a259d94d7edfa3b5cb1 HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the ready card, spec 022,
   release-gap register, working rules, `deny.toml`, both GPUI manifests and
   lockfiles, both audit scripts, and root third-party notice.
6. Run `effigy tasks`, then capture the focused pre-change failures from
   `effigy audit:licenses`, `effigy audit:security`, and exact graph queries.
   Do not run the full release board yet.

### While you work

- Create the external fork only after confirming the target owner and upstream
  base. Keep its patch commit separate and reviewable.
- Use standard `tracing::instrument`; do not delete instrumentation or replace
  it with local macros.
- Prove the fork diff contains only the card's writable files and the exact
  tracing substitution.
- Pin the fork by full commit SHA in every Poodle declaration. Never pin the
  fork branch.
- Derive the exact source/revision audit data from reviewed manifests and
  lockfiles. Do not accept arbitrary Git lines or prefix matches.
- Copy the bzip2 notice terms faithfully and keep notice validation executable.
- Run focused validation after the fork, graph, policy, and notice chunks.
  Save the broad release gate for the completed batch.
- Stop on every condition named by the card or this handoff.

### When the assigned runway is complete

1. Record the fork URL, upstream base SHA, fork branch, patch SHA, changed
   upstream files, and exact diff summary in the August execution log.
2. Prove the node-backend and preview normal graphs contain no `zlog`,
   `ztracing`, `ztracing_macro`, GPL-3.0-or-later, or unapproved licence.
3. Run
   `cargo deny --manifest-path packages/gpui/preview/Cargo.toml check licenses sources`,
   `effigy audit:licenses`, `effigy audit:security`, `effigy ci:native`,
   `effigy smoke:gpui-offscreen-capture`, and `effigy docs:check`.
4. Run `effigy release gates` once as the final broad read-only board. Record
   the named gate and exact result. Do not mutate a release.
5. Run `git diff --check origin/main...HEAD`. Confirm no workflow, version,
   release-note, tag, package-publication, component, specimen, visual,
   Jetstream, or unrelated dependency change exists.
6. Push the worker branch and open one reviewable PR against current `main`.
   The planning base above is the pre-handoff commit, not the commit containing
   this handoff.
7. In the PR body, link `g15.051`, spec 022, release-gap register, the external
   fork commit, exact source/licence evidence, August log, validation, and the
   continuation state. Report the PR URL to the operator. Do not merge.

### Review and merge path

The orchestrator will independently review the external fork diff, ancestry,
Poodle manifest/lock changes, resolved graphs, licence/source/notice policy,
security audit strictness, headless regression evidence, and GitHub checks.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation after review.

- **Requested changes:** none yet.
- **Closeout refs:** `g15.051`, its August log, `docs/roadmaps/g15/README.md`,
  `docs/roadmaps/g15/release-gap-register.md`,
  `docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.

### Handoff closeout

Once the PR is merged, the orchestrator owns runway/log/dispatch closeout and
the advance decision. The worker does not mark `g15.051` complete on `main`.
