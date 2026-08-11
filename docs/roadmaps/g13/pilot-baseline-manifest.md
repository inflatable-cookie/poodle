# g13.002 Pilot Baseline Manifest

Status: frozen fixtures + measured quantitative before-state
Milestone: `g13.001`
Owner: Poodle core (measurement by batch `002`)
Branch: `thread/g13-002-pilot-fixture-metrics`
Updated: 2026-08-11
Governing refs: `docs/roadmaps/g13/batch-cards/002-pilot-fixture-and-metrics-freeze.md`,
`docs/roadmaps/g13/authority-inventory.md`,
`docs/roadmaps/g13/pilot-expressiveness-corpus.md`,
`docs/specs/063-rust-authored-component-and-scene-ir.md` (`IR-05`–`IR-11`),
`docs/contracts/components/button.md`, `docs/contracts/components/range-slider.md`,
`docs/contracts/components/text-input.md`

## Purpose

Turns the merged authority inventory and expressiveness corpus into stable,
identified pilot fixtures (`FIX-*`) and a reproducible quantitative
before-state for the g13.002 Rust-authored IR work. It designs no schema,
creates no package, and makes no placement recommendation (`IR-12`). All
commands, exit states, and measured counts are in
`docs/logs/2026-08/11-g13-002-pilot-fixture-and-metrics-freeze.md`.

## Reading Rules

- Component contracts are semantic authority; the merged inventory and corpus
  are the measured evidence base.
