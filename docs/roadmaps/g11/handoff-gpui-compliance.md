# Continue g11 GPUI Contract Compliance — Component-by-Component Fix Pass

## Context

You are continuing a systematic pass through all GPUI components to bring them into full compliance with their contract specifications. The Svelte implementation is the visual reference. The gap report at `docs/roadmaps/g11/gap-report.md` (777 lines) documents every gap for every component.

## What's Been Done

g11.001-002 (structural): Complete — 118/118 Svelte-GPUI coverage, misplaced components reorganized.

g11 gap report: Complete — all 118 components audited against contracts.

20 components have been fixed so far:

**Batch 005 Buttons (complete):** button, icon_button, split_button
**Batch 004 Selection (7/8):** toggle (rewritten with new ToggleSpec), toggle_group (rewritten with new ToggleGroupSpec), checkbox, radio_group, switch, segmented_control, select. **tri_state_switch deferred** — needs full architectural rewrite from sliding switch to 3-segment radiogroup per contract.
**Batch 007 Layout (2/13):** stack (added direction/justify/wrap/aria_label to StackSpec), form_actions (removed incorrect border, added flex_wrap)
**Batch 008 Feedback (3/13):** progress (track height 6→8px), status_indicator (dot 8→9px, gap 6→7px), eyebrow (font 12→11px, added uppercase, line-height 1.5)
**Batch 006 Navigation (1/8):** tabs (border-subtle, font 12px, weight SEMIBOLD)
**Batch 009 Overlay (1/8):** collapsible (major rewrite — bordered card, grid trigger, Icon chevron, description, proper padding/bg)
**Batch 003 Inputs (2/11):** text_input (use spec tokens, min-height, focus ring, remove hover), text_area (focus ring, remove hover, disabled cursor)

## The Fix Pattern

Every component needs the same cross-cutting fixes:

1. **Focus ring**: Add `.focus(move |s| s.border_color(focus_ring_color))` — GPUI has no CSS outline, so we use border-color change on focus as approximation
2. **Disabled cursor**: Add `.cursor(CursorStyle::OperationNotAllowed)` when disabled
3. **Remove incorrect hover**: Many input components add hover bg that the contract doesn't specify (contract uses focus-within instead)
4. **Use spec token methods**: Replace hardcoded token path strings like `"semantic.color.background.surface"` with spec methods like `spec.fill_token()`
5. **Fix hardcoded px**: Replace literal pixel values with contract-specified values (e.g., indicator 18px not icon.md token, dot 8px not indicator/3)
6. **Typography**: Replace `.text_sm()` / `.text_xs()` with contract-specified sizes (often different)

## What's Remaining

### Batch 003 Inputs (9 remaining)
search_field, number_entry, pin_input, duration_input, time_field, color_picker, file_upload, editable_label, combobox

### Batch 004 Selection (1 remaining)
tri_state_switch — **ARCHITECTURAL REWRITE** needed: currently renders as a binary sliding switch, contract specifies a 3-segment radiogroup with excluded/default/included states using status-danger/background-elevated/status-success colors

### Batch 006 Navigation (7 remaining)
tab_strip, breadcrumbs, pagination, menu, menubar, navigation_menu, context_menu

### Batch 007 Layout (11 remaining)
grid, box (bx.rs), surface, surface_elevation (unimplemented), card, region, separator, spacer (no spec), resize_handle, scroll_shell (essentially a stub), toolbar

### Batch 008 Feedback (10 remaining)
meter, skeleton, rating, pill (**SPEC MISALIGNED** — spec models removable chip, contract models label pill), callout (major anatomy gaps — no icon, no actions slot, no dismiss), badge (**ORPHANED** — no contract), banner (**ORPHANED** — should be consolidated into callout), code (no inline mode, no toolbar, no copy), time_ago (no relative time computation — stub), status_bar

### Batch 009 Overlay (7 remaining)
dialog (no overlay wrapper, no backdrop), drawer (renders inline not fixed), popover (no placement), tooltip (wrong colors/tokens), hover_card (no trigger), alert_dialog (reimplements Dialog), collapse_toggle (wrong radius/color tokens)

### Batch 010 Temporal (8 remaining — all need significant work)
calendar (no nav buttons, no grid roles, hardcoded), range_calendar, date_picker (emoji indicator, no overlay positioning), date_range_picker, date_time_picker (**trigger only — no overlay**), date_time_range_picker (**trigger only — no overlay**), zoned_date_time_picker (severely reduced spec), time_zone_select (severely reduced spec, 3 hardcoded options)

### Batch 011 Composites (41+ remaining)
Most existing composites are in good shape. The 17 new composites are all minimal stubs needing real implementation. See gap report for details per component.

## Key Structural Issues Still Outstanding

1. **Orphaned Rust specs** (no contract): badge.rs, banner.rs, call_out.rs (should be callout.rs), autonomous_list.rs, form_shell.rs, inline_remediation.rs, remediation_banner.rs, shell_status_bar.rs, state_tile.rs, validation_summary.rs — verify with user if these should be deleted
2. **Missing Rust specs**: spacer (no SpacerSpec), status_bar (uses ShellStatusBarSpec from composites instead of foundation spec)
3. **Naming mismatch**: call_out.rs should be callout.rs to match contract
4. **surface_elevation**: Contract exists, nothing implemented anywhere
5. **pagination_summary**: Rust spec in composites, GPUI in primitives, contract in foundation — misaligned location

## File Locations

- Contracts: `docs/contracts/foundation/*.md` and `docs/contracts/composites/*.md`
- Rust specs: `packages/contracts/primitives/src/*.rs` and `packages/contracts/composites/src/*.rs`
- GPUI components: `packages/gpui/components/src/primitives/*.rs` and `packages/gpui/components/src/composites/*.rs`
- GPUI specimens: `packages/gpui/preview/src/specimens/*.rs`
- Svelte reference: `packages/svelte/primitives/src/*.svelte` and `packages/svelte/composites/src/*.svelte`
- Gap report: `docs/roadmaps/g11/gap-report.md`
- Milestone files: `docs/roadmaps/g11/003-inputs-batch.md` through `013-generation-closeout.md`

## Workflow Per Component

1. Read the contract (`docs/contracts/foundation/<component>.md`)
2. Read the current GPUI implementation
3. Check the gap report entry for known issues
4. Apply the fix pattern (focus ring, disabled cursor, correct tokens, correct sizes)
5. For major rewrites, read the Svelte implementation as visual reference
6. `cargo check -p poodle-gpui-components` after each fix
7. Commit per-batch or per-component with descriptive messages
8. Update milestone file status when batch is done

## Compile Command

```bash
cd /Users/betterthanclay/Dev/projects/poodle/packages/gpui/preview && cargo check
```

This checks the full GPUI stack (components + preview).
