---
title: g16.027 inbound files and drag-out worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-31
updated: 2026-08-31
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260831-180228-g16-027-inbound-files-drag-out.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The dependable drag-and-drop runway has landed through `g16.026`. This
dispatches the next ordered card: explicit inbound-file and native file
drag-out capabilities over the shared semantic substrate.

This is one bounded implementation lane. No transcript or second prompt is
part of the authority chain.

## Why It Matters

Poodle needs honest external-file boundaries without teaching components about
browser `File` objects, shell paths, Electron, Tauri, temporary directories, or
application export policy. The card completes that host seam before the final
drag-and-drop migration and certification pass.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `6a7b88478edf420e0bc35996ace106aedbc039d7`
- **Pushed main verification:** planning base matched `origin/main` before this
  handoff commit
- **Planning checkout:** clean at dispatch preparation
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** this tracked handoff plus the
  `g16.027` readiness and spec-069 status corrections in its containing commit
- **Worker branch:** `codex/g16-027-inbound-files-drag-out`
- **Worker worktree:** Paseo-managed worktree created from pushed `origin/main`
- **Worktree creation command:** Paseo `create_workspace`, worktree
  `branch-off`, base `origin/main`, branch
  `codex/g16-027-inbound-files-drag-out`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** `docs/specs/069-dependable-drag-and-drop-substrate.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/027-drag-drop-inbound-files-and-drag-out.md`
- **Allowed runway:** `g16.027` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; `g16.028` waits for this merge
- **Parallel safety check:** no second drag-and-drop worker may edit the shared
  controller, contracts, Rust construction, or browser probe during this lane
- **Canonical refs:** `docs/architecture/011-drag-and-drop-substrate.md`,
  `docs/specs/069-dependable-drag-and-drop-substrate.md`;
  `docs/contracts/001-working-rules.md`
- **Review oracle:** the card's Acceptance Criteria and Stop Conditions plus
  spec 069's Native DataTransfer Adapter, Native File Drag-Out, Inbound Files,
  Failure And Cancellation Rules, and Certification Matrix sections
- **Model capability profile:** Opus Worker — complex implementation handoff
- **Tool/runtime restrictions:** Effigy only for routed validation; never run
  windowed/native-visual, Jetstream, release, workflow, or sibling mutation
  commands
- **Required validation:** focused paired contract/machine/adapter tests;
  `effigy test:drag-drop-browser`; `effigy ci:web`; `effigy ci:rust`;
  `effigy ci:native`; `effigy docs:check`; one final headless `effigy qa`;
  unchanged ledger checks; `git diff --check`
- **PR base/head:** `main` / `codex/g16-027-inbound-files-drag-out`
- **PR URL:** pending
- **Review state:** awaiting worker implementation and PR
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** paired TypeScript/Rust external-file capability, receipt, and
  lifecycle seams; bounded browser/native-data adapters; renderer-neutral Rust
  construction and GPUI preparation-state projection; fake hosts, browser
  probes, contracts/guides, curated Svelte/React specimens, card/log/roadmap
  closeout, and a small directly encountered papercut entry if needed
- **Out of scope:** Electron, Tauri, Longhorn, Loophole, filesystem or shell
  dependencies; application materialization/export/cleanup policy; real paths
  or native file objects in public component props; other component
  migrations; ledger movement; package versions, releases, workflows,
  Jetstream, native visual tests, or sibling mutation
- **Outcome shape:** complete implementation, evidence, and reviewable PR. If
  the fixed opaque host boundary cannot express the implementation without a
  new public product/API choice, stop and return that exact choice.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's assigned
  scope; if shared mutable scope or a hidden dependency appears, stop and report
  it through the active control plane or the operator.
- Work only in the clean worker worktree selected by `Completion Protocol`.
  Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge belongs to the orchestrator after its accepted
  review/check gate.

## Important Context

- **Planning lineage:** `g16.021`–`g16.026` are merged. PR #113 landed the
  split source/window host bridge, opaque receipt transport, Tabs/DockRegion
  migration, and GPUI window-owned bridge pump. No ledger cell moved.
