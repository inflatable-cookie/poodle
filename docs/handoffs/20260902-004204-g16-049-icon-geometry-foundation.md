---
title: g16.049 icon geometry foundation worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-004204-g16-049-icon-geometry-foundation.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, icon-geometry]
---

## What This Thread Was Doing

Poodle recorded a renderer-neutral icon-geometry programme after the native
feasibility spike. This handoff dispatches only `g16.049`: the deterministic
24×24 stroke-geometry format, paired TypeScript/Rust normalizers, and curated
generated registry foundation. It does not dispatch the runtime substrate,
native visual admission, or public IconMorph work.

This is one bounded implementation lane. No transcript or second prompt is
part of the authority chain.

## Why It Matters

Future icon continuity needs one reproducible geometry and provenance layer
before any renderer receives a path frame. This card establishes that internal
truth while keeping the current Icon, provider, nodes, packages, and public API
unchanged. Once this foundation is merged, orchestrator review may continue
with `g16.050`; candidate geometry remains fixture-only and this card does not
admit a pair, component, or visual capability.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `7f59ae42f4917c675968819eb23a5e41dc90013c`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `7f59ae42f4917c675968819eb23a5e41dc90013c` before this handoff was drafted
- **Planning checkout:** clean before these uncommitted handoff drafts
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** merged PR #148, `g16.049`, the
  canonical continuation runway, and the operator-approved icon-geometry programme
- **Worker branch:** `feat/g16-049-icon-geometry-foundation`
- **Worker worktree:** `/Users/tom/.t3/worktrees/poodle/g16-049-icon-geometry-foundation`
- **Worktree creation command:** fallback only:
  `git worktree add /Users/tom/.t3/worktrees/poodle/g16-049-icon-geometry-foundation -b feat/g16-049-icon-geometry-foundation origin/main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required
- **Required sibling worktree links:** none
- **Active spec lane:**
  `docs/roadmaps/g16/component-continuation-runway.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/049-icon-geometry-format-and-registry-foundation.md`
- **Allowed runway:** `g16.049` only
- **Remaining card budget:** one card
- **Dispatch topology:** independent ready frontier beside `g16.045`–`g16.048`
  and `g16.053`; `g16.055` is a separate already-dispatched lane
- **Parallel safety check:** the lane owns only icon-geometry architecture,
  pure geometry/codegen lineage, vectors, generated internal registry, and its
  own card/log. It shares no intended mutable source with the other ready lanes.
  Stop if a shared barrel, generated file, or closeout surface appears.
- **Surfaces this lane owns:**
  `docs/architecture/013-icon-geometry-substrate.md`; pure internal geometry
  modules under `packages/core` and the appropriate Rust contract crate;
  shared geometry vectors; `packages/core/src/icons/` pair-manifest and
  generated internal projection surfaces; `scripts/build-default-icons.ts`
  and the narrow icon audit/generation tests when required; existing generated
  Rust projection lineage when required; focused tests;
  `docs/roadmaps/g16/049-icon-geometry-format-and-registry-foundation.md`; one
  `g16.049` execution log; new `PAPERCUTS.md` entries only for newly observed
  execution friction
- **Integration ownership:** the orchestrator owns `docs/roadmaps/g16/README.md`,
  `docs/roadmaps/generation-index.md`, continuation-runway/register front doors,
  cross-lane status, dispatch state, review, merge, and the decision to launch
  `g16.050`
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if a sibling lane
  merges first
- **Canonical refs:** `docs/architecture/012-semantic-motion-policy.md`;
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/icon.md`,
  `docs/contracts/components/icon-provider.md`,
  `docs/triage/20260901-230405-icon-geometry-programme.md`
- **Review oracle:** `g16.049` `## Review Oracle`
- **Model capability profile:** `day-to-day` non-frontier implementation
  worker; paired TypeScript/Rust normalization and deterministic generation
  need solid ordinary engineering judgment, not frontier escalation
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** no `*-windowed` or native-visual selector; no
  public API, runtime shell/backend, release/workflow, consumer, sibling-repo,
  or Jetstream mutation
- **Required validation:** focused TypeScript/Rust geometry vectors and
  generator tests; deterministic regeneration; `effigy audit:icons`; relevant
  licence/docs checks; `effigy ci:web`; `effigy ci:rust`;
  `effigy docs:check`; one final headless `effigy qa`; and
  `git diff --check origin/main...HEAD`
- **PR base/head:** current pushed `main` at dispatch / worker branch head
  pending
- **PR URL:** pending
- **Review state:** awaiting worker implementation and PR, then exact-head
  orchestrator review
- **Merge path:** orchestrator after exact-head review of the current head and
  passing required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** deliver every ordered-work, acceptance, evidence, and review-
  oracle row in `g16.049`: the architecture record; closed 24×24 stroke format;
  deterministic TypeScript/Rust normalization; shared positive/negative vectors;
  curated 8–12-pair candidate manifest with distinct candidate/rejected states
  and an accepted state reserved for a later visual gate; paired generated
  internal projections; provenance, notice, payload,
  drift, and clean-regeneration proof; one execution log and reviewable PR.
- **Out of scope:** `g16.050` or `g16.051`; runtime planning/interpolation;
  clocks or scheduling; a geometry node; GPUI production painting; Svelte or
  React shells; native/browser visual evidence; public IconMorph; changes to
  Icon, IconProvider, IconButton, NodeKind::Icon, NodeAnimation, package exports,
  default provider behavior, releases, workflows, consumers, sibling repos, or
  Jetstream.
