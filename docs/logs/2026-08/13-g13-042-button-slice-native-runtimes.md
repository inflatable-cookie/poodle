---
title: g13 batch 042 — Button vertical slice, GPUI and Jetstream through `poodle-render`
status: complete
milestone: g13.005 (part 2 of 2 — closes the milestone)
owner: Poodle core
updated: 2026-08-13
tags: [log, g13, IR, button, component, gpui, jetstream, render, spec-063, g13.005]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/042-button-slice-native-runtimes.md` on branch
`thread/g13-042-button-slice-native-runtimes`: added the `button-rust`
emitter target (R2 — a sibling of `button-ts`, which is byte-frozen by
b041's tests), emitted a self-contained Rust artifact into
`packages/render/src/generated/` (R1/R1a — plain data, no `use` of any
Poodle crate, pulled in via `#[path]`, in the package that ships it), and
rewired `poodle-render::button` to take its vocabulary — variants, tones,
the state-attribute names, and their value domains — from the generated
definition instead of its own literals (R3). The four-runtime proof ran
live twice: a `data-loading` rename moved both web previews' DOM and the
GPUI render visibly (the Jetstream snap's fixed 640px viewport clips
Button's States row, so that leg was not observable in the snap — recorded
below), and a `data-has-leading` rename moved **all four** previews. Both
natives were screenshotted (R5), the exception inventory for both natives
was written (R6), and `g13.005` closed.

Per the card's worker rules: no sub-agents; sources read directly
(`b036`'s and `b041`'s logs, the `shell-rust` target, the authored
`button.rs` model, both native Button specimens); no planning/status
authority exercised beyond the card's own writable status line. One stop
condition was examined and classified, not triggered: the GPUI visual
baseline differs from a current capture, and the delta is a stale baseline,
not a moved render (evidence below).

## Environment note — the Jetstream sibling symlink, repointed for the proof

As `b036` recorded, `jetstream-poodle` is a sibling-repo path dep
(`../../../../jetstream/…`) that only resolves cleanly with one spelling of
the tree. The current `poodle-wt/poodle -> /Users/tom/Dev/projects/poodle`
symlink points at the **main** checkout, so a Jetstream build through it
compiles the main repo's `poodle-render` — not this card's. The card's R3
proof and R5 screenshots need Jetstream to exercise **this worktree's**
render, so the symlink was temporarily repointed at
`/Users/tom/Dev/projects/poodle-wt/b042` (the exact `b036` pattern) for the
Jetstream captures, then restored to `/Users/tom/Dev/projects/poodle`. All
Jetstream invocations used
`--manifest-path /Users/tom/Dev/projects/poodle-wt/poodle/packages/jetstream/preview/Cargo.toml`
(one spelling, no lockfile collision). Environment repair, not a repo
change; nothing of it is staged. (`bun test/native-visual/jetstream.ts`
cannot run from a worktree — its cwd-based `cargo run` hits the
two-spellings collision even with the symlink pointed at the worktree; the
manual manifest-path invocation is the worktree-equivalent.)

## Measured before-state — the vocabulary `render/src/button.rs` hard-coded (step 1)

`packages/render/src/button.rs` (621 lines) discriminated on its own
literals: the status tones (`ButtonTone::Danger/Success/Warning`), the
variant treatments (`ButtonVariant::Primary/Secondary/Ghost`), the density
metrics (`ControlDensity::Compact/Default/Comfortable`), and the state
treatments behind the eleven `data-*` attributes (loading spinner +
suppression, pressed accent, icon-only square, has-leading/has-trailing
padding and children). The definition (`packages/codegen/src/models/button.rs`)
owns the vocabulary: `button-variant`/`button-tone`/`control-density`
shared-type members, and the attribute table (names, forms, emission
policies, value domains).

## Deliverables (only the card's writable paths)

