---
title: g16.007 TextInput controlled editing and mounted evidence worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle core
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260826-213805-g16-007-text-input-mounted-evidence-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, parity, text-input, gpui]
---

## What This Thread Was Doing

The orchestrator completed the post-`g16.006` evidence checkpoint and selected
`TextInput` as the next high-leverage primitive. It underpins search, command,
settings, model-connection, embed, token, editable-list, and relation-picker
surfaces. Shared Rust and the GPUI backend already contain a renderer-neutral
editing model, caret/selection paint, clipboard/undo, and IME channels, but the
ledger has no named mounted `TextInput` claim.

The ready card proves core controlled editing through the real GPUI tree and
host rebuild. It deliberately does not claim multiline layout, slug
source/autogeneration, full OS IME coverage, or NumberInput parity. Start from
this file; no copied transcript or second prompt is required.

## Why It Matters

Poodle's active problem is not structural roster presence: all 174 portable
components construct. It is trustworthy evidence that native components accept
the same semantic inputs and produce the same observable results as the web
pair. Text entry is a substrate used across the library. A real mounted proof
here is more useful than several display-only rows, while its explicit
non-claims prevent one test from becoming a false whole-component certificate.

The adjacent NumberInput surface is not ready. Its web value is
`number | string | null`, while shared Rust stores one concrete `f64` and
renders only a value label plus steppers. That decision is recorded separately
and is outside this worker's authority.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `88bdec154b766d6ddfbfd1c1630428524ba08c28`
- **Pushed main verification:** local `HEAD` and `origin/main` both equalled the
  planning base before this handoff was created
- **Planning checkout:** clean at the recorded base
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** ready `g16.007` card, updated
  g16/front-door runway, and the separate NumberInput value-model triage note
- **Worker branch:** `t3code/g16-007-text-input-mounted-evidence`
- **Worker worktree:**
  `/Users/tom/.t3/worktrees/poodle/g16-007-text-input-mounted-evidence`
- **Worktree creation command:** `git worktree add -b t3code/g16-007-text-input-mounted-evidence /Users/tom/.t3/worktrees/poodle/g16-007-text-input-mounted-evidence origin/main`
- **Worker worktree policy:** use a clean launcher-provided non-`main`
  registered worktree first, whatever its generated name. Only fall back to
  the named path and then `.agents.local.env` when the current context is
  unusable. Never create a second worktree merely because names differ.
- **Active contract:** `docs/contracts/components/text-input.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready card:**
  `docs/roadmaps/g16/007-text-input-controlled-editing-and-mounted-evidence.md`
- **Allowed runway:** execute `g16.007` only
- **Remaining card budget:** one card
- **Dispatch topology:** serial; web tests, shared edit rules, Rust rendering,
  GPUI text channels, mounted proof, ledger, and closeout overlap
- **Parallel safety check:** do not split the web/core and native/mounted halves
  across worktrees; the evidence claim depends on one exact behaviour envelope
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/text-input.md`, and
  `docs/roadmaps/g16/parity-evidence-ledger.md`
- **Related but out-of-scope triage:**
  `docs/triage/20260826-213343-number-input-native-value-model.md`
- **Model capability profile:** capable coding model, high reasoning; controlled
  host state, focus/selection identity, and input-event ordering need careful
  review
- **Tool/runtime restrictions:** use the repo-local Effigy skill and selectors;
  everything stays headless; never run `*-windowed`, native visual, Jetstream
  preview/QA, release, tag, or publication tasks
- **Required validation:** focused Svelte/React TextInput tests; focused
  poodle-headless/spec/node/render/GPUI text tests for changed layers; the named
  mounted TextInput proof and retained composite text-entry regressions;
  `effigy regressions:native`; `effigy probe:gpui-specimens`;
  `effigy test:parity-evidence-ledger`;
  `effigy check:parity-evidence-ledger`; `effigy ci:native`;
  `effigy ci:web`; `effigy docs:check`; one final `effigy qa`; and
  `git diff --check origin/main...HEAD`
- **Known orientation finding:** `effigy doctor` is already red on the planning
  base from generated-in-src, oversized-file, and stale/broad suppression scans
  recorded in `PAPERCUTS.md`; report that baseline without absorbing cleanup
- **Planning validation:** `effigy docs:check` and `git diff --check` passed;
  existing Svelte build warnings remain non-failing baseline output
- **PR base/head:** `main` <- worker branch
- **PR URL:** pending
- **Review state:** awaiting worker implementation and orchestrator review
- **Merge authorisation:** worker must not merge; explicit operator authority
  is required after orchestrator review

## Boundaries

Keep this run inside core TextInput controlled editing and its exact evidence.

- **In scope:** focused portable web tests; shared Rust edit rules; TextInput
  handlers/spec only where the existing contract requires a bounded additive
  repair; renderer-neutral text channels and GPUI backend only for a measured
  generic defect; headless driver support for real production input; one
  controlled mounted TextInput regression; exact ledger regeneration; one log.
- **Out of scope:** multiline row/wrap/resize certification; slug
  source/autogeneration closure; debounce or async-validation orchestration;
  NumberInput or other component redesign; a native editor architecture; visual
  fixtures; accessibility promotion; Jetstream admission; workflows, versions,
  releases, and downstream repositories.
- Give every interactive native field an explicit id. The public contract
  requires it. Do not build another generated/fallback identity scheme.
- The host stores value, selection, and focus callback results and rebuilds the
  public spec. The test must not invoke handlers or transitions directly after
  mount.
- Use `poodle_headless::text_input` as the shared Rust editing authority. Do not
  reproduce character/selection rules in GPUI, the fixture, or the driver.
- Search clear reports empty value before the clear command. Enter submit and
  Escape cancel report commands without silently mutating controlled value.
