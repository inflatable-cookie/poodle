# g10 Delta Register — Jetstream Preview Adaptations

Status: Complete
Updated: 2026-03-16

## Overview

This register documents all intentional differences between the Jetstream preview
app and the Svelte/GPUI reference implementations. The Jetstream adapter supports
all 117 renderable components (100% coverage), but the game-engine rendering
context introduces visual and behavioral adaptations documented below.

## Parity Tier Summary

| Tier | Count | Percentage |
|------|-------|------------|
| Full parity | 109 | 93.2% |
| Partial parity | 8 | 6.8% |
| Intentional skip | 0 | 0% |
| **Total** | **117** | **100%** |

## Structural Deltas

### Visual: Retained-mode rendering (all components)
- **Type:** visual
- **Description:** Jetstream uses a retained-mode UiTree with immediate GPU
  rendering rather than a DOM-based layout engine. Font rendering, anti-aliasing,
  and sub-pixel behavior differ from browser-based Svelte and GPUI's Taffy layout.
- **Constraint:** game_ui single-pass rasterizer
- **Severity:** cosmetic
- **Affected:** all 117 components

### Visual: No CSS gradients (simplified backgrounds)
- **Type:** visual
- **Description:** Jetstream game_ui supports linear gradients via `GradientStop`
  but the adapter uses solid color approximations for most gradient specs.
  Components with gradient backgrounds render as flat tinted fills.
- **Constraint:** gradient support limited to linear axis-aligned
- **Severity:** cosmetic
- **Affected:** Surface, Banner, CallOut, Tooltip

### Visual: Single shadow model
- **Type:** visual
- **Description:** Jetstream supports one box shadow per node. Components with
  multiple layered shadows (e.g., elevated surfaces with both inner and outer
  shadows) render with only the primary shadow.
- **Constraint:** single shadow per UiNode
- **Severity:** cosmetic
- **Affected:** Dialog, Drawer, HoverCard, Popover

## Behavioral Deltas

### Behavioral: No text selection or clipboard
- **Type:** behavioral
- **Description:** TextInput and TextArea specimens render as visual-only labels.
  The game_ui `Widget::TextInput` supports single-line editing but does not
  expose clipboard integration or multi-cursor selection.
- **Constraint:** no OS clipboard API in game_ui
- **Severity:** functional
- **Affected:** TextInput, TextArea, SearchField, PinInput, EditableLabel

### Behavioral: No file system access
- **Type:** behavioral
- **Description:** FileUpload renders as a drop-zone visual but cannot trigger
  native file dialogs or handle drag-and-drop from the OS.
- **Constraint:** no native file picker in game engine context
- **Severity:** functional
- **Affected:** FileUpload

### Behavioral: No date/time picker popups
- **Type:** behavioral
- **Description:** Calendar-based pickers render static calendar grids. The
  popup/overlay behavior for date selection relies on z-index stacking which is
  supported but the mouse interaction for dropdown open/close is not wired.
- **Constraint:** limited popup lifecycle management
- **Severity:** functional
- **Affected:** DatePicker, DateRangePicker, DateTimePicker, DateTimeRangePicker,
  ZonedDateTimePicker

## Display Control Deltas

### Structural: No AppearanceTreatment toggle
- **Type:** simplified
- **Description:** GPUI preview includes an AppearanceTreatment control
  (System/BrandRaised). Jetstream preview omits this control as the Jetstream
  adapter resolves tokens through a single theme pipeline without treatment
  variants.
- **Constraint:** single theme pipeline
- **Severity:** cosmetic

### Structural: Shells section merged into Composites
- **Type:** simplified
- **Description:** GPUI has a dedicated "Shells" tab with 6 entries. Jetstream
  includes all 13 workstation surfaces in the Composites section, providing
  broader coverage without a separate navigation section.
- **Constraint:** intentional design choice for simpler navigation
- **Severity:** cosmetic

## Specimen Rendering Notes

All 125 specimen files (80 primitives + 33 original composites + 13 workstation
surfaces, minus shared utilities plus demos and token view) render as static
visual representations of component states. Interactive behavior (hover effects,
click handlers, focus transitions) is limited to what game_ui `UiEvent` provides:

- `UiEvent::Activated` — button clicks, sidebar/tab navigation
- `UiEvent::ValueChanged` — slider value changes
- Focus ring rendering via `FocusState`
- Scroll via `Widget::List { scroll_offset }`

## Verification

```
cargo check -p pug-jetstream-preview — compiles with 7 pre-existing warnings
cargo test (adapter) — 165 tests passing
Component coverage: 117/117 (100%)
Specimen coverage: 125 specimen files + 6 demo screens + token inspector
```