- `docs/parity/*.md` is historical evidence only, never authority (corpus
  `OBS-04`; `docs/parity/README.md` "Do not use them to decide whether a
  current component exists or is release-ready").
- Repository reality is authoritative over card assumptions; mismatches are
  recorded below with citations, not patched.
- Every fixture binds to existing corpus requirement IDs (`CROSS-*`, `BTN-*`,
  `RNG-*`, `TXT-*`, `SHELL-*`). No second requirement vocabulary is invented.

## 1. Fixture Tables

Legend: one row per fixture. The Svelte/React/GPUI/Jetstream column carries the
implementation or specimen path per runtime, or `absent` + the gap ID. Status is
`frozen`, `blocked:<UNKNOWN-id>`, or `gap:<GAP-id>`. No specimen, test, or
registry entry was added by this card — every path below pre-exists.

### 1.1 Button (`FIX-BTN-*`) — owning contract `docs/contracts/components/button.md` §13

| Fixture ID | Scenario | Corpus IDs | Owning contract | Svelte / React / GPUI / Jetstream | Existing evidence | Status |
|---|---|---|---|---|---|---|
| FIX-BTN-01 | Variants: primary/secondary/ghost in a row | BTN-01, BTN-02, BTN-28 | button.md §13 "Variants" | `packages/svelte/preview/src/specimens/ButtonSpecimen.svelte` (Variants)<br>`packages/react/preview/src/gallery/specimens/ButtonSpecimen.tsx` (Variants)<br>`packages/gpui/preview/src/specimens/button.rs:107`<br>`packages/jetstream/preview/src/specimens/button.rs` ("Variants") | visual axis tier (`test/visual/config.ts` `AXIS_TIER_SLUGS` `button`); axe sweep (`test/a11y/component-a11y.test.ts`, `COMPONENT_PROPS.Button` = `{ ariaLabel: "Action" }`, `A11Y_BASELINE` empty) | frozen |
| FIX-BTN-02 | Danger tone: primary/secondary/ghost × `tone=danger` | BTN-02, BTN-28 | button.md §13 "Danger tone" | Svelte specimen (Danger tone)<br>React specimen (Danger tone)<br>`packages/gpui/preview/src/specimens/button.rs:145` (`ButtonTone::Danger`)<br>Jetstream specimen ("Danger tone" tone_row) | parity doc `docs/parity/button.md` (historical); visual axis tier | frozen |
| FIX-BTN-03 | With icons: leading/trailing registry icons (plus/external-link/save/check) | BTN-09, BTN-16, BTN-17, BTN-28 | button.md §13 "With icons" | Svelte specimen (With icons)<br>React specimen (With icons)<br>`packages/gpui/preview/src/specimens/button.rs:259`<br>`packages/jetstream/preview/src/specimens/button.rs` ("With icons") | visual axis tier | frozen |
| FIX-BTN-04 | With chevron: trailing disclosure indicator | BTN-10, BTN-17, BTN-28 | button.md §13 "With chevron" | Svelte specimen (With chevron)<br>React specimen (With chevron)<br>`packages/gpui/preview/src/specimens/button.rs:298`<br>`packages/jetstream/preview/src/specimens/button.rs` ("With chevron") | visual axis tier | frozen |
| FIX-BTN-05 | Sizes: contract sm/md/lg + specimen xs–xl ladder | BTN-03, BTN-23, BTN-28 | button.md §13 "Sizes" + §7/§8 size tables | Svelte specimen `sizes` snippet<br>React specimen `sizes` callback<br>GPUI specimen Sizes pane via `specimen_layout.rs`<br>`packages/jetstream/preview/src/specimens/button.rs` ("Sizes", xs–xl) | axis tier: 12 size×density axes (`AXIS_TIER_AXES`) | frozen |
| FIX-BTN-06 | States: disabled / loading / disabled secondary | BTN-07, BTN-08, BTN-19, BTN-20, BTN-28 | button.md §13 "States" + §4/§6 | Svelte specimen (States)<br>React specimen (States)<br>`packages/gpui/preview/src/specimens/button.rs:338`<br>`packages/jetstream/preview/src/specimens/button.rs` ("States") | axe sweep (aria-busy/disabled); visual axis tier | frozen |
| FIX-BTN-07 | Click counter: click log increments on any button click | CROSS-05, CROSS-06, BTN-14, BTN-28 | button.md §13 "Click counter" | Svelte specimen `clickLog` paragraph<br>React specimen `clickLog` paragraph<br>`packages/gpui/preview/src/specimens/button.rs:483` (`Clicks: {click_count}`)<br>Jetstream: `absent` (static render; `on_click` carries no payload — `BTN-27` EXT, preview-loop) | parity doc specimen section (historical) | frozen |
| FIX-BTN-08 | Rust spec superset render: `ButtonVariant::Danger` / `ButtonTone::Success` through `poodle-render` | BTN-29 | button.md §3 (union) + `UNKNOWN-02` | web: `absent` (web union is primary/secondary/ghost × default/danger/warning)<br>`packages/contracts/components/src/types.rs` (enum superset, lines 233–304)<br>`packages/render/src/button.rs` (status match)<br>`packages/gpui/preview/src/specimens/button.rs:183` ("Success tone")<br>`packages/jetstream/preview/src/specimens/button.rs` (Success tone row) | parity doc pass 42–43 notes (historical) | `blocked:UNKNOWN-02` |

### 1.2 RangeSlider (`FIX-RNG-*`) — owning contract `docs/contracts/components/range-slider.md` §13

| Fixture ID | Scenario | Corpus IDs | Owning contract | Svelte / React / GPUI / Jetstream | Existing evidence | Status |
|---|---|---|---|---|---|---|
| FIX-RNG-01 | Default: `[20,80]`, 0..100, "Price range", live `$lo – $hi` readout | RNG-01, RNG-02, RNG-11, RNG-12, RNG-14, RNG-19, RNG-25 | range-slider.md §13 "Group: Default" | Svelte specimen (Default)<br>React specimen (Default)<br>`packages/gpui/preview/src/specimens/range_slider.rs:112`<br>`packages/jetstream/preview/src/specimens/range_slider.rs` ("Default (20–80)") | visual sweep tier only (`range-slider` not in `AXIS_TIER_SLUGS`); axe sweep | frozen |
| FIX-RNG-02 | With step: `[25,45]`, 18..65, step 5, snap anchored at min | RNG-02, RNG-19, RNG-25 | range-slider.md §13 "Group: With step" | Svelte specimen (With step)<br>React specimen (With step)<br>`packages/gpui/preview/src/specimens/range_slider.rs:147`<br>`packages/jetstream/preview/src/specimens/range_slider.rs` ("With step") | conformance: `core/src/slider.ts` `snapToStep`/`safeSliderMax` + `core/test/wave1.test.ts` (per-implementation, `GAP-01`) | frozen |
| FIX-RNG-03 | Disabled: `[30,70]`, disabled, reduced opacity, inert | RNG-08, RNG-25 | range-slider.md §13 "Group: Disabled" | Svelte specimen (Disabled)<br>React specimen (Disabled)<br>`packages/gpui/preview/src/specimens/range_slider.rs:287`<br>`packages/jetstream/preview/src/specimens/range_slider.rs` ("Disabled (30–70)") | axe sweep; `range-slider.css` `[data-disabled="true"]` | frozen |
| FIX-RNG-04 | Embedded unipolar: `[0.2,0.75]`, 0..1, step 0.01, `variant=embedded`, `polarity=unipolar` | RNG-15, RNG-23, RNG-25 | range-slider.md §13 "Group: Embedded unipolar control" + §2/§6 | Svelte specimen (Embedded unipolar)<br>React specimen (Embedded unipolar)<br>`packages/gpui/preview/src/specimens/range_slider.rs:219`<br>`packages/jetstream/preview/src/specimens/range_slider.rs` ("Embedded unipolar control") | `range-slider.css` embedded blocks; `slider.ts` embedded VisualState arms | `blocked:UNKNOWN-01` (embedded `role="slider"` stops emit `aria-orientation`; contract silent) |
| FIX-RNG-05 | Embedded bipolar: `[-0.6,0.35]`, −1..1, step 0.01, `polarity=bipolar` | RNG-04, RNG-05, RNG-23, RNG-24, RNG-25 | range-slider.md §13 "Group: Embedded bipolar control" | Svelte specimen (Embedded bipolar)<br>React specimen (Embedded bipolar)<br>`packages/gpui/preview/src/specimens/range_slider.rs:243`<br>`packages/jetstream/preview/src/specimens/range_slider.rs` ("Embedded bipolar control") | `wave1.test.ts` bipolar case; `range-slider.css` `[data-fill-split="true"]` | `blocked:UNKNOWN-01` |
| FIX-RNG-06 | Sizes tab: standard + embedded unipolar + embedded bipolar at every xs–xl; densities use embedded bipolar | RNG-09, RNG-25 | range-slider.md §13 "The Sizes tab renders … at every `xs`–`xl` size" | Svelte specimen `sizes` snippet (standard + embedded unipolar + embedded bipolar)<br>React specimen `sizes` callback (same)<br>GPUI Sizes/Densities panes via `specimen_layout.rs` (`ALL_SIZES`/`ALL_DENSITIES`)<br>`packages/jetstream/preview/src/specimens/range_slider.rs` (`size_variants` xs–xl × 3 variants; "Densities" group) | axis tier N/A (sweep only); `range-slider.css` size/density blocks | frozen |

### 1.3 TextInput (`FIX-TXT-*`) — owning contract `docs/contracts/components/text-input.md` §13

| Fixture ID | Scenario | Corpus IDs | Owning contract | Svelte / React / GPUI / Jetstream | Existing evidence | Status |
|---|---|---|---|---|---|---|
| FIX-TXT-01 | Default: label + placeholder + help text, empty and interactive | TXT-01, TXT-02, TXT-03, TXT-32 | text-input.md §13 "Default" | Svelte specimen (Default, `name-field`)<br>React specimen (Default)<br>`packages/gpui/preview/src/specimens/text_input.rs:50`<br>`packages/jetstream/preview/src/specimens/text_input.rs` ("Default (placeholder)"; typing absent — `TXT-31`/`GAP-05`) | axe sweep (`COMPONENT_PROPS.TextInput` = `{ id: "t1", ariaLabel: "Search" }`); visual axis tier (`text-input` in `AXIS_TIER_SLUGS`) | frozen |
| FIX-TXT-02 | With validation: invalid when missing `@`, error message shown | TXT-12, TXT-13, TXT-19, TXT-32 | text-input.md §13 "With validation" | Svelte specimen (With validation, `email-field`)<br>React specimen (With validation)<br>`packages/gpui/preview/src/specimens/text_input.rs:77`<br>Jetstream specimen ("Invalid"/"Valid" groups) | `core/src/input.ts` `validationStatusToState`; `text-input.css` validation borders | frozen |
| FIX-TXT-03 | Pending validation: async in progress, trailing spinner + pending message | TXT-12, TXT-19, TXT-32 | text-input.md §13 "Pending validation" | Svelte specimen (Slug group's async `validate` → pending; contract's Workspace case is exercised via the same path)<br>React specimen (Slug, same)<br>`packages/gpui/preview/src/specimens/text_input.rs:112` ("Async validation")<br>`packages/jetstream/preview/src/specimens/text_input.rs` ("Pending") | `core/src/input.ts` validation-state mapping; `text-input.css` pending ring spinner | frozen |
| FIX-TXT-04 | Disabled: `disabled: true`, reduced opacity, non-interactive | TXT-05, TXT-19, TXT-32 | text-input.md §13 "Disabled" | Svelte specimen (Disabled, `disabled-field`)<br>React specimen (Disabled)<br>`packages/gpui/preview/src/specimens/text_input.rs:394`<br>`packages/jetstream/preview/src/specimens/text_input.rs` ("Disabled") | axe sweep; `text-input.css` disabled opacity | frozen |

### 1.4 Preview shells (`FIX-SHELL-*`) — owning contract `docs/specs/063-rust-authored-component-and-scene-ir.md` Scene IR (theme/size/density/contrast axes, layout, search, tabs)

| Fixture ID | Scenario | Corpus IDs | Owning contract | Svelte / React / GPUI / Jetstream | Existing evidence | Status |
|---|---|---|---|---|---|---|
| FIX-SHELL-01 | Theme selection control in the top bar | SHELL-01 | S063 Scene IR "theme … axes" | `packages/svelte/preview/src/components/DisplayControls.svelte:59` (real `ThemeSelect`)<br>`packages/react/preview/src/gallery/DisplayControls.tsx` (`ToggleGroup` over `themes`)<br>`packages/gpui/preview/src/main.rs` `render_display_controls` (:377) + `app_state.rs` `ThemePreset` (:55)<br>`packages/jetstream/preview/src/shell.rs` `build_controls_bar` `js_theme_select` (:183) | theme source `packages/core/src/tokens/theme-options.ts` (generated); GPUI/Jetstream `poodle_tokens` typed themes | frozen |
| FIX-SHELL-02 | Control-size axis control (xs–xl) | SHELL-02 | S063 Scene IR "size" axis | `DisplayControls.svelte:79` (`ToggleGroup`)<br>`DisplayControls.tsx` (Size `ToggleGroup`)<br>`packages/gpui/preview/src/app_state.rs` `ControlSize`<br>`packages/jetstream/preview/src/shell.rs:195-196` (Size group) | `test/visual/config.ts` `AXIS_TIER_AXES` (5 sizes × 2 densities) | frozen |
| FIX-SHELL-03 | Density axis control (compact/default/comfortable) | SHELL-03 | S063 Scene IR "density" axis | `DisplayControls.svelte:69` (`ToggleGroup`)<br>`DisplayControls.tsx` (Density `ToggleGroup`)<br>`packages/gpui/preview/src/app_state.rs` `Density`<br>`packages/jetstream/preview/src/shell.rs:192-193` (Density group) | density tokens `packages/core/src/tokens`; `test/visual/config.ts` densities | frozen |
| FIX-SHELL-04 | Contrast control: continuous neutral-contrast slider | SHELL-04 | S063 Scene IR "contrast axes" | `DisplayControls.svelte:88-95` (`Slider`, 0.4–1.6)<br>`DisplayControls.tsx` (Contrast `Slider` min 0.4 max 1.6 step 0.05)<br>`packages/gpui/preview/src/app_state.rs` `CONTRAST_MIN`/`CONTRAST_MAX` + `main.rs`<br>`packages/jetstream/preview/src/shell.rs:200-214` (contrast slider 0..1) | `test/visual/config.ts` contrast extremes (0.9 / 0.1 axes) | frozen |
| FIX-SHELL-05 | Navigation: top-level sections, component sidebar groups, route state | SHELL-05 | S063 Scene IR "layout nodes … groups" | `packages/svelte/preview/src/App.svelte` (top-bar `Tabs`, `:137`) + `router.ts` (`parseRoute`)<br>`packages/react/preview/src/gallery/App.tsx` (`parseRoute` :27)<br>`packages/gpui/preview/src/main.rs` `CatalogueSidebar` (:150)<br>`packages/jetstream/preview/src/shell.rs` `build_tab_bar` + sidebar | route state: hash + query params (web) | frozen |
| FIX-SHELL-06 | Component search: case-insensitive filter over display name/description | SHELL-06 | S063 Scene IR | `packages/svelte/preview/src/sections/ComponentsSection.svelte:17-21` (`filteredComponents`)<br>`packages/react/preview/src/gallery/App.tsx` (search state :41) + `DisplayControls.tsx`<br>`packages/gpui/preview/src/app_state.rs:563` (`component_search`)<br>`packages/jetstream/preview/src/app_state.rs:523` (`matches_search`) | axe sweep (search input) | frozen |
| FIX-SHELL-07 | Specimen tabs: Examples / Sizes / Densities | SHELL-07 | S063 Scene IR "size/density matrices" | `packages/svelte/preview/src/components/SpecimenLayout.svelte` (Tabs, Examples/Sizes/Densities)<br>`packages/react/preview/src/gallery/SpecimenLayout.tsx`<br>`packages/gpui/preview/src/specimens/specimen_layout.rs` (`TABS`) + `app_state.rs` `SpecimenView`<br>`packages/jetstream/preview/src/app_state.rs:32` (`SpecimenView`) | visual sweep tiers; native visual gate | frozen |
| FIX-SHELL-08 | Preview state serialization: theme/density/controlSize/contrast in URL query + hash | SHELL-08 | S063 Scene IR | `packages/svelte/preview/src/App.svelte` `syncCurrentLocation` (:80) + `replaceState` (:114); `packages/svelte/preview/src/parity.ts` `normalizePreviewState`<br>`packages/react/preview/src/gallery/App.tsx` (:88-94)<br>GPUI: not applicable (state lives in `AppState`; no URL surface)<br>Jetstream: not applicable (state lives in `AppState`) | web shells: `replaceState` URL writes | frozen |
| FIX-SHELL-09 | Parity harness vocabulary: defaults, review presets, parity targets, package-surface coverage, axe sweep, visual tiers, native baseline gate | SHELL-09 | S063 IR-10 | `packages/svelte/preview/src/parity.ts` + `accessibility.ts`<br>`test/visual/config.ts` (`SMOKE_AXES`/`AXIS_TIER_AXES`/`SWEEP_AXES`, `AXIS_TIER_SLUGS` incl. `button`, `text-input`)<br>`test/a11y/component-a11y.test.ts`<br>`test/native-visual/config.ts` (`BASELINE_DIR`, `MAX_DIFF_RATIO`) | parity reports `packages/{svelte,react}/preview/artifacts/parity-report.json`; `docs:lint` validates | frozen |
| FIX-SHELL-10 | Specimen registry + specimen-map wiring per shell | SHELL-10 | S063 Scene IR "component references and typed prop bindings" | `packages/svelte/preview/src/specimens/registry.ts` (`button`:193, `range-slider`:269, `text-input`:308)<br>`packages/react/preview/src/gallery/specimen-map.ts`<br>`packages/gpui/preview/src/component_registry.rs`<br>`packages/jetstream/preview/src/component_registry.rs` | `svelte:surface-audit` (registry parity) | frozen |

### 1.5 Size/density axis mechanism per runtime (`FIX-AXIS-*`) — corpus `CROSS-07`, `CROSS-08`, `SHELL-07`

| Fixture ID | Scenario | Corpus IDs | Owning contract | Svelte / React / GPUI / Jetstream | Existing evidence | Status |
|---|---|---|---|---|---|---|
| FIX-AXIS-01 | Svelte shared axis helper: `sizes`/`densities` snippets + Examples/Sizes/Densities tabs | CROSS-07, CROSS-08, SHELL-07 | B/R/T §3 + §7/§8 size tables | `packages/svelte/preview/src/components/SpecimenLayout.svelte` (`controlSizes` xs–xl, `controlDensities` compact/default/comfortable) | axis visual tier for `button`/`text-input`; `AXIS_TIER_AXES` 12 axes | frozen |
| FIX-AXIS-02 | React shared axis helper: same API as Svelte | CROSS-07, CROSS-08, SHELL-07 | B/R/T §3 + §7/§8 | `packages/react/preview/src/gallery/SpecimenLayout.tsx` (`CONTROL_SIZES`, `CONTROL_DENSITIES`) | Svelte↔React visual diff gate (same DOM contract) | frozen |
| FIX-AXIS-03 | GPUI axis mechanism: three-pane builder closures per size/density | CROSS-07, CROSS-08, SHELL-07 | B/R/T §3 + §7/§8 | `packages/gpui/preview/src/specimens/specimen_layout.rs` (`specimen_layout` + `ALL_SIZES`/`ALL_DENSITIES`); per-specimen closures in `packages/gpui/preview/src/specimens/*.rs` | `test/native-visual` baseline gate (GPUI); `check:gpui` | frozen |
| FIX-AXIS-04 | Jetstream axis mechanism: per-specimen inline sweeps; **no shared `SpecimenLayout` helper** | CROSS-07, CROSS-08, SHELL-07 | B/R/T §3 + §7/§8 (inventory §6) | shared helper: `absent` (inventory §6: "Jetstream | none (no shared helper)")<br>inline sweeps: `packages/jetstream/preview/src/specimens/button.rs` (Sizes), `range_slider.rs` (`size_variants`, Densities), `text_input.rs` (Sizes, Densities) | `test/jetstream-visual` (offscreen baselines) | frozen |

## 2. Unknowns — blocked fixtures, no assumed answer

- **`UNKNOWN-01`** (embedded RangeSlider `aria-orientation` scope) blocks
  `FIX-RNG-04` and `FIX-RNG-05`. The embedded `role="slider"` focus stops emit
  `aria-orientation` in Svelte/React; the contract (`range-slider.md` §6) is
  silent on the embedded stops. Maintainer decision owned by `g13-b003` —
  choosing either reading is a stop condition. No answer is assumed here.
- **`UNKNOWN-02`** (Rust Button `Danger`/`Success` enum superset scope) blocks
  `FIX-BTN-08`. `packages/contracts/components/src/types.rs` (lines 233–304)
  and `packages/render/src/button.rs` implement `ButtonVariant::Danger` /
  `ButtonTone::Success` beyond the contract union (button.md §3). Maintainer
  decision owned by `g13-b003`. No answer is assumed here.

## 3. GAP-01–GAP-07 Mapping (measured, not closed)

| Gap | Content | Fixture coverage |
|---|---|---|
| GAP-01 | No `rangeSlider`/text-input conformance vectors in `packages/contracts/headless/vectors/machines.json` (RNG-29, CROSS-18) | **No fixture covers.** `FIX-RNG-01/02/05` cite `CROSS-18`-adjacent behavior (`wave1.test.ts`, `headless/src/slider.rs` tests) but those are per-implementation tests, not one shared vector file. Producing the vector file is out of scope. |
| GAP-02 | No executed assistive-technology traces for the pilots on native targets (BTN-21, RNG-19/20, TXT-26) | **No fixture covers.** `FIX-BTN-01/06`, `FIX-RNG-01/03`, `FIX-TXT-01/04` render the a11y targets (roles, ARIA, per-thumb labels) but executed AT traces are absent (`packages/gpui/native-accessibility-proof.json` records manual proof pending). |
| GAP-03 | Native vertical RangeSlider unimplemented on both Rust targets (RNG-07, RNG-26) | **No fixture covers.** `FIX-RNG-01/06` cite `RNG-07` for horizontal orientation only; `render/src/range_slider.rs` ignores `orientation` for layout. |
| GAP-04 | GPUI per-thumb focus / Tab cycling unimplemented (RNG-20) | **No fixture covers.** `FIX-RNG-01` cites `RNG-20`; GPUI renders a single wrapper focus ring (accepted delta in `docs/parity/range-slider.md`). |
| GAP-05 | Jetstream TextInput typing/key events absent; host owns the editor (TXT-31) | **Covered by `FIX-TXT-01`** (Jetstream cell: typing `absent` + `GAP-05`; also `FIX-TXT-02/03` Jetstream cells render validation states statically). |
| GAP-06 | Button density adjustment values contract-silent, stylesheet-only (BTN-05) | **Covered by `FIX-AXIS-01..04`** (density axis mechanism rows cite `CROSS-08`/`BTN-05`); values exist only in `button.css` + `render/src/button.rs`, not in button.md §8. |
| GAP-07 | `truncate`/`fit`/`maxWidth` absent from `ButtonSpec` (BTN-11/12/13) | **No fixture covers.** No specimen renders `fit`/`truncate`/`maxWidth`; Jetstream specimen notes "full-width is not representable — `ButtonSpec` exposes no fit/full-width/max-width field" (`packages/jetstream/preview/src/specimens/button.rs` NOTE). |

## 4. Quantitative Before-State (measurement record)

Machine: darwin 25.5.0, arm64, Apple M5 Max, 18 cores, macOS 26.5.2. All `wc -l`
counts run on this tree (`6eade887`). Cache state for build times: cold
(`cargo clean` per manifest immediately before each build; logs show full
recompilation).

### 4.1 Authored LOC — per pilot component, per runtime (inventory §5 files)

Command: `wc -l <file>` for each file listed; count = lines including blank/comment lines.

| Component | Runtime surface | File(s) | LOC |
|---|---|---|---|
| Button | Contract | `docs/contracts/components/button.md` | 575 |
| Button | Web core (styled-only, no machine) | `packages/core/src/styles/button.css` | 317 |
| Button | Svelte shell | `packages/svelte/components/src/Button.svelte` | 220 |
| Button | React shell | `packages/react/components/src/Button.tsx` | 164 |
| Button | Rust spec | `packages/contracts/components/src/button.rs` | 273 |
| Button | Headless | (styled-only; no headless module — inventory §5) | n/a |
| Button | Render | `packages/render/src/button.rs` | 621 |
| Button | GPUI preview | `packages/gpui/preview/src/specimens/button.rs` | 522 |
| Button | Jetstream preview | `packages/jetstream/preview/src/specimens/button.rs` | 245 |
| RangeSlider | Contract | `docs/contracts/components/range-slider.md` | 549 |
| RangeSlider | Web core | `packages/core/src/slider.ts` + `packages/core/src/styles/range-slider.css` | 344 + 227 |
| RangeSlider | Svelte shell | `packages/svelte/components/src/RangeSlider.svelte` | 176 |
| RangeSlider | React shell | `packages/react/components/src/RangeSlider.tsx` | 185 |
| RangeSlider | Rust spec | `packages/contracts/components/src/range_slider.rs` | 162 |
| RangeSlider | Headless | `packages/contracts/headless/src/slider.rs` | 607 |
| RangeSlider | Render | `packages/render/src/range_slider.rs` | 497 |
| RangeSlider | GPUI preview | `packages/gpui/preview/src/specimens/range_slider.rs` | 387 |
| RangeSlider | Jetstream preview | `packages/jetstream/preview/src/specimens/range_slider.rs` | 192 |
| TextInput | Contract | `docs/contracts/components/text-input.md` | 697 |
| TextInput | Web core | `packages/core/src/input.ts` + `packages/core/src/styles/text-input.css` | 76 + 252 |
| TextInput | Svelte shell | `packages/svelte/components/src/TextInput.svelte` | 617 |
| TextInput | React shell | `packages/react/components/src/TextInput.tsx` | 530 |
| TextInput | Rust spec | `packages/contracts/components/src/text_input.rs` | 459 |
| TextInput | Headless | `packages/contracts/headless/src/text_input.rs` | 837 |
| TextInput | Render | `packages/render/src/text_input.rs` | 625 |
| TextInput | GPUI preview | `packages/gpui/preview/src/specimens/text_input.rs` | 459 |
| TextInput | Jetstream preview | `packages/jetstream/preview/src/specimens/text_input.rs` | 267 |

### 4.2 Authored LOC — surface totals (inventory §1 globs)

Command: `wc -l <glob>` (glob expansion; file counts include the files the glob
matches on this tree).

| Glob | Files | LOC |
|---|---|---|
| `docs/contracts/components/*.md` | 171 (incl. README; inventory §1 counts 170 excl. README) | 53,645 |
| `packages/svelte/components/src/*.svelte` | 164 | 28,835 |
| `packages/react/components/src/*.tsx` | 165 | 26,178 |
| `packages/core/src/*.ts` | 41 (incl. `index.ts`; inventory §1 counts 40 excl. `index.ts`, `tokens/`) | 6,642 |
| `packages/core/src/styles/*.css` | 159 | 21,532 |
| `packages/contracts/components/src/*.rs` | 157 (incl. `lib.rs`; inventory §1 counts 156 excl. lib) | 33,611 |
| `packages/contracts/headless/src/*.rs` | 24 (incl. lib; inventory §1 counts 23) | 6,864 |
| `packages/render/src/*.rs` | 160 (incl. lib; inventory §1 counts 159) | 40,797 |
| `packages/gpui/preview/src/specimens/*.rs` | 146 | 32,299 |
| `packages/jetstream/preview/src/specimens/*.rs` | 151 | 24,231 |
| `packages/gpui/adapter/src/render_*.rs` | 9 | 2,831 |
| `packages/jetstream/adapter/src/render_*.rs` | 7 | 3,328 |
| `packages/svelte/preview/src/specimens/*` | 162 | 15,155 |
| `packages/react/preview/src/gallery/specimens/*` | 156 | 13,339 |
| `packages/svelte/preview/src/component-registry.ts` | 1 | 278 |
| `docs/parity/*.md` | 141 (incl. README.md + TEMPLATE.md; inventory §1 counts 139) | 9,592 |

### 4.3 Generated LOC

Command: `wc -l` over each glob (bash `globstar` for `**`; without `globstar`,
`packages/core/src/tokens/generated/*` and `packages/tokens/artifacts/**`
silently match only directories → 0 — see PAPERCUTS entry).

| Glob | Files | LOC |
|---|---|---|
| `packages/svelte/preview/artifacts/*.json` | 4 | 19,197 |
| `packages/react/preview/artifacts/*.json` | 3 | 16,771 |
| `packages/core/src/tokens/generated/**/*` | 25 | 1,813 |
| `packages/core/src/icons/generated.ts` | 1 | 309 |
| `packages/tokens/artifacts/**/*` | 31 | 2,407 |

### 4.4 Duplicated definition count (per pilot component)

Four authored expressions of one contract per component (inventory §8 item 1 —
the card cites this as §8.1; the inventory numbers the four-runtime
duplication as item 1 of §8):

| Component | Paths | Count |
|---|---|---|
| Button | `packages/svelte/components/src/Button.svelte`, `packages/react/components/src/Button.tsx`, `packages/contracts/components/src/button.rs`, `packages/render/src/button.rs` | 4 |
| RangeSlider | `packages/svelte/components/src/RangeSlider.svelte`, `packages/react/components/src/RangeSlider.tsx`, `packages/contracts/components/src/range_slider.rs`, `packages/render/src/range_slider.rs` | 4 |
| TextInput | `packages/svelte/components/src/TextInput.svelte`, `packages/react/components/src/TextInput.tsx`, `packages/contracts/components/src/text_input.rs`, `packages/render/src/text_input.rs` | 4 |

### 4.5 Runtime extension count (`EXT`-classified corpus rows)

Measured from the corpus rows' classification column
(`docs/roadmaps/g13/pilot-expressiveness-corpus.md` §2–§4):

| Component | EXT rows | IDs |
|---|---|---|
| Button | 3 | BTN-26, BTN-27, BTN-29 |
| RangeSlider | 2 | RNG-26, RNG-27 |
| TextInput | 1 | TXT-31 |
| **Total** | **6** | |

Measurement note (corpus-internal arithmetic conflict, recorded not resolved):
the corpus §8 counts table states `BTN EXT=5, RNG EXT=3, TXT EXT=2` (total 10,
rows `312–318`), which does not equal the row-level `EXT` marks (3/2/1 = 6).
The row-level classification is the measured evidence; the §8 table appears to
be planning-time arithmetic. Both numbers are recorded; no authority is chosen.

### 4.6 Clean build time

Method: `cargo clean --manifest-path <pkg>/Cargo.toml`, then one wall-clock
`cargo build -p <pkg> --manifest-path <pkg>/Cargo.toml`; web:
`bun run --cwd packages/svelte/preview build`. Times via `/usr/bin/time -p`.

| Build | Wall (real) | User | Sys | Cache | Exit |
|---|---|---|---|---|---|
| `cargo build -p poodle-specs` | 2.13 s | 3.00 s | 0.55 s | cold | 0 |
| `cargo build -p poodle-render` | 3.18 s | 6.85 s | 1.18 s | cold | 0 |
| `bun run --cwd packages/svelte/preview build` | 2.13 s | 3.02 s | 0.46 s | cold (post-`bun install`) | 0 |

Jetstream builds require the sibling jetstream checkout — present at
`../../jetstream` in this environment, but no Jetstream build is required by
this card's measure set.

### 4.7 Diagnostic quality (drift gate failure-message construction)

Quoted verbatim from each gate's own source; failures were **not** induced by
mutating source.

| Gate | Source | Failure-message construction (verbatim, file:line) |
|---|---|---|
| `docs:contract-drift` | `packages/svelte/preview/scripts/contract-prop-drift.ts` | `:139-141` `` `contract prop drift: ${f.slug}.md documents prop(s) not implemented in ${f.slug} Svelte component: ${f.contractOnly.join(", ")}` `` · `:161-162` `` `FAIL — ${n} documented prop(s) missing from Svelte across ${gated.length} component(s):` `` + per-slug `` `  [${f.slug}] contract-only: ${f.contractOnly.join(", ")}` `` · `:172` `if (gated.length > 0 && process.env.DRIFT_REPORT !== "1") process.exit(1);` |
| `docs:spec-drift` | `packages/svelte/preview/scripts/contract-spec-drift.ts` | `:308-312` `` `contract/spec drift: ${f.slug}.md documents prop(s) absent from its poodle-specs Spec: ${f.missing.join(", ")}` `` · `:320` `` `${n} documented prop(s) missing from poodle-specs across ${findings.length} component(s):` `` · `:326` `if (findings.length > 0 && process.env.DRIFT_REPORT !== "1") process.exit(1);` |
| `drift:roles` | `packages/svelte/preview/scripts/contract-role-drift.ts` | `:98` `` console.error(`role census failed:\n${censusRaw.stderr.toString().split("\n").slice(-15).join("\n")}`) `` + `:99` `process.exit(1)` · `:264-266` `` `\n${gaps.length} contract role(s) never projected, across ${bySlug.size} components:` `` + per-slug `` `  ${slug.padEnd(28)} ${list.map((g) => g.aria).join(", ")}` `` · `:271` `process.exit(1);` |
| `drift:adapter-manifests` | `packages/gpui/preview/scripts/adapter-manifest-drift.ts` | `:70` `` console.error(`${adapter.name} adapter manifest drift:`) `` + `:71-73` `` `  missing: ${missing.join(", ")}` `` / `` `  no implementation: ${phantom.join(", ")}` `` / `` `  duplicate: ${repeated.join(", ")}` `` · `:82` `if (failed) process.exit(1);` |
| `svelte:surface-audit` | `packages/svelte/components/scripts/surface-audit.ts` | `:116-121` `` if (coverage.gaps.length > 0) { … console.log(`- ${gap.name} (${gap.slug}): ${gap.missing.join(", ")}`); … process.exitCode = 1; } `` (per-gap line lists missing surfaces from `:189-194`: `contract`, `component registry`, `specimen registry`, `usage docs`) |
| `docs:lint` | `packages/svelte/preview/scripts/lint-docs.ts` | `:3263-3266` `` `contract prop drift: ${f.slug}.md documents prop(s) not implemented in the ${f.slug} Svelte component: ${f.contractOnly.join(", ")}` `` · `:3271-3274` `` `contract/spec drift: ${f.slug}.md documents prop(s) absent from its poodle-specs Spec: ${f.missing.join(", ")}` `` · `:3281-3283` `if (errors.length > 0) { throw new Error(errors.join("\n")); }` |

### 4.8 Four-runtime drift count (per pilot component)

Counting rule: per source, documented intentional deltas + open parity items,
reported separately before totalling. Parity-doc open items are the enumerated
open bullets in the runtime-gap sections (the pass-level status line is quoted
alongside; for `text-input.md` the status line lags the bullets — historical
file lag per `OBS-04`). The cross-runtime parity reports contain no per-pilot
entries (GPUI `deltaRegister` is suite-scoped; Jetstream `parityWithSvelte` is
summary strings) — counted as 0 with citations.

| Source | Button | RangeSlider | TextInput |
|---|---|---|---|
| `docs/parity/button.md` status line `gpui=2 jetstream=2` | 4 open | — | — |
| `docs/parity/range-slider.md` status line `gpui=1 jetstream=1` | — | 2 open | — |
| `docs/parity/text-input.md` status line `gpui=2 jetstream=2` | — | — | 4 open (status line; enumerated bullets: 8 GPUI + 9 Jetstream = 17) |
| `docs/parity/*.md` accepted deltas (enumerated `accepted:` bullets) | 4 (GPUI: no-ARIA, translateY omitted; Jetstream: no-ARIA channel, interaction in event loop) | 8 (GPUI: track-height fixed, per-thumb focus/Tab, no-ARIA, commit timing, pointer overlap; Jetstream: no focus ring/keyboard, no-ARIA channel, drag in event loop) | 5 (GPUI: no-ARIA, per-size font ramp; Jetstream: no-ARIA, editing in event loop, per-size font ramp) |
| `packages/gpui/cross-runtime-parity-report.json` | 0 (deltaRegister `sectionIds` are suites: form/table/browse/detail/picker/media/notification/command/workspace; no `button`/`range-slider`/`text-input` entry) | 0 | 0 |
| `packages/jetstream/cross-runtime-parity-report.json` | 0 (no pilot mentions; `parityWithSvelte.componentCoverage` is a coverage string) | 0 | 0 |
| Corpus EXT rows | 3 | 2 | 1 |
| Corpus GAP rows touching component (by GAP register affected requirements) | 3 (GAP-02, GAP-06, GAP-07) | 4 (GAP-01, GAP-02, GAP-03, GAP-04) | 3 (GAP-01, GAP-02, GAP-05) |
| **Total (status-line parity opens + accepted + reports + EXT + GAP)** | **4+4+0+0+3+3 = 14** | **2+8+0+0+2+4 = 16** | **4+5+0+0+1+3 = 13** (or **26** counting the enumerated text-input open bullets 17 instead of the status-line 4) |

## 5. Measured Observations (recorded, not resolved)

- **Jetstream RangeSlider densities specimen uses the standard variant** while
  contract §13 / corpus `RNG-25` states density specimens use embedded bipolar:
  `packages/jetstream/preview/src/specimens/range_slider.rs` ("Densities" group
  builds `RangeSliderSpec::new(25.0, 75.0)` without
  `with_embedded_control`); Svelte/React densities snippets use embedded
  bipolar. Specimen-content divergence, not a public-semantics disagreement —
  recorded for the orchestrator, not a stop condition.
- **Corpus §8 EXT counts (10) ≠ row-level EXT marks (6)** — see §4.5.
- **GPUI Button specimen renders "Success tone" and "Warning tone" rows** beyond
  the contract §13 set (`packages/gpui/preview/src/specimens/button.rs:183,221`);
  the Success row is the `UNKNOWN-02` superset surface (`FIX-BTN-08`). Warning
  tone is contract-covered (`BTN-02`).
- **Svelte/React TextInput specimens render Slug/Search/Prefix-suffix/Multiline
  groups** beyond contract §13's four — the contract set is a subset of the
  specimen, not a mismatch.
- **`packages/tokens/artifacts/rust/*` are tracked in git and present in the
  worktree** (10 files, last touched by `45caae82`); they are rewritten by
  `effigy docs:check` and must be restored with
  `git checkout -- packages/tokens/artifacts/rust/` (card ruling 6). This card
  restored them after validation and committed nothing from that directory.

## 6. Scope Boundaries

This manifest records fixtures and measurements only. It does not alter
component behavior, public APIs, contracts, specs, working rules, roadmap/card
status, or Effigy configuration; it does not create packages, crates, shims,
generators, or schema; it does not refresh any visual or native baseline. Only
the three writable paths changed: this manifest, the batch log, and
`PAPERCUTS.md`.
