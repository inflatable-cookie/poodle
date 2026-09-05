---
title: g15.041 Popover interactive trigger semantics worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-21
updated: 2026-08-21
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260821-151745-g15-041-popover-interactive-trigger-semantics.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, popover, accessibility, breaking-api]
---

## What This Thread Was Doing

Poodle's human specimen review found that Popover cannot currently compose a
real Button or IconButton without breaking one side of the trigger contract.
The default wrapper creates nested interactive semantics; the web-only
`triggerIsInteractive` mode removes the nesting but also removes the required
`aria-expanded` and `aria-controls` relationship.

Execute `g15.041`: replace that broken interactive-trigger shape with the
operator-approved state-aware composition, add Button's matching `controls`
seam across the active cohort, migrate every current Poodle caller, restore the
paired Popover specimen, and close the evidence blocker.

This is one bounded implementation handoff. You do not need the originating
transcript or a second prompt.

## Why It Matters

Popover is a foundation overlay used by HistoryCenter, MessageCenter, and
UpdateCenter. Its current API forces consumers to choose between nested
controls and missing disclosure semantics. That undermines exactly the
contract-bound parity work g15 is trying to certify, and it leaves the human
catalogue audit blocked before visual conformance and v0.2.0 release.

The migration also establishes the intended web boundary: shared core computes
the semantic trigger state once; Svelte and React project it idiomatically onto
the real control. No DOM patching and no adapter-local state shape.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `c0bc8412f47c95754376b6562f3e5fff4999eb98`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts at the base:** ready card `g15.041`, updated g15 current
  task, and `g15.032` explicitly blocked on this repair.
- **Worker branch:** `t3code/popover-interactive-trigger-semantics`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of generated path or branch
  name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Active spec lane:** g15 release-baseline blocker repair.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:**
  `docs/roadmaps/g15/041-popover-interactive-trigger-semantics.md`
- **Allowed runway:** `g15.041` only.
- **Remaining budget:** one contract/API migration, one August batch log, one
  PR, then stop.
- **Dispatch topology:** serial. This card edits Popover, Button, three current
  composites, the audit, and g15.032 evidence. `g15.033` stays paused until
  this PR is reviewed, live-approved, merged, and closed.
- **Parallel safety:** no other worker may edit this lane's shared component,
  audit, or closeout surfaces.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
  `docs/contracts/components/popover.md`,
  `docs/contracts/components/button.md`, and
  `docs/roadmaps/g15/specimen-catalogue-audit.md`.
- **Finding evidence:**
  `docs/roadmaps/g15/032-review-composition-navigation-overlays.md` and
  `docs/logs/2026-08/20260821-g15-032-navigation-overlays-review.md`.
- **Model capability profile:** frontier coding model, high reasoning. This is
  a public API and accessibility migration.
- **Tool/runtime restrictions:** live Svelte/React previews are allowed; all
  native validation stays headless. Never run `*-windowed`,
  `test:native-visual`, Jetstream, visual-conformance, or release selectors.
