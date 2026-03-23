# g06 Shared Multi-Renderer Contract Layer

Status: completed
Updated: 2026-03-14

## Context

`g05` established the GPUI foundation: spec crates with renderer-agnostic types
(enums, builder-pattern structs, token accessor methods), cross-runtime parity
evidence, and demo-app alignment. However, the spec crates live under
`packages/gpui/` and the token system emits string constants rather than typed
values. The crate naming and token format are GPUI-biased even though the spec
types themselves carry no GPUI dependency.

`g06` restructures the shared Rust layer to be explicitly multi-renderer. The
existing spec types become the foundation for rendering adapters targeting both
GPUI (Zed's native UI framework) and Jetstream (a wgpu-based game engine with
a retained-mode UI system). A future Jetstream adapter (Poodle g08) requires that
contracts, tokens, and layout intent are expressed in renderer-agnostic terms
that map cleanly to Jetstream's `UiTree`, `UiStyle`, `Vec4` colors, and `f32`
pixel values — not just GPUI's styling API and CSS-like string tokens.

This generation also expands spec coverage from 71 components to the full 124
Svelte surface established in `g04`.

## Starting State

- 42 primitive specs, 13 composite specs, 12 workstation specs (all Rust,
  builder pattern, token accessor methods)
- spec crates live under `packages/gpui/` with `poodle-gpui-*` naming
- token artifacts are string constants (`&'static str` hex colors, rem values)
- 124 Svelte components total (71 primitives, 41 composites, 12 workstation)
- 53 Svelte components have no Rust spec equivalent
- Jetstream `game_ui` system provides: `UiTree` (retained mode), 8 widget
  variants, flexbox layout (`Direction`, `Sizing`, `Align`, `Justify`, `Edges`),
  `FocusState`, `Theme` with `Vec4` colors and `f32` pixel values, `UiEvent`
  pipeline, `ScreenStack`, GPU render pass for colored quads with rounded corners

## Exit State

- shared contract crates are renamed and restructured as `poodle-contracts-*`
  (no renderer name in the crate path)
- token system supports typed output (parsed `Vec4`-compatible colors, `f32`
  pixel values) alongside existing string constants
- layout intent types are renderer-agnostic and map cleanly to both GPUI's
  styling API and Jetstream's `UiStyle`
- event model abstraction defines semantic events that both adapters can map to
- style descriptor intermediate representation captures resolved visual
  properties without committing to a renderer
- spec coverage reaches the full 124-component Svelte surface
- renderer adapter trait is defined and documented
- Jetstream rendering constraint document is published

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Multi-renderer architecture audit and extraction plan | g05.014 | Foundation | Completed |
| 002 | Contract crate restructuring and rename | 001 | Foundation | Completed |
| 003 | Typed token resolution system | 002 | Foundation | Completed |
| 004 | Layout intent abstraction | 003 | Foundation | Completed |
| 005 | Event model and interaction abstraction | 004 | Foundation | Completed |
| 006 | Style descriptor intermediate representation | 004, 005 | Foundation | Completed |
| 007 | Renderer adapter trait definition | 006 | Foundation | Completed |
| 008 | Primitive specs — structural and input extensions | 002 | Expansion | Completed |
| 009 | Primitive specs — selection, feedback, and temporal | 008 | Expansion | Completed |
| 010 | Primitive specs — informational, code, and color | 009 | Expansion | Completed |
| 011 | Composite specs — editing, media, and operational | 010 | Expansion | Completed |
| 012 | Composite specs — navigation, list interaction, and inline editing | 011 | Expansion | Completed |
| 013 | Jetstream rendering constraint document | 007 | Alignment | Completed |
| 014 | Multi-renderer parity validation tooling | 007, 012 | Hardening | Completed |
| 015 | Generation closeout and g07 cutover plan | 013, 014 | Closure | Completed |

## Dependency Shape

