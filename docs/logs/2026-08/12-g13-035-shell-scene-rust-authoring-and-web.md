---
title: g13 batch 035 — shell scene Rust authoring and the two web shells
status: complete
milestone: g13.004 (part 1 of 2 — does not close the milestone)
owner: Poodle core
updated: 2026-08-12
tags: [log, g13, IR, scene, shell, authoring, svelte, react, spec-063, g13.004]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/035-shell-scene-rust-authoring-and-web.md` on
branch `thread/g13-035-shell-scene-rust-authoring-and-web`: authored the
`g13.004` preview shell scene in Rust (R1), serialized it to the JSON the
existing pipeline consumes, emitted it to both web packages' `generated/`
directories, and rewired the Svelte and React display-control surfaces to
render from the scene. The native shells (card 036) and the milestone's
close are explicitly not this card's.

Per the card's worker rules: no sub-agents; sources read directly
(`packages/contracts/ir/src/scenes.rs` in full before authoring); no
planning/status authority exercised beyond the card's own writable status
line. No stop condition was reached: every shell capability this card wired
has a Scene IR field (R2 held — no `poodle-ir` change), the authored model
serializes into `load_and_validate`'s existing contract unchanged, sharing
cost no interactivity in either web shell (both previews driven live), and
the parity test is scene-derived, not hand-listed.

## Measured before-state — capability + label matrix (step 2)

Both web shells composed their own control surface; labels were hardcoded
per shell. Measured from the working tree before any change:

| Control | Svelte (`DisplayControls.svelte`) | React (`DisplayControls.tsx`) | Label text (both) | Value surface |
|---|---|---|---|---|
| Theme | `ThemeSelect` over `themeOptions()` | `ToggleGroup` over `Object.keys(themes)` | `Theme` (Eyebrow + ariaLabel) | 12 presets (clay…solarized), default `eclipse` |
| Density | `ToggleGroup`, order compact/default/comfortable | same | `Density` | compact / default / comfortable |
| Size | `ToggleGroup`, order xs–xl | same | `Size` (ariaLabel `Control size`) | xs sm md lg xl |
| Contrast | `Slider` 0.4–1.6 step 0.05 | `Slider` 0.4–1.6 step 0.05 | `Contrast` (ariaLabel `Neutral contrast`) | continuous 0.4–1.6, default 0.5 |
| Search | `TextInput` type=search | `TextInput` type=search | `Search` (ariaLabel `Search components`) | case-insensitive over display name + description (SHELL-06) |

Navigation labels (`Components` / `Tokens` top tabs) were hardcoded in both
`App` files and in `router.ts` / React's `parseRoute`. The card's R4 drift
example (web `Theme`/`Search` vs GPUI `THEME`/`SEARCH`) is a labelling
problem: every capability already has a field, so the label text is a
deterministic projection of the scene (see Design), which 036's native
shells will consume the same way.

## Deliverables (only the card's writable paths)

- `packages/codegen/src/models/preview_shell.rs` (new) — the Rust-authored
  shell scene (`shell_scene()`), the one scene of the pilot: `SHELL-01`–04
  axes (theme/size/density/contrast, named values derived from
  `poodle_ir::{theme_names, control_size_names, density_names}`, contrast
  continuous 0.4–1.6 default 0.5), `SHELL-05` layout (Components/Tokens
  sections, route-state persisted params), `SHELL-06` search config,
  `SHELL-07` tabs, `SHELL-08` preview state, `SHELL-09` parity vocabulary
  (defaults, review presets, four runtime targets, smoke/axis/sweep visual
  gates, native baseline), capture identifiers. `shell_model()` wraps it in
  an `IrModel` with no components/shared types/vectors and a null registry
  (`R5` — no component migration). Module header records the pilot-scoped
  placement (`g13.008` decides where production models are authored) and
  the no-macros rule (spec 063: the pilot has not run).
- `packages/codegen/src/models/mod.rs` (new) — module root.
- `packages/codegen/src/targets/shell.rs` (new) — the `shell-scene` target
  (`output_root` `generated`): one `<scene-id>.ts` per scene, exporting the
  scene as a typed readonly constant. The **labels are the emitter's
  deterministic projection of the scene** (card R4): an axis's label is its
  `SceneAxisKind` display name, search's label/placeholder appear only when
  the scene configures search, a tab's label is its identifier's display
  form. This projection is the single Rust home of the control label text;
  both shells read the artifact verbatim.
- `packages/codegen/src/targets/mod.rs` — `shell-scene` registered in a new
  `selectable()` list, **not** in `all()`: a plain `ir:build` over the
  synthetic fixture must never write into a web package, and the existing
  per-target tests require every `all()` target to emit for the fixture
  (scenes are empty there). `by_id` consults `selectable()`.
- `packages/codegen/src/bin/poodle-codegen.rs` — two additions. `--target
  <ID>` restricts emission (the shell target is select-only); `--author-shell
  <OUT> [--check]` serializes the Rust-authored model to the fixture after a
  validate round trip (the bytes written are exactly the bytes
  `load_and_validate` accepts), with a read-only byte-compare twin. The
  check branches remain structurally incapable of writing.
- `packages/codegen/src/targets/ts.rs` — `format_number` and
  `ts_string_literal` widened to `pub(crate)` for the shell target to reuse.
- `packages/codegen/fixtures/shell-model.json` (new) — the serialized shell
  model, generated by `--author-shell`. `synthetic-model.json` untouched.
- `packages/{svelte,react}/preview/src/generated/preview-shell.ts` (new) —
  the committed artifact both shells consume.
- `packages/svelte/preview/src/components/DisplayControls.svelte` and
  `packages/react/preview/src/gallery/DisplayControls.tsx` — the control
  surface is now the scene's: each control renders from its artifact entry
  (`{#if themeControl}` / `{themeControl && …}` blocks), label text and
  value sets come from the artifact, widget mechanics stay per-shell idiom
  (Svelte keeps `ThemeSelect`, React keeps `ToggleGroup`). Kinds are
  compared as plain strings on purpose: deleting an axis from the scene
  removes the control cleanly instead of producing a literal-typed compile
  error (the R3 removal property).
- `packages/svelte/preview/src/App.svelte` and
  `packages/react/preview/src/gallery/App.tsx` — the top navigation tabs
  are the scene's layout sections (labels `Components`/`Tokens` are now
  scene-supplied, R4). No routing mechanics changed (host-owned, R3).
