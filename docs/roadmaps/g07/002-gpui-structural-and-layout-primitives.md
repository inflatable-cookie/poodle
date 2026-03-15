# g07.002 — GPUI Structural and Layout Primitives

Status: Completed
Updated: 2026-03-14

## Objective

Implement `RenderComponent` for structural and layout primitives in the
GPUI adapter. These 8 components form the foundational layer that all
other components compose on.

## Components Implemented

| Spec | Category | Token Resolution |
|------|----------|-----------------|
| `BoxSpec` | Container | Padding via `PaddingScale` |
| `StackSpec` | Layout | Gap via `PaddingScale`, alignment |
| `GridSpec` | Layout | Column/row gaps, padding |
| `SurfaceSpec` | Themed container | Background, border color/width, shadow from tone |
| `SeparatorSpec` | Divider | Stroke color from `RuleTone`, width |
| `ScrollShellSpec` | Scrollable region | Focus ring when focusable |
| `BannerSpec` | Status messaging | Fill, border, icon from `StatusTone` |
| `CallOutSpec` | Status messaging | Fill, border from `StatusTone` |

## Implementation

New module `render_structural.rs` containing 8 `RenderComponent<Spec>`
implementations. Each:

1. Converts the `StyleDescriptor` to `GpuiStyle` via `map_style()`
2. Resolves spec-specific tokens through the `ThemeProvider`
3. Applies resolved values to the GPUI style struct
4. Returns a `GpuiElementHandle` identifying the rendered element

## Tests

8 new tests (28 total across crate):

- `render_box_produces_handle`
- `render_stack_produces_handle`
- `render_grid_produces_handle`
- `render_surface_resolves_background`
- `render_separator_produces_handle`
- `render_scroll_shell_with_focus_ring`
- `render_banner_resolves_tone_tokens`
- `render_callout_resolves_tone_tokens`

## Verification

- [x] All 8 structural primitives have `RenderComponent` implementations
- [x] `AdapterManifest` updated with 8 supported component names
- [x] 28 tests passing (20 from g07.001 + 8 new)
- [x] No compiler errors
