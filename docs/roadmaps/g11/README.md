# g11 GPUI Contract Compliance

Status: in-progress
Updated: 2026-03-21

## Context

The GPUI component library has ~97 component implementations, but many have
incomplete contract compliance: missing anatomy parts, incorrect token usage,
broken states, missing accessibility attributes, and visual deviations from
the Svelte reference. The Svelte implementation is being audited against the
contracts to produce a definitive specification for each component.

This generation brings every GPUI component into full compliance with its
contract, using the Svelte implementation as the visual reference.

## Starting State

- 97 GPUI components with Deref containment pattern (from g09)
- 6 components misplaced in composites/ that belong in primitives/
- 17 Svelte composites with no GPUI counterpart
- Unknown number of contract compliance gaps across existing components
- Contract audit against Svelte in progress (will land as updated
  `docs/contracts/foundation/` files)

## Exit State

- All Svelte components have GPUI counterparts
- Every GPUI component passes a contract compliance checklist:
  - Anatomy matches contract (all parts present, correct nesting)
  - All props from contract supported
  - Every dimension resolves from tokens (zero hardcoded px)
  - Every color resolves from tokens
  - Disabled/loading states handled correctly
  - Hover/active/focus states match contract
  - ARIA attributes applied where specified
  - Visual output matches Svelte reference
- Component tracking document records compliance status per component

## Milestone Status

| ID  | Milestone | Depends On | Status |
|-----|-----------|------------|--------|
| 001 | Reorganize misplaced components (6) | — | Complete |
| 002 | Implement missing composites (17) | 001 | Complete |
| 003 | Inputs batch | contract audit | Planned |
| 004 | Selection batch | contract audit | Planned |
| 005 | Buttons batch | contract audit | Planned |
| 006 | Navigation batch | contract audit | Planned |
| 007 | Layout batch | contract audit | Planned |
| 008 | Feedback batch | contract audit | Planned |
| 009 | Overlay batch | contract audit | Planned |
| 010 | Temporal batch | contract audit | Planned |
| 011 | Composites batch | contract audit | Planned |
| 012 | Visual parity verification | 003–011 | Planned |
| 013 | Generation closeout | 012 | Planned |

## Dependency Shape

```text
001 Reorganize
  -> 002 Missing Composites
      -> [contract audit lands]
          -> 003 Inputs        ─┐
          -> 004 Selection      │
          -> 005 Buttons        │
          -> 006 Navigation     │
          -> 007 Layout         ├─> 012 Visual Parity -> 013 Closeout
          -> 008 Feedback       │
          -> 009 Overlay        │
          -> 010 Temporal       │
          -> 011 Composites    ─┘
```

## Component Batches

### Batch 003 — Inputs
text_input, text_area, search_field, number_entry, pin_input, duration_input,
time_field, color_picker, file_upload, editable_label, combobox

### Batch 004 — Selection
checkbox, radio_group, switch, tri_state_switch, toggle, toggle_group,
segmented_control, select

### Batch 005 — Buttons
button, icon_button, split_button

### Batch 006 — Navigation
tabs, tab_strip, breadcrumbs, pagination, menu, menubar, navigation_menu,
context_menu

### Batch 007 — Layout
stack, grid, box, surface, card, region, separator, spacer, resize_handle,
scroll_shell, toolbar, form_actions

### Batch 008 — Feedback
progress, meter, skeleton, status_indicator, rating, badge, pill, callout,
banner, eyebrow, code, time_ago, status_bar

### Batch 009 — Overlay
dialog, drawer, popover, tooltip, hover_card, alert_dialog, collapsible,
collapse_toggle

### Batch 010 — Temporal
calendar, range_calendar, date_picker, date_range_picker, date_time_picker,
date_time_range_picker, zoned_date_time_picker, time_zone_select

### Batch 011 — Composites
All composites: data_table, detail_section, detail_shell, page_header,
filter_toolbar, empty_state, picker_shell, relation_picker, selection_summary,
pagination_summary, split_view, toast_stack, metric_tile, nav_card,
nav_card_grid, list_card, order_by, dock_region, app_header, command_palette,
action_discovery_panel, media_preview, media_thumbnail, breadcrumbs,
+ 16 newly implemented composites

## Per-Component Compliance Checklist

For each component in batches 003–011:

1. Read the contract (`docs/contracts/foundation/<component>.md`)
2. Read the Svelte implementation (visual reference)
3. Read the GPUI implementation
4. Check:
   - [ ] Anatomy: all parts present with correct nesting
   - [ ] Props: all contract props supported in spec
   - [ ] Tokens: every dimension/color/radius resolves from tokens
   - [ ] States: disabled, loading, hover, active, focus all handled
   - [ ] Accessibility: ARIA roles, attributes, keyboard behavior
   - [ ] Visual: matches Svelte reference output
5. Fix all gaps
6. Verify specimen renders correctly

## Non-Goals

- No new components beyond what Svelte has
- No contract changes (contracts are the spec, not the other way around)
- No Jetstream work (that's g10)
