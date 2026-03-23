# g10.001 Sync with Contracts and Feasibility Assessment

Status: complete
Owner: Poodle Core
Depends on: g09

## Starting State

8 real Jetstream components exist: accordion, badge, button, checkbox, progress,
separator, status_indicator, switch. All use token resolution via `js_*()` functions
in `poodle-jetstream-components`. The adapter layer has ~20 real renders and ~71 stubs.

## Jetstream Rendering Constraints

- No SVG (icons must be text glyphs or omitted)
- No gradients
- No transforms (rotation, scale)
- No rich text or IME
- No ARIA / screen readers
- No stacked/inset shadows
- No animations or transitions

## Feasibility Classification

### Tier 1 — Straightforward (layout + color + text only)

These components are boxes with backgrounds, borders, text, and spacing.
No SVG, no animation, no complex interaction required.

| Component | Notes |
|-----------|-------|
| box | Container with padding/overflow |
| stack | Flex column with gap |
| grid | CSS grid layout |
| surface | Themed container |
| region | Semantic container |
| card | Bordered container with slots |
| separator | Already implemented — skip |
| eyebrow | Small label text |
| pill | Rounded label |
| callout | Themed box with icon placeholder |
| banner | Status-toned message bar |
| skeleton | Pulsing placeholder (static gray box in Jetstream) |
| toolbar | Horizontal flex container |
| form_actions | Row of action buttons |
| detail_row | Label/value pair row |
| field | Label + control wrapper |
| text_input | Bordered input box |
| text_area | Multi-line input box |
| search_field | Input with icon |
| icon_button | Button with icon (text glyph fallback) |
| split_button | Primary + dropdown trigger |
| collapse_toggle | Clickable expand/collapse control |
| collapsible | Container that shows/hides content |
| radio_group | List of selectable options |
| toggle | Pressable button state |
| toggle_group | Row of toggle buttons |
| segmented_control | Tab-like selector |
| alert_dialog | Modal with title, description, actions |
| dialog | Overlay container with backdrop |
| drawer | Slide-out panel |
| meter | Horizontal bar indicator |
| time_ago | Text label |
| code | Monospace text block |
| rating | Row of indicators (star text glyphs) |
| number_entry | Input with +/- buttons |
| breadcrumbs | Text links with separators |
| pagination | Page number buttons |
| tabs | Tab bar with content panels |
| tab_strip | Standalone tab bar |
| empty_state (composite) | Message + action centered layout |
| detail_section (composite) | Header + body section |
| detail_shell (composite) | Page layout shell |
| page_header (composite) | Title + breadcrumb + actions bar |
| metric_tile (composite) | Label + value card |
| toast_stack (composite) | Stacked notification cards |
| selection_summary (composite) | Selected items bar |
| filter_toolbar (composite) | Search + filter row |
| pagination_summary (composite) | "Showing X of Y" text |
| split_view (composite) | Resizable two-panel layout |
| data_table (composite) | Rows/columns with headers |
| nav_card (composite) | Clickable card with title |
| nav_card_grid (composite) | Grid of nav cards |
| list_card (composite) | Row card with leading/trailing |
| order_by (composite) | Sort control |
| picker_shell (composite) | Modal picker layout |
| relation_picker (composite) | Searchable selection list |
| dock_region (composite) | Tabbed panel region |
| app_header (composite) | Top-level header bar |
| action_discovery_panel (composite) | Keyboard shortcut panel |
| command_palette (composite) | Search-driven action list |

### Tier 2 — Feasible with constraints

These need adaptation but are achievable:

| Component | Constraint | Adaptation |
|-----------|-----------|------------|
| icon | No SVG | Use text glyphs or omit; icon_button already handles this |
| slider | No drag interaction | Render track + thumb at position; interaction via click |
| range_slider | Same as slider | Two thumbs on track |
| resize_handle | No drag | Render divider bar; interaction on click |
| progress | Already implemented | Skip |
| status_indicator | Already implemented | Skip |
| popover | No overlay stacking | Render inline below trigger |
| hover_card | No hover detection | Render inline or skip |
| tooltip | No hover detection | Skip visual, keep ARIA-like data |
| context_menu | No right-click | Render as inline menu |
| select | Dropdown overlay | Render as inline list |
| combobox | Input + dropdown | Render input + inline list |
| date_picker | Calendar overlay | Render inline calendar |
| date_range_picker | Same | Inline range calendar |
| date_time_picker | Calendar + time | Inline combined |
| date_time_range_picker | Same | Inline combined |
| zoned_date_time_picker | Same + timezone | Inline combined |
| calendar | Grid of days | Feasible as text grid |
| range_calendar | Same with range | Feasible |
| time_field | Time input | Segmented number inputs |
| pin_input | Row of single-char inputs | Row of boxes |
| duration_input | Multiple number fields | Row of inputs |
| color_picker | Color selection | Simplified: text input + swatch grid |
| editable_label | Click-to-edit text | Text display + input toggle |
| scroll_shell | Scrollable container | Jetstream has scroll support |
| bulk_action_bar | Action bar | Standard toolbar |
| menu | Vertical item list | Feasible |
| menubar | Horizontal menu triggers | Feasible with inline menus |
| navigation_menu | Link list | Feasible |
| file_upload | Drop zone | Simplified: button trigger only |

### Tier 3 — Unfeasible or deferred

| Component | Reason |
|-----------|--------|
| time_zone_select | Complex timezone DB + searchable dropdown |
| table | Complex column sizing, sort indicators, selection — large effort, defer |

## Implementation Plan

### Batch 1 (g10.002): Fix existing 8 components

Audit existing 8 against current contracts. Fix any divergence.

### Batch 2 (g10.003): Structural primitives (~15 components)

box, stack, grid, surface, region, card, eyebrow, pill, callout, banner,
skeleton, toolbar, form_actions, detail_row, field

### Batch 3 (g10.004): Interactive primitives (~20 components)

text_input, text_area, search_field, icon_button, split_button, icon,
collapse_toggle, collapsible, radio_group, toggle, toggle_group,
segmented_control, tabs, tab_strip, slider, range_slider, meter,
number_entry, code, time_ago, rating

### Batch 4 (g10.005): Complex primitives + composites (~25 components)

alert_dialog, dialog, drawer, breadcrumbs, pagination, menu, menubar,
navigation_menu, select, combobox, popover, context_menu, calendar,
date_picker, editable_label, pin_input, scroll_shell, bulk_action_bar,
resize_handle, empty_state, detail_section, detail_shell, page_header,
metric_tile, toast_stack

### Batch 5 (g10.006): Remaining composites + cleanup

selection_summary, filter_toolbar, pagination_summary, split_view,
data_table, nav_card, nav_card_grid, list_card, order_by, picker_shell,
relation_picker, dock_region, app_header, action_discovery_panel,
command_palette, file_upload, hover_card, duration_input, color_picker

### Final (g10.007-008): Specimens, parity verification, closeout
