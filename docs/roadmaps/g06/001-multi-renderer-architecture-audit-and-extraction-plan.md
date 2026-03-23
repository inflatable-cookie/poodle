# g06.001 — Multi-Renderer Architecture Audit and Extraction Plan

Status: Completed
Updated: 2026-03-14

## Objective

Audit the existing `poodle-gpui-*` crates, classify every module and type as
renderer-agnostic or GPUI-biased, and produce the extraction plan that defines
the new crate topology, naming conventions, and migration path. Document
Jetstream's `game_ui` API surface as a second rendering consumer to validate
that the shared contracts can serve both targets.

## Audit Results

### Current Crate Inventory

| Crate | Location | Modules | Types | Dependencies |
|-------|----------|---------|-------|--------------|
| `poodle-gpui-tokens` | `packages/gpui/tokens/` | 5 (semantic, primitives, themes, density, metadata) | String constants only | None |
| `poodle-gpui-primitives` | `packages/gpui/primitives/` | 43 (42 specs + types) | 42 specs, 50+ shared enums/structs | `poodle-gpui-tokens` |
| `poodle-gpui-composites` | `packages/gpui/composites/` | 18 (16 specs + types + lib) | 16 specs, 18+ shared types | `poodle-gpui-primitives`, `poodle-gpui-tokens` |
| `poodle-gpui-workstation` | `packages/gpui/workstation/` | 16 (13 specs + types + persistence + lib) | 13 specs, 10 shared types | `poodle-gpui-composites`, `poodle-gpui-primitives`, `poodle-gpui-tokens`, `serde`, `serde_json` |

### Renderer-Agnostic vs GPUI-Biased Classification

**Fully renderer-agnostic (no changes needed):**

- All spec structs (`ButtonSpec`, `DialogSpec`, `DataTableSpec`, etc.) — these
  are pure data structures with builder methods and token accessor methods.
  They carry no GPUI imports and no rendering logic.
- All shared enum/struct types (`ButtonVariant`, `ControlSize`, `DialogKind`,
  `Direction`, `Alignment`, etc.) — pure value types with no renderer coupling.
- Token accessor methods (`resolved_fill_token()`, `border_token()`, etc.) —
  return `&'static str` references to generated token constants. No GPUI types.
- Behavioral query methods (`activation_allowed()`, `requires_aria_label()`,
  `current_state()`, etc.) — pure logic over spec fields.
- Persistence helpers (`serialize_workspace_layout_snapshot`,
  `parse_workspace_layout_snapshot`) — JSON serialization via `serde`.
- Test suites — all tests exercise spec construction and token resolution,
  no rendering.

**GPUI-biased by naming only (rename required, logic unchanged):**

- Crate names: `poodle-gpui-tokens`, `poodle-gpui-primitives`, `poodle-gpui-composites`,
  `poodle-gpui-workstation`
- Import paths: `poodle_gpui_tokens::semantic`, `poodle_gpui_primitives::ButtonSpec`, etc.
- `CURRENT_GENERATION` constants reference GPUI-specific milestone IDs
- Cargo.toml `name` and `description` fields mention GPUI

**GPUI-biased by token format (typed alternatives needed):**

- Token values are `&'static str` (e.g., `"#2d86f3"`, `"0.375rem"`, `"2.25rem"`)
- Jetstream requires `[f32; 4]` for colors, `f32` for pixel values
- The token build pipeline (`build-tokens.ts`) only emits string constants
  for Rust; it already has the resolved numeric values internally but
  discards them during Rust codegen

**No GPUI rendering code exists anywhere:**

The spec crates contain zero rendering logic. They are already pure contract
definitions. The "extraction" is primarily a rename + typed token extension,
not a logic separation.

### Jetstream `game_ui` API Surface (Second Consumer)

Based on the Jetstream rendering constraint profile:

