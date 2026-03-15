# g09.010 — Specimen Upgrade: Overlay and Date/Time Primitives

Status: complete
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.003
Primary repos: `pug`

## Goals

- [ ] ensure all overlay and date/time specimens use real Pug component
  instances with interactive state
- [ ] add specimens for components currently missing them

## Execution Checklist

- [ ] add `hover_card.rs` specimen using `PugHoverCard` with trigger element
  and rich content display on hover
- [ ] verify `calendar.rs` specimen uses `PugCalendar` with month navigation,
  day selection, and week-starts-on configuration
- [ ] verify `range_calendar.rs` specimen uses `PugRangeCalendar` with
  range highlight and endpoint selection
- [ ] verify `date_picker.rs` specimen uses `PugDatePicker` with trigger
  button and calendar dropdown
- [ ] verify `date_range_picker.rs` specimen uses `PugDateRangePicker` with
  range display and calendar dropdown
- [ ] verify `time_field.rs` specimen demonstrates `PugTimeField` with
  populated, placeholder, disabled, and error states
- [ ] verify `date_time_picker.rs` specimen uses `PugDateTimePicker` with
  combined date and time display
- [ ] verify `date_time_range_picker.rs` specimen uses
  `PugDateTimeRangePicker` with start/end display
- [ ] add `tri_state_switch.rs` specimen if not already covered by
  `switch.rs` — show on/off/indeterminate states
- [ ] update `mod.rs` to route `hover-card` to its own specimen file instead
  of sharing with `popover`
- [ ] verify all specimen slugs render without panic

## Acceptance Criteria

- [ ] `hover-card` has a dedicated real-component specimen
- [ ] all date/time specimens show interactive calendar/picker behavior
- [ ] `tri-state-switch` slug renders a real specimen demonstrating all
  three states
- [ ] `cargo check` passes for the preview crate

## Next Task

Open `g09.011` and begin composite specimen upgrades.
