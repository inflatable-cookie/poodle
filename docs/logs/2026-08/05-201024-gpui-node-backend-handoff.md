# Handoff — GPUI Node Backend Lane

For a fresh thread. Written 2026-08-05 by the Jetstream-side thread that
completed the g06.013 inversion (Batches E + F).

## What This Thread Was Doing

The parent thread lived in the Jetstream repo and executed the
Poodle↔Jetstream inversion end-to-end: it ported all 149 components of the
old `poodle-jetstream-components` JsEl tier into `packages/render`
(`Spec + Theme → poodle_node::Node`), proved every port with a byte-exact
draw-command parity gate (190 cases), migrated the Jetstream specimen preview
onto the node tier, then deleted the old tier and retired the gate. The
render vocabulary (`packages/contracts/node`) is now the single component
authority, and Jetstream consumes it through its own `jetstream-poodle`
(Node → JsEl) adapter.

This handoff opens the symmetric lane on the GPUI side: **build a
Node → GPUI backend, migrate the GPUI preview onto it, then delete
`packages/gpui/components`** — the last duplicate component tier.

## Why It Matters

Two component tiers still exist for GPUI: `poodle-render` (canonical, node
vocabulary) and `packages/gpui/components` (170 files, a full hand-written
GPUI implementation of the same components). Every contract change currently
has to be made twice or silently diverges. Once GPUI renders nodes, Poodle
has exactly one component implementation with N thin backends — the end state
the whole inversion (`jetstream:docs/roadmaps/g06/013-poodle-node-backend.md`)
was aiming at. It also unlocks deleting ~½ of the GPUI package tree.

## Current State

- **Finished:** the Jetstream half is complete and pushed. Poodle
  `ee704699` deleted the old Jetstream tier; `03cf8aa5` migrated the
  Jetstream preview; Jetstream `4dcb233c`/`05457be9` retired the parity gate
  and marked Batch F done. `scripts/check-sibling-boundaries.sh` (Jetstream
  repo) is green on the post-deletion graph.
- **Not started:** everything GPUI-side. No Node → GPUI interpreter exists.
  `packages/gpui/adapter` (`poodle-gpui`) is a *theme/spec-level* adapter
  (ThemeProvider impl + style helpers), not a node interpreter — do not
  confuse the two.
- **No roadmap card yet.** The active generation is `g12` (React conversion
  lane, `docs/roadmaps/g12/`). This lane needs its own card — either a g12
  addition or the seed of the next generation; that placement is the first
  planning decision.
- Key artifacts (absolute paths):
  - Reference implementation of a node backend:
    `/Users/tom/Dev/projects/jetstream/crates/jetstream-poodle/src/lib.rs`
    (~600 lines; the complete vocabulary walk: kinds, layout channels,
    per-side borders, gradients, shadow layers, hover/active patches,
    positions/overlay, a11y roles, interaction closures, animations).
  - The vocabulary: `/Users/tom/Dev/projects/poodle/packages/contracts/node/src/lib.rs`.
  - Canonical components: `/Users/tom/Dev/projects/poodle/packages/render/src/`.
  - The tier to delete: `/Users/tom/Dev/projects/poodle/packages/gpui/components/`
    (170 files); its consumer `/Users/tom/Dev/projects/poodle/packages/gpui/preview/`
    (152 files, macOS-only, gpui 0.2.2 from crates.io).
  - Migration playbook from the Jetstream side (reusable shape):
    `/Users/tom/Dev/projects/poodle/packages/jetstream/preview/src/{nel.rs,compat.rs,jsx.rs}` —
    chrome shim + generated `js_*`-style compat wrappers + framework-side
    helpers, one adapter conversion at the specimen dispatch boundary.

## Boundaries

- **Out of scope: touching the Jetstream repo.** The Jetstream backend is
  done; this lane is Poodle-internal. Also out of scope: npm publishes
  (g06.014, operator-side) and the React/g12 conversion lane.
- **Do not delete `packages/gpui/components` before the GPUI preview runs on
  the new backend** — the preview is its only consumer and the natural
  verification surface (same order the Jetstream side used: migrate preview
  → retire gate → delete).
