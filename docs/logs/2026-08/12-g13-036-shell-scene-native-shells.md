---
title: g13 batch 036 — shell scene native shells (GPUI and Jetstream)
status: complete
milestone: g13.004 (part 2 of 2 — closes the milestone)
owner: Poodle core
updated: 2026-08-12
tags: [log, g13, IR, scene, shell, gpui, jetstream, spec-063, g13.004]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/036-shell-scene-native-shells.md` on branch
`thread/g13-036-shell-scene-native-shells`: added the `shell-rust` emitter
target (R2 — a sibling of `shell-scene`, which is byte-frozen), emitted a
self-contained Rust artifact to both native previews' `generated/`
directories (R1 — plain data, no `use` of any Poodle crate, pulled in via
the `poodle-tokens` `#[path]` mechanism), and rewired the GPUI and
Jetstream display-control surfaces to render from the scene. The removal
test (R4) was repeated live across all four shells, both natives were
screenshotted (R5), and `g13.004` closed.

Per the card's worker rules: no sub-agents; sources read directly; no
planning/status authority exercised beyond the card's own writable status
line. No stop condition was reached: the artifact carries everything the
natives need without a Poodle crate import (R1 held), every control stays
drivable from generated data with runtime state ownership in the host (R4
held), and no native visual baseline moved — the visible label text in both
natives is unchanged (casing is styling, R3), so nothing was refreshed.

## Environment note — the missing sibling symlink

The Jetstream preview builds through the sibling jetstream repo, whose
`jetstream-poodle` crate resolves its Poodle path deps at
`../../../poodle/packages/…`. In this workspace the `poodle-wt/jetstream`
symlink existed but `poodle-wt/poodle` did not, so cargo resolved that
path to a nonexistent directory and every Jetstream build failed with
`failed to read …/poodle-wt/poodle/packages/contracts/node/Cargo.toml`.
Created `poodle-wt/poodle -> /Users/tom/Dev/projects/poodle-wt/b036` (the
same pattern as the existing `jetstream` symlink; the entry did not exist,
so nothing was clobbered). Cargo then rejects two *spellings* of the same
tree (the preview's own `../../contracts/…` vs the sibling's
`../../../poodle/…`), so Jetstream builds run with
`--manifest-path /Users/tom/Dev/projects/poodle-wt/poodle/packages/jetstream/preview/Cargo.toml`
— one spelling, no lockfile collision. This is an environment repair, not
a repo change; nothing of it is staged.

## Measured before-state — capability + label matrix (step 2)

Both natives composed their own control surface; labels were hardcoded per
shell. Measured from the working tree before any change:

| Control | Jetstream (`preview/src/shell.rs`) | GPUI (`preview/src/main.rs`) | Scene projection (artifact) |
|---|---|---|---|
| Theme | `Eyebrow "Theme"` + ThemeSelect (12 presets) | eyebrow `"THEME"` + ThemeSelect (12 presets) | `Theme` |
| Density | `Eyebrow "Density"` + ToggleGroup over `Density::ALL` (comfortable/compact — host subset of 3) | toggle group label `"Density"` uppercased → `DENSITY` (3 values) | `Density` |
| Size | `Eyebrow "Size"` + ToggleGroup over `ControlSize::ALL` | toggle group label `"Size"` → `SIZE` | `Size` |
| Contrast | caption `"CONTRAST"` + slider 0.0–1.0, aria `"Contrast"` | header `"CONTRAST  0.25"` + slider 0.0–1.0 (`CONTRAST_MIN/MAX`), aria **`"Neutral contrast"`** | `Contrast` |
| Search | caption `"SEARCH"` + text input, placeholder `"Find component..."`, aria `"Search components"` | eyebrow `"SEARCH"` + TextInput, placeholder `"Find component..."`, aria `"Search components"` | `Search` (placeholder `Find component...`) |

The card's three GPUI deltas: `THEME`/`SEARCH` uppercase (casing —
presentation) and contrast aria `Neutral contrast` (different text — R3).
Jetstream's labels agreed modulo casing. Both natives hold runtime state
(theme preset, density, size, contrast value, search string) in host
`AppState`; that ownership is unchanged (R4).

