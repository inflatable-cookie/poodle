---
title: g15.047 primitive visual comparison worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-22
updated: 2026-08-22
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260822-130425-g15-047-primitive-visual-comparison.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, visual, button, gpui]
---

## What This Thread Was Doing

Poodle now has an accepted 18-case Button fixture inventory, paired TypeScript
and Rust validators, exact headless web capture tooling, and a deterministic
no-focus GPUI Metal capture seam. The orchestrator compiled the missing final
child of the primitive visual-conformance lane: one same-run comparator over
Svelte, React, and GPUI with fixed geometry, visual-role, pixel, determinism,
and operator-review rules.

Execute `g15.047` only. This handoff and its ready card are the complete worker
boundary; no transcript or second prompt is needed.

## Why It Matters

Poodle has already lost substantial time to two over-general conformance
architectures. This card proves the useful idea at the smallest honest scale:
the same 18 real Button cases in three runtimes, measured tightly, without a
portable component schema or completion authority. The result must show
whether the comparator catches real drift without turning antialiasing into
noise. The operator reviews every first-batch capture before merge.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `41d3a4fdcfc9602b94eba6784af3ce45b5791f14`
- **Pushed main verification:** local `HEAD` and `origin/main` matched that SHA
  before this handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** accepted `g15.045`/`g15.046`
  evidence; exact ready card `g15.047`; parent, gap register, front doors, and
  Longhorn-lab triage note reconciled.
- **Worker branch:** `t3code/g15-047-primitive-visual-comparison`.
- **Worker worktree:** use the clean, registered, dedicated non-`main`
  worktree supplied by the launcher, regardless of generated path or branch.
- **Worktree creation command:** none. The launcher owns normal creation. A
  manual fallback may only use the operator-selected
  `AGENTS_WORKTREE_CONTAINER_DIR` from ignored `.agents.local.env`.
- **Active lane:** `docs/roadmaps/g15/012-visual-conformance-lane.md`.
- **Ready card:**
  `docs/roadmaps/g15/047-primitive-visual-comparison.md`.
- **Allowed runway:** `g15.047` only.
- **Remaining budget:** one Button-only capture/comparison implementation, one
  August evidence set/log, one PR, then operator review.
