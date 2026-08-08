# Handoff — GPUI Node Backend Lane, Batch B Mid-Wave

For a fresh thread. Written 2026-08-05 by the Poodle-side thread that opened
the g12.019 lane and completed Batch A. Stopped for operator token allowance,
mid Batch B wave 1.

## What This Thread Was Doing

Executing `docs/roadmaps/g12/019-gpui-node-backend.md` — the GPUI half of the
Poodle inversion (the card this thread wrote, promoted from the Jetstream-side
handoff `docs/logs/2026-08/05-201024-gpui-node-backend-handoff.md`). Goal:
build a Node → GPUI interpreter, migrate the GPUI preview onto it, delete
`packages/gpui/components` (the last duplicate component tier). Read the card
first; its Decision Log records the two operator decisions below.

Batch A is done. Batch B wave 1 (call-shape census + button family) was
interrupted: the subagent was killed mid-run, but its partial work is in the
working tree and **builds green with all 64 poodle-render tests passing**.

## Operator Decisions (binding)

1. **Fix `poodle-render` recipes to the axis-faithful form.** The zero-diff
   pilot proved the two old tiers genuinely diverge: the old GPUI tier uses
   axis-layered tokens + per-size offsets (matches Svelte's CSS-var behavior);
   `poodle-render` carried the old Jetstream tier's fixed tables. The old GPUI
   tier is the correctness reference. Jetstream's parity gate is retired, so
   this is Poodle-internal; local visual baselines re-baseline with recorded
   reasons. Select was the proving component; apply the same pattern
   per-specimen as Batch B flags divergences.
2. **The operator runs the pixel gate.** This session had no Screen Recording
   grant, so `bun test/native-visual/run.ts` never ran. In-session proofs are
   builds, `cargo test`, the in-process NSEvent click driver, and side-by-side
   pixel arithmetic at the gate axis (`eclipse-compact-sm`; axis values in
   `packages/tokens/artifacts/rust/density.rs`). The gate passing on migrated
   slugs is the card's acceptance bar — do not call Batch B done without it.

## Current State

Nothing is committed. Working tree contains:

- **Card + front doors (done):** `docs/roadmaps/g12/019-gpui-node-backend.md`
  (untracked); `docs/contracts/001-working-rules.md`, `docs/roadmaps/README.md`,
  `docs/roadmaps/g12/README.md`, `docs/roadmaps/generation-index.md` updated to
  point at it.
- **Batch A (done):** `packages/gpui/node-backend/` (`poodle-gpui-node-backend`,
  ~700 lines, untracked): `to_gpui(&Node) -> gpui::AnyElement`, the full channel
  walk transcribed from `jetstream:crates/jetstream-poodle/src/lib.rs`; color is
  raw sRGB passthrough (unit-tested, no transfer function); 4 unit tests green.
  `packages/gpui/preview/src/specimens/select.rs` is migrated through
  `poodle_render::select` + `to_gpui`, with a `NodeSpecimenEvent` queue in
  `app_state.rs` drained in `PreviewRoot::render` and `print_specimen_state`
  (the established shim seam — generalize it). `packages/render/src/select.rs`
  reconciled to the axis-faithful recipe (verified by arithmetic: 28px trigger,
  8px pad-x, ladder 20/22/28/34/36 at the gate axis; placeholder =
  `color.text.secondary`). Click-driver toggle proof passed for select.
- **Batch B wave 1 (PARTIAL, killed mid-run):** modified but not yet reviewed
  by anyone — `packages/render/src/{button,icon_button,split_button,toggle_group,
  card_toggle_group,segmented_control}.rs` (recipe reconciliations) and
  `packages/gpui/preview/src/specimens/{button,icon_button,split_button,
  toggle_group,card_toggle_group_specimen,segmented_control}.rs` (migrations),
  plus preview `Cargo.toml`. It compiles and the 64 render tests pass, but the
  recipe arithmetic, click-driver proofs, and the call-shape census report were
  never delivered. **First job: review these diffs and re-derive the proofs**
  (or redo them) before building on them.
- Untracked `test/native-visual/out-jetstream/` is generated gate output;
  check `.gitignore` intent before ever committing it.

## Boundaries

- Never modify the sibling Jetstream repo (`/Users/tom/Dev/projects/jetstream`);
  reading `crates/jetstream-poodle/src/lib.rs` as the transcription reference
  is required.
- Do NOT delete `packages/gpui/components` until every specimen is migrated
  and the pixel gate is green (Batch C; order: migrate → gate → delete).
- Vocabulary changes additive only; `poodle-node` never names a backend crate.
  `packages/render/src/presentation.rs` fixed tables are shared by ~148
  components — add axis-aware helpers or per-component recipes; do not change
  other components' behavior as a side effect.
- Do NOT run `cargo fmt` in `packages/gpui/preview` — the tree is not
  rustfmt-clean. No git commits without explicit operator approval.
- macOS-only build/test surface; gpui 0.2.2 from crates.io.

## Important Context

- **Placeholder verdict (settled):** `color.text.placeholder` does not exist
  anywhere; Svelte's `--poodle-recipe-select-placeholder-*` vars are never
  defined and fall back to text-secondary; the contracts crate already encodes
  `placeholder_color_token() → COLOR_TEXT_SECONDARY` for other components.
  Resolution: resolve to `color.text.secondary` in render code; no token or
  provider changes.
- **Recipe reconciliation pattern:** replace `control_height_rem(size)` /
  `control_space_x_rem(density)` fixed tables with
  `theme.resolve_space("size.control.height"/"space.control.x") +
  rem_to_px(size_*_offset_rem(size))`, transcribing the old GPUI tier exactly
  (`packages/gpui/components/src/...`; helpers in
  `components/src/presentation/metrics_a.rs`). Alpha-scaled fills (×0.82/×0.72,
  hover ×0.88/×0.92) where the old tier has them. Update the render crate's
  component tests to the corrected recipes.
- **Reconciliation census:** `control_space_x_rem` — 31 files;
  `control_height_rem` — 21 files (62 usages; lists in the wave-1 brief, or
  re-run `grep -l`). `resolve_semantic_size` (77 files) and `size_font_rem`
  (43 files) are axis-compatible, no action. button/time_field/tab_strip
  already use offset helpers anchored to fixed tables — re-anchor to tokens.
- **Interpreter approximations** (flagged inline in node-backend; per-component
  judgment calls during migration): per-side border colors collapse to one,
  inset shadows dropped, >2-stop gradients keep endpoints, `overlay` renders
  as a normal absolute child (what the old tier did), grayscale and
  letter-spacing dropped, transform animations svg-only, `Input` renders value
  as text (no real `Editor` entity from a pure `&Node→element` fn), a11y roles
  unmapped (gpui 0.2.2 has no ARIA; g12.015 holds this), drag End phase never
  emitted (no mouse-up listener).
- **Select open-state divergence (recorded, not fixed):** node tier keeps the
  Jetstream panel anatomy (check icons, pad_x rows, group headers); old tier
  had different option padding, separators, search row. The gate captures
  closed states only — record such divergences per component, don't chase them
  unless the gate or operator says otherwise.
- **Click driver:** `packages/gpui/preview/target/debug/poodle-preview
  --click x,y[/x2/y2...] --print-state <slug>`. Toggle proofs work headless;
  open→choose sequences do NOT (window occlusion stops the display link after
  the first click — environmental, not a defect). Coordinates can be measured
  off baseline PNGs; the driver has affine calibration.
- **Preview dispatch:** `specimens/mod.rs::render_single_specimen(slug,
  &AppState, &mut Context<PreviewRoot>)` — one match arm per slug; migrate at
  the per-specimen render-fn seam, keep chrome (Eyebrow, captions,
  `specimen_card`) untouched, extend `NodeSpecimenEvent` per family.
- **Trap ledger:** node containers default Column (gpui divs default row —
  always emit direction); `.grow()` is flex not width; builder-call order was
  semantic in old tiers; `StyleDescriptor::default()` is invisible, nodes use
  `new()`; three color-mix recipes exist — `packages/render/src/color.rs` is
  the authority, the backend never mixes.
- **Subagent note:** resumed-agent state did not survive a session restart
  once already (resume failed with "does not exist"). Brief fresh agents with
  this file + the card instead of relying on resume.

## Suggested Next Move

1. Review the uncommitted wave-1 diffs (6 render recipe files, 6 specimen
   files): confirm each recipe change against the old tier with pixel
   arithmetic at the gate axis, run the click-driver toggle proofs per
   migrated specimen, keep `cargo test -p poodle-render` and the preview build
   green. Redo whatever doesn't hold up.
2. Redo the call-shape census the killed agent never reported
   (`grep -rho '[A-Z][A-Za-z]*::from_spec' packages/gpui/preview/src/specimens
   | sort | uniq -c | sort -rn`, plus free-fn shapes) and lay out the
   remaining family waves from it.
3. Ask the operator to run `bun test/native-visual/run.ts
   --slug=select,button,icon_button,split_button,toggle_group,card_toggle_group,segmented_control`
   in a Screen-Recording-permitted terminal — the first real pixel checkpoint.
4. Continue family waves (per-specimen: reconcile recipe → migrate specimen →
   arithmetic + click proof), operator pixel-gate at each checkpoint.
5. Batch C when every specimen is migrated and the gate is green: drop the
   dep, delete `packages/gpui/components`, port mined probe tests, closeout.

## Completion Protocol

- Update the card's Execution Plan checkboxes as batches land; mark Batch B
  done only with the pixel gate green on all migrated slugs (operator-run).
- On completion: log the deletion in `docs/logs/`, update
  `docs/roadmaps/generation-index.md` + `docs/roadmaps/README.md` +
  `docs/contracts/001-working-rules.md` posture, and leave a one-line pointer
  in `jetstream:docs/roadmaps/g06/013-poodle-node-backend.md` that the last
  duplicate tier is gone (the only permitted Jetstream-repo edit).
- Unresolved risks to carry: the pixel gate has never validated any of this
  work (operator checkpoint pending); the wave-1 diffs are unreviewed;
  open-state anatomies diverge per component (recorded list grows in Batch B);
  gpui 0.2.2 API drift judgment calls in the interpreter approximation list.
