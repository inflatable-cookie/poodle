# g10.006 — Overlay and Date/Time Primitive Specimens

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.005
Primary repos: `pug`

## Goals

- [ ] create per-component specimens for overlay and date/time primitives
- [ ] overlays use Jetstream's `ScreenStack` for modal and transparent screens

## Execution Checklist

- [ ] create `accordion.rs` — Accordion with 3 collapsible sections
- [ ] create `collapsible.rs` — Collapsible with toggle header and content
- [ ] create `dialog.rs` — Dialog pushed to `ScreenStack` with modal backdrop,
  title, content, and action buttons
- [ ] create `drawer.rs` — Drawer as edge-aligned overlay screen
- [ ] create `popover.rs` — Popover as transparent overlay positioned
  relative to trigger
- [ ] create `hover_card.rs` — HoverCard triggered on hover with rich content
- [ ] create `tooltip.rs` — Tooltip as small positioned label overlay
- [ ] create `menu.rs` — Menu as vertical overlay list with item hover
- [ ] create `context_menu.rs` — ContextMenu triggered on right-click
- [ ] create `tabs.rs` — Tabs with tab strip and swappable content panel
- [ ] create `tab_strip.rs` — TabStrip standalone horizontal tab selector
- [ ] create `navigation_menu.rs` — NavigationMenu with submenu overlays
- [ ] create `menubar.rs` — Menubar with horizontal menu triggers
- [ ] create `calendar.rs` — Calendar month grid with day selection
- [ ] create `range_calendar.rs` — RangeCalendar with range highlight
- [ ] create `date_picker.rs` — DatePicker with trigger and calendar dropdown
- [ ] create `date_range_picker.rs` — DateRangePicker with range display
- [ ] create `date_time_picker.rs` — DateTimePicker with combined display
- [ ] create `date_time_range_picker.rs` — DateTimeRangePicker
- [ ] create `time_zone_select.rs` — TimeZoneSelect dropdown (if appropriate
  for game context, otherwise document skip)
- [ ] create `zoned_date_time_picker.rs` — ZonedDateTimePicker (if appropriate,
  otherwise document skip)
- [ ] register all modules and wire slug routing
- [ ] verify all specimens render without panic; overlays dismiss correctly

## Acceptance Criteria

- [ ] all overlay specimens open and close correctly via ScreenStack
- [ ] modal overlays have backdrop that blocks interaction with background
- [ ] date/time specimens show navigable calendars and selectable dates
- [ ] components excluded for game context are documented in delta register
- [ ] `cargo check` passes

## Next Task

Open `g10.007` and build form and data composite specimens.
