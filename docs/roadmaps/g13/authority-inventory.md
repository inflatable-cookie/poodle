# g13.001 Authority Inventory

Status: evidence baseline (worker-produced, orchestrator-owned)
Updated: 2026-08-11
Owner: Poodle core (evidence collected by batch `001`)
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md` (`IR-01`–`IR-12`),
`docs/architecture/001-poodle-system-shape.md`,
`docs/architecture/006-headless-core-and-machine-model.md`,
`docs/contracts/001-working-rules.md`

## Purpose

Measured current-state map of every definition and evidence surface the g13
Rust-authored IR must subsume or interoperate with. This document records
authority, authorship, consumers, drift checks, and current duplication for the
orchestrator to rule on authority freezing and crate placement (`IR-12`). It
makes no recommendation.

Method: direct repository reads and generator runs on
`thread/g13-001-authority-inventory` at `535fcf22` + the scoped docs-baseline
repair. All counts are measured on this tree. Command evidence and exit states
are in `docs/logs/2026-08/11-g13-001-authority-inventory.md`.

## Legend

- **Authored**: hand-written source (authoritative where stated).
- **Generated**: emitted by a checked-in generator; never hand-edited.
- **Source of truth**: the file(s) a surface must agree with; when two
  documents conflict, the narrower current authority wins
  (`docs/contracts/001-working-rules.md`).
- **Drift check**: the Effigy gate or script that fails when the surface
  disagrees with its source of truth.
- **Duplication/bypass**: where the same semantic surface is expressed more
  than once, or a consumer skips the shared path.

## 1. Measured Counts

| Surface | Count | Path |
|---|---|---|
| Component contracts | 170 | `docs/contracts/components/*.md` (excl. README) |
| Contract index entries (root README) | 170 + template | `docs/contracts/README.md` |
| Component-contract index entries | 170 | `docs/contracts/components/README.md` |
| Svelte component files | 164 | `packages/svelte/components/src/*.svelte` |
| Svelte public component exports | 163 | `packages/svelte/components/src/index.ts` |
| React component files | 165 | `packages/react/components/src/*.tsx` |
| Core machine/domain modules | 40 | `packages/core/src/*.ts` (excl. `index.ts`, `tokens/`) |
| Core shared styles | 159 | `packages/core/src/styles/*.css` |
| `poodle-specs` spec modules | 156 | `packages/contracts/components/src/*.rs` (excl. lib) |
| `poodle-headless` modules | 23 | `packages/contracts/headless/src/*.rs` |
| `poodle-render` component modules | 159 | `packages/render/src/*.rs` (excl. lib) |
| `poodle-render` `pub fn` total | 269 | across `packages/render/src/*.rs` |
| GPUI preview specimens | 146 | `packages/gpui/preview/src/specimens/*.rs` |
| Jetstream preview specimens | 151 | `packages/jetstream/preview/src/specimens/*.rs` |
| GPUI adapter `RenderComponent<Spec>` impls | 101 | `packages/gpui/adapter/src/render_*.rs` |
| Jetstream adapter `RenderComponent<Spec>` impls | 108 | `packages/jetstream/adapter/src/render_*.rs` |
| Jetstream `js_*` compat shims | 157 | `packages/jetstream/preview/src/compat.rs` (1104 ln) |
| GPUI preview node facades | 6470 ln | `packages/gpui/preview/src/node_compat.rs` |
| Jetstream node-chrome builder (`El`) | 464 ln | `packages/jetstream/preview/src/nel.rs` |
| Svelte specimen files / registry entries | 161 / 163 | `packages/svelte/preview/src/specimens/` |
| React specimen files | 156 | `packages/react/preview/src/gallery/specimens/` |
| Svelte preview component-registry entries | 164 | `packages/svelte/preview/src/component-registry.ts` |
| Docs navigation sections / families | 12 / 4 | `packages/svelte/preview/src/catalog.ts` + `parity.ts` |
| Svelte package exports (components + helpers) | 200 (163 + 37) | `packages/svelte/preview/artifacts/parity-report.json` |
| Historical parity audit files | 139 | `docs/parity/*.md` (explicitly historical) |

## 2. Web Definition And Evidence Surfaces

### 2.1 Component contracts and contract indexes

| Owner/path | Authored/generated | Source of truth | Consumers | Drift check |
|---|---|---|---|---|
| `docs/contracts/components/<slug>.md` | authored | itself (per working rules, contracts define public component semantics) | Svelte/React shells, `poodle-specs`, `poodle-render`, native previews, drift scripts | `docs:contract-drift` (contract ↔ Svelte props), `docs:spec-drift` (contract ↔ `poodle-specs`), `drift:roles` (contract ↔ Jetstream ARIA roles), `svelte:surface-audit` (contract file presence per export), `docs:lint` (indexes) |
| `docs/contracts/components/README.md` | authored index | directory contents of `docs/contracts/components/` | docs readers, `docs:lint` | `docs:lint` `validateContractIndexes` (bullet list vs file set) |
| `docs/contracts/README.md` | authored index | `template/` + `components/` file set | docs readers, `docs:lint` | `docs:lint` `validateContractIndexes` |

Current duplication/bypass: none — 170 contracts, 170 index entries, indexes
repaired by this batch (were missing 3: `keyboard`, `mod-matrix-grid`,
`waveform-display`).

### 2.2 TypeScript machines/styles (core) and Svelte/React shells

| Surface | Owner/path | Authored/generated | Source of truth | Consumers | Drift check |
|---|---|---|---|---|---|
| Behavior machines + domain math | `packages/core/src/<name>.ts` (e.g. `slider.ts`, `input.ts`, `tabs.ts`, `date.ts`) | authored | contract "Behavior Machine" sections | Svelte shells, React shells, core unit tests | `test:core`, contract-prop drift; conformance vectors mirrored in `poodle-headless` (g11.006) |
| Shared component styles | `packages/core/src/styles/<slug>.css` | authored | contract layout/token sections | Svelte + React shells import them | `test:components` anatomy parity; visual gates |
| Generated web tokens | `packages/tokens/artifacts/ts/*`, `packages/core/src/tokens/generated/*`, `packages/core/src/tokens/theme-options.ts` | generated | `packages/tokens/schema/` (DTCG) via `build-tokens.ts` | core, both web previews, display controls | `audit:tokens` (`--check`) |
| Generated icons | `packages/core/src/icons/generated.ts`, `aliases.generated.ts`, `default-icons.json` | generated | `scripts/build-default-icons.ts` + lucide source | icon components, previews | `audit:icons` (`--check`) |
| Svelte shells | `packages/svelte/components/src/<Name>.svelte` (164 files, 163 exported) | authored | contract + core machines/styles | docs preview, consumers of `@inflatable-cookie/poodle-svelte` | `test:components`, `test:parity`, `docs:contract-drift`, `svelte:surface-audit` |
| React shells | `packages/react/components/src/<Name>.tsx` (165 files) | authored | same contracts + core (experimental per architecture 001) | React preview | `test:components`, `test:parity` (Svelte ↔ React anatomy parity) |

Duplication: Svelte and React shells are two authored implementations of one
contract set — the four-runtime duplication the IR targets (`IR-01`). The
framework-free behavior lives once in core; shells stay thin.

### 2.3 Svelte/React specimen definitions, registries, preview shells, reports

| Surface | Owner/path | Authored/generated | Source of truth | Consumers | Drift check |
|---|---|---|---|---|---|
| Svelte specimen definitions | `packages/svelte/preview/src/specimens/<Slug>Specimen.svelte` (161 files) | authored | contract specimen sections | Svelte preview pages, visual sweep (`test/visual`), a11y sweep | `svelte:surface-audit` (slug presence), `test:visual` |
| Svelte specimen registry | `packages/svelte/preview/src/specimens/registry.ts` (163 slugs) | authored | specimen file set | preview routing, visual/a11y sweeps | `svelte:surface-audit` |
| Svelte component registry | `packages/svelte/preview/src/component-registry.ts` (164 `entry(...)` calls) | authored | Svelte index exports + contracts | docs catalog, parity targets, Jetstream registry derivation | `svelte:surface-audit`, `docs:lint` (via parity coverage) |
| Svelte usage docs (per component) | `packages/svelte/preview/src/component-docs.ts` (163 entries) | authored | contract props + Svelte shells | `UsageDocs.svelte`, exported `component-docs.json` (GPUI preview + React gallery) | `svelte:surface-audit` (slug presence); contract drift via shell comparison |
| Svelte docs catalog | `packages/svelte/preview/src/catalog.ts` | authored | component registry + families | parity targets, sections | `docs:lint` `validateDocsCatalog` |
| Svelte preview shell | `packages/svelte/preview/src/App.svelte`, `router.ts`, `pages/*`, `sections/*`, `components/*` (incl. `SpecimenLayout.svelte`, `DisplayControls.svelte`) | authored | catalog + parity state | docs site | `docs:build`, `test:visual` |
| Svelte preview parity catalog | `packages/svelte/preview/src/parity.ts` (`packageSurfaceCoverage`, `parityTargets`, `docsNavigationSections`) | authored | Svelte index exports + catalog sections | parity report generator, lint | `docs:lint` `validatePackageSurfaceCoverage` |
| Svelte accessibility catalog | `packages/svelte/preview/src/accessibility.ts` | authored | contracts' accessibility sections | a11y report generator, lint | `docs:lint` `validateAccessibilityAudit` |
| Parity report (Svelte + React artifacts) | `packages/svelte/preview/scripts/parity-report.ts` → `packages/{svelte,react}/preview/artifacts/parity-report.json` | generated | `parity.ts` + `catalog.ts` + `packages/gpui/cross-runtime-parity-report.json` | `docs:lint`, shared-demo-audit check, consumers | regenerated by `report:parity`; drift surfaces through `docs:lint` |
| Component docs artifact | `packages/{svelte,react}/preview/artifacts/component-docs.json` | generated | `component-docs.ts` | GPUI `contract_usage_docs.rs`, React `UsageDocs.tsx` | regenerated by `docs:export-json` / `react:docs` |
| Accessibility report artifact | `packages/{svelte,react}/preview/artifacts/accessibility-report.json` | generated | `accessibility.ts` | consumers | regenerated by `report:accessibility` |
| React gallery | `packages/react/preview/src/gallery/` (`App.tsx`, `registry.ts`, `specimen-map.ts`, `SpecimenLayout.tsx`, `DisplayControls.tsx`, `specimens/*`) | authored (specimen definitions) / re-export (usage docs from Svelte) | same contracts; usage docs canonically Svelte | React preview | `react:docs` re-exports Svelte canon; `test:visual` cross-framework sweep |
| Shared demo-app audit | `packages/shared-demo-app-audit.json` | authored evidence, counts must equal generated parity report | `packages/svelte/preview/artifacts/parity-report.json` | `docs:lint` `validateSharedDemoAppAudit` | counts compared per package (repaired this batch: 186 → 200) |

Key finding: React's usage docs and parity data are **re-exports of the Svelte
preview sources** (`gallery/component-docs.ts` imports `svelte/preview/src/
component-docs.ts`; React `build-parity-report.ts` calls the Svelte
`writeParityReport`). No second hand-maintained web definition source exists
for docs; the duplication is in the shells (§2.2) and native tiers (§3).

## 3. Rust Definition And Evidence Surfaces

### 3.1 Specs / headless / render / node path

| Surface | Owner/path | Authored/generated | Source of truth | Consumers | Drift check |
|---|---|---|---|---|---|
| `poodle-specs` | `packages/contracts/components/src/<name>.rs` (156 modules, e.g. `button.rs`) | authored | component contracts | `poodle-render`, adapters, native previews, `docs:spec-drift` | `docs:spec-drift` (contract ↔ spec props), `test:contracts` |
| `poodle-headless` | `packages/contracts/headless/src/` (23 modules, e.g. `slider.rs`, `text_input.rs`) | authored Rust mirror of core machines | core TS machines (g11.006 conformance) + contracts | `poodle-render`, node backend (undo rules) | `test:contracts`; core unit tests as vectors |
| `poodle-node` | `packages/contracts/node/src/lib.rs` + related | authored | shared render vocabulary (architecture 001/006) | `poodle-render` emits, GPUI node backend + `jetstream-poodle` interpret | `cargo test -p poodle-node` (in `test:contracts`) |
| `poodle-render` | `packages/render/src/<name>.rs` (159 component modules; `Spec + Theme → Node`) | authored | contracts + specs + headless | GPUI preview (direct), Jetstream preview (via `js_*`), adapters | `check:gpui` runs `cargo test -p poodle-render`; `drift:handlers`, `drift:events` |
| `poodle-adapter` | `packages/contracts/adapter/src/lib.rs` (`RenderComponent<Spec>` trait, theme provider trait) | authored | shared render contract | GPUI + Jetstream adapters | `drift:adapter-manifests` |
| `poodle-tokens` (Rust) | `packages/contracts/tokens/src/lib.rs` includes `packages/tokens/artifacts/rust/*` via `#[path]` | generated artifacts, included authored crate | DTCG schema → `build-tokens.ts` | render, adapters, previews | `audit:tokens` |
| Layout/style/events/markdown contracts | `packages/contracts/{layout,style,events,markdown}/src` | authored | shared vocabulary | `poodle-render`, adapters | `test:contracts` |

No direct native bypass of this path exists in the adapters: both
`RenderComponent` sets delegate to `poodle-render` functions. The bypasses are
in the previews (§3.2/§3.3).

### 3.2 GPUI registry, specimens, adapters, evidence

| Surface | Owner/path | Authored/generated | Source of truth | Consumers | Drift check |
|---|---|---|---|---|---|
| GPUI adapter | `packages/gpui/adapter/src/render_*.rs` (9 files, 101 `impl RenderComponent<XxxSpec> for GpuiAdapter`) + `SUPPORTED_PRIMITIVES/COMPOSITES/SHELL` manifests in `lib.rs` | authored | `poodle-render` + contracts | preview demo, release surface | `drift:adapter-manifests` (impl ↔ manifest exact match) |
| GPUI node backend | `packages/gpui/node-backend/src/{lib,input_text,interaction,style,ime}.rs` | authored | `poodle-node` vocabulary | GPUI preview (`poodle_gpui_node_backend::to_gpui`) | `check:gpui` (`cargo test -p poodle-gpui-node-backend`) |
| GPUI preview specimens | `packages/gpui/preview/src/specimens/*.rs` (146) | authored | Svelte specimen pages (mirror), contracts | live preview | `test:native-visual` (pixel baselines) |
| GPUI preview node facades | `packages/gpui/preview/src/node_compat.rs` (6470 ln, `Type::from_spec(spec, theme)` facades, e.g. `TextInput`, `ThemeSelect`, `Slider`, `Tabs`, `SidebarNav`, `Code`, `Eyebrow`) | authored preview-only compat layer | `poodle-render` | preview chrome + selected specimens | none (preview-internal) |
| GPUI preview shell | `packages/gpui/preview/src/main.rs` (2207 ln), `app_state.rs` (788 ln), `component_registry.rs` (989 ln), `contract_usage_docs.rs` (reads `packages/svelte/preview/artifacts/component-docs.json`), `providers.rs`, `style_bridge.rs`, `token_view.rs`, `usage_docs_view.rs` | authored | Svelte preview IA (registry "mirrors" comment) | live preview | `test:native-visual`; `check:gpui` compile |
| GPUI evidence JSONs | `packages/gpui/*.json`: `parity-priority-matrix.json`, `preview-app-baseline.json`, `structural-primitives-baseline.json`, `action-field-primitives-baseline.json`, `selection-feedback-date-baseline.json`, `overlay-navigation-menu-baseline.json`, `form-validation-remediation-composites-baseline.json`, `data-browse-detail-picker-media-baseline.json`, `native-accessibility-proof.json`, `cross-runtime-parity-report.json` | authored evidence | per-file generation labels (g09.018) | `docs:lint` validators, parity report generator | `docs:lint` (each validator) |

### 3.3 Jetstream registry, specimens, adapters, evidence

| Surface | Owner/path | Authored/generated | Source of truth | Consumers | Drift check |
|---|---|---|---|---|---|
| Jetstream adapter | `packages/jetstream/adapter/src/render_*.rs` (7 files, 108 `impl RenderComponent<XxxSpec> for JetstreamAdapter`) + `SUPPORTED_PRIMITIVES/COMPOSITES` manifests in `lib.rs` | authored | `poodle-render` + contracts | test suites (`test:jetstream-adapter`) | `drift:adapter-manifests` |
| Jetstream engine-boundary adapter | `../../jetstream/crates/jetstream-poodle` (`to_js_el(Node) → JsEl`) — **sibling repo, outside this repo's boundary** | authored (sibling) | `poodle-node` | Jetstream preview shell | none in this repo (sibling-owned) |
| Jetstream preview specimens | `packages/jetstream/preview/src/specimens/*.rs` (151) | authored | Svelte specimen pages (mirror comments) | `bin/snap.rs`, `bin/a11y.rs`, live preview | `test:jetstream-visual` (offscreen baselines), `test:jetstream-a11y` (headless role tree) |
| Jetstream preview compat layer | `packages/jetstream/preview/src/compat.rs` (1104 ln, 157 `js_*` render-only shims wrapping `pr::<component>`), `nel.rs` (464 ln, `El` fluent node-chrome builder) | authored compat shim | `poodle-render` + `poodle-node` | all Jetstream specimens + shell chrome | none (preview-internal) |
| Jetstream preview shell | `packages/jetstream/preview/src/shell.rs` (699 ln), `main.rs` (1579 ln), `app_state.rs` (823 ln), `component_registry.rs` (1115 ln), `jsx.rs`, `lib.rs` | authored | Svelte preview IA; registry claims derivation from `packages/svelte/preview/src/component-registry.ts` | live preview | none in-repo (registry doc comment says "re-derive from Svelte registry"; **no generator script exists in this repo**) |
| Jetstream a11y/visual tooling | `packages/jetstream/preview/src/bin/{a11y,snap}.rs`, `test/native-visual/{jetstream.ts,ax-audit.ts,ax-probe.swift}` | authored | contracts (roles), component registry | CI-local gates | `test:jetstream-a11y`, `jetstream-a11y:roles`, `test:jetstream-ax`, `test:jetstream-visual` |
| Jetstream evidence JSON | `packages/jetstream/cross-runtime-parity-report.json` | authored evidence | — | — | `docs:lint` (in `validateGpuiCrossRuntimeParityReport` dependency chain) |

## 4. Direct Jetstream `RenderComponent<Spec>` Implementations And Preview Compatibility Layers

Enumerated (not summarized as generic debt):

1. **`packages/jetstream/adapter/src/render_*.rs` — 108 direct
   `impl RenderComponent<XxxSpec> for JetstreamAdapter`** across
   `render_action.rs`, `render_input.rs`, `render_selection.rs`,
   `render_overlay.rs`, `render_composites.rs`, `render_feedback.rs`,
   `render_structural.rs`, with `SUPPORTED_PRIMITIVES` (60) /
   `SUPPORTED_COMPOSITES` (48) manifests in `lib.rs`. These delegate to
   `poodle-render`; they are the adapter's test/evidence surface, not a
   component fork.
2. **`packages/jetstream/preview/src/compat.rs` — 157 `js_*` shims** preserving
   the retired `poodle-jetstream-components` call shapes (`js_button`, ...)
   implemented on `poodle-render` (`Spec + Theme → Node`). The retired tier was
   deleted in `ee704699`; the shims keep specimen sources unchanged.
3. **`packages/jetstream/preview/src/nel.rs` — `El` fluent builder** preserving
   the old `ui_element` chrome surface on `poodle_node::Node` (`div().flex_col()
   .gap(..)`). Shell converts once at the edge via
   `jetstream_poodle::to_js_el`.
4. **GPUI analog: `packages/gpui/preview/src/node_compat.rs` — preview facades
   (`Type::from_spec(spec, theme)`)** for chrome and a subset of specimens,
   rendering through `poodle-render` + `poodle_gpui_node_backend::to_gpui`.
   GPUI specimens call `poodle_render::<component>` directly; there is no
   `js_*`-style shim count because the GPUI preview predates the Jetstream
   shim pattern — both are preview-only compat, not public API.

None of these layers ship to consumers; all are `internal-tooling` preview
crats. They are the concrete "compatibility layer" inventory the pilot must
either keep stable (`IR-09`) or subsume via generated scene IR.

## 5. Pilot Component File Maps (Button, RangeSlider, TextInput)

Four-runtime maps; every path verified present on this tree.

### Button

| Runtime | Definition | Preview |
|---|---|---|
| Contract | `docs/contracts/components/button.md` (styled-only, no machine) | — |
| Web core | (no machine) `packages/core/src/styles/button.css` | — |
| Svelte | `packages/svelte/components/src/Button.svelte` | `packages/svelte/preview/src/specimens/ButtonSpecimen.svelte`; docs `component-docs.ts` `"button"` |
| React | `packages/react/components/src/Button.tsx` | `packages/react/preview/src/gallery/specimens/ButtonSpecimen.tsx` |
| Rust spec | `packages/contracts/components/src/button.rs` | — |
| Headless | (styled-only; no headless module) | — |
| Render | `packages/render/src/button.rs` | — |
| GPUI | — | `packages/gpui/preview/src/specimens/button.rs` (calls `poodle_render::button` + `to_gpui`) |
| Jetstream | — | `packages/jetstream/preview/src/specimens/button.rs` (calls `compat::js_button`) |

### RangeSlider

| Runtime | Definition | Preview |
|---|---|---|
| Contract | `docs/contracts/components/range-slider.md` | — |
| Web core | `packages/core/src/slider.ts` (`rangeSliderTransition`, `rangeSliderVisualState`, `createRangeSliderControlContext`) + `styles/range-slider.css` | — |
| Svelte | `packages/svelte/components/src/RangeSlider.svelte` | `packages/svelte/preview/src/specimens/RangeSliderSpecimen.svelte` |
| React | `packages/react/components/src/RangeSlider.tsx` | `packages/react/preview/src/gallery/specimens/RangeSliderSpecimen.tsx` |
| Rust spec | `packages/contracts/components/src/range_slider.rs` | — |
| Headless | `packages/contracts/headless/src/slider.rs` | — |
| Render | `packages/render/src/range_slider.rs` | — |
| GPUI | — | `packages/gpui/preview/src/specimens/range_slider.rs` |
| Jetstream | — | `packages/jetstream/preview/src/specimens/range_slider.rs` |

### TextInput

| Runtime | Definition | Preview |
|---|---|---|
| Contract | `docs/contracts/components/text-input.md` | — |
| Web core | `packages/core/src/input.ts` (validation state, coercion, clamp, slug) + `styles/text-input.css` | — |
| Svelte | `packages/svelte/components/src/TextInput.svelte` (20 KB) | `packages/svelte/preview/src/specimens/TextInputSpecimen.svelte` |
| React | `packages/react/components/src/TextInput.tsx` | `packages/react/preview/src/gallery/specimens/TextInputSpecimen.tsx` |
| Rust spec | `packages/contracts/components/src/text_input.rs` | — |
| Headless | `packages/contracts/headless/src/text_input.rs` (`insert_transition` etc.) | — |
| Render | `packages/render/src/text_input.rs` (`text_input`, `text_input_with_change`, `text_input_with_handlers`) | — |
| GPUI | — | `packages/gpui/preview/src/specimens/text_input.rs`; `node_compat::TextInput` facade used by preview chrome |
| Jetstream | — | `packages/jetstream/preview/src/specimens/text_input.rs` |

Per-component surface audit shows all three at full coverage on the four
surfaces the audit tracks (contract, component registry, specimen registry,
usage docs). Contract-prop and contract-spec drift both green for the pilots.

## 6. Preview Header / Theme Selector And Size/Density Axis Maps

### Preview header + theme selector (four runtimes)

| Runtime | Shell/top bar | Controls | Theme source |
|---|---|---|---|
| Svelte | `packages/svelte/preview/src/App.svelte` (`poodle-app-top-bar`) | `packages/svelte/preview/src/components/DisplayControls.svelte` — real `ThemeSelect`, `ToggleGroup` (density/size), `Slider` (contrast) | `packages/core/src/tokens/theme-options.ts` (generated) |
| React | `packages/react/preview/src/gallery/App.tsx` | `packages/react/preview/src/gallery/DisplayControls.tsx` | same generated `themeOptions` |
| GPUI | `packages/gpui/preview/src/main.rs` (`render_display_controls`, `render_status_pills`, nav tabs) | real `ThemeSelect` facade from `node_compat.rs` + toggle groups; state in `app_state.rs` (`ThemePreset`, `Density`, `ControlSize`, contrast) | `poodle_tokens` typed themes |
| Jetstream | `packages/jetstream/preview/src/shell.rs` (`build_controls_bar`, `build_shell`) | real `js_theme_select` + `js_tabs` (pill) + `js_pill` + `js_toggle_group` + contrast slider; actions routed in `main.rs` | `poodle_tokens` themes via `JetstreamThemeProvider` |

All four use the real `ThemeSelect` component — no hand-rolled theme picker
exists in any preview.

### Size/density specimen axes (four runtimes)

| Runtime | Shared layout helper | Axis mechanism |
|---|---|---|
| Svelte | `packages/svelte/preview/src/components/SpecimenLayout.svelte` | `sizes`/`densities` snippets + Examples/Sizes/Densities tabs |
| React | `packages/react/preview/src/gallery/SpecimenLayout.tsx` | same API (`sizes(size)`, `densities(density)` callbacks) |
| GPUI | `packages/gpui/preview/src/specimens/specimen_layout.rs` | three-pane builder closures (`|size, theme|`, `|density, theme|`), mirrors Svelte |
| Jetstream | none (no shared helper) | per-specimen inline sweeps (`with_size(Xs..Xl)`, `with_density(Compact..Comfortable)` rows, e.g. `button.rs`, `breadcrumbs.rs`, `checkbox.rs`) |

Web + GPUI share one SpecimenLayout contract; Jetstream re-expresses the axis
per specimen — a scene-IR subsume target.

## 7. Crate-Placement Evidence (comparison only — no recommendation)

### 7.1 Workspace boundaries

- **No root `Cargo.toml`.** Crates are independent trees; each manifest is
  resolved individually (`effigy.tasks.toml` header comment). The Jetstream
  preview path-depends on the **sibling jetstream repo**
  (`../../../../jetstream/crates/{jetstream-ui,jetstream-renderer,
  jetstream-platform,jetstream-input,jetstream-text,jetstream-poodle}`),
  which pins this repo to a sibling checkout and forces `local-only` gates.
- Web packages are a bun workspace: `workspaces` in root `package.json`
  (`core`, `tokens`, `svelte/components`, `svelte/preview`,
  `react/components`, `react/preview`, `bridges/underlay`).

### 7.2 Publication metadata (all `publish = false`, but public-intent varies)

| Crate | `[package] publish` | `metadata.poodle` public-intent / channel / stability | Release-manifest kind |
|---|---|---|---|
| `poodle-adapter`, `poodle-headless`, `poodle-markdown`, `poodle-specs`, `poodle-events`, `poodle-layout`, `poodle-style`, `poodle-tokens`, `poodle-node` | false | true / preview / pre-release | `contract-crate` |
| `poodle-render` | false | true / preview / pre-release | `renderer-components` |
| `poodle-gpui` | false | true / preview / pre-release | `renderer-adapter` |
| `poodle-jetstream` | false | true / preview / pre-release | `renderer-adapter` |
| `poodle-gpui-node-backend` | false | true / preview / pre-release | `renderer-backend` |
| `poodle-gpui-preview`, `poodle-jetstream-preview` | false | false / internal / internal-tooling | `tooling` |

Web packages: `@inflatable-cookie/poodle-tokens` (source-of-truth),
`poodle-core`/`poodle-svelte`/`poodle-react` (runtime-package, public-intent
true), previews and install-smoke are tooling.

### 7.3 Dependency direction (verified from manifests)

- `poodle-node` ← `poodle-render` ← `poodle-gpui` / `poodle-jetstream`
  (adapters depend on render; render depends on specs/headless/tokens/
  layout/style/events/markdown/adapter).
- `poodle-render` **dev-depends** on `poodle-jetstream` for the token resolver
  only; the Cargo.toml comment records this as migration debt ("the resolver
  logically belongs in contracts and its relocation is part of the migration").
- `poodle-gpui` ← `gpui` framework crate; `poodle-gpui-node-backend` ← `gpui`,
  `poodle-node`, `poodle-headless` (undo coalescing only).
- `poodle-jetstream` ← jetstream runtime crates (sibling); the
  `jetstream-poodle` converter lives in the sibling repo and is not here.
- Previews depend on adapters + render + all contracts; previews are
  `internal-tooling`.

### 7.4 Source-consumer constraints

- Web TS consumers import generated artifacts (`core/src/tokens/generated/*`,
  `icons/generated.ts`, `tokens/artifacts/ts/*`) and must not hand-edit them
  (`audit:tokens`, `audit:icons` gates).
- Rust consumers compile generated tokens into `poodle-tokens` via
  `#[path = "../../../tokens/artifacts/rust/mod.rs"]` — generated Rust must
  stay reachable from `packages/contracts/tokens`.
- Cross-repo constraint: any crate the Jetstream preview needs must remain
  path-reachable from `packages/jetstream/preview/Cargo.toml` (no crates.io
  publish until public-intent flips).
- Framework purity: core must not import Svelte/React; `poodle-render` must
  not import gpui/jetstream (adapter boundary per architecture 001).

### 7.5 Plausible locations (evidence, not recommendation)

| Placement option | Evidence supporting | Evidence against / cost |
|---|---|---|
| New crate in `packages/contracts/` (e.g. `poodle-ir` beside specs/headless) | contract crates are the pure, publishable, sibling-independent layer; `poodle-ir` needs no runtime deps (`IR-02` serializable boundary); generated web TS already flows from `tokens/`-style artifact dirs | `poodle-render` dev-dep debt shows contract tree is already mixed; codegen producing web TS from a contracts crate crosses the repo's own "contracts are pure Rust" posture |
| Crate under `packages/render/` (e.g. `poodle-codegen` beside render) | render is the single native component implementation; IR lowers to `poodle-node` which render already emits | render is `renderer-components` in the release manifest; codegen is a tool, not a runtime component; render currently has no TS or docs emission |
| New top-level `packages/codegen/` (or `tools/`) | codegen emits web TS + JSON + docs registries, a tool-shaped surface; keeps contract/render crates pure | no precedent directory exists; new package creation is a g13 decision, not a worker call (`IR-12`) |
| Generate into existing web packages | web consumers already accept generated artifacts (`tokens/artifacts/ts`, `core/src/tokens/generated`) | web packages are bun-workspace TS; a Rust generator writing into them must cross the build-tool boundary |

Crate placement ruling is the orchestrator's decision (`IR-12`); this table
records only the observable constraints above.

## 8. Current Duplication And Bypass Summary

1. **Four-runtime component definitions**: Svelte shell, React shell,
   `poodle-specs` (Rust), `poodle-render` (Rust) — four authored expressions of
   one contract per component (the IR's reason for existing, `IR-01`).
2. **Web machines mirrored in Rust**: core TS machines (`slider.ts`,
   `input.ts`, ...) and `poodle-headless` Rust mirrors (g11.006 conformance
   vectors) — mechanical duplication by design.
3. **Preview compat layers**: Jetstream `compat.rs` (157 `js_*` shims) +
   `nel.rs` (464 ln) and GPUI `node_compat.rs` (6470 ln) re-express specimen
   chrome and old call shapes over the node path.
4. **Registry mirrors**: GPUI `component_registry.rs` and Jetstream
   `component_registry.rs` mirror the Svelte `component-registry.ts` taxonomy;
   Jetstream's file claims to be generated from the Svelte source but **no
   generator exists in this repo** (manual re-derivation documented in its
   header) — a drift risk recorded, not repaired (out of scope).
5. **Historical parity audits**: `docs/parity/*.md` (139 files) describe
   deleted tiers (`packages/gpui/components`, `packages/jetstream/components`);
   explicitly non-authoritative (see `docs/parity/README.md`).
6. **Token generation**: single DTCG source → CSS/TS/Rust targets
   (`build-tokens.ts`) — no duplication; the committed Rust artifacts drift
   from the checked-in generator output (formatting) as of `45caae82`
   (`audit:tokens` red at HEAD; see batch log).

## 9. Scope Boundaries

This inventory records the measured state and repaired only the named
docs-baseline failures (contract index, preview coverage, AgentSubagent usage
docs, shared-demo-audit counts). It does not alter component behavior, public
APIs, contracts, architecture, specs, working rules, roadmap/card status, or
Effigy configuration, and it does not create packages, crates, shims, or
generators.
