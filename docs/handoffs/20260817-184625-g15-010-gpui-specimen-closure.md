---
title: g15.010 GPUI specimen closure worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-17
updated: 2026-08-17
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260817-184625-g15-010-gpui-specimen-closure.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, gpui, specimens]
---

## What This Thread Is Doing

Poodle is closing the last measured GPUI specimen gaps in the v0.2.0 release
baseline. This worker adds curated GPUI preview pages for 18 components whose
Rust specs and `poodle-render` implementations already exist.

This is a specimen and preview-host lane. It is not a component redesign, a
visual-conformance lane, or an excuse to add exhaustive case matrices. Start
from this file without a copied transcript or a second prompt.

## Why It Matters

The GPUI catalogue still falls through to `missing_specimen` for these 18
public components. That makes the native implementation hard to inspect and
leaves the frozen release roster incomplete. The new pages must teach the same
component the approved Svelte pages teach while remaining native, live, and
human-readable.

Specimens are documentation. They show representative use, important states,
and real interaction. They are not conformance reports. Sizes, densities, and
every Cartesian variant do not belong in the primary examples view.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `5a514b3efff3fdcf9fc3f12ad2274413578c9cea`
- **Pushed-main verification:** local `HEAD` and `origin/main` both equalled
  the planning base before this handoff was created
- **Planning checkout:** clean `main`; implementation edits are forbidden there
- **Worker branch placeholder:** `t3code/g15-010-gpui-specimen-closure`
- **Worker worktree placeholder:** `/Users/tom/.t3/worktrees/poodle/g15-010-gpui-specimen-closure`
- **Worker worktree policy:** use the clean registered non-`main` worktree
  supplied by the launcher even when its generated path or branch differs from
  these placeholders. Record the actual values. Do not create a second
  worktree merely to match this file.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready card:** `docs/roadmaps/g15/010-display-workstation-agent-gpui-specimens.md`
- **Allowed runway:** g15.010 only, Batches A through C
- **Dispatch topology:** serial. g15.011 depends on this complete catalogue;
  g15.012 remains later in runway order.
- **Canonical refs:** `AGENTS.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/005-agent-local-paths.md`,
  `docs/roadmaps/g15/release-baseline-roster.md`,
  `docs/roadmaps/g15/release-gap-register.md`,
  `docs/roadmaps/g14/conformance-estate.md`
- **Teaching authority:** the corresponding Svelte component contract and
  `packages/svelte/preview/src/specimens/<Name>Specimen.svelte`; React may be
  used to understand paired controlled behaviour, but neither web preview is
  edited by this card
- **Native implementation authority:** the existing `<Name>Spec` in
  `packages/contracts/components`, renderer in `packages/render`, and current
  GPUI node backend/preview patterns
- **Measured gap:** all 18 named Rust specs/renderers exist; all 18 named GPUI
  specimen files are absent
- **Existing preview bridges:** Avatar, Callout, MetaItem, Pill, Spinner,
  EmptyState, ActionDiscoveryPanel, DockRegion
- **Preview bridges still needed:** RemediationBanner, StateTile,
  AgentMessage, AgentPlan, AgentPlanRecord, AgentQuestionRecord, AgentSubagent,
  ChangedFiles, ToolCall, ToolCallGroup
- **Tool/runtime restrictions:** never run `*-windowed`,
  `test:native-visual`, `qa:jetstream`, or any Jetstream selector. Use only
  Effigy's admitted headless paths.
- **Required validation:** focused touched-crate tests, `effigy check:gpui`,
  `effigy regressions:native`, `effigy docs:check`, one final headless
  `effigy qa`, and `git diff --check origin/main...HEAD`
- **PR base/head:** `main` <- selected worker branch
- **PR URL:** pending
- **Merge authorisation:** none. Push a PR and stop for orchestrator review.

## Exact Scope

### Batch A — display and status

1. Avatar
2. Callout
3. RemediationBanner
4. MetaItem
5. Pill
6. Spinner
7. EmptyState
8. StateTile