- `packages/codegen/src/targets/button_rust.rs` (new) — the `button-rust`
  sibling target (R2): the same definition as a self-contained Rust
  artifact — one `<component-id>.rs`, a `pub static BUTTON_DEFINITION` of
  plain data structs (`ButtonDefinition`/`ButtonPart`/`ButtonAttribute`/
  `ButtonRecipeLink`/`ButtonRecipeHook`), zero `use` of any Poodle crate
  (R1), `#![allow(dead_code)]` by design (the artifact is the definition; a
  host consumes the subset it renders). The vocabulary projections are
  shared with `button-ts` via the widened `pub(crate)` helpers
  (`part_class_name`, `emission_name`, `form_name`, `link_kind_name`,
  `source_prop`, `value_visual_field`, `attribute_values`, `prop_domain`,
  `field_domain`, `shared_member_names`) plus `shell_rust`'s
  `rust_string_literal`/`static_name` — one home for each projection; the
  emitted `button-ts` bytes did not move (`ir:check` + the b041 parity
  tests prove it). Emitted entries are multi-line so the artifact is
  rustfmt-clean (the `shell-rust` precedent).
- `packages/codegen/src/targets/mod.rs` — `button-rust` registered in
  `selectable()`, not `all()` (same scoping rule: a plain `ir:build` over
  the synthetic fixture never writes into a consumer package).
- `packages/codegen/src/targets/button.rs` and `targets/shell_rust.rs` —
  visibility only (`pub(crate)` on the shared helpers); no output change.
- `packages/render/src/generated/button.rs` (new, committed) — the
  artifact: `variants`/`tones`/`densities` member lists, six parts with the
  DOM class projection, eleven attributes with names/forms/emission/value
  domains, and the 76 recipe-hook chains. `ir:check` gates drift in it.
- `packages/render/src/lib.rs` — `pub mod generated { #[path = "button.rs"]
  pub mod button; }`, the generated-module declaration only.
- `packages/render/src/button.rs` — consumes the vocabulary (R3), see
  Design.
- `tasks/effigy.tasks.toml` — `ir:build` / `ir:check` gain the
  `--target button-rust` step into `packages/render/src`.
- `packages/codegen/tests/button_rust.rs` (new, 6 tests) — see Tests.
- `packages/codegen/tests/button.rs` — two stale path constants corrected
  (b041's review moved the artifact location but not the tests; the
  codegen suite was red on the branch until this — see `PAPERCUTS.md`).
- `docs/roadmaps/g13/005-button-component-vertical-slice.md` — status line
  only: `in progress` → `complete` (042 closes the milestone).
- `PAPERCUTS.md` — three entries (the GPUI baseline staleness
  classification, the Jetstream snap viewport clip, the resolved test
  constants).
- This log.

Nothing else in the repo changed: no `poodle-ir`/`poodle-codegen` dep in
`packages/render/Cargo.toml` (asserted by test, not just avoided), no
`button-ts`/`shell-rust`/`shell-scene` output change, `synthetic-model.json`
untouched, no native preview source touched, no visual baseline refreshed,
no web-shell source touched.

## Design

- **The artifact is the definition, not a slice.** `button-rust` renders
  the full definition in Rust shape — the shared-type member lists, parts,
  attributes with their value domains, and recipe hooks — mirroring the
  `button-ts` artifact's information content (the `shell-rust` precedent:
  the artifact is the scene, not a projection). Pulled in by `#[path]`, so
  **no manifest change was needed** — which is what makes R1 hold without
  touching `Cargo.toml`.
- **R3 — the render takes its vocabulary from the definition.** The render
  now discriminates through `BUTTON_DEFINITION`:
  - the **status tones** (`danger`/`success`/`warning`) are gated on the
    definition's tone vocabulary — each status arm applies while the
    definition still declares the tone;
  - the **variant treatments** (secondary tint, primary darkened border,
    ghost transparent) are gated on the definition's variant vocabulary —
    the `variant_name` helper is the render's projection of the frozen
    `poodle-specs` enum onto the definition's words (the native counterpart
    of the web components' value derivation, CROSS-14); the legacy `Danger`
    variant resolves to `danger`, which the variant vocabulary does not
    declare — exactly the fall-through the old tier had;
  - the **density metrics** (padding offset, gap ladder) are gated on the
    definition's density vocabulary, falling back to the default rung;
  - the **state treatments** (loading spinner + suppression, pressed
    accent, icon-only square, has-leading/has-trailing padding and
    children) are gated on the definition's state-attribute names via
    `state_declared(id, name)` — the same names the web DOM emits (b041
    R2). The node tree carries no attribute channel (the `Node` vocabulary
    is frozen), so the names gate the treatments rather than flowing into a
    DOM; a rename drops the treatment until restored, which is the drift
    direction the proof demonstrates. All gates pass today, so no pixel
    moves (R4).
