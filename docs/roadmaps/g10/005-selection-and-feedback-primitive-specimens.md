# g10.005 — Selection and Feedback Primitive Specimens

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.004
Primary repos: `pug`

## Goals

- [ ] create per-component specimens for selection and feedback primitives

## Execution Checklist

- [ ] create `checkbox.rs` — Checkbox with checked, unchecked, indeterminate,
  and disabled states
- [ ] create `radio_group.rs` — RadioGroup with 3-4 options showing selected
  and disabled states
- [ ] create `switch.rs` — Switch with on/off and disabled states
- [ ] create `tri_state_switch.rs` — TriStateSwitch showing on/off/
  indeterminate
- [ ] create `select.rs` — Select with option list, selected value, and
  placeholder
- [ ] create `segmented_control.rs` — SegmentedControl with 3-4 segments
  and selected highlight
- [ ] create `slider.rs` — Slider with value label, min/max, and disabled
  state
- [ ] create `range_slider.rs` — RangeSlider with dual thumbs and range fill
- [ ] create `progress.rs` — Progress bar with value, indeterminate, and
  different sizes
- [ ] create `badge.rs` — Badge with variant colors (info, success, warning,
  error) and sizes
- [ ] create `status_indicator.rs` — StatusIndicator with tone-colored dots
- [ ] create `meter.rs` — Meter with semantic color thresholds
- [ ] create `rating.rs` — Rating with interactive stars
- [ ] create `skeleton.rs` — Skeleton with text, avatar, card, and paragraph
  presets
- [ ] create `pill.rs` — Pill tags with optional remove action
- [ ] create `eyebrow.rs` — Eyebrow label with secondary styling
- [ ] create `time_ago.rs` — TimeAgo relative time display
- [ ] create `duration_input.rs` — DurationInput with hours/minutes/seconds
- [ ] create `code.rs` — Code display block with monospace text
- [ ] create `color_picker.rs` — ColorPicker with swatch grid
- [ ] create `file_upload.rs` — FileUpload drop zone with progress
- [ ] register all modules and wire slug routing
- [ ] verify all 21 specimens render without panic

## Acceptance Criteria

- [ ] all 21 selection/feedback specimens render correctly
- [ ] interactive specimens respond to click/keyboard events
- [ ] feedback specimens show correct tone colors from theme
- [ ] `cargo check` passes

## Next Task

Open `g10.006` and build overlay and date/time specimens.