| Capability | Jetstream `game_ui` | Poodle Contract Mapping |
|------------|---------------------|---------------------|
| **Layout** | Flexbox: `Direction`, `Sizing`, `Align`, `Justify`, `Edges` | Maps to `LayoutDirection`, `LayoutSizing`, `LayoutAlignment`, `LayoutSpacing` |
| **Colors** | `Vec4` (`[f32; 4]`) | Requires typed `ColorValue` from token system |
| **Dimensions** | `f32` pixels | Requires typed `SpaceValue` from token system |
| **Corners** | `f32` radius per corner | Maps to `RadiusValue` |
| **Borders** | Solid color + `f32` width | Maps to `BorderValue` |
| **Shadows** | Single box shadow (offset, blur, color) | Maps to `ShadowValue` (subset) |
| **Text** | Glyph atlas, single-style runs, LTR Latin | Maps to `TypographyValue` (font enum, size f32, weight u16) |
| **Opacity** | `f32` 0.0–1.0 | Direct map |
| **Widgets** | 8 variants: Container, Text, Image, Button, Checkbox, Slider, TextInput, ScrollView | Poodle specs decompose into these primitives at render time |
| **Events** | `UiEvent` enum: Clicked, ValueChanged, FocusGained, FocusLost, TextChanged, ScrollChanged, DragStarted, DragMoved, DragEnded, KeyPressed | Maps to semantic event abstraction |
| **Focus** | `FocusState` with tab order, directional navigation | Maps to focus model in specs |
| **Screens** | `ScreenStack` with modal/transparent flags | Maps to dialog/drawer overlay model |
| **Input** | Keyboard, mouse, gamepad via `InputSystem` action bindings | Adapter maps semantic events to `UiEvent` |

**Jetstream limitations (intentional deltas):**

- No CSS Grid — all layout is flexbox. Grid specs emit flexbox fallback.
- No rich text / complex scripts — single-style text runs only
- No transforms (rotate, scale, skew)
- No gradients — solid colors only
- Single box shadow per element
- Vertical scroll only (no horizontal scroll containers)
- No image filters or blend modes
- Gamepad input adds `NavigateUp/Down/Left/Right` and `Activate` events

## Extraction Plan

### New Crate Topology

```text
packages/
  contracts/
    tokens/          → poodle-tokens        (was poodle-gpui-tokens)
    primitives/      → poodle-primitives    (was poodle-gpui-primitives)
    composites/      → poodle-composites    (was poodle-gpui-composites)
    workstation/     → poodle-workstation   (was poodle-gpui-workstation)
    layout/          → poodle-layout        (NEW: layout intent types)
    events/          → poodle-events        (NEW: semantic event types)
    style/           → poodle-style         (NEW: style descriptor IR)
    adapter/         → poodle-adapter       (NEW: renderer adapter traits)
  gpui/
    (retains validation artifacts, baselines, proof JSONs)
    (optional: thin re-export crates for backward compat during transition)
```

### Naming Conventions

| Old | New | Rust Crate Name |
|-----|-----|-----------------|
| `poodle-gpui-tokens` | `poodle-tokens` | `poodle_tokens` |
| `poodle-gpui-primitives` | `poodle-primitives` | `poodle_primitives` |
| `poodle-gpui-composites` | `poodle-composites` | `poodle_composites` |
| `poodle-gpui-workstation` | `poodle-workstation` | `poodle_workstation` |
| (new) | `poodle-layout` | `poodle_layout` |
| (new) | `poodle-events` | `poodle_events` |
| (new) | `poodle-style` | `poodle_style` |
| (new) | `poodle-adapter` | `poodle_adapter` |

### Migration Path

1. **g06.002** — Create `packages/contracts/` directory. Move and rename crates.
   Update all `Cargo.toml` names, descriptions, and dependency paths. Update
   `use` statements in all spec modules and tests. Leave `packages/gpui/` for
   validation artifacts. No logic changes — pure rename.

