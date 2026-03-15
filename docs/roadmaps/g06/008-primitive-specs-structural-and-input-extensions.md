# g06.008 — Primitive Specs: Structural and Input Extensions

Status: Completed
Updated: 2026-03-14

## Objective

Add 10 new primitive spec structs for structural and input components that exist
in the Svelte layer but lack Rust contract equivalents.

## New Specs

| Spec | Key Props | Token Methods |
|------|-----------|---------------|
| `BannerSpec` | tone, title, message, is_dismissible, has_icon | fill_token, border_token, icon_color_token |
| `CallOutSpec` | tone, title, content | fill_token, border_token |
| `EditableLabelSpec` | value, placeholder, is_editing, is_disabled | text_color_token, edit_border_token |
| `EyebrowSpec` | content | text_color_token, font_size_token |
| `HoverCardSpec` | is_open, placement | fill_token, shadow_token |
| `NumberEntrySpec` | value, min, max, step, validation_state | border_token, clamped_value() |
| `PinInputSpec` | length, value, is_masked, is_disabled | border_token, focus_ring_color_token, is_complete() |
| `RangeSliderSpec` | low, high, min, max, step, orientation | range_fill_token, track_fill_token, normalized_low/high() |
| `ToolbarSpec` | alignment, has_separator | border_token, gap_token |
| `TriStateSwitchSpec` | state (CheckState), label, is_disabled | track_fill_token, aria_checked() |

## Running Total

Primitive specs: 42 (existing) + 10 (new) = **52**

## Verification

- [x] All 10 specs compile and are exported from `pug-primitives`
- [x] All 29 existing tests continue to pass
- [x] Reuses existing shared types (StatusTone, CheckState, Alignment, etc.)
