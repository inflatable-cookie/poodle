# g10.017 GPUI Formula Sweep — Second Pass

Status: complete
Owner: Poodle core
Depends on: g10.016
Updated: 2026-04-17

## Purpose

Continuation of the systematic audit replacing hardcoded `px(N.0)` literals
with `rem_to_px` formula expressions and token lookups across the remaining
GPUI component files.

---

## Batch 1 — Complex component dimension fixes

**alert_dialog.rs**
- `.w(px(420.0))` → `.w(px(rem_to_px(26.25)))` (Svelte: 26.25rem)

**calendar.rs**
- `calendar_width = px(288.0)` → `px(rem_to_px(18.0))` (18rem comment updated)
- Header row gap `px(2.0)` → `px(rem_to_px(0.125))` (×2, replace_all)

**code.rs**
- Default max-height `px(320.0)` → `px(rem_to_px(20.0))` (Svelte: 20rem)

**collapse_toggle.rs**
- Added `rem_to_px` import
- Icon padding `px(2.0)` → `px(rem_to_px(0.125))` (Svelte: 0.125rem)

**collapsible.rs**
- Open gap `px(8.0)` → `resolve_px(theme, "space.inline.sm")`

**color_picker.rs**
- Swatch size `px(20.0)` → `px(rem_to_px(1.25))` (contract: 1.25rem)
- Swatch radius `px(3.0)` → `px(rem_to_px(0.1875))` (contract: 0.1875rem)
- Overlay width `px(384.0)` → `px(rem_to_px(24.0))` (contract: 24rem)
- Hex input height `px(28.0)` → `px(rem_to_px(1.75))` (contract: 1.75rem)

**spinner.rs**
- Added `rem_to_px` import
- Grid variant per-size cell/gap table: all values converted to `rem_to_px` formulas
  - Xs: (0.125rem, 0.0625rem), Sm: (0.15625rem, 0.078125rem),
    Md: (0.203125rem, 0.09375rem), Lg: (0.28125rem, 0.125rem),
    Xl: (0.375rem, 0.15625rem)
- Grid cell border-radius `px(2.0)` → `px(rem_to_px(0.125))`

---

## Batch 2 — Small component typography and spacing

**meta_bar.rs**
- Added `rem_to_px` and `resolve_px` imports
- Row gap `px(8.0)` → `resolve_px(theme, "space.inline.sm")`
- Separator dot size `px(4.0)` → `px(rem_to_px(0.25))` each axis

**meta_item.rs**
- Added `rem_to_px` import
- Item gap `px(6.0)` → `px(rem_to_px(0.375))` (×2, all occurrences)
- Label text size `px(11.0)` → `px(rem_to_px(0.6875))`
- Value text size `px(14.0)` → `px(rem_to_px(0.875))` (×2)

**time_ago.rs**
- Added `rem_to_px` import
- Text size `px(12.0)` → `px(rem_to_px(0.75))`

**status_bar.rs**
- Added `rem_to_px` and `resolve_px` imports
- Bar height `px(24.0)` → `px(rem_to_px(1.5))` (Svelte: 1.5rem)
- Bar padding-x `px(8.0)` → `resolve_px(theme, "space.inline.sm")`
- Section gaps `px(6.0)` → `px(rem_to_px(0.375))` (×2)
- Summary text size `px(12.0)` → `px(rem_to_px(0.75))`

**nav_card.rs**
- Added `rem_to_px` import
- Icon slot size `px(32.0)` → `px(rem_to_px(2.0))` each axis
- Icon slot radius `px(8.0)` → `px(rem_to_px(0.5))`
- Title row gap `px(8.0)` → `px(rem_to_px(0.5))`
- Badge px `px(8.0)` → `px(rem_to_px(0.5))`
- Badge py `px(2.0)` → `px(rem_to_px(0.125))`
- Badge text size `px(11.0)` → `px(rem_to_px(0.6875))`
- Content gap `px(4.0)` → `px(rem_to_px(0.25))`
- Root gap `px(12.0)` → `px(rem_to_px(0.75))`
- Root padding-x `px(16.0)` → `px(rem_to_px(1.0))`
- Root padding-y `px(14.0)` → `px(rem_to_px(0.875))`

**table.rs**
- Added `rem_to_px` import
- Cell vertical padding `px(11.0)` → `px(rem_to_px(0.6875))`
- Cell horizontal padding `px(14.0)` → `px(rem_to_px(0.875))`
- Caption padding-y `px(8.0)` → `px(rem_to_px(0.5))`
- Caption text size `px(12.0)` → `px(rem_to_px(0.75))`
- Header text size `px(11.0)` → `px(rem_to_px(0.6875))`
- Empty state padding-y `px(32.0)` → `px(rem_to_px(2.0))`

**detail_item.rs**
- Added `rem_to_px` import
- Inline label width `px(180.0)` → `px(rem_to_px(11.25))` (Svelte: 11.25rem)

---

## Execution checklist

**Batch 1:**
- [x] alert_dialog: dialog card width formula
- [x] calendar: container width formula, header/day row gaps formula
- [x] code: max-height formula
- [x] collapse_toggle: icon padding formula
- [x] collapsible: open gap token
- [x] color_picker: swatch size/radius, overlay width, hex input height
- [x] spinner: grid cell/gap table, cell radius

**Batch 2:**
- [x] meta_bar: row gap token, separator dot formula
- [x] meta_item: gaps, label/value text sizes
- [x] time_ago: text size formula
- [x] status_bar: height, padding, gaps, text size
- [x] nav_card: icon slot, badge, title row, content, root geometry
- [x] table: cell padding, caption/header text sizes, empty state padding
- [x] detail_item: inline label width formula

## Next task

Continue audit — skeleton.rs preset bone dimensions, resize_handle.rs affordance
geometry, and any remaining components with unresolved hardcoded values.
