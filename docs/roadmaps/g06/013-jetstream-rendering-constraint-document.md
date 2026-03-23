# g06.013 — Jetstream Rendering Constraint Document

Status: Completed
Updated: 2026-03-14

## Objective

Publish a document defining Jetstream's UI system capabilities and constraints
as a Flint rendering target. This is the contract between Flint g08 (Jetstream
build-out) and Jetstream g04.016 (UI rendering infrastructure).

## Jetstream `game_ui` System Overview

Jetstream is a wgpu-based game engine with a retained-mode UI system (`game_ui`)
designed for in-game interfaces — settings menus, inventory screens, HUDs, and
tool palettes. The UI runs inside the game's render loop and shares the GPU
surface with 3D content.

## Widget Primitives

Jetstream provides 8 built-in widget variants:

| Widget | Purpose | Flint Mapping |
|--------|---------|------------|
| `Container` | Box with background, border, corners, padding | BoxSpec, SurfaceSpec, structural containers |
| `Text` | Single-style text run | Labels, descriptions, headings |
| `Image` | Texture-backed rectangle | Icons, thumbnails, decorative images |
| `Button` | Interactive region with hover/press states | ButtonSpec, IconButtonSpec |
| `Checkbox` | Toggle with indicator | CheckboxSpec, SwitchSpec |
| `Slider` | Single-thumb range input | SliderSpec |
| `TextInput` | Single-line text entry | TextInputSpec |
| `ScrollView` | Vertical-scroll container | ScrollShellSpec |

Complex Flint components decompose into combinations of these primitives.

## Layout Model

**System**: Flexbox-like, pixel-based (`f32` values throughout)

| Capability | Details |
|------------|---------|
| Direction | `Horizontal`, `Vertical` — maps from `LayoutDirection` |
| Sizing | `Fixed(f32)`, `Grow(f32)`, `FitContent` — maps from `LayoutSizing` |
| Alignment | `Align::Start/Center/End/Stretch` — maps from `CrossAxisAlignment` |
| Justify | `Justify::Start/Center/End/SpaceBetween` — maps from `MainAxisAlignment` |
| Gap | `f32` pixels between children — maps from `LayoutSpacing.gap` |
| Padding | `Edges { top, right, bottom, left }` as `f32` — maps from `LayoutEdges` |
| Margin | `Edges` as `f32` — maps from `LayoutEdges` |

**Not supported**:
- CSS Grid — all grid layouts must fall back to nested flexbox
- Percentage-based sizing — all values are absolute pixels
- `calc()` or dynamic expressions — values must be pre-resolved

## Styling Capabilities

| Property | Jetstream Support | Flint Mapping |
|----------|-------------------|------------|
| Background color | `Vec4` solid color | `StyleDescriptor.background` via `ColorValue` |
| Border | Solid color, uniform width, per-side not supported | `StyleDescriptor.border` |
| Corner radius | `f32` per corner | `StyleDescriptor.corner_radii` |
| Shadow | Single box shadow (offset, blur, color) | `StyleDescriptor.shadow` (single only) |
| Opacity | `f32` 0.0–1.0 | `StyleDescriptor.opacity` |
| Transforms | Not supported | Adapters skip transform properties |
| Gradients | Not supported | Adapters use solid color fallback |
| Image filters | Not supported | Adapters skip filter properties |
| Blend modes | Not supported | Normal blend only |

## Text Rendering

| Capability | Details |
|------------|---------|
| Engine | Glyph atlas, GPU-rendered quads |
| Style runs | Single style per text widget (no inline bold/italic mixing) |
| Font selection | By family enum (Sans, Mono), resolved to engine's loaded fonts |
| Size | `f32` pixels |
| Weight | Integer (400, 500, 600) mapped to nearest available |
| Script support | Latin, basic LTR only |
| Direction | LTR only (no RTL, no BiDi) |
| Line breaking | Simple word-wrap |
| Rich text | Not supported — MarkdownEditor, BlockEditor render as plain text |

## Input Model

| Source | Events |
|--------|--------|
| Mouse | Click, hover, drag (start/move/end), scroll |
| Keyboard | Key press, text input, focus navigation |
| Gamepad | D-pad navigation (Up/Down/Left/Right), Activate (A/Cross), Back (B/Circle) |

**`UiEvent` enum variants:**
- `Clicked` → `SemanticEvent::Activated`
- `ValueChanged { value }` → `SemanticEvent::ValueChanged`
- `TextChanged { text }` → `SemanticEvent::ValueChanged { Text }`
- `FocusGained` / `FocusLost` → `SemanticEvent::FocusChanged`
- `ScrollChanged { offset }` → `SemanticEvent::ScrollChanged`
- `DragStarted` / `DragMoved` / `DragEnded` → `SemanticEvent::DragChanged`
- `KeyPressed { key }` → `SemanticEvent::KeyPressed`

**Gamepad additions** (not in GPUI):
- D-pad maps to `SemanticEvent::Navigate { direction }`
- Activate button maps to `SemanticEvent::Activated`
- Back button maps to `SemanticEvent::Cancelled`

## Focus Model

| Capability | Details |
|------------|---------|
| System | `FocusState` tracking per-screen |
| Tab order | Linear, determined by widget tree order |
| Directional | D-pad / arrow key navigation between focusable widgets |
| Focus ring | Rendered by the engine (color + width from theme) |
| Trap | Modal screens trap focus within their widget tree |
| Restore | Focus restored when screen is re-surfaced from stack |

## Screen Management

| Capability | Details |
|------------|---------|
| System | `ScreenStack` — push/pop model |
| Modal | `is_modal: true` blocks input to lower screens |
| Transparent | `is_transparent: true` renders lower screens (dimmed) |
| Backdrop | Engine renders scrim between modal and parent |
| Mapping | Dialog/Drawer → modal screen push; Popover → transparent screen push |

## Known Limitations (Intentional Deltas)

These are capabilities that Flint supports in Svelte/GPUI but Jetstream
intentionally does not implement:

| Limitation | Impact | Adapter Strategy |
|------------|--------|-----------------|
| No CSS Grid | Grid layouts need flexbox fallback | Adapter emits nested row/column containers |
| No horizontal scroll | Only vertical ScrollView | `overflow_x: Scroll` maps to `Hidden` |
| No rich text | No inline formatting in text widgets | Rich text components render as plain text |
| No complex scripts | Latin LTR only | International text displays but may not wrap correctly |
| No gradients | Solid colors only | Gradient tokens map to dominant solid color |
| No transforms | No rotate/scale/skew | Transform properties are ignored |
| No image filters | No blur/brightness/contrast | Filter properties are ignored |
| Single box shadow | One shadow per element | Specs with multiple shadows use the primary only |
| No percentage sizing | All values are absolute pixels | Percentage-based layouts pre-resolve to pixels |
| Gamepad input | Extra input dimension | `Navigate` events added to semantic event model |

## Performance Considerations

- Widget tree rebuilds should be minimized (retained mode)
- Complex component decompositions should cache intermediate results
- Text rendering is the most expensive operation — minimize text widget count
- Scroll containers use viewport culling for off-screen children
- Maximum recommended widget count: ~500 per screen

## Dependency

This document depends on Jetstream g04.016 (UI Rendering Infrastructure)
delivering the `game_ui` system with the capabilities listed above. If g04.016
modifies the API surface, this document should be updated accordingly.