2. **g06.003** — Extend `build-tokens.ts` to emit a `typed` submodule in the
   Rust artifacts with `ColorValue`, `SpaceValue`, `RadiusValue`, `BorderValue`,
   `ShadowValue`, `TypographyValue`. String constants remain for backward compat.

3. **g06.004** — Create `poodle-layout` crate with renderer-agnostic layout intent
   types. These map to both GPUI's styling API and Jetstream's `UiStyle`.

4. **g06.005** — Create `poodle-events` crate with semantic event types that map
   to both GPUI's event subscriptions and Jetstream's `UiEvent` enum.

5. **g06.006** — Create `poodle-style` crate with `StyleDescriptor` that captures
   resolved visual properties using typed tokens. Spec structs gain
   `resolve_style()` methods.

6. **g06.007** — Create `poodle-adapter` crate defining `RenderComponent<Target>`,
   `ThemeProvider`, and `EventSink` traits.

7. **g06.008–012** — Add 53 new spec structs to `poodle-primitives` and
   `poodle-composites`, using typed tokens and layout intent from the start.

### Dependency Graph (Exit State)

```text
poodle-tokens
  ├── poodle-layout (uses typed token values for spacing)
  ├── poodle-events (no token dependency, pure semantic types)
  ├── poodle-primitives (spec structs, uses tokens + layout)
  │     └── poodle-composites (uses primitives + tokens)
  │           └── poodle-workstation (uses composites + primitives + tokens)
  ├── poodle-style (uses tokens, layout; resolves spec → visual descriptor)
  └── poodle-adapter (uses style, events; defines renderer traits)
```

### Token Build Pipeline Changes (g06.003 Preview)

The existing `build-tokens.ts` already resolves all token values to their
final form internally. The Rust codegen currently discards numeric information
and emits only string constants. The typed extension will:

- Parse color strings (`#rrggbb`, `rgba(...)`) → `[f32; 4]`
- Parse dimension strings (`Xrem`, `Xpx`) → `f32` pixels (base size 16)
- Parse shadow strings → `ShadowValue { offset_x, offset_y, blur, color }`
- Parse border-width strings → `f32`
- Emit a `typed` submodule alongside existing string modules

### Spec Coverage Gap (53 Components)

The 53 Svelte components without Rust specs will be added in g06.008–012:

**008 — Structural and input extensions (10):**
Banner, CallOut, EditableLabel, Eyebrow, HoverCard, NumberEntry, PinInput,
RangeSlider, Toolbar, TriStateSwitch

**009 — Selection, feedback, and temporal (8):**
Meter, Pill, Rating, Skeleton, TimeAgo, DurationInput, TimeZoneSelect,
ZonedDateTimePicker

**010 — Informational, code, and color (4):**
Code, ColorPicker, FileUpload, SplitButton

**011 — Editing, media, and operational composites (12):**
AudioPlayer, VideoPlayer, MediaPicker, MarkdownEditor, BlockEditor,
EmbedInput, EmbedPreview, EmbedShell, LogList, PageLoading, StateTile,
ToastStack

**012 — Navigation, list interaction, and inline editing (13):**
AutonomousList, Breadcrumbs, CardRadioGroup, ConfirmAction, DetailSection,
InlineEditableField, ListCard, NavCard, NavCardGrid, OrderBy, PageHeader,
ReorderableList, SlugField

## Verification

- [x] Every module in all four existing crates classified
- [x] Zero GPUI rendering imports found — crates are already renderer-agnostic
- [x] Jetstream `game_ui` API surface documented with mapping table
- [x] Intentional Jetstream deltas identified (no grid, no rich text, etc.)
- [x] New crate topology defined with naming conventions
- [x] Migration path sequenced across g06.002–007
- [x] Spec coverage gap enumerated (53 components across 5 batches)
- [x] Token build pipeline extension strategy documented

## Deliverables

| Artifact | Location |
|----------|----------|
| Architecture audit | This document |
| Extraction plan | This document (sections above) |
| Jetstream API mapping | This document (table above) |