- **The proof value.** b041 renamed `data-tone`; this card renames
  `data-has-leading` — the one attribute whose treatment is visible in all
  four previews (web DOM attribute name; native leading-icon children).
  `data-loading` was also proven (web DOM + GPUI; the Jetstream snap's
  640px viewport clips Button's States row, so that leg is unobservable in
  the snap — recorded as the honest partial).
- **What stayed hand-written and why (R6).** The inventory is below, per
  runtime. The natives keep runtime state and widget mechanics host-owned
  (R4), exactly as b036 established for the shell.

## The R3 proof (step 5, live — two renames, restored)

### Rename 1: `data-loading` → `data-busy` (one definition edit, one `ir:build`)

| Runtime | Observation |
|---|---|
| Svelte | 0 buttons emit `data-loading`, 23 emit `data-busy` (values `false`/`true`); every specimen button's DOM attribute name changed |
| React | 0 `data-loading`, 23 `data-busy` |
| GPUI | pixel diff between the renamed and restored screenshots: 3386 changed samples, bbox (718,1860)–(1184,1914) at 2696×2396 — the Loading button lost its spinner and its disabled dimming (the render's `loading` treatment dropped) |
| Jetstream | **not observable in the snap** — the `snap -- specimens` scene is a fixed 900×640 and Button's States row (disabled/loading) sits below the fold; only the upper specimen rows render. The treatment drop is covered by the render tests and the GPUI leg. Reported honestly rather than claimed |

Restored → one `ir:build` → all artifacts back (`data-loading`, zero
`data-busy`), `ir:check` 0.

### Rename 2: `data-has-leading` → `data-has-leading-content` (the four-runtime proof)

| Runtime | Observation |
|---|---|
| Svelte | 0 buttons emit `data-has-leading`, 7 emit `data-has-leading-content` (Create, Save, Filter, Loading, Bookmark, …) |
| React | 0 `data-has-leading`, 7 `data-has-leading-content` |
| GPUI | pixel diff renamed vs restored: 2840 changed samples, bbox (564,1554)–(1184,2050) — the "With icons" row: the leading-icon buttons (Create, Save) dropped their leading icons (the `has-leading` state gate closed the `has_icons` child block) |
| Jetstream | pixel diff renamed vs restored: 770 changed samples, bbox (36,456)–(310,572) — the "With icons" row (mid-page, inside the 640px frame): same leading-icon drop |

Restored → one `ir:build` → Svelte back to 7×`data-has-leading` / 0×new
name / 23×`data-loading` / 6×`data-tone` (exactly the before-state),
GPUI before-vs-restored = 23 samples (antialiasing noise), `ir:check` 0.
The only file touched during either proof was
`packages/codegen/src/models/button.rs` (renamed, then restored); the
restored artifacts contain zero occurrences of either temporary name.

## R5 screenshots (step 6, both natives, current state)

- **GPUI** `/tmp/g13-042-gpui-button-hl-restored.png` (2696×2396, retina
  2x): the Button specimen page — nav chrome (Components/Tokens tabs and
  the shell controls bar) above, then the specimen sections: Variants
  (Primary/Secondary/Ghost), the four tone rows, With icons, With chevron,
  Sizes, Toggle (pressed state), States (disabled/loading). No vision
  model was available, so content was verified from pixel evidence: the
  saturated-color histogram shows the status-tone families (danger red,
  success green) in the tone rows, and the capture is self-consistent with
  the pre-proof capture (23-sample noise). All gates pass, so the render is
  the pre-042 code path; the Jetstream zero below is the cross-renderer
  check on the same `poodle-render` output.
- **Jetstream** `/tmp/g13-042-jetstream-button-hl-restored.png` (900×640,
  headless): the Button specimen through the worktree render — Variants,
  the four tone rows, With icons, With chevron, Sizes (the States row is
  clipped by the snap viewport). **R4 check: pixel-diffed against the
  committed baseline `packages/jetstream/preview/baselines/button.png`
  (main checkout): 0 changed pixels.** The native visual baseline did not
  move.

**R4 — the GPUI baseline, classified not refreshed.** The current capture
at the baseline's nominal axes differs from
`button-eclipse-compact-sm.png` by ~600k samples. Classification evidence:
the top chrome (nav + shell controls, y<400) agrees 99–100%; the specimen
content region below differs systematically (~45% agreement) with a
near-identical saturated-color histogram (same palette features, same
counts); the Jetstream counterpart is a true zero on the **same
`poodle-render` output**; and the render's new gates all pass today (the
code path is the pre-042 one). The baseline is therefore **stale**
(predates the current preview layout — g12.014-era), not moved by this
card. Also found while classifying: `test/native-visual/capture.ts` passes
`--control-size`, which the preview ignores (it parses `--size`), so every
GPUI gate capture has run at the default size — recorded in `PAPERCUTS.md`.
No baseline refreshed (R4 stop condition not triggered).

