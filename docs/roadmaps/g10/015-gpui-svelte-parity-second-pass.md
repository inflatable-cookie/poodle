# g10.015 GPUI Svelte Parity Second Pass

Status: active
Owner: Poodle core
Depends on: g10.014
Updated: 2026-04-17

## Purpose

A targeted fix pass following the April 2026 audit sweep. g10.014 closed the
architectural gaps (overlay, keyboard, events). This milestone closes the
remaining confirmed visual divergences between GPUI components and the Svelte
reference — all self-contained token or formula corrections.

---

## 1. Callout — per-size geometry

**File:** `packages/gpui/components/src/primitives/callout.rs`

Four hardcoded values replaced with per-size helpers:

- **Outer gap** (`gap(inline_md)` fixed for all sizes): Svelte overrides gap
  per size — xs: 0.375rem, sm: 0.5rem, md: 0.75rem (base), lg: 0.875rem,
  xl: 1.0rem. Added `callout_gap_rem(size)` to `presentation.rs`.

- **Icon container size** (`px(22.0)` fixed for all sizes): Svelte per-size
  icon container — xs: 0.875rem, sm: 1.125rem, md: 1.375rem, lg: 1.75rem,
  xl: 2.0rem. Added `callout_icon_size_rem(size)`.

- **Content column gap** (`px(4.0)` hardcoded): Svelte uses
  `var(--poodle-space-inline-sm)`. Now resolved via token.

- **Dismiss button size** (`px(28.0)` fixed for all sizes): Svelte per-size —
  xs: 1.25rem, sm: 1.5rem, md: 1.75rem, lg: 2.0rem, xl: 2.25rem. Added
  `callout_dismiss_size_rem(size)`.

- **Dismiss hover color** (`hsla(0.0, 0.0, 0.5, 0.08)` neutral gray): Replaced
  with `Hsla { a: 0.08, ..text_secondary }` — theme-aware, responds to dark/
  light mode.

---

## 2. Button — chevron margin formula

**File:** `packages/gpui/components/src/primitives/button.rs`

`ml(px(-2.0))` → `ml(px(-theme.resolve_space("space.inline.sm") * 0.25))`

Svelte: `margin-left: calc(var(--poodle-space-inline-sm) * -0.25)`. Same
numeric result at current token values (0.5rem × 0.25 × 16 = 2px) but now
tracks token changes automatically.

---

## 3. TextInput — char count font size

**File:** `packages/gpui/components/src/primitives/text_input.rs`

`text_size(px(11.0))` → `text_size(px(rem_to_px(0.6875)))`

Svelte: `font-size: 0.6875rem`. Same numeric result (11px at 16px base) but
the value is now expressed as its rem origin, matching the formula pattern
used across all other font sizes.

---

## 4. Menu — item radius formula

**File:** `packages/gpui/components/src/primitives/menu.rs`

`control_radius - px(2.0)` → `control_radius - px(rem_to_px(0.125))`

Contract comment already read "control - 0.125rem". The subtracted value is
now the formula evaluation of 0.125rem (= 2px at 16px base) rather than a
magic literal.

---

## Execution checklist

- [x] Add `callout_gap_rem`, `callout_icon_size_rem`, `callout_dismiss_size_rem`
      to `presentation.rs`
- [x] Callout outer gap per-size
- [x] Callout icon container size per-size
- [x] Callout content column gap → `space.inline.sm` token
- [x] Callout dismiss button size per-size
- [x] Callout dismiss hover → theme-aware `text_secondary` at 8% opacity
- [x] Button: chevron margin-left resolved from `space.inline.sm * -0.25`
- [x] TextInput: char count font size expressed as `rem_to_px(0.6875)`
- [x] Menu: item radius expressed as `control_radius - rem_to_px(0.125)`

## Next task

Continue audit sweep — likely Pill hardcoded size table, RadioGroup geometry
vs Svelte, or further inspection of component vertical padding consistency.