- `tasks/effigy.tasks.toml` — `ir:build` / `ir:check` are now composite
  selectors: `--author-shell` first (write / byte-compare), then the
  synthetic fixture over `packages/codegen/generated` (unchanged), then the
  shell model through `--target shell-scene` into each web package's
  `generated/`. `ir:check` remains the only gate-shaped selector; `ir:build`
  never composes into a gate (the `45caae82` rule holds).
- `packages/codegen/tests/shell_scene.rs` (new, 7 tests) — see Tests.
- `docs/roadmaps/g13/004-shared-preview-shell-scene-pilot.md` — status line
  only: `planned` → `in progress` (part 1 of 2 landed; 036 closes the
  milestone).
- `PAPERCUTS.md` — two appended entries (svelte SHELL-08 URL sync dead,
  catalogue-landing grids ignore their filtered-components prop).
- This log.

Nothing else in the repo changed. No HistoryCenter, Longhorn, Loophole,
GPUI, Jetstream, or component/specimen file touched; no visual baselines
refreshed; no `poodle-ir` change (R2).

## Design

- **The authoring form.** `preview_shell.rs` is ordinary Rust types and
  constructor helpers (spec 063 "Authoring Form"), no macros. Named axis
  values come from the `poodle-tokens` registries through `poodle-ir`, so
  this module and validation cannot disagree. The scene carries no component
  instances or sidebar groups (`R5`; the `SHELL-10` registry is a later-card
  generated target).
- **The serialization path.** `--author-shell` round-trips the model through
  JSON and `validate()` before writing — the committed fixture is
  pipeline-valid by construction. `--author-shell --check` regenerates in
  memory and byte-compares; it contains no write call.
- **Why the label projection lives in the emitter.** Scene IR has no label
  field and R2 forbids adding one; the *capability* is modeled (axis kind,
  search presence). The label text is therefore the emitter's pure function
  of the scene — authored once, in Rust, reviewed — and both shells read it
  from the artifact. A shell may restyle (uppercase via CSS, the native
  equivalent in 036) but cannot author different text. This is the only
  reading that does not trip the R2 stop condition, and it is what makes the
  card's parity test derivable.
- **The R3 removal property.** The artifact's `controls` array is the shell
  surface. Removing an axis or search from the scene removes the entry from
  both artifacts; each shell's control block renders only when its control
  exists. Kind comparisons are widened to `string` so deletion is a clean
  removal, not a type error (the literal-typed comparison would fail the
  build — the drift gate at compile time — which is not what R3 asks for).
  Proven live (see Validation): contrast axis removed → 4 groups, zero range
  sliders; restored → 5 groups, one slider; identical in both shells.
- **One change, one build, both shells.** Both web artifacts are renders of
  the same model by the same target; `ir:build` regenerates both, so one
  authored value change moves both in one command (proven by test).

## Tests (40 total in `poodle-codegen`, all passing)

`tests/shell_scene.rs` (7 new):

- `shell_model_validates_and_round_trips_as_json` — in-memory validate
  clean; serialization round-trips; the committed fixture equals the
  authored model (the fixture cannot drift from the Rust source without
  `ir:check` failing).