```text
g05.014 Demo Alignment Complete
  -> 001 Architecture Audit
      -> 002 Crate Restructuring
          -> 003 Typed Tokens
              -> 004 Layout Intent
                  -> 005 Event Model
                      -> 006 Style Descriptors
                          -> 007 Adapter Traits
                              -> 013 Jetstream Constraints
                              -> 014 Parity Tooling -> 015 Closeout
          -> 008 Primitive Specs: Structural + Input
              -> 009 Primitive Specs: Selection + Feedback
                  -> 010 Primitive Specs: Informational + Code
                      -> 011 Composite Specs: Editing + Media
                          -> 012 Composite Specs: Navigation + Lists -> 014
```

## Execution Lanes

### Lane A: Contract Infrastructure

`001 -> 002 -> 003 -> 004 -> 005 -> 006 -> 007 -> 013`

### Lane B: Spec Expansion (parallelizable with Lane A after 002)

`008 -> 009 -> 010 -> 011 -> 012`

### Lane C: Hardening and Closeout

`014 -> 015`

## Milestone Details

### 001 — Multi-Renderer Architecture Audit and Extraction Plan

Audit the existing `poodle-gpui-*` crates and identify what is genuinely
renderer-agnostic vs what carries GPUI assumptions. Produce an extraction plan
that defines the new crate topology, naming conventions, and migration path.
Document Jetstream's `game_ui` API surface as a second consumer to validate
that the contracts can serve both targets.

### 002 — Contract Crate Restructuring and Rename

Rename and restructure the shared Rust crates:
- `poodle-gpui-tokens` → `poodle-tokens` (token constants and typed resolution)
- `poodle-gpui-primitives` → `poodle-primitives` (spec structs and shared enums)
- `poodle-gpui-composites` → `poodle-composites` (composite spec structs)
- `poodle-gpui-workstation` → `poodle-workstation` (workstation spec structs)

Preserve the existing `poodle-gpui-*` crate names as thin re-export wrappers
during transition if needed for existing consumers. Update all internal imports.

### 003 — Typed Token Resolution System

Extend the token build pipeline (`build-tokens.ts`) to emit typed Rust
artifacts alongside string constants:
- `ColorValue` — parsed RGBA as `[f32; 4]` (0.0–1.0 range)
- `SpaceValue` — parsed rem/px as `f32` pixels (given configurable base size)
- `RadiusValue`, `BorderValue`, `ShadowValue` — parsed into numeric types
- `TypographyValue` — font family enum, size as `f32`, weight as `u16`

Keep string constants for backward compatibility. Add a `typed` submodule with
the numeric alternatives. Both GPUI (which can consume either) and Jetstream
(which needs numeric values) should be able to use the appropriate form.

### 004 — Layout Intent Abstraction

Define renderer-agnostic layout intent types that both GPUI and Jetstream can
map to their native layout systems:
- `LayoutDirection` — row, column
- `LayoutSizing` — fixed, grow, fit, with min/max constraints
- `LayoutSpacing` — gap, padding, margin (as token references or explicit values)
- `LayoutAlignment` — main-axis and cross-axis alignment
- `LayoutOverflow` — visible, hidden, scroll

These should be separate from GPUI's styling API and Jetstream's `UiStyle`.
Each adapter maps layout intent to its native types. Validate that the mapping
is lossless for both targets.

### 005 — Event Model and Interaction Abstraction

Define renderer-agnostic semantic events:
- `Activated` — button press, enter key, etc.
- `ValueChanged { value }` — slider, input, checkbox, select
- `FocusChanged { gained: bool }` — focus enter/leave
- `OpenChanged { open: bool }` — dialog, popover, drawer, accordion
- `SelectionChanged { value }` — select, radio, tabs
- `Submitted` / `Cancelled` — form actions, dialogs
- `Hovered { entered: bool }` — pointer enter/leave

Map to Jetstream's `UiEvent` enum and GPUI's event subscription model. Define
which events each component category emits.

