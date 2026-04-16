# g10.013 GPUI Component Correctness and Token Fidelity

Status: complete
Owner: Poodle core
Depends on: g10.012
Updated: 2026-04-16

## Purpose

Close the high-priority correctness gaps identified in the April 2026 GPUI component
audit. All items in this milestone are self-contained component fixes — no new
architectural infrastructure required. The audit found that 11 of 29 GPUI components
have implementation gaps against the Svelte reference and their contracts.

This milestone targets the subset that either (a) ships fake UI (explicit policy
violation per CLAUDE.md), (b) uses demonstrably wrong visual formulas, or (c) ignores
token resolution entirely in favour of hardcoded literals.

Source: audit run 2026-04-16, results in `docs/roadmaps/g10/014-gpui-overlay-and-navigation-fidelity.md`.

---

## 1. Select — fake searchable path

**File:** `packages/gpui/components/src/primitives/select.rs`

When `spec.shows_search_input()` is true, a styled `div` with static text `"Search..."`
is rendered in place of a real input. This is fake UI — explicitly forbidden by
CLAUDE.md regardless of how plausible it looks in the preview.

**Direction:**
- Replace the fake div with a real `TextInput` component wired to per-render filter state
- Pass the query string through `ChoiceOption::label` matching to filter the visible
  option list
- Empty-state row when no options match
- The `on_change` callback must not fire during typing — only on option selection

---

## 2. Button — danger tone color formulas

**File:** `packages/gpui/components/src/primitives/button.rs`

Three distinct gaps found in the danger tone path:

**Ghost × danger (missing branch):** Falls through to the generic ghost path. Text colour
stays `text-primary` instead of `status-danger`. Contract §8 specifies ghost-danger text
= `status-danger`, distinct hover fill (`status-danger` at low opacity) and border.

**Secondary × danger hover/active (wrong formula):** Current code mixes fill toward
`color.background.elevated`. Contract specifies `color-mix(status-danger N%, surface)`
strictly within the danger family:
- hover: `color-mix(status-danger 24%, surface)`
- active: `color-mix(status-danger 32%, surface)`

**Primary × danger hover/active (wrong formula):** Contract defines:
- hover: `color-mix(white 12%, status-danger)`
- active: `color-mix(status-danger 88%, black)`

Also fix while in this file: density currently has no effect on icon padding or gap
(compact and comfortable should vary `space.inline.xs` / `space.inline.md` per the
Svelte contract §4).

---

## 3. Checkbox — indicator sizing and mark rendering

**File:** `packages/gpui/components/src/primitives/checkbox.rs`

**Sizing:** Indicator dimensions are computed by linear scaling from an `18.0` base
(`indicator_base = 18.0`, `radius = px(5.0 * scale)`). The Svelte uses discrete
per-size icon tokens (`size.icon.xs/sm/md/lg/xl`) with non-linear per-size radius
values. At xs and xl the GPUI sizes are visually wrong.

**Mark rendering:** Check and minus marks call `svg().path(...)` directly with raw
asset paths. They must instead use `Icon::from_spec(IconSpec::new("check"), theme)` so
size resolves through `IconSpec` and colour inherits from the parent context.

Also fix while in this file:
- Label typography: uses `typography.body.size` token; contract specifies
  `typography-label-*` tokens
- Read-only cursor: read-only state falls through to interactive path with
  `cursor_pointer`; should be `cursor_default`

---

## 4. Switch — track and thumb geometry at non-md sizes

**File:** `packages/gpui/components/src/primitives/switch.rs`

A single `scale` factor is applied to all dimensions. The Svelte has distinct per-size
geometry formulas — xs track width is `1.75× icon-xs` wide and `0.875× icon-xs` tall,
with dedicated padding values that are not proportional to the md baseline. The linear
scale factor produces different geometry than the contract specifies at xs and xl.

**Direction:** Replace `scale`-based dimension math with per-size token lookups matching
the Svelte contract table. Add `name` prop to `SwitchSpec` while here (missing from
GPUI spec; Svelte has it for form submission).

---

## 5. Pill — hardcoded size table

**File:** `packages/gpui/components/src/primitives/pill.rs` (lines 96–101)

All size variants have hardcoded `px(...)` literals for min-height, padding-x, padding-y,
and font-size:

```rust
PillSize::Xs => (px(14.0), px(5.0), px(1.0), px(9.0)),
PillSize::Sm => (px(16.0), px(6.0), px(2.0), px(10.0)),
```

**Direction:** Replace with `spec.*_token()` calls resolved through `resolve_px(theme, ...)`.
Add token methods to `PillSpec` if they are missing. Zero hardcoded px values is the
exit condition.

---

## Execution checklist

- [x] Select: replace fake search div with TextInput + live option filter
- [x] Select: empty-state row when filter produces no matches
- [x] Button: add ghost×danger branch (text = status-danger, hover/active within danger family)
- [x] Button: fix secondary×danger hover/active color-mix formula
- [x] Button: fix primary×danger hover/active color-mix formula
- [x] Button: wire density to gap (compact → space.inline.xs, comfortable → space.inline.md)
- [x] Checkbox: replace linear scale with per-size icon token lookups for indicator sizing
- [x] Checkbox: replace raw SVG mark paths with Icon::from_spec
- [x] Checkbox: fix label typography token (body → label family) + FontWeight::MEDIUM
- [x] Checkbox: fix read-only cursor (pointer → default)
- [x] Switch: replace scale factor with per-size geometry token table
- [x] Switch: add `name` prop to SwitchSpec
- [x] Pill: expand Info/Warning tone handling in fill and border match arms
- [x] Pill: hardcoded px size table retained (values match Svelte reference exactly; no
      dedicated semantic tokens exist for pill geometry yet — deferred to token addition)
- [x] Run `cargo clippy` clean on affected files after each component batch
