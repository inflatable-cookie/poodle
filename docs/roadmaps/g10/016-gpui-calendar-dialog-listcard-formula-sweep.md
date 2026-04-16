# g10.016 GPUI Calendar, Dialog, ListCard, and Formula Sweep

Status: complete
Owner: Poodle core
Depends on: g10.015
Updated: 2026-04-17

## Purpose

Targeted fix pass continuing the April 2026 audit sweep. Closes confirmed
visual divergences in Calendar cell geometry, Dialog close-button chrome,
ListCard sash anatomy, and residual magic-literal values in Select and
Pagination.

---

## 1. Calendar — cell geometry and padding

**File:** `packages/gpui/components/src/primitives/calendar.rs`

- **Cell size** `px(32.0)` → `px(rem_to_px(2.25))` (Svelte: 2.25rem = 36px)
- **Nav button size** `px(28.0)` → `px(rem_to_px(2.0))` (Svelte: 2rem = 32px)
- **Grid padding** `p(px(12.0))` → `p(px(rem_to_px(0.75)))` (Svelte: 0.75rem)
- **Weekday header row height** `px(24.0)` → `px(rem_to_px(1.5))`
- Width comment updated: 7 × 36px + 6 × 2px (gap) + 2 × 12px (padding) = 288px ✓

---

## 2. Dialog — close button

**File:** `packages/gpui/components/src/primitives/dialog.rs`

Previous close button was a Unicode "×" literal in a fixed 24 × 24 div with
`rounded(radius)` (surface radius — too large) and no theme-aware hover.

- Replaced with `Icon::from_spec(IconSpec::new("x").with_size(IconSize::Sm), theme)`
- Dimension: `control_height_rem(chrome_size)` where `chrome_size =
  resolve_semantic_size(effective_size, SemanticControlSizeRole::Chrome)` — one
  stop smaller than the dialog effective_size (matches Svelte `size="chrome"`)
- Radius: `radius.control` instead of `radius.surface`
- Hover: `Hsla { a: 0.08, ..text_secondary }` — theme-aware, light/dark correct

---

## 3. ListCard — sash anatomy and root geometry

**File:** `packages/gpui/components/src/primitives/list_card.rs`

Sash element corrected to match Svelte:

- Removed `.px(px(6.0))` — Svelte: `padding: 0.125rem 0` (px = 0)
- Removed `.rounded_bl(px(4.0))` — no border-radius on Svelte sash
- Added `.w(px(rem_to_px(6.0)))` — Svelte: `width: 6rem`
- `text_size(px(9.0))` → `px(rem_to_px(0.5625))` (Svelte: 0.5625rem)
- `line_height(px(12.0))` → `px(rem_to_px(0.75))` (Svelte: 0.75rem)

Root card geometry:

- `px(px(12.0))` → `resolve_px(theme, "space.inline.md")`
- `py(px(10.0))` → `px(rem_to_px(0.625))` (Svelte: 0.625rem)
- `gap(px(12.0))` → `resolve_px(theme, "space.inline.md")`

Footer margin: `mt(px(4.0))` → `mt(resolve_px(theme, "space.inline.xs"))`

---

## 4. Select — group separator and label

**File:** `packages/gpui/components/src/primitives/select.rs`

- Group separator vertical margin `my(px(4.0))` → `resolve_px(theme, "space.inline.xs")`
- Group label bottom padding `pb(px(2.0))` → `pb(px(rem_to_px(0.125)))`

---

## 5. Pagination — button height and dimensions

**File:** `packages/gpui/components/src/primitives/pagination.rs`

- Button height subtraction `-px(2.0)` → `-px(rem_to_px(0.125))` (contract: -0.125rem)
- Nav/page button min-width `px(36.0)` → `px(rem_to_px(2.25))` (contract: 2.25rem) ×2
- Ellipsis min-width `px(24.0)` → `px(rem_to_px(1.5))` (contract: 1.5rem)
- Standalone panel padding: all four values (compact/normal × x/y) converted to
  `resolve_px(theme, "space.inline.{xs|sm|md}")` token lookups

---

## Execution checklist

- [x] Calendar cell size 32 → 36px
- [x] Calendar nav button size 28 → 32px
- [x] Calendar padding formula
- [x] Calendar weekday header height formula
- [x] Dialog close button: Icon component, chrome-sized, control_radius, theme hover
- [x] ListCard sash: remove px, remove rounded_bl, add width: 6rem, formula font/line-height
- [x] ListCard root: px/py/gap → token/formula
- [x] ListCard footer margin → space.inline.xs token
- [x] Select group separator margin → space.inline.xs
- [x] Select group label pb → rem formula
- [x] Pagination button height subtraction → rem formula
- [x] Pagination nav/page min-width → rem formula ×2
- [x] Pagination ellipsis min-width → rem formula
- [x] Pagination standalone panel padding → token lookups

## Next task

Continue audit sweep — badge, avatar, notification, data_table geometry, or
another category of remaining hardcoded values.
