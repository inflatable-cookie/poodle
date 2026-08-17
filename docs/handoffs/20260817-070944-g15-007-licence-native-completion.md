---
title: g15.007 licence native completion worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-17
updated: 2026-08-17
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260817-070944-g15-007-licence-native-completion.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, licence, gpui]
---

## What This Thread Was Doing

Poodle is executing the release-first g15 runway. The Svelte and React release
rosters are now fully evidenced at 175/175. This worker takes the first native
component tranche: LicenceActivation, LicenceSeats, and LicenceStatus across
hand-written Rust declarations, shared `poodle-render` composition, and GPUI.

This is one bounded implementation lane. Start from this file without a copied
transcript or a second prompt.

## Why It Matters

Longhorn and the other downstream applications need the approved Licence web
surface to work natively without Poodle depending on Longhorn or reviving the
rejected g14 portable-interface/conformance architecture. The native work must
preserve the product decisions already approved with the operator: licence UI
reports state but never enforces entitlement; account activation and offline
file activation are one model; key activation is a separate model; machine IDs
remain callback-only; LicenceCentre stays unbuilt.

The original carry-forward card hid two prerequisites. They are explicit now:
native CodeInput must support the web contract's real grouping/result model,
and GPUI FileUpload needs a genuine reusable single-file selection/read seam.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `fb29b854bea7d2c3a2fa4bdb39e5946c927e694f`
- **Pushed main verification:** local `HEAD` and `origin/main` both equal the planning base
- **Planning checkout:** clean `main`; implementation edits are forbidden there
- **Planning artifacts included at the base:** g15.007 is ready with its native binding boundary and prerequisite scope; g15.014 is recorded in flight
- **Worker branch:** `t3code/g15-007-licence-native-completion`
- **Worker worktree:** `/Users/tom/.t3/worktrees/poodle/g15-007-licence-native-completion`
- **Worktree creation command:** `git fetch origin && git worktree add /Users/tom/.t3/worktrees/poodle/g15-007-licence-native-completion -b t3code/g15-007-licence-native-completion origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask the
  operator first if the file or key is absent; never use `/tmp`, `TMPDIR`, or a
  guessed path.
- **Active spec lane:** no provisional spec; implement the approved component contracts under the g15 release baseline
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`
- **Ready cards, in order:** `docs/roadmaps/g15/007-licence-family-native-completion.md`
- **Allowed runway:** g15.007 only, in Batches A through D
- **Remaining card budget:** one whole roadmap card
- **Dispatch topology:** parallel with g15.014 only
- **Parallel safety check:** g15.014 owns dependency manifests/lockfiles; this lane owns Licence Rust/GPUI surfaces, its exact CodeInput/FileUpload prerequisites, named contract notes, native roster rows, and a separate log
- **Canonical refs:** `docs/architecture/001-poodle-system-shape.md`, `docs/contracts/001-working-rules.md`, `docs/contracts/005-agent-local-paths.md`, `docs/roadmaps/g15/release-baseline-roster.md`, `docs/roadmaps/g15/release-gap-register.md`, `docs/roadmaps/g14/017-licence-active-runtime-completion.md`, `docs/roadmaps/g14/conformance-estate.md`; `docs/contracts/components/licence-activation.md`, `docs/contracts/components/licence-seats.md`, `docs/contracts/components/licence-status.md`, `docs/contracts/components/code-input.md`, `docs/contracts/components/file-upload.md`
- **Model capability profile:** frontier coding model, high reasoning — public Rust surface plus reusable runtime file-selection capability
- **Tool/runtime restrictions:** no `*-windowed`, `test:native-visual`, `qa:jetstream`, or Jetstream selector; never open an OS prompt during automated evidence
- **Required validation:** `cargo test -p poodle-render`, focused tests for every touched Rust/backend crate, `effigy check:gpui`, `effigy regressions:native`, `effigy docs:check`, and `git diff --check origin/main...HEAD`
- **PR base/head:** `main` ← selected worker branch
- **PR URL:** pending
- **Review state:** awaiting implementation and orchestrator review
- **Merge authorisation:** none; operator authorises merge after review

## Boundaries

Please keep this run inside g15.007.

- **In scope:** the three Licence Rust declarations/renderers/GPUI specimens;
  focused headless evidence; exact CodeInput groups/separator/completion-result
  parity; generic GPUI FileUpload single-file path prompt, read, accept check,
  and base64 result seam; required native-binding contract notes; native roster
  rows; one August batch log; new papercuts.
- **Out of scope:** web component or specimen redesign; Longhorn code or
  imports; LicenceCentre; entitlement enforcement; a portable interface,
  shared corpus, normalized observation, comparator, or generic component
  factory; Jetstream; broad CodeInput/FileUpload redesign.
- Keep `<Name>Spec` cloneable data. Parsers, callbacks, host account content,
  and async account work stay in render handlers/composition, not specs.
- Account submit emits a host request. Poodle does not store or render the
  returned token. Key/file activation emits the exact structural credential and
  trimmed optional label.
- File selection must be generic FileUpload/GPUI capability wiring. Do not put
  OS-dialog logic in LicenceActivation. GPUI 0.2.2 cannot express an accept
  filter in `PathPromptOptions`; enforce it after selection and report rejection
  honestly.