### Batch B — workstation

9. ActionDiscoveryPanel
10. DockRegion

### Batch C — agent surfaces

11. AgentMessage
12. AgentPlan
13. AgentPlanRecord
14. AgentQuestionRecord
15. AgentSubagent
16. ChangedFiles
17. ToolCall
18. ToolCallGroup

## Boundaries

- Keep this run inside g15.010.
- Add one clearly named GPUI specimen module and one catalogue route for every
  scoped component. Do not group several roster rows behind one file.
- Compose the existing `poodle-render` output through the node backend. Do not
  hand-paint a lookalike directly in GPUI.
- Preview-local wrappers in `node_compat.rs`, host state/events in
  `app_state.rs`, and catalogue dispatch changes are in scope only where a
  scoped specimen needs them.
- Specs remain controlled data. Preview callbacks request changes; `AppState`
  owns the resulting state and supplies the next spec.
- A callback invoked directly from a Node unit test is not live specimen
  evidence. Interactive examples must work through mounted GPUI keyboard or
  pointer input where the backend supports the interaction.
- Preserve the approved Svelte and React implementations and specimens.
- A renderer defect exposed by the specimen may receive a bounded
  contract-first fix with focused evidence. Stop before any public API change,
  product-policy choice, or architectural expansion.
- Do not add a portable specimen schema, shared corpus, normalized observer,
  screenshot comparator, runtime interface, or new registry beyond the
  existing generated catalogue.
- Do not turn a specimen page into an exhaustive size/density/state listing.
  The later g15.011 audit owns catalogue-wide curation and the separate
  question of how exhaustive reference views should work.
- Jetstream is program-deferred. Do not create a sibling symlink or run it.
- Do not edit g15.011–g15.013, generation status, dispatch, front doors,
  workflows, task definitions, dependency manifests, or release machinery.
- Work only in the selected worker worktree. Never edit, clean, reset, stash,
  or remove the orchestrator checkout or another worker's checkout.
- Do not merge the PR.

## Specimen Standard

Every page should answer three questions quickly:

1. What is this component for?
2. What are its representative states or compositions?
3. If it is interactive, what happens when I use it?

Use the Svelte specimen's information architecture as the teaching reference,
then express it with existing GPUI specimen helpers. Prefer a small number of
separate, honestly labelled surfaces over one dense panel. Avoid repeated
examples that differ only by size or density. Do not copy a web page blindly
when the native binding has a documented intentional difference.

Static components need meaningful visual examples, not artificial callbacks.
Interactive components need live controlled host state:

- Callout and Pill: dismiss/remove where the approved specimen exposes it.
- RemediationBanner: action and dismiss requests.
- ActionDiscoveryPanel: selection and disabled/loading/empty posture as taught
  by the web reference.
- DockRegion: selected tab and collapse state; use the retained headless
  driver rather than inventing input machinery.
- AgentPlan: accept, revise, and dismiss requests where present.
- AgentPlanRecord and AgentSubagent: controlled disclosure.
- ChangedFiles: group disclosure and file selection.
- ToolCall and ToolCallGroup: controlled disclosure at the correct level.

AgentMessage and AgentQuestionRecord are currently display projections in the
native binding; do not manufacture web-only link or answer ownership.

## Evidence Standard

- Registration alone is not evidence. Each route must render the real specimen
  instead of `missing_specimen`.
- Add focused render or mounted regression cases only where new bridge/state
  plumbing could regress behaviour. Do not duplicate existing renderer tests
  merely to inflate counts.
- For controlled interactions, assert the mounted input reaches the preview
  host and that the next rendered spec reflects the stored state.
- Use instance-scoped IDs when several interactive examples share one page.
- Keep accessibility names, roles, expanded state, disabled state, and focus
  destinations intact through the existing node backend.
- Update only the 18 owned GPUI rows/counts in
  `release-baseline-roster.md` and `release-gap-register.md`.
- Write one August batch log naming each page, each interactive proof, every
  intentional native difference, and the exact selectors run.

## Suggested Execution

