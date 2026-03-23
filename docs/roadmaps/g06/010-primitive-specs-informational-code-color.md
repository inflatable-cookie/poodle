# g06.010 — Primitive Specs: Informational, Code, and Color

Status: Completed
Updated: 2026-03-14

## Objective

Add 4 new primitive spec structs for informational, code display, and color
selection components.

## New Specs

| Spec | Key Props | Token Methods |
|------|-----------|---------------|
| `CodeSpec` | content, language, show_line_numbers, is_copyable | fill_token, text_color_token, font_family_token, font_size_token, border_token |
| `ColorPickerSpec` | value, default_value, is_open, show_alpha | current_value(), border_token, overlay_fill_token, shadow_token |
| `FileUploadSpec` | accept, max_size, is_multiple, is_dragging | fill_token (drag-dependent), border_token (drag-dependent) |
| `SplitButtonSpec` | variant, size, label, is_open | fill_token, border_token, separator_token, overlay_fill_token, shadow_token |

## Running Total

Primitive specs: 60 (after 009) + 4 (new) = **64**

## Verification

- [x] All 4 specs compile and are exported from `poodle-primitives`
- [x] All 29 existing tests continue to pass