- **Why this card is ready:** its dependency is merged; architecture 011 and
  spec 069 fix host versus Poodle ownership, transport families, capability
  distinctions, receipt opacity, validation, abort/supersession behavior, and
  cleanup authority.
- **Decisions and preferences:** files are the portable drag-out baseline;
  promised files and custom types are advertised optional capabilities;
  inbound data enters normal eligibility and is revalidated; a native drag end
  never authorizes deletion; specimens stay curated and exhaustive cases stay
  in tests.
- **Open tensions:** browser `DataTransfer` is phase-limited and webview
  ownership may conflict with Tauri inbound capture. Resolve these behind
  adapters without shell detection. OS/DAW consumption remains honest manual
  downstream evidence.
- **Report after:** the first coherent paired lifecycle and fake-host chunk,
  then after adapter/specimen integration, or immediately on a stop condition
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, the active milestone, assigned card, and canonical refs from the
selected worker worktree. Map the existing cross-window seams before naming new
public types. Start with paired lifecycle contracts and fake-host proofs.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run: `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare its generated path/branch with the named
   placeholders or create another worktree merely because they differ.
3. If current context is `main`, dirty, unregistered, or unusable, inspect the
   named worktree. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique worktree/branch there from pushed `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash-over, or discard dirty
   state. Report a launcher-supplied dirty or `main` worktree instead of
   creating another.
4. From the selected worktree, record this handoff's repository-relative path.
   Run `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch
   origin`. Confirm `HEAD` equals `origin/main`, confirm the planning base is an
   ancestor, and confirm the relative handoff path exists in `HEAD`. Load the
   tracked handoff with `git show HEAD:<relative-path>`. If the absolute
   dispatch file is readable and differs from that tracked blob, stop and
   report. The committed `HEAD` copy is canonical.
5. Required sibling links are `none`; create none.
6. Read the active milestone, assigned card, `AGENTS.md`, and canonical refs.
7. Run `effigy tasks` and narrow orientation checks. `effigy doctor` currently
   reports the repository's recorded generated-source, god-file, and stale-
   suppression baseline; do not turn that unrelated baseline into this lane.

### While you work

- Execute the one ready card in coherent chunks, with commits aligned to those
  chunks rather than model turns.
- Preserve one semantic external-file lifecycle across TypeScript and Rust.
  Validate all external metadata before eligibility; revalidate before commit.
- Keep every host/native value opaque beyond the adapter boundary. Poodle may
  present display names and states, but it never receives or deletes a host
  artifact.
- After each meaningful chunk, report changed files, validation actually run,
  remaining work, risks, and blockers through Paseo.
- Stop on a missing contract, ambiguous public shape, scope expansion,
  authority/access failure, or validation result that changes the plan.

### When the assigned runway is complete

1. Run the required validation listed above.
2. Falsify the diff against every Acceptance Criteria and Stop Conditions row.
   In particular, prove late preparation cannot resurrect superseded work,
   terminal/cleanup effects are exact, unsupported capabilities stay inert,
   retained cleanup remains host-owned, inbound validation precedes
   eligibility, and no real path/native file object crosses the public seam.
3. Update the card, one execution log, milestone/front-door next-task state,
   and spec status. Keep the ledger unchanged.
4. Push the worker branch.
5. Open a reviewable PR against current pushed `main`.
6. In the PR body, link the spec, milestone, card, changed surfaces, evidence,
   validation, falsification results, and unresolved items.
7. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator will review the exact PR head against the canonical refs,
diff, tests, and falsification evidence. With a shared GitHub identity, it will
post the verdict as a PR comment. Requested changes stay on this branch and
return to the same worker. Blocking findings use `execution-miss`,
`oracle-gap`, `planning-change`, `validation-gap`, or `integration-drift`.
When the reviewed head is still current, required checks pass, the PR is
mergeable into `main`, and no stricter rule or explicit operator pause applies,
the orchestrator merges without another approval prompt.

- **Requested changes:** none
- **Closeout refs:** card 027, one `docs/logs/2026-08/` execution log, g16
  milestone, spec 069, and roadmap front doors

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, spec, log, and
next-task state honest. If blocked, record the blocker and stop rather than
making the handoff look complete.