## Deliverables (only the card's writable paths)

- `packages/codegen/src/targets/shell_rust.rs` (new) — the `shell-rust`
  sibling target (R2): the same scene rendered as a self-contained Rust
  artifact — one `<scene-id>.rs` per scene, a `pub static` of plain data
  structs (`ShellScene`/`ShellControl`/…), zero `use` of any Poodle crate
  (R1). `#![allow(dead_code)]` by design: the artifact is the scene, and a
  host consumes the subset it renders. Rust string literals use Rust
  escaping (JSON's `\u00XX`/`\b`/`\f` would not compile). The label
  projection is shared with `shell-scene` via `pub(crate)`
  `axis_label`/`kind_name`/`humanize` — one home for the label text; the
  TS target's output is untouched (verified byte-identical by `ir:check`).
- `packages/codegen/src/targets/mod.rs` — `shell-rust` registered in
  `selectable()`, not `all()` (same scoping rule as `shell-scene`: a plain
  `ir:build` over the synthetic fixture never writes into a consumer
  package).
- `packages/codegen/src/bin/poodle-codegen.rs` — usage text names
  `shell-rust` as select-only.
- `packages/codegen/src/targets/shell.rs` — visibility only (`pub(crate)`
  on the three projection helpers); no output change.
- `packages/gpui/preview/src/generated/preview-shell.rs` and
  `packages/jetstream/preview/src/generated/preview-shell.rs` (new) — the
  committed Rust artifacts, byte-identical across both packages (one target,
  one scene).
- `packages/gpui/preview/src/main.rs` — `#[path = "generated/preview-shell.rs"]`
  module include; `render_display_controls` now finds each control by kind
  in `generated_shell::PREVIEW_SHELL.controls` and renders the group only
  when present, labels from the artifact (uppercased as this shell's house
  look), contrast aria label now the scene's `Contrast` (R3), search
  placeholder from the artifact. Widget mechanics, value sets, handlers,
  and runtime state untouched.
- `packages/jetstream/preview/src/shell.rs` — same `#[path]` include;
  `build_controls_bar` renders each group only when its kind exists in the
  artifact; theme/density/size eyebrows and the contrast/search captions
  read the scene labels (captions uppercased as house style), contrast aria
  and search placeholder from the artifact. `Density::ALL`/`ControlSize::ALL`
  value sets stay host-owned.
- `tasks/effigy.tasks.toml` — `ir:build` / `ir:check` gain the two
  `--target shell-rust` steps (GPUI and Jetstream packages).
- `packages/codegen/tests/shell_scene.rs` — see Tests.
- `docs/roadmaps/g13/004-shared-preview-shell-scene-pilot.md` — status line
  only: `in progress` → `complete` (036 closes the milestone).
- `PAPERCUTS.md` — one entry: the web shells still hardcode aria labels
  (`Neutral contrast`/`Search components`/`Control size`) while the scene
  owns the visible labels; the natives are aligned.
- This log.

Nothing else in the repo changed: no `poodle-ir`/`poodle-codegen` dep in
either native manifest (asserted by test, not just avoided), no
`targets/shell.rs` output change, `synthetic-model.json` untouched, no
component/specimen/HistoryCenter file, no visual baseline refreshed, no
web-shell source touched.

## Design

- **The artifact is the scene, not a slice.** `shell-rust` renders the full
  scene (controls, tabs, layout, preview state, parity vocabulary,
  captures) in Rust shape, mirroring the TS artifact's information content;
  the natives consume the control surface now and later cards can consume
  the rest. It is pulled in by `#[path]`, so **no manifest change was
  needed** — which is what makes R1 hold without touching either
  `Cargo.toml`.
- **Presence + labels from the scene; mechanics + state in the host.**
  Both natives keep the web pattern from b035: kinds compared as plain
  strings so deleting an axis or search from the scene removes the control
  cleanly instead of becoming a compile error; widget choice, value sets
  (`Density::ALL` etc.), ranges (natives' 0.0–1.0 contrast axis vs the
  scene's 0.4–1.6 web axis), handlers, and runtime state stay host-owned
  (R4). Casing is presentation: both natives render the scene's labels in
  their established style (GPUI uppercases all eyebrows; Jetstream uppercases
  the contrast/search captions), so **no pixel changes** — the visual
  baseline does not move (stop condition not triggered).
- **R3's third delta.** GPUI's contrast aria label was `Neutral contrast` —
  different text, not casing. It is now the scene's `Contrast`, matching
  Jetstream's existing aria label. The web shells' hardcoded aria labels are
  outside this card's writable paths; recorded in `PAPERCUTS.md`.

## Tests (12 in `poodle-codegen` shell_scene.rs — 10 tests + 2 helpers; all green)

`tests/shell_scene.rs` (7 → 10):

- `both_native_shells_carry_the_same_scene_derived_artifact` — the card's
  parity test for the natives: the expected artifact is the `shell-rust`
  target's render of the authored scene (derived, never hand-listed), and
  both committed native artifacts must equal it byte-exact. Also asserts the
  artifact imports no Poodle crate (R1).
- `shell_rust_artifacts_fail_check_on_drift_and_check_never_writes` —
  planted drift + orphan in a Rust artifact fails `ir:check` and check mode
  leaves the tree byte-identical.
- `native_preview_manifests_carry_no_poodle_ir_or_codegen_dependency` —
  reads both preview `Cargo.toml`s and asserts neither names `poodle-ir` or
  `poodle-codegen` (R1 asserted, not just avoided).
- `artifact_labels_are_a_projection_of_the_scene` — extended: the Rust
  artifact carries the same kinds/labels as the TS artifact, and deleting
  search from the scene removes the control from both.
- `artifact_header_names_the_source_definition_and_generator_version` —
  extended to both artifact shapes.
- `one_scene_change_moves_both_web_artifacts` → `one_scene_change_moves_all_four_artifacts`
  — one authored value change moves all four committed artifacts.

Also: `cargo test` for `poodle-codegen` all green (54 total), clippy
`-D warnings` and `cargo fmt --check` clean. Jetstream preview lib tests
green, including the new `the_controls_bar_renders_exactly_the_scene_controls`
(R4 agreement test: every scene control renders a keyed widget —
`theme-select-trigger`, `toggle:{value}` options for the host density/size
sets, `contrast`, `search` — so a control deleted from the scene cannot
silently stay composed). GPUI preview `cargo check` clean.

## Validation (all step-7 commands exit 0)

| Command | Exit state |
|---|---|
| `effigy ir:build` | 0 — four artifacts (2 TS web + 2 Rust native) |
| `effigy ir:check` | 0 — all current |
| `effigy ci:rust` | 0 |
| `effigy ci:web` | 0 |
| `effigy test:parity` | 0 |
| `effigy check:svelte` | 0 |
| `effigy docs:lint` | 0 |
| `effigy docs:callback-drift` | 0 |
| `git diff --check` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 54 passed |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 |
| `cargo test --manifest-path <poodle-wt/poodle/…>/packages/jetstream/preview/Cargo.toml --lib` | 0 — 2 passed, 1 ignored (diagnostic) |
| `cargo check -p poodle-gpui-preview` | 0 |

**Rust drift proof (required test, live).** Planted one line into
`packages/gpui/preview/src/generated/preview-shell.rs` → `effigy ir:check`
exits 1 naming `preview-shell.rs (content drift)` under target `shell-rust`
→ restored → exits 0, both native artifacts byte-identical. (The test suite
covers the same property for a scratch tree.)

**One-value-change proof (required test, live).** Changed the authored
`preview_state.contrast` 0.5 → 0.8 in `preview_shell.rs` → one
`effigy ir:build` moved all four artifacts (`contrast: 0.8` in both TS
artifacts, `contrast: Some(0.8)` in both Rust artifacts) → reverted →
`ir:check` 0.

**R4 removal proof (step 6, live across all four shells).** Removed the
Density axis from `shell_scene()` → `effigy ir:build` → all four committed
artifacts lost the density control (`kind: "density"` count 0 in each) →
observed live:

- **Svelte** (browser, fresh dev server on 4174 — note: a stale pre-existing
  vite server on 4173 served a cached module graph and still showed five
  groups; re-verified on a clean port): 4 control groups — Theme, Size,
  Contrast, Search. Density gone.
- **React** (browser, port 4181): 4 control groups — Theme, Size, Contrast,
  Search. Density gone.
- **GPUI** (`--screenshot`, OCR): controls bar shows `THEME`, `SIZE`,
  `CONTRAST 0.25`, `SEARCH` — no density group, no compact/default/
  comfortable row. Nav tabs (`Components`/`Tokens`), status pills
  (`eclipse`/`compact`/`sm`), and the catalogue below unchanged.
- **Jetstream** (`snap specimens` → `_shell.png`, quads-only): pixel diff
  between the 5-control and 4-control renders is confined to the controls
  bar — bounding box (155, 61)–(1011, 102), 12,499 changed pixels; every
  pixel below the bar and in the tab bar is byte-identical. The density
  group's quads vanished and the remaining groups reflowed.

Restored the axis → one `ir:build` → all four artifacts regained density,
both web shells back to 5 groups (browser), `ir:check` 0, Jetstream R4 test
green against the restored artifact.

**R5 screenshots (step 7).** Both natives captured with all five controls:

- GPUI `/tmp/g13-036-gpui-shell-5.png` (1280×1600 window): OCR reads the
  nav tabs `Components`/`Tokens`, status pills `eclipse compact sm`, and
  the five control groups `THEME DENSITY SIZE CONTRAST 0.25 SEARCH`, plus
  the component catalogue (`Browse the full Poodle component library …
  155 components`). Label text is unchanged from the before-state — the
  scene's words, house-styled.
- Jetstream `/tmp/g13-036-jetstream-shell-5.png` (1500×900, headless): the
  shell renders (tab bar, controls bar with five groups, sidebar, landing);
  quads-only output, so layout/elevation evidence (used for the step-6
  diff above).

**Interactivity.** No mechanism changed: GPUI's `ThemeSelect`/toggle-group/
`Slider`/`TextInput` handlers and ids (`density-*`, `size-*`, `contrast`,
`component-search`) are preserved verbatim; Jetstream's token keys
(`theme-select-trigger`, `theme-select-tile-*`, `toggle:*`, `contrast`,
`search`) and `parse_action` routing are untouched, and the R4 test proves
the keys the dispatcher routes on still materialise. The web pair is
unchanged code (b035 proved its axes interactive live). State ownership
stays in host `AppState` in both natives.

## Acceptance criteria

- [x] All four shells render their controls and labels from one scene
  (R3/R4; removal test across all four, recorded above).
- [x] Neither native preview depends on `poodle-ir` or `poodle-codegen`
  (asserted by test; artifacts pulled in by `#[path]`).
- [x] `targets/shell.rs` and `synthetic-model.json` unchanged (only helper
  visibility; web artifacts byte-identical — `ir:check` + parity tests).
- [x] `ir:check` exits 0 clean and non-zero on drift in a Rust artifact
  (proven both ways, live).
- [x] The step-6 removal test drops the control from all four.
- [x] Both natives screenshotted; both keep every axis interactive.
- [x] `g13.004` marked complete.
- [x] All step-8 commands exit 0; no baseline refreshed.

## Not done

Per batch card and worker rules: no merge (branch pushed only), no
component definition or specimen conversion (R6 — `g13.005` is the first
component slice), no HistoryCenter/Longhorn/Loophole file, no visual
baseline refresh, no schema change, no `poodle-ir` change. The web shells'
hardcoded aria labels and the Jetstream density value subset
(comfortable/compact vs the scene's three) are pre-existing host
differences recorded in the matrix and `PAPERCUTS.md`, not fixed here.
