# g14 Orchestrator Handoff — 2026-08-13

Status: superseded 2026-08-14 by `../../roadmaps/g14/README.md` and
`../../roadmaps/archive/2026-08-14-g14-machine-pinning-false-start.md`

You are the new orchestrator for the Poodle repository's g14 runway, taking
over mid-programme. The maintainer drives execution through t3code worker
threads; your job is planning, review, and merge. Work from the main checkout
at `/Users/tom/Dev/projects/poodle`. Read `AGENTS.md`, `docs/roadmaps/dispatch.md`,
`docs/roadmaps/g14/README.md`, and `PAPERCUTS.md` before anything else.

## What This Thread Was Doing

The previous thread ran the tail of g13 and opened g14. It started with a
viability question about a Rust-authored cross-runtime IR (spec 063). The
g13.008 verdict recorded **revise**, the g13.020 verdict **retired the
vocabulary authority as a corpus mechanism** (b052 measured 8.9× definition
cost and zero marginal catch over the existing drift gates), and card 053
unwound the three pilot components back to hand-written states. The durable
outcome is the **pinning doctrine** (spec 064): two authority pairs
(`poodle-core` for Svelte+React, `poodle-render` for GPUI+Jetstream) stay
two, pinned to each other by execution — generated machine interfaces,
differential traces, vector completeness, capability absence registry,
specimen evidence gates. Spec 065 keeps the scene system as the one fixture
authority. The g14 roadmap (10 milestones) executes that doctrine.

## Why It Matters

The user's real goal: kill cross-runtime drift between four implementations
by enforcement, not description. The IR codegen path was measured and
rejected; the surviving line is cheaper pinning + shared scene fixtures so
implementation differences are diagnosable instead of confounded. The
scene-specimen tranche (b005) proved the scene lane wins ~30× (699 authored
lines replacing 2,425 hand-written across four runtimes). The runway is
finishing the pinning stack, then closing the 18-component native
registration gap, then standing specimen evidence gates.

## Current State

- **g13 is closed.** Verdicts recorded, pilots unwound, spec 063 retired
  (scene half → 065, component half retired), spec 064/065 active.
- **g14.001 (doctrine) closed** — architecture 001/006 + working rules 001
  amended; `docs/specs/064-*.md` and `065-*.md` active.
- **g14.002 (baseline) closed** — `docs/roadmaps/g14/g14-baseline-manifest.md`
  frozen: 18-component native registration gap, 21/21 machines pinned but
  slider/menu vectors thin, 14 drift gates inventoried.
- **g14.003 tranche one done** (b005) — five display specimens scene-authored,
  per-specimen cost 0.29×; the scene lane rolls out. Second tranche not yet
  compiled.
- **g14.004 (machine interfaces) closed** — `machine-interfaces.json`
  generates TS/Rust declarations for hover/menu/modal/popover, drift-gated.
- **g14-b006 (differential testing) is in flight** — milestone 005, the
  TS↔Rust trace harness over the same four machines. Solo card; do not
  dispatch anything touching `packages/contracts/headless/tests/` or
  `packages/core/test/` until it merges.
- Compiled-and-ready batch cards: none. Next to compile: **b007 (vector
  completeness, milestone 006)** after b006, then **b008 (capability
  registry, milestone 007)**. Milestones 008 (native gap), 009 (specimen
  evidence gates), 010 (reassess) follow.

## Boundaries

- **No revival of cross-language codegen.** Spec 064's boundary is
  permanent: interfaces in, behaviour out, no evaluator, no expression
  vocabulary. Generated machine *interfaces* and scene *fixtures* are the
  only generated surfaces.
- The user drives worker threads. You compile batch cards, maintain the
  dispatch ledger (`docs/roadmaps/dispatch.md` — you are the only writer),
  review PRs (checkout branch, run the full gate stack **in the main
  checkout** including `ci:native` — worktrees cannot resolve the Jetstream
  sibling path-dep), merge with `--no-ff` merge commits, delete the branch
  after merge, and update the ledger row.