### 006 — Style Descriptor Intermediate Representation

Define a resolved style descriptor that captures the visual properties of a
component instance after token resolution but before renderer-specific
translation:
- Background, border, text colors as `[f32; 4]`
- Corner radius, border width, shadow params as `f32`
- Font properties (family, size, weight)
- Opacity, visibility, cursor hint
- Spacing values as `f32` pixels

Spec structs gain a `resolve_style(theme: &Theme) -> StyleDescriptor` method
that both adapters can use as input. This eliminates duplicate token resolution
logic in each adapter.

### 007 — Renderer Adapter Trait Definition

Define the trait(s) that rendering adapters implement:
- `trait RenderComponent<Target>` — maps a spec + resolved style to the
  renderer's native node/element type
- `trait ThemeProvider` — resolves token references to concrete values
- `trait EventSink` — receives semantic events from the adapter

Document the adapter contract with examples for both GPUI and Jetstream.
Publish as part of the contract crate so adapter authors have a clear interface.

### 008–012 — Spec Expansion Batches

Add Rust spec structs for the 53 Svelte components that lack them:

**008 — Structural and input extensions (est. 10 specs):**
Banner, CallOut, EditableLabel, Eyebrow, HoverCard, NumberEntry, PinInput,
RangeSlider, Toolbar, TriStateSwitch

**009 — Selection, feedback, and temporal (est. 8 specs):**
Meter, Pill, Rating, Skeleton, TimeAgo, DurationInput, TimeZoneSelect,
ZonedDateTimePicker

**010 — Informational, code, and color (est. 4 specs):**
Code, ColorPicker, FileUpload, RangeSlider (if not covered in 008)

**011 — Editing, media, and operational composites (est. 12 specs):**
AudioPlayer, VideoPlayer, MediaPicker, MarkdownEditor, BlockEditor,
EmbedInput, EmbedPreview, EmbedShell, LogList, PageLoading, StateTile,
ToastStack

**012 — Navigation, list interaction, and inline editing (est. 12 specs):**
AutonomousList, Breadcrumbs, CardRadioGroup, ConfirmAction, DetailSection,
InlineEditableField, ListCard, NavCard, NavCardGrid, OrderBy, PageHeader,
ReorderableList, SlugField

Each batch follows the existing builder-pattern + token accessor convention.
New specs should use the typed token system (003) and layout intent types (004)
from the start.

### 013 — Jetstream Rendering Constraint Document

Publish a document in the Poodle repo that describes Jetstream's UI system
capabilities and constraints as a rendering target:
- Available widget primitives and their properties
- Layout model (flexbox-like, pixel units, no grid)
- Styling capabilities (solid colors, rounded corners, borders, shadows, opacity)
- Text rendering (glyph atlas, single-style runs, LTR Latin)
- Input model (`UiEvent` enum, `InputSystem` action bindings)
- Focus model (`FocusState`, tab order, directional navigation)
- Screen management (`ScreenStack`, modal/transparent flags)
- Known limitations (no rich text, no complex scripts, no CSS Grid, no
  transforms, no gradients)

This document is the contract between Poodle g08 and Jetstream g04.016.

### 014 — Multi-Renderer Parity Validation Tooling

Extend the existing parity tooling to validate that every spec struct can be
rendered by both adapter targets. At minimum:
- compile-time check that every spec has a `resolve_style()` method
- inventory check that every Svelte component has a corresponding spec
- trait-check that adapter implementations exist for all specs (deferred to
  g07/g08 for actual adapter code, but the tooling framework lands here)

### 015 — Generation Closeout and g07 Cutover Plan

Verify all milestones are complete. Document any deferred items. Confirm that
g07 (GPUI build-out) and g08 (Jetstream build-out) can proceed independently
against the shared contract layer.

## Next Task

Open g06.001 and begin the multi-renderer architecture audit.
