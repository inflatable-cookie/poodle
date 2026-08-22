---
title: g15.046 primitive visual fixture inventory worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260822-083804-g15-046-primitive-visual-fixture-inventory.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, visual, fixtures, button]
---

## What This Thread Was Doing

Poodle is finishing the bounded visual-conformance lane required before the
v0.2.0 operator gate. The web capture seam already runs headlessly, and
`g15.045` adopted a deterministic no-window GPUI pixel path. The next job is
not comparison yet. It is to freeze one small cross-language inventory of
named Button visual cases that Svelte, React, and GPUI can consume in
`g15.047`.

Execute `g15.046` only. Land one versioned Button-specific data file, small
TypeScript and Rust validators over that same file, boundary documentation,
focused evidence, and one August log. Do not render or compare pixels.

This is one worker handoff. You do not need the originating transcript or a
second prompt.

## Why It Matters

The previous two cross-runtime mechanisms failed because their shared formats
expanded into component API and behavior authorities. This lane keeps the
useful part — one name for the same visual case in every renderer — and rejects
the rest. Button is the proving ground because its geometry and token recipes
flow into much of the component library. If this 18-case inventory cannot stay
small, explicit, and diagnostic, the comparator must not scale.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `2eb65afdc51c19fd74ded9dd9b9ff5171a71382d`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** exact Button-only `g15.046`,
  corrected `g15.012` continuation, and current Longhorn-lab triage state.
- **Worker branch:** `t3code/g15-046-primitive-visual-fixture-inventory`
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of its generated path or
  branch name.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Active spec lane:** `docs/roadmaps/g15/012-visual-conformance-lane.md`.
- **Roadmap milestone:** `docs/roadmaps/g15/README.md`.
- **Ready card:**
  `docs/roadmaps/g15/046-primitive-visual-fixture-inventory.md`.
- **Allowed runway:** `g15.046` only.
- **Remaining budget:** one Button inventory batch, one August log, one PR,
  then stop.
- **Dispatch topology:** serial visual lane; no open PR or overlapping worker
  exists at dispatch.
- **Parallel safety check:** the next comparator (`g15.047`) is blocked on this
  accepted inventory. Release, presentation-context, and automation lanes do
  not share this card's test-data surface and remain undispatched.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/button.md`,
  `docs/roadmaps/g15/012-visual-conformance-lane.md`,
  `docs/roadmaps/g14/conformance-estate.md`, and
  `docs/logs/2026-08/20260821-g15-045-gpui-offscreen-capture-adoption.md`.
- **Existing capture surfaces:** `test/visual/` for web and
  `packages/gpui/preview/src/bin/offscreen_capture.rs` plus
  `packages/gpui/preview/scripts/offscreen-capture-smoke.ts` for GPUI. Inspect
  them for boundary and domain context; do not extend them into comparison.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** headless inventory validation only. Never run
  `*-windowed`, `test:native-visual`, a GPUI preview, any Jetstream selector,
  release mutation, or workflow edit.
- **Required validation:** focused TypeScript inventory tests, focused Rust
  inventory tests, `effigy docs:check`, and
  `git diff --check origin/main...HEAD`. Add one narrow `effigy.toml` selector
  only if no existing selector can route the focused inventory tests cleanly.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and orchestrator review.
- **Merge authorisation:** absent. Push the PR and stop for review.

The motion-learning and Longhorn conformance-lab triage notes remain open and
out of scope. The future lab consumes a proved comparator; it does not define
this inventory.

## Boundaries

In scope:

- one canonical, versioned, Button-specific fixture data file under
  `test/visual/fixtures/`;
- exactly the 18 fixture identities named by `g15.046`, with explicit resolved
  theme, size, density, viewport, scale, content, variant, tone, and visual
  state on every row;
- a small TypeScript loader/validator and a small Rust loader/validator that
  consume the same checked-in data file;
- negative evidence for missing, extra, duplicate, malformed, or unknown
  fixture identities and values, with the offending exact name in the error;
- renderer-neutral geometry landmark names (`root`, `content`, conditional
  `icon`/`spinner`) and semantic report roles (`fill`, `border`, `text`,
  `shadow`, `focus-ring`) without expected renderer output;
- focused documentation under `test/visual/` explaining the denominator,
  fixed environment, authority boundary, and `g15.047` continuation;
- one August `g15.046` execution log with source cost, registry count, actual
  worktree/branch, and validation.

Writable scope:

- `test/visual/fixtures/` and the smallest adjacent validator/test/doc files;
- one focused GPUI preview test module needed to parse the same data;
- `effigy.toml` only for one narrow selector if existing routing is genuinely
  insufficient;
- one August `g15.046` log;
- dev/test metadata only if an already-present dependency must be declared;
- root `PAPERCUTS.md` only for newly encountered small execution friction.

Out of scope:

- captures, baselines, image files, geometry receipts, pixel diffs,
  thresholds, tolerance policy, or comparison reports — all belong to
  `g15.047`;
- Svelte, React, shared Rust, GPUI component behavior or visual fixes;
- component contracts, public package APIs, specimen pages, themes, tokens,
  generated component adapters, or a generic component schema;
- action scripts, callbacks, event sequences, behavior machines, normalized
  renderer output, node trees, scenes, or code generation;
- any component other than Button, including IconButton;
- the complete Button variant × tone cross-product. This first batch samples
  status tones on secondary only and says so;
