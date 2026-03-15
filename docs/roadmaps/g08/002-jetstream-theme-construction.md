# g08.002 — Jetstream Theme Construction from Pug Tokens

Status: Completed
Updated: 2026-03-14

## Objective

Implement `JetstreamThemeProvider` that resolves Pug token paths to typed values
compatible with Jetstream's Vec4 colors and f32 pixel values.

## Deliverables

### JetstreamThemeProvider (theme.rs)

- Implements `ThemeProvider` trait from pug-adapter
- `resolve_color`: hex parsing (#RRGGBB, #RRGGBBAA), rgba() parsing, semantic
  constant matching against 19 typed constants
- `resolve_space`: rem→px (×16), px passthrough, plain number parsing
- `resolve_radius`, `resolve_border_width`: delegate to resolve_space
- `resolve_opacity`: direct float parsing
- Scale factor support via `with_scale_factor()` builder

### Semantic color coverage

19 constants mapped: COLOR_ACCENT_BASE, COLOR_ACCENT_HOVER,
COLOR_ACCENT_FOCUS_RING, COLOR_BACKGROUND_CANVAS/SURFACE/PANEL/OVERLAY/ELEVATED,
COLOR_TEXT_PRIMARY/SECONDARY/INVERSE, COLOR_BORDER_SUBTLE/DEFAULT/STRONG,
COLOR_STATUS_SUCCESS/WARNING/DANGER, COLOR_ICON_PRIMARY/MUTED

### Style mapping (style_map.rs)

- `JetstreamStyle` struct with direction, sizing, alignment, gap, padding,
  margin, background, border, corner_radius, opacity, visibility, clip
- `JetstreamSizing`: Fixed(f32), Grow, Fit — maps from LayoutSizing
- `map_layout()`: LayoutIntent → JetstreamStyle
- `map_style()`: StyleDescriptor → JetstreamStyle (includes color, border,
  opacity, visibility)
- Constrained sizing approximated as Fixed at midpoint

## Verification

```
cargo test — 8 theme tests + 7 style_map tests passing
All token resolution strategies verified: hex, rgba, semantic, rem, px, plain
Scale factor multiplication verified
```