- `shell_scene_authors_every_shell_capability` — the four axes in display
  order with registry-derived values and the continuous contrast range;
  layout sections, search config, preview state (SHELL-01..08 rows).
- `both_web_shells_carry_the_same_scene_derived_artifact` — the card's
  required parity test: the expectation is the shell target's render of the
  authored scene (derived, not hand-listed), and **both** committed web
  artifacts must equal it byte-exact. A shell drifting on capability set or
  label text fails the comparison.
- `artifact_labels_are_a_projection_of_the_scene` — every scene axis's kind
  and label text is present in the artifact; removing search from the scene
  removes the control from the artifact (the R3 projection, proven without
  a hand-written list).
- `artifact_header_names_the_source_definition_and_generator_version` —
  the Generated Artifact Contract: `Source:` fixture path, generator
  version, IR schema version in the header.
- `one_scene_change_moves_both_web_artifacts` — one authored value change
  (contrast default 0.5 → 0.8) moves both committed artifacts in one build.
- `shell_web_artifacts_fail_check_on_drift_and_check_never_writes` — the
  CLI `--target shell-scene --check` fails on planted drift + stale orphan
  and leaves the tree byte-identical.

Existing suites untouched and green: `emission.rs` (14), `targets.rs` (14),
unit tests (5 — including 2 new in `targets/shell.rs` for the humanize and
camel-case helpers).

## Validation (all step-7 commands exit 0)

| Command | Exit state |
|---|---|
| `effigy ir:build` | 0 — authored shell model + synthetic fixture + both web packages |
| `effigy ir:check` | 0 — all current |
| `effigy ci:rust` | 0 — all contract crates green |
| `effigy test:core` | 0 — 496 pass |
| `effigy test:components` | 0 — 1004 pass |
| `effigy test:parity` | 0 — 164 pass |
| `effigy check:svelte` | 0 — install-smoke + 701 component files, 0 errors |
| `effigy docs:lint` | 0 |
| `effigy docs:callback-drift` | 0 — 101 checked, 63 skipped |
| `effigy svelte:surface-audit` | 0 — 176 files, 0 legacy markers, 164/164 coverage |
| `git diff --check` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 40 passed |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 |

**Drift proof (required test).** Planted one line into
`packages/svelte/preview/src/generated/preview-shell.ts` → `effigy ir:check`
exits 1 naming `preview-shell.ts (content drift)` → restored → exits 0.
`--author-shell --check` gates the fixture the same way.

**R3 removal proof (step 6, live).** Removed the contrast axis from
`shell_scene()` → `effigy ir:build` → both artifacts lost the contrast
control → Svelte preview rendered 4 groups (Theme/Density/Size/Search) and
zero range inputs (browser-driven) → restored the axis → both artifacts and
both shells regained it (5 groups, one slider). The nav tabs render the
scene's section labels in both shells.

**Interactivity proof (acceptance: axes stay interactive).** Browser-driven
in both previews: theme (Svelte `ThemeSelect` → `data-theme` + pill, React
`ToggleGroup`), density and size (`aria-checked` + top-bar pills),
contrast slider (`--poodle-contrast` applied), search (sidebar filters to
the matching components, count line reflects the filtered set), and the
Tokens/Components tab navigation. No behaviour lost to gain sharing.

## Acceptance criteria

- [x] The shell scene is authored in Rust, not hand-written JSON (R1).
- [x] `poodle-ir` gained no `[[bin]]` and no new schema field (R1, R2);
  `packages/codegen/fixtures/synthetic-model.json` unchanged.
- [x] Both web shells render their controls and labels from the scene; the
  step-6 removal test proves it (R3, R4).
- [x] `ir:check` exits 0 clean and non-zero on drift (proven both ways).
- [x] Both web previews keep every axis interactive (proven live).
- [x] All step-7 commands exit 0; no baseline refreshed.
- [x] The parity test is scene-derived, not a hand-listed expectation.
- [x] No macros added (spec 063).

## Not done

Per batch card and worker rules: no merge (branch pushed only), no GPUI /
Jetstream shells (036), no component definition or specimen conversion
(`R5`), no HistoryCenter / Longhorn / Loophole file, no visual-baseline
refresh, no schema change. The specimen-tabs and sidebar-group wiring
(`SHELL-07`, `SHELL-10`) are authored in the scene but not consumed this
card: `SpecimenLayout` / `ComponentsSection` are outside the card's writable
paths, and the registry is a later-card generated target. Two pre-existing
web-shell defects observed during validation are recorded in `PAPERCUTS.md`
(Svelte's SHELL-08 URL persistence effect never fires on state changes —
React's does; both catalogue-landing grids ignore their filtered-components
prop and render every component). Both are outside this card's writable
paths and were not fixed here.
