# g08 GPUI Production Quality

Status: complete
Updated: 2026-03-19

## Context

Prior generations built GPUI component files with a working preview app and
specimen pages. However, a thorough audit found that **every existing component
is at Partial quality** — colors are mostly token-resolved, but dimensions are
universally hardcoded, focus rings are absent, ARIA attributes are minimal, and
several components have functional bugs.

Additionally, 20 contracts have Svelte implementations but no GPUI component
at all. These need to be built from scratch.

Concurrently, the Svelte reference implementation is being actively refined —
component names, contracts, and composite boundaries are a moving target. Each
milestone in this generation must verify current contract state before
implementing, not assume contracts are stable from the outset.

This generation brings every GPUI component to production quality: full token
resolution, contract compliance, and visual parity with Svelte.

## Inventory (as of g08 opening)

### Existing GPUI components (84) — all Partial quality

These components exist and render real content, but have hardcoded dimensions,
missing focus rings, missing ARIA, and in some cases functional bugs:

**Structural (8):** accordion, box, collapsible, grid, scroll_shell, separator,
spacer, stack

**Action (9):** button, editable_label, form_actions, icon_button, number_entry,
pin_input, search_field, text_input, toolbar

**Selection (8):** checkbox, radio_group, range_slider, segmented_control,
select, slider, switch, tri_state_switch

**Feedback (12):** callout, code, color_picker, eyebrow, file_upload, meter,
pill, progress, rating, skeleton, status_indicator, time_ago

**Overlay (9):** context_menu, dialog, drawer, hover_card, menu, menubar,
navigation_menu, popover, tooltip

**Input (6):** duration_input, text_area, time_field, time_zone_select, tabs,
toggle

**Date/Time (6):** calendar, date_picker, date_range_picker, date_time_picker,
date_time_range_picker, zoned_date_time_picker

**Composites (17):** action_discovery_panel, app_header, command_palette,
data_table, detail_shell, dock_region, empty_state, filter_toolbar,
icon (foundation), media_preview, media_thumbnail,
picker_shell, relation_picker, selection_summary, split_view

**Other (9):** field, form_actions, range_calendar, split_button, surface,
status_bar, tab_strip, toggle_group, surface

### Missing GPUI components (20) — need new implementations

These contracts have Svelte component + specimen but no GPUI component:

**Foundation (16):** alert-dialog, breadcrumbs, bulk-action-bar, card,
collapse-toggle, combobox, detail-row, list-card, nav-card, nav-card-grid,
order-by, pagination, region, resize-handle, status-bar, table

**Composites (4):** detail-section, metric-tile, page-header, toast-stack

### Contracts to ignore (no Svelte implementation)

- `icon-provider` — context provider, not a visual component
- `surface-elevation` — visual stacking concept, not a component
- `browse-search-shell` — may be removed (no Svelte component)
- `embed-shell` — may be removed (no Svelte component)

### Known broken components (2)

- `color_picker` — swatch colors never applied (loop variable discarded)
- `range_slider` — fill segment and thumbs not rendered (values discarded)

### Cross-cutting quality issues

- **Disabled opacity**: ~18 components hardcode `0.48` instead of resolving
  from `disabled_opacity_token()`
- **Hover colors**: ~10 components use hardcoded `hsla(0.0, 0.0, 0.5, 0.04)`
  instead of `color_mix` with elevated/surface tokens
- **Geometry**: ~15 components hardcode height (36px), padding (12px),
  radius (6px), gap (8px) instead of resolving from spec tokens
- **Focus rings**: Zero components implement focus rings
- **ARIA**: Only 8 of 84 components reference ARIA in any form
- **Icons**: `icon_button` renders icon names as raw text, not `PoodleIcon` SVG

## Exit State

- Every GPUI component resolves all visual properties from semantic tokens
- Zero hardcoded pixel values in component rendering code
- All interactive components have focus rings per contract
- ARIA attributes applied per contract where GPUI supports them
- All icon slots use `PoodleIcon` with real SVG rendering
- All 20 missing components implemented
- Broken components (`color_picker`, `range_slider`) fully functional
- Specimen pages match contract specimen definitions
- Visual parity with Svelte systematically verified and documented
- Any intentional GPUI-specific deltas documented with rationale

## Non-Goals

- No Jetstream work (deferred to g09)
- No downstream app adoption proof
- No new component families or features beyond what contracts specify