## The R6 hand-written exception inventory

Spec 063's acceptance: *"hand-written exceptions are zero or explicitly
justified in the pilot log."* b041 covered Svelte and React; this is the
GPUI and Jetstream extension. Both natives consume the same
`poodle_render::button` node, so the render-side story is one row; the
host-side rows differ per preview.

### From the definition (via the artifact → `poodle-render`)

The variant treatments and their value domains; the status-tone set and
colors; the density metrics and their value domains; the state treatments
behind the eleven `data-*` attributes (names gate the treatments; the
web-only value domains are carried in the artifact); the anatomy children
(spinner/icon/label/chevron) rendered per the spec's parts.

### GPUI (`packages/gpui/preview/src/specimens/button.rs` — host side)

| Exception | Reason |
|---|---|
| the specimen layout (Eyebrow headings, rows, section structure) | specimen chrome — the preview's arrangement of rendered nodes, not Button's definition (the web specimen pages are the same category) |
| the `ButtonSpec` constructions (variant/tone/size/density/label per specimen) | specimens are authored content — the pilot's specimen layer stays hand-authored (spec 063) |
| click/toggle handlers and `NodeSpecimenEvent` state | interaction wiring is host-owned (IR-05); the toggle specimens' pressed state is runtime state |
| `poodle_gpui_node_backend::to_gpui` node interpretation | the adapter's drawing (IR-06): the render produces the node, the backend paints it |
| `ButtonSpec` defaults and `resolve_semantic_size` | the frozen poodle-specs surface (R4): ButtonSpec keeps its fields; the render projects the enums onto the definition's names (CROSS-14) |

### Jetstream (`packages/jetstream/preview/src/specimens/button.rs`)

Mirrors GPUI (same `poodle_render::button` via `compat::js_button`), plus:

| Exception | Reason |
|---|---|
| the `nel`/`compat` El-building (group/label/row wrappers) | the preview's element language for specimen chrome — host-owned, like GPUI's Eyebrow rows |
| the `snap`/`a11y` bins' viewport and projection | tooling, not Button rendering |

### Both natives, shared

| Exception | Reason |
|---|---|
| the status token names (`color.status.danger` etc.) resolved in the render | the definition records the tone→token mapping in its recipe hooks (BTN-22), but the render resolves the family color directly; the recipe chains are consumed by the web styling seam (`button.css`). Same inventory line as b041's recipe-hooks entry — a g13.008 question |
| the boolean value domains of `data-loading`/`data-pressed` carried, not consumed | the native node tree has no value channel (no DOM): the domains gate the web DOM's values; the names gate the native treatments |
| the enum→name projection (`ButtonVariant::Ghost` → `"ghost"`) | the frozen poodle-specs enums (R4) projected onto the definition's words — the native counterpart of the web components' value derivation (CROSS-14) |

## Tests (60 in `poodle-codegen`, all green)

`tests/button_rust.rs` (6 new):

- `render_artifact_matches_the_target_render` — the card's parity test: the
  expected artifact is the `button-rust` target's render of the authored
  definition (derived, never hand-listed), and the committed
  `packages/render/src/generated/button.rs` must equal it byte-exact. Also
  asserts the artifact imports no Poodle crate (R1).