- **Required validation:** the exact card validation, ending with
  `effigy qa` and `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation, orchestrator review, and operator
  live sign-off on the paired Popover route.
- **Merge authorisation:** absent. Push the PR and stop for review.

The open `docs/triage/20260820-205249-transitions-dev-motion-learning.md` note
is unrelated and remains open. Do not pull it into this card. Do not run
`effigy doctor`; the card's selectors are explicit and the repository records
known doctor/scan friction separately.

## Boundaries

Implement the card's fixed API exactly:

- core publicly authors one readonly `PopoverTriggerState` with exactly
  `expanded: boolean`, `controls: string | null`, and `disabled: boolean`;
- `popoverParts` returns that payload beside its existing parts;
- Svelte interactive mode requires a one-argument state snippet;
- React interactive mode requires a state render function;
- default mode keeps the current zero-argument Svelte snippet / React node;
- interactive wrappers keep layout/data hooks and bubbled-click observation,
  but no role, tab stop, keyboard handler, disabled semantics, or disclosure
  ARIA;
- the real trigger receives all three state fields;
- Button adds `controls` beside `ariaExpanded` in paired web and
  `ButtonSpec.controls` / `with_controls` in Rust, projecting to the existing
  `NodeA11y.controls` field.

Migrate all current production `triggerIsInteractive` callers in both web
runtimes: HistoryCenter, MessageCenter, and UpdateCenter. Also migrate every
retained test, fixture, package-install consumer, and specimen found by the
final repository search. Preserve their visible composition and behavior.

Writable scope:

- Popover and Button contracts;
- Popover core parts/state, paired adapters, framework root type exports,
  focused tests, and paired specimen;
- Button paired adapters, portable spec, shared renderer projection, focused
  active-cohort evidence, and required public exports;
- the six named composite implementations and migration evidence;
- generated docs/artifacts only through their canonical generator;
- `CHANGELOG.md`, the Popover audit row/totals, the existing g15.032 log, one
  g15.041 August log, and `PAPERCUTS.md` for new execution friction.

Out of scope:

- compatibility shims, aliases, deprecated overloads, runtime detection, or
  silent fallback for the old interactive-trigger shape;
- React child cloning, Svelte/DOM action mutation, post-mount ARIA projection,
  or a generic `asChild` abstraction;
- other overlay components, new tokens/styles, exhaustive specimens, or a
  repository-wide id architecture;
- GPUI platform accessibility claims, Jetstream runtime work, visual
  conformance, `g15.033`, release mutation, or external-repository edits;
- roadmap/card status, generation front doors, and the dispatch ledger;
- merging the PR.

Stop on any condition in the card. In particular, stop rather than widening
the design if Svelte cannot check the discriminated snippet contract, SSR-safe
identity needs a repository-wide redesign, the fixed three-field payload cannot
serve an existing caller, or focus restoration needs a new public ref contract.

## Important Context

- The breaking migration is already approved. Do not ask whether to preserve
  the old API, and do not preserve it.
- Shared core is the TypeScript state authority. The Svelte and React packages
  re-export its type; they do not declare structurally similar local copies.
- `controls` is `null` while closed and the exact rendered surface id while
  open. This preserves Popover's existing conditional `aria-controls`
  contract.
- The default wrapper path is not being redesigned. It remains the convenient
  text/non-interactive trigger path and owns its existing role/keyboard/ARIA.
- Focus restoration may still query the actual interactive descendant. The
  prohibition is on using DOM queries or effects to project semantic
  attributes.
- IconButton already has the desired paired-web/Rust `controls` seam and
  shared render projection. Use it as the Button precedent; do not invent a
  second vocabulary.
- React already uses framework-native `useId` in many components. Keep any
  Popover identity repair local and prove server output. Apply the equivalent
  bounded Svelte approach supported by the current toolchain.
- Server evidence must exercise both closed and initially open output. Client
  evidence must inspect the actual inner trigger, not merely the wrapper.
- Existing composites include decorated trigger hosts: MessageCenter badges
  and UpdateCenter progress. Keep decoration non-interactive and thread state
  only into the actual Button/IconButton/native button.
- The audit's exact post-repair counts are fixed in the card. Recount from rows
  and change them only after all implementation and live evidence supports the
  A/A/A `keep` grade.
- The worker may update the audit and execution logs, but the orchestrator owns
  card/roadmap status and final g15.032 closure after merge.
- The operator must review the changed Svelte and React Popover routes live.
  List both URLs/routes in the PR report and leave sign-off pending.
- Run a read-only `~/Dev/projects` search for downstream
  `triggerIsInteractive` use. Record affected repositories and migration work;
  never edit them from this Poodle worker.

Work in three meaningful chunks:

1. contract/core/API shape plus focused core/SSR type evidence;
2. paired Popover/Button implementation and all in-repo caller migration;
3. specimen, package/native evidence, audit/log/changelog, and full headless
   validation.

Report after each chunk with changed files, validation actually run, remaining
work, and blockers.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, read the
ready card and canonical refs, use `effigy tasks` to confirm the named
selectors, and inspect the exact Popover/Button implementations and all
`triggerIsInteractive` callers.

Update the contracts first. Then make core return the fixed state payload and
prove it directly before changing either framework adapter. That gives the two
web runtimes one executable semantic source and makes adapter drift visible in
their focused tests.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad read, run only:
   - `git rev-parse --show-toplevel`
   - `git branch --show-current`
   - `git status --porcelain`
   - `git worktree list --porcelain`
2. If the current root is a registered, clean, non-`main` worktree, accept it
   as the launcher-provided worktree regardless of generated path or branch
   name. Record the actual values and do not create another worktree.
3. If the launcher supplied a dirty, `main`, or unregistered context, stop and
   report it. Do not clean or reset it. A manual fallback is allowed only after
   reading `.agents.local.env`, finding a valid
   `AGENTS_WORKTREE_CONTAINER_DIR`, and creating a unique worktree there from
   `origin/main`; ask the operator if the key is absent. Never use `/tmp`,
   `TMPDIR`, or a guessed path.
4. From the accepted worktree, run `git fetch origin`, confirm `HEAD` equals
   current `origin/main`, confirm
   `git merge-base --is-ancestor c0bc8412f47c95754376b6562f3e5fff4999eb98 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.032`, `g15.041`, both component contracts, spec 022, the audit row,
   and the g15.032 log.
6. Use `effigy tasks` to confirm selectors. Do not run `effigy doctor` or any
   windowed, native-visual, Jetstream, visual-conformance, or release path.

### While you work

- Change the Popover and Button contracts before observable implementation.
- Keep the API and payload exact. Stop rather than adding extra state fields,
  a generic composition abstraction, or a compatibility path.
- Add failing focused evidence for the current nested/missing-ARIA defect, then
  make it pass in both frameworks and server output.
- Migrate every repository caller found by `rg`; do not rely only on the six
  production files named at planning time.
- Keep Button's Rust change on the existing spec → shared render → node a11y
  path, with no component-specific GPUI escape hatch.
- Keep Examples human-centred and alter only the paired Popover specimen needed
  to demonstrate normal real-Button composition.
- Update generated material only by running its owning generator/check.
- Record the public package classifications and downstream search in both the
  changelog migration note and g15.041 log as the card requires.
- Append a PAPERCUTS entry only for newly encountered small execution friction.
- Stop and report any card stop condition or validation result that changes
  the plan.

### When the assigned runway is complete

1. Run the card's complete headless validation list. Finish with
   `effigy qa` and `git diff --check origin/main...HEAD`.
2. Confirm no old static/zero-argument interactive-trigger call remains and no
   compatibility route accepts it.
3. Confirm the packed Svelte and React roots expose the shared type and mount
   the state-aware Popover with Button `controls`.
4. Confirm the audit recount matches the card exactly and the logs do not claim
   operator sign-off.
5. List the changed Svelte and React Popover routes for live operator review.
6. Push the worker branch and open one reviewable PR against current `main`.
   The handoff's planning base is the pre-handoff commit, not the commit that
   contains this file.
7. In the PR body, link g15.032, g15.041, both contracts, the audit, g15.041
   log, changed public surfaces, migration note, downstream search, and every
   validation result.
8. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect the PR independently, rerun the required checks,
and open the paired Popover routes for operator review. Because the
orchestrator and worker may share a GitHub identity, the verdict may be a PR
comment rather than formal approval.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation after code
review, checks, and live operator sign-off.

- **Requested changes:** none yet.
- **Closeout refs:**
  `docs/roadmaps/g15/041-popover-interactive-trigger-semantics.md`,
  `docs/roadmaps/g15/032-review-composition-navigation-overlays.md`,
  `docs/roadmaps/g15/specimen-catalogue-audit.md`, the g15.032 and g15.041
  August logs, `CHANGELOG.md`, `docs/roadmaps/g15/README.md`,
  `docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns implementation, migration, focused evidence, the audit row,
changelog entry, and batch log. The orchestrator owns live review, merge,
card/roadmap status, g15.032 closure, and promotion of g15.033. Leave the lane
open if any semantic proof, package proof, downstream record, or operator
checkpoint remains.