1. **Orientation:** read all 18 component contracts and their Svelte specimen
   pages; inspect the existing renderer tests and current GPUI specimen/helper
   patterns. Build a short page/proof matrix before editing.
2. **Batch A:** add the eight display/status pages, the two missing preview
   bridges, catalogue routes, and only the focused evidence they need.
3. **Batch B:** add the two workstation pages with live host-owned selection/
   collapse behaviour and mounted evidence.
4. **Batch C:** add the eight agent pages, required preview bridges/host state,
   and mounted disclosure/action evidence.
5. **Closeout:** reconcile the 18 GPUI roster rows, write the batch log, rebase,
   run the final headless validation board, push, and open the PR.

Report after each batch. Name files changed, validation actually run,
remaining work, and any stop condition. Work in coherent batches; do not run
the full suite after every page.

## Completion Protocol

### Before starting

1. Read this handoff, then run one read-only safety probe:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Accept a clean registered non-`main` launcher worktree even when its path or
   branch differs from the placeholders. Do not create another worktree for
   that reason.
3. If the current context is `main`, dirty, or unregistered, stop before broad
   reads and follow `docs/contracts/005-agent-local-paths.md`. Never guess a
   temporary or repository-adjacent worktree path.
4. Fetch origin. Confirm this handoff exists in `HEAD`; confirm
   `git merge-base --is-ancestor 5a514b3efff3fdcf9fc3f12ad2274413578c9cea HEAD`.
   A launcher branch may include this handoff commit beyond the planning base.
5. Read `AGENTS.md`, the repo-local Effigy skill, the g15 README, g15.010, all
   canonical refs above, then the 18 contracts and their Svelte specimens.
6. Run `effigy tasks` for selector routing. Do not guess selector names.

### While working

- Keep commits aligned with the three batches and final evidence closeout.
- Reuse `specimen_layout` and existing node-backed specimen conventions.
- Update the contract first if a bounded native binding defect requires an
  observable decision already implied by the web authority.
- Record small execution friction in `PAPERCUTS.md`. Stop on architecture,
  public API, product policy, security, or unowned web changes.
- Never run a focus-taking/windowed conformance selector locally. If Effigy
  unexpectedly routes a required selector to one, stop and report the routing
  defect.

### Before opening the PR

1. Fetch and rebase onto current `origin/main`, then rerun final validation on
   the rebased head.
2. Run at minimum:
   - focused tests for every touched Rust/preview module
   - `effigy check:gpui`
   - `effigy regressions:native`
   - `effigy docs:check`
   - `effigy qa` once, headlessly
   - `git diff --check origin/main...HEAD`
3. Confirm all 18 catalogue routes render their named specimen and none falls
   through to `missing_specimen`.
4. Update only g15.010's GPUI roster/register rows and write one August log.
   Do not edit roadmap status, dispatch, or generation front doors.
5. Push the selected branch and open a reviewable PR against current `main`.
6. In the PR body, link this handoff, g15.010, the milestone, contracts,
   specimen/proof matrix, validation, and unresolved items.
7. Report the PR URL and pushed SHA. Do not merge.

## Review and Merge Path

The orchestrator will independently inspect scope, page quality, catalogue
registration, preview host state, mounted interactions, roster counts, logs,
and headless checks. The canonical verdict will be a PR comment. The worker
addresses findings and rebases when asked. Only the orchestrator merges after
acceptance.

Closeout authority remains with the orchestrator:

- `docs/roadmaps/g15/010-display-workstation-agent-gpui-specimens.md`
- `docs/roadmaps/g15/README.md`
- `docs/roadmaps/generation-index.md`
- `docs/roadmaps/README.md`
- `docs/roadmaps/dispatch.md`
- `docs/roadmaps/g15/release-baseline-roster.md`
- `docs/roadmaps/g15/release-gap-register.md`
- the scoped component contracts and worker batch log

Leave the card, evidence, roster rows, log, and next-task state honest. If a
stop condition fires, record the exact boundary and stop instead of weakening
the release claim.