- **Outcome shape:** internal architecture foundation. Unsupported geometry
  fails explicitly; the worker must not create a fallback or widen the subset
  to make a candidate pass.
- Do not invent architecture beyond the card, change public contracts, widen
  the roadmap, or choose a new API, topology, paint, motion, or admission rule.
- This handoff represents one worker lane, and sibling lanes may be running
  concurrently. Write only inside **Surfaces this lane owns**. Leave global
  closeout and front-door surfaces to **Integration ownership**. If shared
  mutable scope, a hidden dependency, or another lane's write appears, stop and
  report it instead of resolving it yourself.
- Work only in the clean worker worktree selected by `Completion Protocol`.
  Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge belongs to the orchestrator after its exact-head
  review/check gate.

## Important Context

- **Planning lineage:** the native feasibility dossier proved dynamic GPUI path
  construction but not production transport or pixels. The operator-approved icon-
  geometry packet funded a curated, renderer-neutral programme while keeping
  Icon static. PR #148 compiled IG-01/02 as `g16.049`; later runtime and visual
  stages remain serially blocked.
- **Why this card is ready:** the operator-approved programme fixes the subset,
  canonical grid, contour/correspondence laws, budgets, provenance, public-
  surface exclusions, review counterexamples, and validation boundary.
- **Decisions and preferences:** canonical endpoints stay separate from sampled
  flight geometry; exact endpoints never use approximate samples; aliases
  canonicalize before identity; correspondence may reorder/reverse/offset but
  never invent topology; numerical cost cannot approve a visually bad pair;
  TypeScript and Rust share vectors and generated lineage.
- **Open tensions:** candidate quality may force explicit rejection or a
  narrower accepted set. Stop on a new format, topology, paint, provider, raw-
  path, public, or runtime requirement rather than expanding the card.
- **Report after:** architecture plus pure format/vectors are coherent, then
  again after the generated registry/audit batch and final validation.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, the canonical continuation runway, `g16.049`, the operator-approved icon-
geometry packet, Icon/IconProvider contracts, architecture 012, and the current
icon generator/audit lineage. Start with the architecture and shared vector
contract before writing generator or registry code.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare its generated path/branch with the planned
   fallback or create another worktree merely because they differ.
3. If current context is `main`, dirty, unregistered, or unusable, inspect the
   named worktree. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique worktree/branch there from pushed `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard dirty
   state. Report a launcher-supplied dirty or `main` worktree instead.
4. From the selected worktree, record this handoff's repository-relative path.
   Run `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch
   origin`. Confirm `HEAD` equals `origin/main`, confirm
   `git merge-base --is-ancestor 7f59ae42f4917c675968819eb23a5e41dc90013c HEAD`,
   and confirm this relative path exists in `HEAD`. Load it with
   `git show HEAD:docs/handoffs/20260902-004204-g16-049-icon-geometry-foundation.md`.
   If the absolute dispatch file differs from that tracked blob, stop.
5. Required sibling links are `none`.
6. Read the active milestone, assigned card, `AGENTS.md`, and canonical refs.
7. Use Effigy only where it fits the job. Run cheap orientation checks and
   record what actually ran.

### While you work

- Execute `g16.049` as one ordered runway. Keep commits aligned with the
  architecture/vector batch and the registry/codegen batch, not model turns.
- Work from the current generated icon lineage. Do not create a second icon
  catalogue or import Morphicons source/tests.
- After each meaningful chunk, report changed files, validation actually run,
  what remains, risks, and blockers.
- Stop if a contract is missing, scope expands, authority is absent, another
  lane owns a surface, or validation changes the plan. Do not quietly turn a
  failed candidate into a new topology or public design.

### When the assigned runway is complete

1. Run the required final validation exactly as listed in **Current State**.
2. Falsify the diff against every `g16.049` oracle row. At minimum plant and
   restore: an off-grid/transformed asset, topology mismatch, reversed/closed-
   start correspondence, source-byte drift, repeated clean generation, and an
   attempted public/static-Icon change. Record why each proof bites.
3. Update the card and one execution log with actual evidence. Do not edit the
   global g16 README, generation index, continuation front doors, or later
   cards.
4. Push the worker branch. If a sibling lane merged first, rebase onto current
   `main`, rerun the required validation, and report the new exact head.
5. Open one PR against current pushed `main`. The planning base above is not a
   self-referential hash for the handoff commit.
6. Link the card, programme packet, architecture, changed surfaces, evidence,
   validation, explicit rejected candidates, and unresolved items.
7. Report the PR URL and exact head. Do not merge and do not start `g16.050`.

### Review and merge path

The orchestrator reviews the current PR head against the card, full diff,
generated artifacts, vector parity, and validation. Shared-identity review is
posted as the canonical PR comment when formal self-approval is unavailable.
Requested changes stay on this branch. Blocking classes are `execution-miss`,
`oracle-gap`, `planning-change`, `validation-gap`, and `integration-drift`.
Requested changes: none. The orchestrator alone merges a current,
mergeable head after required checks.

- **Closeout refs:**
  `docs/roadmaps/g16/049-icon-geometry-format-and-registry-foundation.md` and
  one `docs/logs/2026-09/` g16.049 execution log; global runway/front-door and
  `g16.050` readiness updates remain orchestrator-owned after merge.

### Handoff closeout

Before calling the runway complete, leave the card, log, generated lineage,
and next-task state honest. Once the `g16.049` foundation is merged,
orchestrator review may continue with `g16.050`; it does not launch it or admit
IconMorph. Candidate geometry remains fixture-only. If
blocked, record the blocker and stop.
