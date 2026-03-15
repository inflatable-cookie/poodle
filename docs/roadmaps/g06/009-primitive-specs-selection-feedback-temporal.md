# g06.009 — Primitive Specs: Selection, Feedback, and Temporal

Status: Completed
Updated: 2026-03-14

## Objective

Add 8 new primitive spec structs for selection, feedback, and temporal components.

## New Specs

| Spec | Key Props | Token Methods |
|------|-----------|---------------|
| `MeterSpec` | value, min, max, low, high, optimum | fill_token, track_fill_token, normalized_progress() |
| `PillSpec` | label, is_removable, is_selected, is_disabled | fill_token, text_color_token |
| `RatingSpec` | value, max, is_readonly, precision | active_color_token, inactive_color_token, filled_count() |
| `SkeletonSpec` | shape, width, height, is_animated | fill_token, radius_token |
| `TimeAgoSpec` | timestamp, live | text_color_token, font_size_token |
| `DurationInputSpec` | value, is_disabled, validation_state, show_seconds | border_token |
| `TimeZoneSelectSpec` | value, placeholder, is_open, is_disabled | trigger_text(), border_token, overlay_fill_token |
| `ZonedDateTimePickerSpec` | value, time_zone, is_open, is_disabled | border_token, overlay_fill_token, shadow_token |

## Running Total

Primitive specs: 52 (after 008) + 8 (new) = **60**

## Verification

- [x] All 8 specs compile and are exported from `pug-primitives`
- [x] All 29 existing tests continue to pass