- Disabled is inert; read-only stays focusable/selectable and cannot mutate.
- Keep placeholder separate from value, selection, clipboard, and undo state.
- `maxLength` is enforced before the host receives the next value. If missing,
  repair it once at the shared component/machine boundary, not in GPUI.
- Node role/state assertions support mounted behaviour only. GPUI
  accessibility remains `manual`; GPUI visual remains `missing`.
- Do not mark multiline, slug, validation timing, OS IME, or NumberInput closed
  in the card, log, ledger, report, or PR narrative.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or clean/reset an unrelated checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** `g16.001` measured 29 mounted / 145 missing GPUI
  behaviour rows. `g16.002`–`g16.006` moved seven controls to 36 mounted / 138
  missing. This card moves only TextInput to 37 / 137.
- **Why the card is ready:** the contract already fixes the controlled native
  value/caret/focus model; the Rust edit transition and GPUI text channels
  already exist; explicit ids avoid an unresolved identity decision; existing
  mounted composites prove the path can type without certifying the primitive.
- **Current web evidence:** focused tests cover autofocus, imperative focus,
  and value-change-before-clear. Add only the portable submit/cancel and
  disabled/read-only cases required by the card.
- **Current native evidence:** renderer unit tests inspect structure and
  callbacks; backend tests cover caret/selection/clipboard/undo/IME mechanisms;
  existing licence tests type through descendant fields. None is the named
  controlled TextInput mounted claim.
- **Decisions and preferences:** Svelte remains the semantic reference;
  renderer-neutral Rust stays host-controlled; specimens remain human-centred;
  stock crates.io GPUI 0.2.2 only; Jetstream remains deferred.
- **Open tension:** multiline and slug are contractually portable but need
  more state/layout closure than this mounted core proof. Record exact findings
  in the execution log; stop if core editing cannot be separated from deciding
  those modes.
- **Report after:** first the focused web/shared-machine envelope; then the
  mounted controlled-host proof and ledger closeout. Report earlier on any stop
  condition.
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this handoff and run the worktree preflight below before broad reads. Then
read the ready card, TextInput contract, Svelte and React implementations/tests,
`poodle_headless::text_input`, `TextInputSpec`, shared Rust renderer, node text
channels, GPUI input/IME/interaction modules, headless driver, existing mounted
text-entry consumers, and ledger generator.

Implement in two meaningful chunks. First lock the named portable web and
shared-machine behaviour. Then build one controlled mounted host, drive real
GPUI focus/keyboard input, retain backend/composite regressions, regenerate the
ledger, and close the docs. If a test exposes multiline, slug, or NumberInput
ownership as a prerequisite, stop with exact evidence instead of widening.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad repository read, run:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch. Do not compare generated names with this handoff or
   create another worktree because they differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable,
   inspect the named worktree. If that cannot be used, read
   `.agents.local.env`, require the absolute `AGENTS_WORKTREE_CONTAINER_DIR`,
   and create a unique worktree/branch there from `origin/main`. Ask the
   operator if the key is absent. Never use `/tmp`, `TMPDIR`, a repository
   child, or a guessed path. Never clean, reset, stash, or discard the original
   checkout. If the launcher itself supplied a dirty or `main` worktree, stop
   and report it instead of silently creating another.
4. From the selected worktree, run `git fetch origin`; confirm `HEAD` equals
   `origin/main`; confirm
   `git merge-base --is-ancestor 88bdec154b766d6ddfbfd1c1630428524ba08c28 HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `docs/roadmaps/g16/README.md`, the assigned card,
   TextInput contract, ledger, and canonical architecture/working-rule refs.
6. Read `.agents/skills/effigy/SKILL.md`, then use `effigy tasks` and
   `effigy doctor` for orientation. Record the known doctor baseline without
   widening into unrelated cleanup.

### While you work

- Execute only `g16.007`. Keep commits aligned with the two meaningful chunks,
  not model turns.
- Use focused direct machine/backend tests as support, but only real mounted
  backend input plus controlled host rebuild satisfies the ledger claim.
- Preserve existing composite text-entry regressions and the backend's
  caret/selection/clipboard/undo/IME tests after any shared correction.
- After each chunk, report changed files, validation actually run, remaining
  acceptance, exact callback/event ordering, defects, and blockers through the
  operator.
- Stop on any card stop condition. Do not invent a generic editor, widen mode
  scope, change NumberInput, patch GPUI, or admit Jetstream.

### When the assigned runway is complete

1. Run the full validation named in `Current State`, entirely headlessly.
2. Mark the card complete, regenerate the ledger through its source, add the
   August execution log, and leave g16's next task as an orchestrator review
   checkpoint. Do not compile or implement another card.
3. Run `git diff --check origin/main...HEAD` and confirm the worktree is clean
   after committing.
4. Push the worker branch and open a reviewable PR against current `main`.
5. The PR body must link the g16 milestone/card and TextInput contract; name
   focused and mounted tests; report controlled host state, exact command
   ordering, identity proof, retained regressions, ledger before/after counts,
   validation, and explicit non-claims.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect PR metadata, commits, changed files, diff,
portable web tests, shared edit ownership, mounted real-input proof, identity,
ledger lineage, explicit non-claims, and checks independently. Because worker
and orchestrator share the GitHub identity, the orchestrator will post the
canonical verdict as a PR comment rather than formal self-approval. Requested
changes are currently none. The operator must explicitly authorise merge after
a green review.

- **Closeout refs:** assigned card, g16 README/front doors, generated parity
  ledger, one August log, and the still-open NumberInput triage note

### Handoff closeout

Before calling the runway complete, leave the card, log, ledger, roadmap, and
next-task state honest. If blocked, record the exact blocker and stop rather
than making the handoff appear complete.