- workflows, release versions, tags, publication, `g15.047` implementation,
  `g15.043`, `g15.049`, `g15.050`, Jetstream, or the Longhorn lab;
- merging the PR.

Stop and report if one data shape cannot be consumed directly by both
languages without generated bindings, if an identity requires framework or
node vocabulary, if existing Button contract domains disagree across
languages, if a new production dependency appears necessary, or if the batch
needs to grow beyond the exact 18 cases. Do not hide a domain mismatch behind
normalisation or fallback values.

## Important Context

- The exact fixed environment is Eclipse, `md`, default density, 240×80
  logical pixels, 2× scale, and label `Run`, except where an identity explicitly
  changes one value. Every row still stores all resolved values.
- The 18 names are authority for this batch. Do not derive a larger matrix:
  three resting variants; three secondary status tones; four non-`md` size
  stops; two non-default densities; disabled/loading/pressed states; leading
  icon and icon-only content; and one Iceberg reference-theme case.
- Use a schema/version discriminator local to the Button inventory. A generic
  `props: Record<string, unknown>` or component-name registry is a stop
  condition, not flexibility.
- Keep the canonical file under test tooling. Published Poodle packages and
  runtime components must not import it.
- The two language loaders may duplicate a tiny Button-specific type. Record
  that source cost honestly. Do not generate one language from the other.
- Validate exact domains against current Button authority. The portable Button
  contract is the common surface; HTML form-only props do not enter fixtures.
- Visual state names describe already-real rendering inputs (`disabled`,
  `loading`, `pressed`), not interactions that produce them.
- The inventory may name future receipt landmarks and token roles, but it must
  not include expected bounds, colors, hashes, or pass thresholds.
- Do not reuse specimen pages as the fixture host. Specimens remain
  human-facing documentation under the working rules.
- `g15.045`'s GPUI path is 2×-only and host-font-dependent. That constraint is
  why scale is explicit now and tolerance stays deferred to `g15.047`.
- Report after the data shape and validators first pass red/green, before
  adding documentation or widening any dependency/task surface.

## Suggested Next Move

Read this handoff from the top, then run the four-command worktree preflight
below before broad repository reads. Once the worktree is accepted, inspect
the exact `g15.046` table, Button contract and public domains, current web
visual tooling, and GPUI offscreen receipt/domain code.

Design the Button-specific data shape first. Write the negative exact-name
tests before filling the 18 rows. Prove both language loaders consume the same
file without code generation or fallback defaults. Only then document the
boundary and write the batch log. Do not start capture work.

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
   `git merge-base --is-ancestor 2eb65afdc51c19fd74ded9dd9b9ff5171a71382d HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, the g15 milestone,
   `g15.012`, `g15.046`, the Button contract, g14 conformance-estate ledger,
   g15.045 log, and the existing web/GPUI offscreen test surfaces.
6. Use `effigy tasks` to confirm selectors. Use `effigy test --plan` only if
   focused test routing is unclear. Do not run windowed/native-visual, preview,
   Jetstream, workflow, or release paths.

### While you work

- Keep one checked-in Button data file as the only fixture-list authority.
- Make both loaders reject unknown or incomplete data; no default filling,
  aliases, loose extra fields, or silent skips.
- Keep the format Button-specific and the denominator fixed at 18.
- Use planted negative cases to prove exact missing/extra/duplicate/invalid
  reporting without committing a broken canonical inventory.
- Do not build renderer adapters, captures, comparison, or baselines.
- Append one August execution log. Record actual file/registry count and say
  explicitly what the inventory cannot prove.
- Stop on every condition named by the card or this handoff.

### When the assigned runway is complete

1. Run the required final validation named in Current State. Finish with
   `git diff --check origin/main...HEAD`.
2. Confirm the canonical file contains exactly 18 unique names and every row
   carries all resolved fields.
3. Confirm the focused negative evidence plants and detects missing, extra,
   duplicate, unknown-domain, unresolved-default, and invalid viewport/scale
   cases by exact name.
4. Confirm TypeScript and Rust consume the same checked-in file and no
   published package imports it.
5. Confirm there are no captures, baselines, comparison thresholds, component,
   contract, specimen, token, workflow, release, Jetstream, or Longhorn-lab
   changes.
6. Push the worker branch and open one reviewable PR against current `main`.
   The handoff's planning base is the pre-handoff commit, not the commit that
   contains this file.
7. Link `g15.012`, `g15.046`, the Button contract, changed test surfaces,
   August log, validation, source/registry cost, and continuation to `g15.047`
   in the PR body.
8. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will independently inspect the canonical denominator, both
loaders, planted negative evidence, authority boundary, dependency/task
impact, and checks. The operator will review any judgment-bearing fixture
inventory before authorising merge.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorisation after review
and checks.

- **Requested changes:** none yet.
- **Closeout refs:**
  `docs/roadmaps/g15/046-primitive-visual-fixture-inventory.md`, the August
  batch log, `docs/roadmaps/g15/012-visual-conformance-lane.md`,
  `docs/roadmaps/g15/README.md`, `docs/roadmaps/g15/release-gap-register.md`,
  `docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.

### Handoff closeout

The worker owns the bounded Button inventory, shared-file parsing evidence,
focused tests, boundary documentation, and batch log. The orchestrator owns
card/roadmap status, review, merge, operator sign-off, and promotion of
`g15.047`. Leave the card open if the same small data file cannot be consumed
honestly by both languages or if any stop condition fires.