## Milestone Status

| ID  | Milestone | Depends On | Class | Status |
|-----|-----------|------------|-------|--------|
| 001 | Sync with contracts: verify names, props, and token methods | — | Foundation | Complete |
| 002 | Implement missing components batch 1 (foundation primitives) | 001 | Implementation | Complete |
| 003 | Implement missing components batch 2 (composites + remaining) | 001 | Implementation | Complete |
| 004 | Cross-cutting fixes: disabled opacity, hover colors, geometry tokens | 001 | Implementation | Complete |
| 005 | Component quality fixes batch 1: high-visibility (6 components) | 004 | Implementation | Complete |
| 006 | Component quality fixes batch 2: inputs and selection (6 components) | 004 | Implementation | Complete |
| 007 | Component quality fixes batch 3: remaining + broken (8 components) | 004 | Implementation | Complete |
| 008 | Focus rings and ARIA attributes | 005, 006, 007 | Implementation | Complete (platform delta) |
| 009 | Specimen pages aligned to contract definitions | 002, 003, 008 | Implementation | Complete |
| 010 | Visual parity verification and delta register | 009 | Hardening | Complete |
| 011 | Generation closeout | 010 | Closure | Complete |

## Dependency Shape

```text
001 Sync with Contracts (complete)
  -> 002 Missing Batch 1 (foundation) ──────────────────────┐
  -> 003 Missing Batch 2 (composites)  ─────────────────────┤
  -> 004 Cross-Cutting Fixes                                │
      -> 005 Quality Batch 1: High-Visibility  ─┐           │
      -> 006 Quality Batch 2: Input/Selection   ├─> 008     │
      -> 007 Quality Batch 3: Remaining/Broken ─┘  Focus  ──┤
                                                   + ARIA   │
                                                      └─> 009 Specimens
                                                              -> 010 Parity
                                                                  -> 011 Closeout
```

## Execution Lanes

### Lane A: Missing Components

`001 -> { 002, 003 }`

Can start immediately after 001 completes. These components should be built
to production quality from the start (token-resolved, with focus rings and
ARIA) so they don't need to go through the quality fix batches.

### Lane B: Quality Fixes (parallel batches)

`001 -> 004 -> { 005, 006, 007 } -> 008`

Batches 005, 006, 007 can execute in parallel once 004 lands shared fixes.

### Lane C: Specimens and Verification

`009 -> 010 -> 011`

009 waits for both lanes to complete.

## Contract Verification Rule

**Every milestone must begin by checking the current state of relevant
contracts in `docs/contracts/`.** Component names, props, token targets, and
composite boundaries are being actively refactored on the Svelte side.
Assumptions from previous sessions may be stale. Specifically:

- Verify the component still exists in the contract (not renamed/removed)
- Verify prop names and types match current contract
- Verify token target names match current contract
- Verify the spec struct in `poodle-primitives`/`poodle-composites` matches
- If a contract has changed, update the GPUI implementation to match before
  proceeding with quality fixes

This verification is not a one-time gate — it applies at the start of each
milestone because the Svelte side is a concurrent moving target.

## How To Verify "Done" For Each Component

A component is production quality when ALL of the following are true:

1. **Zero hardcoded px values** — every dimension resolves from a spec token
   method (`resolve_px(theme, spec.some_token())`)
2. **Zero hardcoded colors** — every color resolves from a spec token method
   (`resolve_color(theme, spec.some_token())`)
3. **Hover/active states** use `color_mix` with elevated/surface tokens, not
   hardcoded `hsla()` values
4. **Disabled state** uses `resolve_opacity(theme, spec.disabled_opacity_token())`
   or element-level `.opacity()` with a token-resolved value
5. **Focus ring** present on interactive components (buttons, inputs, checkboxes,
   etc.) using the accent focus ring token
6. **ARIA attributes** match contract requirements (role, aria-label, etc.)
7. **Icons** render via `PoodleIcon` with real SVGs, not text/emoji placeholders
8. **Anatomy** matches contract — all parts present, correct nesting
9. **All props** from contract supported in the spec struct
10. **Specimen** matches contract specimen definitions exactly

A milestone is NOT done until every component in its scope passes all 10 checks.
If any check fails, the component is still Partial and the milestone is not
complete. No exceptions, no "close enough."