- The live GPUI route uses the real path prompt/read/base64 path. Headless
  evidence injects fixture selection/bytes through the same generic seam; a
  static filename or prefilled credential is not proof.
- Preserve the approved Svelte/React specimens unchanged.
- Never render raw or shortened machine IDs, and never gate a feature from
  licence state.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, persistence, or security policy.
- Do not edit g15.014's dependency files. If a shared mutable file or hidden
  dependency appears, stop and report it through the operator.
- Work only in the selected worker worktree. Never edit the orchestrator's
  planning checkout or clean/reset another checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** g14.015/g14.016 landed and the operator approved the web
  references. g14.017's portable-interface execution method was superseded,
  but its component semantics and native prerequisites survived. g15.007
  recompiles them onto hand-written specs, `poodle-render`, and owner-local
  headless evidence.
- **Why the card is ready:** outcome, component list, host/runtime ownership,
  CodeInput and FileUpload prerequisites, evidence, stop conditions, writable
  scope, and final gates are now explicit. The required GPUI API exists as
  `App::prompt_for_paths`; its test platform does not implement that prompt, so
  the card requires a generic injected-result seam for headless proof.
- **Decisions and preferences:** Poodle renders; hosts supply policy and async
  behaviour. Account plus offline file is one activation model. Key mode is the
  alternative model. Machine naming is opt-in. Empty names are honest. Status
  is calm and informational unless supplied attention says otherwise.
- **Open tensions:** the current Rust CodeInput hardcodes a six-digit 3+3 split;
  remove that inference. The existing FileUpload node composition does not open
  a picker. Extend the reusable capability boundary only as far as required for
  one-file selection/read and its deterministic headless injection. If this
  cannot be done without a broader public architecture choice, stop before
  implementing the Licence shell.
- **Report after:** Batch A (the two generic prerequisites), then after Batches
  B/C, then with the final PR after Batch D and the complete validation round
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Read this handoff from the top. Before broad reads, run the worktree-safety
preflight below. Use the launcher-provided clean non-main worktree even when its
generated path or branch differs from the placeholders. Then read the card and
canonical refs, inspect the current CodeInput/FileUpload node and GPUI seams,
and implement Batch A as one coherent prerequisite chunk. Do not begin the
Licence components until the generic prerequisites have focused green evidence.

## Completion Protocol

### Before you start

1. Read this handoff path, then run one quick read-only safety probe before
   broad repository reads: `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch. Do not compare it with the placeholder or create another
   worktree merely because it differs.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the
   operator if it is absent. Create a unique worktree below that container from
   `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path; never clean,
   reset, stash over, or discard another checkout. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. From the selected worktree, fetch origin; confirm `HEAD == origin/main`;
   confirm `git merge-base --is-ancestor fb29b854bea7d2c3a2fa4bdb39e5946c927e694f HEAD`;
   confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, the g15 milestone, g15.007, and every canonical ref named
   above. Use the repo-local Effigy skill for selector routing.
6. Run the cheap orientation checks. Treat the known doctor findings as
   baseline unless your diff worsens them.

### While you work

- Execute Batches A through D in order. Keep commits aligned with meaningful
  chunks.
- After each reporting boundary, tell the operator the files changed,
  validation actually run, remaining batches, new risks, and blockers.
- Update component contracts before any observable/native binding change.
- Keep component behavior in `poodle-render`; keep the GPUI backend limited to
  runtime interpretation, prompt/input, lifecycle, and drawing.
- Stop if a contract is missing, host/runtime ownership is ambiguous, the
  reusable file capability grows beyond this one-file seam, or validation
  changes the plan.

### When the assigned runway is complete

1. Rebase onto current `origin/main` if g15.014 or another accepted planning
   commit landed, then run the full required validation on that rebased head.
2. Run: `cargo test -p poodle-render`; focused tests for each touched Rust or
   backend crate; `effigy check:gpui`; `effigy regressions:native`;
   `effigy docs:check`; `git diff --check origin/main...HEAD`.
3. Update g15.007's owned native rows in the roster/register and write one
   August batch log naming evidence per runtime and every intentional binding
   difference. Do not edit roadmap status, dispatch, or generation front doors.
4. Push the selected worker branch and open a reviewable PR against current
   `main`.
5. In the PR body, link this handoff, g15.007, the milestone, contracts,
   changed public surfaces, focused evidence, validation, and unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review metadata, commits, diff, checks, public Rust/API
impact, component contracts, and evidence independently. Because the worker and
orchestrator share a GitHub identity, the canonical verdict will be a PR
comment. Requested changes: none yet. The operator must explicitly authorise
merge after a ready verdict.

- **Closeout refs:** `docs/roadmaps/g15/007-licence-family-native-completion.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/generation-index.md`,
  `docs/roadmaps/README.md`, `docs/roadmaps/dispatch.md`,
  `docs/roadmaps/g15/release-baseline-roster.md`,
  `docs/roadmaps/g15/release-gap-register.md`, the three Licence contracts,
  CodeInput/FileUpload contracts, and the worker's August log

### Handoff closeout

Leave the card, native evidence, roster/register rows, log, and next-task state
honest. If blocked, record the exact boundary and stop rather than weakening the
active-cohort claim.
