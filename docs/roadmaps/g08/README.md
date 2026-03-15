# g08 Jetstream Rendering Build-Out

Status: completed
Updated: 2026-03-14

## Context

`g06` established the shared multi-renderer contract layer: renamed crates,
typed token resolution, layout intent abstraction, event model, style
descriptors, and adapter traits. `g07` built the GPUI rendering adapter
against those contracts. `g08` builds the Jetstream rendering adapter.

Jetstream is a wgpu-based game engine with a retained-mode UI system
(`game_ui.rs`): `UiTree` with generational indices, flexbox layout, focus
navigation, input routing via `InputSystem`, screen management via
`ScreenStack`, and a GPU instanced quad render pass. Jetstream g04.016 (UI
Rendering Infrastructure) adds text rendering, image display, clipping,
scrolling, extended styling, and text editing support — the rendering
primitives needed to display Pug components.

This generation implements the Jetstream rendering adapter: the code that maps
Pug spec structs + resolved styles to Jetstream's `UiTree` nodes with
correct `UiStyle`, `Widget` variants, and `UiEvent` routing.

`g08` can proceed in parallel with `g07` after `g06` is complete. However, it
has a hard dependency on Jetstream g04.016 for the rendering infrastructure
(text, images, clipping, scrolling).

## Starting State