- `render_artifact_carries_the_rendered_vocabulary` — variants/tones/
  densities member lists; every part with its DOM class (including the
  shared icon-span class); every attribute name/form/emission/value domain
  (variant domain, tone domain minus the default, size domain, fit domain
  minus the default, boolean domains, presence-only); every recipe hook
  with its chain kinds.
- `artifact_header_names_the_source_definition_and_generator_version` —
  the Generated Artifact Contract.
- `one_definition_change_moves_all_three_button_artifacts` — the R2 proof
  encoded: renaming `data-tone` → `data-tone-level` moves the render
  artifact and both web artifacts in one build.
- `render_manifest_carries_no_poodle_ir_or_codegen_dependency` — reads
  `packages/render/Cargo.toml` and asserts neither crate is named (R1
  asserted, not just avoided).
- `button_rust_artifacts_fail_check_on_drift_and_check_never_writes` —
  planted drift + stale orphan in a render artifact fails `--target
  button-rust --check` and check mode leaves the tree byte-identical.

Existing suites: `poodle-codegen` 60 total (was 54 — the six new), all
green; the b041 button-ts parity tests pass unchanged, proving the shared
helpers' widening moved no bytes; `poodle-render` 179 tests pass **unedited**
(R4 — the gates all pass, so the node output is unchanged), clippy
`-D warnings` clean; both native previews build clean (GPUI worktree
manifest, Jetstream through the symlink path).

**Rust drift proof (required test, live).** Planted a line into
`packages/render/src/generated/button.rs` → `effigy ir:check` exits 1
naming it under target `button-rust` → restored → exits 0. (The test suite
covers the same property for a scratch tree.)

## Validation (all step-8 commands)

| Command | Exit state |
|---|---|
| `effigy ir:build` | 0 — authored models, synthetic fixture, all targets incl. `button-rust` |
| `effigy ir:check` | 0 — all current (button fixture + both `button-ts` artifacts + the render artifact gated) |
| `effigy ci:rust` | 0 |
| `effigy ci:web` | 0 (includes `test:web-pack-install`) |
| `effigy test:parity` | 0 |
| `effigy check:svelte` | 0 |
| `effigy docs:lint` | 0 |
| `git diff --check` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 60 passed |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 |
| `cargo test --manifest-path packages/render/Cargo.toml` | 0 — 179 passed |
| `cargo clippy --manifest-path packages/render/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo build -p poodle-gpui-preview` | 0 |
| Jetstream preview build + `snap -- specimens --slug=button` through the symlink path | 0 |

(The `cargo fmt --check` on the render crate reports pre-existing drift in
files this card did not touch — `app_header.rs`, `tabs.rs`, `split_button.rs`,
etc. — present on the branch before this card; the card's own files and the
generated artifact are rustfmt-clean. Render-crate fmt is not part of any
gate this card runs.)

## Acceptance criteria

- [x] One definition change visible in **all four** previews, demonstrated
  (`data-has-leading` rename, live; `data-loading` rename recorded as the
  partial with the Jetstream viewport clip named).
- [x] `poodle-render` depends on neither `poodle-ir` nor `poodle-codegen`
  (asserted by test; artifact pulled in by `#[path]`).
- [x] The artifact sits in `packages/render/src/generated/`.
- [x] `button-ts` and `shell-rust` output byte-identical (ir:check + the
  b041/b036 parity tests; only helper visibility changed).
- [x] Both natives screenshotted; no baseline refreshed (Jetstream true
  zero; GPUI baseline classified stale, not moved).
- [x] Exception inventory covers GPUI and Jetstream (R6).
- [x] `g13.005` marked complete.
- [x] All step-8 commands exit 0.

## Not done

Per batch card and worker rules: no merge (branch pushed only), no
`poodle-ir` schema change (R1 out of scope), no other component, no native
preview source change, no visual baseline refresh, no hand edit of
generated files. The GPUI baseline staleness and the harness's
`--control-size` flag mismatch are recorded in `PAPERCUTS.md`, not fixed
here (the visual gate's baselines and harness are outside this card's
writable paths). The web shells' hardcoded aria labels and Jetstream's
density value subset remain the pre-existing `PAPERCUTS.md` entries from
b036.
