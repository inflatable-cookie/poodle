# g07.001 — GPUI Adapter Crate Setup and Theme Integration

Status: Completed
Updated: 2026-03-14

## Objective

Create the `poodle-gpui` adapter crate that implements the renderer adapter traits
from g06.007. Set up GPUI theme integration using g06.003's typed token values.
Verify that GPUI's styling API can consume resolved style descriptors from
g06.006.

## Deliverables

### New Crate: `poodle-gpui`

| Property | Value |
|----------|-------|
| Path | `packages/gpui/adapter/` |
| Crate name | `poodle-gpui` |
| Dependencies | `poodle-adapter`, `poodle-events`, `poodle-layout`, `poodle-primitives`, `poodle-composites`, `poodle-workstation`, `poodle-style`, `poodle-tokens` |

### Modules

| Module | Purpose |
|--------|---------|
| `lib.rs` | `GpuiTarget` (RenderTarget), `GpuiElementHandle`, `GpuiAdapter` (AdapterManifest), supported/unsupported component lists |
| `theme.rs` | `GpuiThemeProvider` implementing `ThemeProvider` — resolves string token values to typed numeric values via parsing + semantic constant lookup |
| `style_map.rs` | `StyleDescriptor` → `GpuiStyle` mapping — converts layout intent, colors, borders, shadows, typography, cursor to GPUI-compatible intermediate types |

### Theme Resolution Strategy

The `GpuiThemeProvider` resolves token strings (the `&'static str` values
emitted by spec `resolved_*_token()` methods) to typed values:

1. **Direct parsing**: Hex colors (`#RRGGBB`), rgba strings, rem/px dimensions
2. **Semantic lookup**: String constants matched against `poodle_tokens::semantic`
   are mapped to their typed equivalents from `poodle_tokens::typed::semantic`
3. **Safe fallback**: Unknown tokens return neutral defaults (black color, 0.0
   size, 1.0 opacity)

### Style Mapping Types

GPUI-compatible intermediate types (will map directly to `gpui::Style` fields
when GPUI runtime dependency is added):

- `GpuiColor` — `[f32; 4]` RGBA
- `GpuiEdges` — top/right/bottom/left `f32` pixels
- `GpuiCornerRadii` — per-corner `f32` radii
- `GpuiShadow` — offset, blur, color
- `GpuiTypography` — family, size, line-height, weight
- `GpuiStyle` — complete style struct with layout + visual properties
- `GpuiCursorStyle` — 8 cursor variants mapping from `CursorHint`

### Test Results

- 20 tests passing across 3 modules:
  - `theme::tests` — 8 tests (hex/rgba/rem resolution, border width, radius, opacity, unknown fallback, scale factor)
  - `style_map::tests` — 9 tests (flex direction, fixed/grow/constrained sizing, alignment, spacing, full descriptor mapping, cursor coverage)
  - `tests` — 3 tests (element handles, adapter manifest, theme access)

## Verification

- [x] `poodle-gpui` crate compiles with all 8 contract crate dependencies
- [x] `GpuiThemeProvider` implements `ThemeProvider` trait
- [x] Theme resolves all 19 semantic color tokens to typed `ColorValue`
- [x] Theme resolves all 17 semantic space/size tokens to `f32` pixels
- [x] Theme resolves border width, radius, and opacity tokens
- [x] `StyleDescriptor` maps to `GpuiStyle` with correct layout, colors, borders, shadows, typography, cursor
- [x] `GpuiAdapter` implements `AdapterManifest` (empty supported list — populated in g07.002–010)
- [x] 20 tests passing
- [x] No compiler errors, only `#[allow(dead_code)]` on future-use constants