- Threads are append-only on `PAPERCUTS.md` and `tasks/effigy.tasks.toml`;
  reconcile at merge (conflicts are expected — b004 ∥ b005 collided in
  `poodle-codegen` bin + tasks, resolved by keeping both CLI paths).
- The Thread Reuse Protocol is in `docs/roadmaps/dispatch.md` — workers
  reset to main per card, never branch from a previous card's branch.
- Do not touch `generation-index.md` / `roadmaps/README.md` until rollover
  is authorized (see Important Context).

## Important Context

- **Front doors are stale.** `generation-index.md` and `docs/roadmaps/README.md`
  still present g13 as active and g14 as not-opened. The maintainer approved
  g14 but the rollover edit (mark g13 closed, g14 active, refresh the
  index) was never made. Offer it; do not do it unilaterally.
- **Pre-existing main failure, unowned:** svelte-check reports 3 errors in
  `packages/svelte/components/test/AppHeaderCenterHarness.svelte`
  (Snippet type-identity), reproduced identically on origin/main in the same
  checkout. Environment-dependent — the b005 worker's ci:web passed in
  their worktree. It blocks `ci:web` on main and needs an owning card.
  Recorded in the b005 ledger row.
- **The 18-component native registration gap** (baseline §1) is milestone
  008's job — HistoryCentre is a named maintainer decision point there
  (flat-list core, no recursive renderer; native parity was deferred since
  b028).
- **Jetstream worktree recipe** (PAPERCUTS, b002): branch clone of poodle at
  `/Users/tom/.t3/worktrees/poodle/poodle` + sources-only copy of jetstream
  at `/Users/tom/.t3/worktrees/poodle/jetstream` + `CARGO_TARGET_DIR` to the
  main checkout's jetstream `target/`. The old symlink workaround is broken
  (lockfile collision).
- **Specimen scene schema gained one field** — `ComponentInstance.group`
  (`#[serde(default)]`, spec 063 "groups" was always listed but never
  implemented). Maintainer-ruled in b005; it is the sanctioned limit — do
  not let further schema creep in without a ruling.
- House style: glue-light docs, logs per card in `docs/logs/2026-08/`,
  validation via `effigy` selectors (`effigy tasks` to list; `git diff
  --check` always).

## Suggested Next Move

1. Verify the environment: `effigy tasks`, `effigy doctor`, `git status`.
2. Check for the b006 PR (`gh pr list`); if it exists, review per the
   protocol above (full gate stack in main checkout, incl. `ci:native` and
   the new `docs:differential` selector), merge, update the ledger.
3. Compile **b007 vector completeness** (`docs/roadmaps/g14/batch-cards/007-...`)
   from milestone 006 — thin vectors are slider (3 cases, zero two-thumb)
   and menu (no dismissal events in its block); the completeness gate is
   the deliverable. Hand it to the user as a thread.
4. Offer the front-door rollover (g13 closed, g14 active) to the maintainer.
5. After b007, compile b008 (capability registry) — the `capabilities.json`
   home in `contracts/headless/capabilities/` is ready from b053.

## Completion Protocol

- Every merged card: ledger row updated in `docs/roadmaps/dispatch.md` with
  evidence, branch deleted, PR closed.
- Every compiled card: committed to main with a `g14-bNNN` ledger row
  before the user starts the thread.
- Front doors (`generation-index.md`, `roadmaps/README.md`) only with
  maintainer approval; the stale-g13 state is known and recorded here.
- Batch logs per card land in `docs/logs/2026-08/` as `DD-g14-bNNN-<slug>.md`.
- The svelte-check main failure and the environment-consistency issue are
  open findings with no owner — do not let them silently become the next
  review's surprise; surface them to the maintainer.
- On b006 landing, the differential harness is the milestone's evidence;
  on b007 landing, the vector completeness gate is. Do not mark a
  milestone closed without its acceptance evidence from the batch log.
