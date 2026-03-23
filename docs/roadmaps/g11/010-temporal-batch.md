# g11.010 Temporal Batch

Status: planned
Owner: Flint Core
Depends on: contract audit

## Components

calendar, range_calendar, date_picker, date_range_picker, date_time_picker,
date_time_range_picker, zoned_date_time_picker, time_zone_select

## Structural Issues

None — all components have contracts, Rust specs, and GPUI implementations.

## Notes

The temporal components are complex composites of inputs, calendars, and
dropdowns. The GPUI implementations may need significant rework to match
the contract anatomy (calendar grid rendering, date selection state, time
input segments, timezone dropdown).

## Per-Component Compliance

- [ ] calendar — audit against `docs/contracts/foundation/calendar.md`
- [ ] range_calendar — audit against `docs/contracts/foundation/range-calendar.md`
- [ ] date_picker — audit against `docs/contracts/foundation/date-picker.md`
- [ ] date_range_picker — audit against `docs/contracts/foundation/date-range-picker.md`
- [ ] date_time_picker — audit against `docs/contracts/foundation/date-time-picker.md`
- [ ] date_time_range_picker — audit against `docs/contracts/foundation/date-time-range-picker.md`
- [ ] zoned_date_time_picker — audit against `docs/contracts/foundation/zoned-date-time-picker.md`
- [ ] time_zone_select — audit against `docs/contracts/foundation/time-zone-select.md`