- **Dispatch topology:** serial. `g15.043`, `g15.050`, and `g15.013` stay out.
- **Parallel safety:** no second worker lane is ready or authorised.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/components/button.md`,
  `docs/roadmaps/g14/conformance-estate.md`,
  `docs/roadmaps/g15/{012-visual-conformance-lane,045-gpui-offscreen-capture-adoption,046-primitive-visual-fixture-inventory,047-primitive-visual-comparison}.md`,
  `docs/logs/2026-08/20260822-g15-046-primitive-visual-fixture-inventory.md`,
  `test/visual/fixtures/`, and the adopted GPUI capture target.
- **Model capability profile:** frontier coding/review model, high reasoning;
  the work crosses two UI frameworks, Rust, raster comparison, and evidence
  integrity.
- **Tool/runtime restrictions:** headless only. Never run a `*-windowed`,
  `test:native-visual`, GPUI preview, paired-Jetstream, workflow, release,
  tag, or publication path.
- **Required validation:** `effigy test:visual-fixtures`; focused comparator
  negative tests; the new full comparison twice; `effigy
  smoke:gpui-offscreen-capture`; focused tests for repairs; `effigy
  check:svelte`; `effigy react:build`; `effigy check:gpui`; `effigy
  docs:check`; `git diff --check origin/main...HEAD`.
- **PR base/head:** `main` <- worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation and operator visual review.
- **Merge authorisation:** absent. Push the PR and stop.

## Boundaries

In scope:

- consume the unchanged accepted 18-case Button inventory;
- add private Button-only Svelte and React capture hosts, separate from
  catalogue specimens;
- move/reuse the accepted Rust loader so the GPUI capture binary consumes the
  same inventory without adding a third parser;
- extend the adopted GPUI offscreen target to render any exact fixture name;
- emit verified Button-only receipts with bounds, fixed visual roles,
  environment, dimensions, and PNG hash;
- compare Svelte↔React exactly and Svelte↔GPUI under the card's one fixed
  tolerance table;
- prove determinism and the card's planted failures through production compare
  paths;
- make only contract-dictated Button repairs allowed by Bounded Repair
  Authority, with focused regressions and preserved initial findings;
- generate all 54 captures, 36 diffs, receipts, summary, contact sheets, and
  one August log/evidence directory;
- add one headless Effigy selector for the complete first batch.

Out of scope:

- another component or wider fixture roster;
- specimen-page changes;
- generic component/prop/scene schemas, code generation, behaviour corpora,
  portable observation planes, or reusable component registries;
- per-fixture allowlists, committed-baseline comparison, or an update/refresh
  command;
- public APIs, contracts, tokens, generated files, package versions, release
  notes, workflows, Jetstream, Longhorn-lab implementation, tags, publication,
  or merge;
- `g15.043`, `g15.050`, or `g15.013`.

Stop on every card condition. In particular, stop if the 3% native pixel cap
cannot separate renderer antialiasing from planted drift, repeat captures are
not byte-identical, a repair needs a contract/token/architecture decision, or
any path needs a desktop window or permission. Do not widen a tolerance or
silently accept a mismatch.

## Important Context

- The canonical denominator is exactly
  `test/visual/fixtures/button-visual-inventory.json`: 18 identities, 240×80
  logical viewport, 2× scale, `(16,16)` Button origin, explicit theme/size/
  density/variant/tone/content/state.
- `g15.046` measured 12 duplicated lists plus one duplicated rule across its
  two loaders. Reduce that cost where the card explicitly permits moving a
  loader; do not add another parser or registry.
- `poodle-offscreen-capture` is currently one hard-coded primary Button smoke.
  Preserve its no-window one-shot seam, typed publication safety, exact GPUI
  fork pin, Metal path, and Rust 1.95 floor while making fixture identity
  explicit.
- The current GPUI fork is
  `inflatable-cookie/zed@87d9afbe71ef06ea0634499dc35d104bb29dc020`.
  The receipt must record that exact current revision, not the pre-licence-fix
  upstream SHA.
- Existing web specimen capture is not the fixture host. Do not turn catalogue
  Examples into exhaustive cases again.
- The fixed comparison policy is authority. Svelte and React have zero
  tolerance; GPUI has separate bounded geometry, role, and 3% pixel channels.
  Geometry or role failure cannot be hidden by a pixel pass.
- The Button contract already requires the visual roles the receipt names.
  Current source says GPUI omits the web Button's idle shadow and cannot emit
  letter spacing. Treat measured consequences honestly: repair only when the
  contract already decides the result; otherwise stop with evidence.
- Browser and native captures must use the fixture theme's canvas background,
  not the current smoke target's hard-coded white scene.
- The loading spinner is the likely determinism edge. Freeze it at a declared
  frame without changing the shipped component state.
- Contact sheets are the operator review surface. Preserve canonical fixture
  order and readable native-scale output. Raw captures and receipts remain
  available beside them.
- Committed assets are point-in-time log evidence. The comparator must never
  read them as expected output.
- A fresh worktree may need `bun install --frozen-lockfile` before web/docs
  checks. This is known setup, not package scope.
- `effigy doctor` currently reports existing generated-source, god-file, and
  stale-suppression scan debt. Record it; do not absorb it into this card.

Report after three meaningful chunks:

1. all three adapters and receipts render one fixture deterministically;
2. the first full 18-fixture comparison names every initial mismatch and any
   stop/repair decision;
3. final evidence/assets/log and PR are ready for operator review.

## Suggested Next Move

Read this handoff from the top and run the four-command preflight below before
broad repository reads. Confirm the pushed base and the accepted inventory.

Build one end-to-end vertical slice first: `button/rest-secondary` through
Svelte, React, GPUI, verified receipts, separate geometry/role/pixel verdicts,
and repeat-capture equality. Exercise a planted failure before scaling to all
18 cases. This tests the architecture while the adapter surface is still
small. Once the slice has teeth, fan out over the frozen inventory, record the
initial mismatches, apply only bounded contract-backed repairs, then generate
the review assets.

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
   its actual path/branch; do not create another worktree. If the launcher
   supplied `main`, a dirty checkout, or an unregistered checkout, stop and
   report it. Never clean, reset, stash over, or discard it.
3. A manual fallback is allowed only after reading `.agents.local.env` and
   finding the operator-selected `AGENTS_WORKTREE_CONTAINER_DIR`. Never use
   `/tmp`, `TMPDIR`, or a guessed repository-adjacent path.
4. Run `git fetch origin`; confirm `HEAD` equals current `origin/main`; confirm
   `git merge-base --is-ancestor 41d3a4fdcfc9602b94eba6784af3ce45b5791f14 HEAD`;
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `.agents/skills/effigy/SKILL.md`, parent `g15.012`, ready
   card `g15.047`, `g15.045`/`g15.046` plus their logs, Button contract,
   conformance-estate, inventory code/data, visual tooling, and GPUI capture
   target.
6. Run `effigy tasks` and `effigy doctor`. Record the known unrelated doctor
   scan debt. Run `effigy test:visual-fixtures` as the clean starting proof.

### While you work

- Keep the first vertical slice small enough to delete if it starts rebuilding
  g14's generic machinery.
- Keep receipt and comparison types Button-only and closed to unknown fields.
- Verify every PNG against its own receipt before any pair comparison.
- Compare channels independently. A geometry/role failure stays red even when
  pixels fit the cap.
- Capture each runtime/fixture twice. Do not average, retry-away, or choose the
  nicer frame when bytes differ.
- Plant failures in memory or disposable outputs. Never commit broken fixture
  data or mutate expected assets.
- Preserve initial mismatch measurements before applying a bounded repair.
- Report each meaningful chunk through the operator with files, validation,
  findings, remaining work, and blockers.

### When the assigned runway is complete

1. Run `effigy test:visual-fixtures`, focused comparator tests, and the full
   headless comparison twice. Record matching repeat hashes and report metrics.
2. Run `effigy smoke:gpui-offscreen-capture` and focused Button/runtime tests
   for every repair.
3. Run `effigy check:svelte`, `effigy react:build`, `effigy check:gpui`, and
   `effigy docs:check`.
4. Write `docs/logs/2026-08/20260822-g15-047-*.md`. Commit the complete review
   evidence under `docs/logs/2026-08/assets/g15-047/`, including summary,
   contact sheets, captures, receipts, diffs, environment, source cost,
   duplicated registry count, initial mismatches, repairs, and final metrics.
5. Run `git diff --check origin/main...HEAD`. Confirm no specimen, second
   component, generic schema, generated output, workflow, version, release,
   Jetstream, Longhorn, tag, or publication change exists.
6. Push the worker branch and open one reviewable PR against current `main`.
   The planning base above predates this handoff commit; it is intentionally
   not self-referential.
7. Link the card, parent, inventory/capture evidence, August log, validation,
   initial/final findings, and unresolved items in the PR body. Report the PR
   URL and review asset paths to the operator. Do not merge.

### Review and merge path

The orchestrator independently reviews scope, all three real adapters,
receipt integrity, determinism, planted failures, fixed thresholds, initial
mismatch handling, contract-backed repairs, assets, logs, and checks. The
operator reviews all 54 captures and every native tolerance through the
contact sheets/report.

If changes are requested, make only those changes on this branch, push again,
and report back. Merge requires explicit operator authorization after both
technical and visual review.

- **Requested changes:** none yet.
- **Closeout refs:** `g15.047`, parent `g15.012`, its August log,
  `docs/roadmaps/g15/README.md`, release-gap register, generation index,
  front-door roadmap, dispatch ledger, and Longhorn-lab triage note.

### Handoff closeout

Once the PR is accepted and merged, the orchestrator owns runway/log/dispatch
closeout and the advance decision. The worker does not mark `g15.047` or
parent `g15.012` complete on `main`.