- The node vocabulary is **sRGB; backends convert at their own edge**. GPUI
  wants sRGB-ish `Hsla`/`Rgba`, so the GPUI backend likely needs *no*
  transfer-function conversion — verify against `poodle-gpui`'s existing
  color handling before assuming either way. Getting this wrong is the #1
  known failure mode (see the Jetstream parity history).
- Vocabulary changes must stay additive, and `poodle-node` must never name a
  backend crate.
- macOS-only build/test surface (GPUI): plan verification accordingly. The
  GPUI preview has an in-process click driver (NSEvent-based) — reuse it.

## Important Context

- **Lineage:** the inversion master doc is
  `jetstream:docs/roadmaps/g06/013-poodle-node-backend.md` (Batches A–F all
  checked). Poodle-side history: g09 merged spec crates (`g09/004` deleted
  the GPUI *spec* duplicates — the *components* tier survived that pass);
  `docs/roadmaps/generation-index.md` says treat g12 as active until closed.
- **Verification strategy decision (open):** the Jetstream side had a
  byte-exact draw-command gate because both paths rendered through one
  pipeline. GPUI has no draw-command dump harness in-repo. Options:
  (a) GPUI element-tree structural diff old-vs-new, (b) pixel snapshots via
  the preview's screenshot path if one exists, (c) accept
  render-probe-style assertions + visual QA. The old GPUI components have
  probe-style tests worth mining. Pick deliberately and record it in the
  card; don't drift into "it compiles".
- **Compat-shim precedent:** the Jetstream preview migration generated its
  compat layer by aligning old/new fn signatures programmatically (script in
  thread history; the pattern is reproducible in an afternoon). The GPUI
  preview likely calls the old tier through builder structs
  (`X::from_spec(...).on_y(...)`) as well as free fns — census first, as the
  parent thread did (`grep -o` census of call shapes before writing any
  shim).
- **Handler mapping:** node interaction closures (`on_activate`, `on_drag`
  with per-frame deltas, hover/active `StylePatch`) must map to GPUI's
  listener model. The old GPUI components already solved
  drag/click/hover per-component; the backend centralises it once.
- **Trap ledger** (from 149 ports; lives in the Jetstream thread's memory,
  summarised): node containers default Column while both old fluent surfaces
  defaulted Row (the "silent-Row" trap); `.grow()`-style calls are flex
  props, not width; builder-call order was semantic in the old tiers; three
  distinct color-mix recipes exist (`mix_srgb`, `mix_linear`, alpha tint) —
  `packages/render/src/color.rs` is the authority.
- Sibling threads: the Poodle thread is dormant (operator confirmed
  2026-08-05) — this lane was operator-cleared for autonomous execution.

## Suggested Next Move

1. Write the lane card (placement decision: g12 addendum vs new generation),
   including the chosen verification strategy.
2. Build `packages/gpui/node-backend` (name it deliberately; `poodle-gpui`
   is taken by the theme adapter): a `to_gpui(&Node) -> impl IntoElement`
   interpreter, transcribing the channel walk from
   `jetstream-poodle/src/lib.rs` onto GPUI's fluent API. Start with the
   Select fixture set — same pilot the Jetstream side used to prove the
   vocabulary.
3. Then the preview migration replay: census call shapes → chrome shim +
   compat wrappers → per-specimen conversion → visual QA → drop the dep →
   delete `packages/gpui/components`.

## Completion Protocol

- The Jetstream-side surfaces already reflect the stopping point: g06.013
  marks Batches A–F complete and explicitly scopes `gpui/components` out of
  Batch F with a pointer to this lane.
- This lane has **no card yet** — creating one is step 1 above; the roadmap
  spine is not yet authoritative for this work.
- On completion: delete the duplicate tier, log the deletion in
  `docs/logs/`, update `generation-index.md` if a new generation was opened,
  and note in the g06.013 roadmap (Jetstream repo) that the last duplicate
  tier is gone.
- Unresolved risks to carry: verification-strategy choice (above), GPUI
  color-space assumption, and gpui 0.2.2 API drift vs what the old
  components were written against.