- shared contract crates provide renderer-agnostic specs for all 124 components
- typed token resolution emits `[f32; 4]` colors, `f32` pixel values (directly
  compatible with Jetstream's `Vec4` and `f32` types)
- renderer adapter trait is defined
- Jetstream g04.016 has landed: text rendering, image display, clipping,
  scrolling, extended styling, text editing, token-compatible theming
- Jetstream `game_ui` provides: `UiTree`, `UiNode`, `UiNodeId`, `Widget`
  (Panel, Label, Button, Slider, ProgressBar, Image, List, TextInput),
  `UiStyle` (flexbox layout, colors, borders, corner radius, opacity),
  `FocusState`, `Theme`, `UiDrawCommand`, `UiEvent`, `ScreenStack`

## Exit State

- Jetstream rendering adapter crate (`pug-jetstream`) implements the adapter
  trait for all component categories that are appropriate for a game engine UI
- Pug components render correctly in Jetstream's runtime UI pass
- token-based theming produces visually consistent output across GPUI and
  Jetstream for tier-2 (visual parity) requirements
- integration demo scene in Jetstream demonstrates Pug components in a game
  context (menus, HUD, settings, dialogs)
- intentional deltas between Jetstream and GPUI/Svelte are documented
  (capabilities that don't apply to a game engine context)
- cross-runtime parity evidence covers the Jetstream target

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Jetstream adapter crate setup and token bridge | g06.015, JS-g04.016 | Foundation | Completed |
| 002 | Jetstream theme construction from Pug tokens | 001 | Foundation | Completed |
| 003 | Structural primitives — Box, Stack, Grid, Surface, Separator, ScrollShell | 002 | Core build | Completed |
| 004 | Action primitives — Button, IconButton, FormActions, Toolbar | 003 | Core build | Completed |
| 005 | Input primitives — TextInput, TextArea, SearchField, Field, NumberEntry | 004 | Core build | Completed |
| 006 | Selection primitives — Checkbox, RadioGroup, Switch, Select, Slider, SegmentedControl | 005 | Core build | Completed |
| 007 | Feedback and display primitives — Progress, Badge, StatusIndicator, Skeleton, Meter | 006 | Core build | Completed |
| 008 | Overlay primitives — Dialog, Drawer, Popover, Menu, Tooltip, Tabs, Accordion | 007 | Core build | Completed |
| 009 | Form and validation composites | 008 | Depth | Completed |
| 010 | Data and browse composites | 009 | Depth | Completed |
| 011 | Jetstream-appropriate component subset review and delta register | 010 | Alignment | Completed |
| 012 | Integration demo scene in Jetstream | 011 | Alignment | Completed |
| 013 | Cross-runtime parity report — Jetstream target | 012 | Hardening | Completed |
| 014 | Generation closeout | 013 | Closure | Completed |

## Dependency Shape

```text
g06.015 Shared Contracts Complete
  \
   -> 001 Adapter Setup (also depends on Jetstream g04.016)
       -> 002 Theme Bridge
           -> 003 Structural Primitives
               -> 004 Action Primitives
                   -> 005 Input Primitives
                       -> 006 Selection Primitives
                           -> 007 Feedback Primitives
                               -> 008 Overlay Primitives
                                   -> 009 Form Composites
                                       -> 010 Data Composites
                                           -> 011 Delta Register
                                               -> 012 Demo Scene
                                                   -> 013 Parity Report
                                                       -> 014 Closeout
```

## Execution Lanes

### Lane A: Foundation and Adapter Core

`001 -> 002 -> 003 -> 004 -> 005 -> 006 -> 007 -> 008`

### Lane B: Composite Depth

`009 -> 010`

### Lane C: Parity and Integration

`011 -> 012 -> 013 -> 014`

## Milestone Details

### 001 — Jetstream Adapter Crate Setup and Token Bridge

Create `pug-jetstream` crate that implements the renderer adapter trait from
g06.007. Set up the type bridge between Pug's typed tokens (`[f32; 4]` colors,
`f32` pixels) and Jetstream's `Vec4`, `f32`, `Edges` types. Establish the
dependency on `jetstream-runtime` for `UiTree`, `UiStyle`, `Widget`, and
`jetstream-renderer` for `UiPass`, `UiQuadInstance`.

Key design decision: the adapter creates and manages `UiTree` nodes. A Pug
component maps to one or more `UiNode` entries with appropriate `Widget`
variants and `UiStyle` values. The adapter owns the mapping between Pug spec
identity and `UiNodeId` handles.

### 002 — Jetstream Theme Construction from Pug Tokens

Implement `PugTheme` → Jetstream `Theme` conversion. Map Pug's semantic token
categories to Jetstream's theme fields:
- `COLOR_BACKGROUND_*` → `panel_bg`, `input_bg`
- `COLOR_TEXT_*` → `text_color`
- `COLOR_ACCENT_*` → `button_bg`, `focus_color`
- `COLOR_STATUS_*` → mapped to per-tone color values
- `SPACE_*` → converted from rem to pixels using base font size
- `RADIUS_*` → `corner_radius` values
- `TYPOGRAPHY_*` → font selection, size, weight

### 003 — Structural Primitives

Map Pug structural specs to Jetstream `UiTree` nodes:
- `BoxSpec` → `Widget::Panel` with `UiStyle` (padding, dimensions, overflow)
- `StackSpec` → `Widget::Panel` with `direction`, `gap`, `align`, `justify`
- `GridSpec` → multiple `Widget::Panel` nodes with row/column layout
  (Jetstream has no grid layout, so grid must be emulated with nested
  row/column panels)
- `SurfaceSpec` → `Widget::Panel` with tone-resolved background, border
- `SeparatorSpec` → `Widget::Panel` with fixed height/width 1px, border color
- `ScrollShellSpec` → `Widget::List` with scroll offset and clipping

### 004 — Action Primitives

- `ButtonSpec` → `Widget::Button` with variant-resolved colors, size-resolved
  height, icon images, disabled opacity
- `IconButtonSpec` → `Widget::Button` with icon-only layout
- `FormActionsSpec` → `Widget::Panel` with alignment-based child layout
- `ToolbarSpec` → `Widget::Panel` with horizontal layout and action children

### 005 — Input Primitives

- `TextInputSpec` → `Widget::TextInput` with placeholder, validation border
  color, prefix/suffix icon images
- `TextAreaSpec` → `Widget::TextInput` (extended for multi-line when Jetstream
  supports it) or multiple `Widget::TextInput` rows
- `SearchFieldSpec` → `Widget::TextInput` with clear button child
- `FieldSpec` → wrapper `Widget::Panel` containing label (`Widget::Label`),
  input child, description/error (`Widget::Label`)
- `NumberEntrySpec` → `Widget::TextInput` with increment/decrement button
  children

### 006 — Selection Primitives

- `CheckboxSpec` → `Widget::Button` with check indicator (image or styled panel)
- `RadioGroupSpec` → `Widget::Panel` containing radio item buttons
- `SwitchSpec` → `Widget::Button` with track/thumb styled panels
- `SelectSpec` → `Widget::Button` (trigger) + overlay panel with option list
- `SliderSpec` → `Widget::Slider` with token-resolved track/fill colors
- `SegmentedControlSpec` → `Widget::Panel` with segment button children

### 007 — Feedback and Display Primitives

- `ProgressSpec` → `Widget::ProgressBar` with token-resolved colors
- `BadgeSpec` → `Widget::Label` with variant-resolved background, small padding
- `StatusIndicatorSpec` → `Widget::Panel` with small fixed size, tone color
- `SkeletonSpec` → `Widget::Panel` with placeholder background
- `MeterSpec` → `Widget::ProgressBar` variant with semantic thresholds

### 008 — Overlay Primitives

- `DialogSpec` → new `UiScreen` pushed to `ScreenStack` with `modal: true`,
  containing a centered panel with title, content, and action buttons
- `DrawerSpec` → new `UiScreen` with edge-aligned panel
- `PopoverSpec` → new `UiScreen` with `transparent: true`, positioned relative
  to anchor node
- `MenuSpec` → overlay screen with vertical list of menu items
- `TooltipSpec` → overlay screen with small positioned label
- `TabsSpec` → `Widget::Panel` with tab strip (horizontal button row) and
  content panel that swaps children
- `AccordionSpec` → `Widget::Panel` with collapsible item panels

### 009 — Form and Validation Composites

- `FormShellSpec` → `Widget::Panel` with section children, validation tracking,
  submit/cancel actions
- `ValidationSummarySpec` → `Widget::Panel` with error list labels
- `RemediationBannerSpec` → `Widget::Panel` with status tone background,
  message label, action buttons
- `ConfirmActionSpec` → dialog screen with confirmation message and actions

### 010 — Data and Browse Composites

- `DataTableSpec` → scrollable panel with header row and data rows (each row
  is a horizontal panel with cell labels)
- `ListShellSpec` → `Widget::List` with state-dependent content (empty state,
  loading, error, items)
- `FilterToolbarSpec` → horizontal panel with search input and filter chips
- `DetailShellSpec` → panel with header and content sections
- `SelectionSummarySpec` → panel with selected item labels and clear button

### 011 — Jetstream-Appropriate Component Subset Review and Delta Register

Not all 124 Pug components make sense in a game engine context. Review the
full surface and document:
- **Full parity** — components that map cleanly and are useful in game UI
  (buttons, sliders, dialogs, menus, HUD elements, data displays)
- **Partial parity** — components with reduced functionality (e.g., DataTable
  without CSV export, text inputs without IME)
- **Intentional skip** — components that don't apply to game UI context (e.g.,
  MarkdownEditor, BlockEditor, ZonedDateTimePicker, breadcrumbs may not make
  sense)
