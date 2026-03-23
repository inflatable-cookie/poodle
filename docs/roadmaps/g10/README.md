# g10 Jetstream Production Quality

Status: in-progress
Updated: 2026-03-21

## Context

g09 unified the crate architecture so both GPUI and Jetstream share the same
contract specs. This generation builds out Jetstream component implementations
to match the GPUI target's coverage, achieving visual parity with Svelte for
all supported components.

## Starting State (at g10 start)

- 8 real Jetstream components: button, accordion, checkbox, switch, badge,
  progress, separator, status_indicator
- ~90 adapter render stubs returning placeholder node handles
- Theme bridge and layout mapper are solid
- 8 specimen pages in preview app
- Unified crate architecture from g09

## Current State

- **86 Jetstream components** implemented in `flint-jetstream-components`
  (up from 8 at start)
- All components resolve visual properties from tokens via `JetstreamThemeProvider`
- All components compile against current contract specs
- Engine feature handoff document created for Jetstream runtime team
  (see `jetstream-engine-handoff.md`)

### Components implemented (86 total)

**Primitives (62):**
accordion, alert_dialog, badge, banner, breadcrumbs, bulk_action_bar, button,
box, calendar, callout, card, checkbox, code, collapse_toggle, collapsible,
color_picker (stub), combobox, context_menu, date_picker, date_range_picker
(stub), date_time_picker (stub), date_time_range_picker (stub), detail_row,
dialog, drawer, editable_label, eyebrow, field, form_actions, grid, hover_card,
icon, icon_button, menu, menubar, meter, navigation_menu, number_entry,
pagination, pill, pin_input, popover, progress, radio_group, range_slider,
rating, region, resize_handle, scroll_shell, search_field, segmented_control,
select, separator, skeleton, slider, split_button, stack, status_indicator,
surface, switch, tabs, tab_strip, text_area, text_input, time_ago, toolbar,
tooltip

**Composites (24):**
action_discovery_panel, app_header, command_palette, data_table,
detail_section, detail_shell, dock_region, empty_state, filter_toolbar,
list_card, media_preview, media_thumbnail, metric_tile, nav_card,
nav_card_grid, order_by, page_header, pagination_summary, picker_shell,
relation_picker, selection_summary, split_view, toast_stack

## Milestone Status

| ID  | Milestone | Status | Notes |
|-----|-----------|--------|-------|
| 001 | Sync with contracts and feasibility assessment | Complete | See `001-sync-and-feasibility.md` |
| 002 | Audit existing 8 components | Complete | All in good shape; ARIA is N/A for Jetstream |
| 003 | Structural primitives (15 components) | Complete | box, stack, grid, surface, region, card, eyebrow, pill, callout, banner, skeleton, toolbar, form_actions, detail_row, field |
| 004 | Interactive primitives (19 components) | Complete | text_input, tabs, slider, radio_group, segmented_control, etc. |
| 005 | Complex primitives (22 components) | Complete | dialog, drawer, breadcrumbs, pagination, menu, select, etc. |
| 006 | Composites (24 components) | Complete | data_table, page_header, toast_stack, dock_region, etc. |
| 007 | Engine feature handoff | Complete | See `jetstream-engine-handoff.md` |
| 008 | Specimen pages for new components | Blocked | Waiting on engine features (SVG icons, overlays) |
| 009 | Visual parity verification | Blocked | Waiting on engine features |
| 010 | Generation closeout | Blocked | Waiting on 008, 009 |

## Blocked On: Jetstream Engine Features

Full component visual fidelity requires engine features that don't exist yet.
A handoff document has been delivered to the Jetstream runtime team. Key gaps:

1. **SVG/icon rendering** (HIGH) — ~40 components need real icons
2. **Overlay/portal rendering** (HIGH) — ~15 components need dropdown/dialog overlays
3. **Drag interaction** (HIGH) — sliders, resize handles, reorderable lists
4. **Color mixing utility** (HIGH) — ~30 components need hover/active state blending
5. **Per-side border colors** (MEDIUM) — tab indicators
6. **Image/texture rendering** (MEDIUM) — media components
7. **Pointer enter/leave events** (MEDIUM) — tooltips, hover cards

See `jetstream-engine-handoff.md` for full specification.

## What Can Proceed Without Engine Changes

- Specimen pages for structural components (box, stack, grid, surface, card, etc.)
- Specimen pages for text-only components (eyebrow, pill, time_ago, code, etc.)
- Adapter stub cleanup (replace old stubs with calls to new `js_*()` functions)
- Additional contract compliance auditing

## Non-Goals

- No new contract work (contracts are stable from g08/g09)
- No Svelte changes
- No downstream app adoption proof
