# 005 Scene-authored Specimen Tranche One

Status: ready
Milestone: `g14.003`
Owner: Poodle core
Branch: `thread/g14-005-scene-authored-specimen-tranche`
Depends on: `g14.002` (merged), `g14.001` (merged)
Governing refs: `docs/specs/065-scene-authoring-and-specimen-fixtures.md`,
`docs/roadmaps/g14/003-scene-authored-specimen-migration.md`,
`docs/roadmaps/g14/g14-baseline-manifest.md` §5,
`../g13/batch-cards/035-shell-scene-rust-authoring-and-web.md` (the
proven pattern)

## Goal

Migrate one family's static specimen surface onto the scene system and
measure the cost. This is the tranche that decides whether the specimen
lane rolls out — the b052 rule applies: measure before sweeping.

## Fixed By Ruling (recorded — do not re-decide)

- **R1 — The family is the five display components:** `Callout`,
  `EmptyState`, `Avatar`, `Pill`, `Spinner`. Static tier, small surfaces,
  present in all four native registries (baseline §1), and the same five
  that measured the vocabulary sweep to death — if scenes beat their 9×
  definition cost, the lane is real.
- **R2 — No scene schema extension.** Fixtures bind literals and declared
  axes; nothing executes. Anything the static tier cannot express is
  classified (interactive harness, or per-runtime absence with a reason),
  never a schema change. This is the boundary spec 065 sets; a schema
  change is a stop condition, not a deliverable.
- **R3 — The pattern is b035/b036, not a new one:** author the scene in
  Rust under `packages/codegen/src/models/`, emit to the four previews
  via new specimen targets, gate with `ir:check`. The hand-written
  specimens these scenes replace are deleted — this is the one lane where
  replacement is the point.

## Deliverables

- Scene definitions for the five display specimens: states rows, prop
  demos, size/density matrices, axes.
- Emitted fixtures in all four previews; the hand-written specimen files
  for the five deleted from each runtime.
- `ir:check` coverage for the new fixtures; a planted fixture divergence
  fails it.
- The measurement: authored scene LOC vs the hand-written specimen LOC it
  replaced, per component and total, against baseline §5's numbers.

## Acceptance

- [ ] The same specimen renders in all four runtimes from one definition
  (verified in both web previews live, native via the census/registry
  path).
- [ ] Per-specimen cost measured and reported; the verdict on rolling out
  the next family is data, not opinion.
- [ ] No scene schema change; no hand-written specimen file for the five
  remains.
- [ ] `effigy ir:build`/`ir:check`, `effigy test:components`,
  `effigy ci:web`, `effigy docs:lint`, `git diff --check` exit 0.
  (`ci:native` runs at review in the main checkout.)

## Stop Conditions

- A specimen needs something the static scene vocabulary cannot say (an
  interactive harness case, a callback binding) — classify it and keep it
  hand-written with the reason; do not extend the schema.
- The per-specimen cost lands near the b052 definition cost (~9×) — stop,
  report, and do not pretend the lane won.

## Writable Paths

- `packages/codegen/src/models/**` (new display-specimen scene model)
- `packages/codegen/src/targets/**` (new specimen targets)
- `packages/codegen/fixtures/**` (new fixture)
- `packages/{svelte,react}/preview/src/generated/**` (new specimen
  artifacts)
- `packages/{svelte,react}/preview/src/specimens/**` (the five, deleted)
- `packages/{svelte,react}/preview/src/` specimen registry wiring for the
  five
- `packages/{gpui,jetstream}/preview/src/generated/**` (new specimen
  artifacts)
- `packages/{gpui,jetstream}/preview/src/specimens/**` (the five, deleted
  or rehomed to scene consumption)
- `tasks/effigy.tasks.toml`
- `docs/logs/2026-08/14-g14-005-scene-authored-specimen-tranche.md`
- `PAPERCUTS.md` (append only)

## Steps

1. Reset per the Thread Reuse Protocol; baseline
   `effigy ir:build`, `effigy ir:check`, `effigy test:components`,
   `effigy ci:web`, `git diff --check`.
2. Read spec 065 and the b035/b036 logs; read the five hand-written
   specimens in all four runtimes.
3. Author the scene model; add the specimen targets; emit the fixtures.
4. Verify each specimen renders identically (label text, state rows,
   matrices) in both web previews; confirm native previews consume the
   new artifacts.
5. Delete the hand-written specimens; update registries.
6. Measure and record the LOC swap per component against baseline §5.
7. Validate the acceptance gate list; write the batch log; push with
   `git push -u origin thread/g14-005-scene-authored-specimen-tranche`.
   Do not merge.