- **Jetstream-specific additions** — game UI patterns that don't exist in
  Pug's web/desktop-focused surface (e.g., gamepad-friendly navigation,
  full-screen transition screens)

### 012 — Integration Demo Scene in Jetstream

Build a demo scene in Jetstream that exercises Pug components in a game
context:
- Main menu (title, start/options/quit buttons) using Pug button specs
- Settings screen (sliders, toggles, select dropdowns) using Pug input specs
- HUD overlay (health bar, status indicators, score) using Pug feedback specs
- Pause dialog (modal overlay with resume/quit) using Pug dialog spec
- Verify theming consistency (Pug tokens produce correct visual output)

### 013 — Cross-Runtime Parity Report — Jetstream Target

Produce parity evidence covering the Jetstream rendering target:
- visual comparison for tier-2 parity components
- behavioral verification for tier-1 parity components
- documented deltas for components with reduced or skipped support
- token coverage verification (all referenced tokens resolve correctly)

### 014 — Generation Closeout

Verify all milestones complete. Document the full three-renderer state
(Svelte, GPUI, Jetstream). Identify any follow-up work for future generations.

## Cross-Project Dependencies

| Dependency | Direction | Description |
|-----------|-----------|-------------|
| Pug g06 | Pug → Pug g08 | Shared contracts must be complete before adapter work |
| Jetstream g04.016 | Jetstream → Pug g08 | UI rendering infrastructure must be complete before Pug components can render |
| Pug g08.013 (constraint doc) | Pug g06 → Pug g08 | Rendering constraints inform adapter design |
| Pug g07 | Independent | GPUI build-out can run in parallel with g08 |

## Known Constraints

From the Jetstream rendering constraint document (g06.013):
- **Layout**: flexbox-like only (no CSS Grid). Grid specs must be emulated with
  nested row/column panels.
- **Text**: single-style runs, LTR, Latin/common scripts. No rich text or
  complex shaping.
- **Colors**: solid colors only. No gradients.
- **Images**: GPU texture handles. No SVG rendering.
- **Shadows**: single box shadow per node. No stacked or inset shadows.
- **Transforms**: none. No rotation, scale, or skew.
- **Scrolling**: vertical scroll with clipping. No momentum or snap.
- **Input**: keyboard + mouse + gamepad. No touch/multi-touch. No IME.

## Next Task

All milestones complete. g08 (Jetstream Rendering Build-Out) is closed.
Full three-runtime parity achieved: 117 renderable components (63 primitives +
41 composites + 13 workstation) across Svelte, GPUI, and Jetstream.
142 tests passing across the pug-jetstream crate.
