# PinInput

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `PinInput`
- Layer: `foundation`
- Summary: a fixed-length code-entry control split across multiple visible
  cells, with auto-advance, masking support, and completion signaling
- In scope: fixed-length digit entry, per-cell focus management, auto-advance
  on input, backspace navigation, completion callback, optional masking
- Out of scope: arbitrary text entry, secret-management workflows, variable-
  length token entry

## 2. Anatomy

```text
[Root .pin-input]  <div role="group">
  └── [Cell .pin-input__cell]...  <input type="text"|"password">  (repeated `length` times)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | grouping container | inline-flex layout, gap |
| Cell | yes | single-character input | border, radius, background, typography, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled string value (one char per cell) |
| `defaultValue` | `string` | `""` | no | uncontrolled initial value |
| `length` | `number` | `6` | no | number of cells to render |
| `disabled` | `boolean` | `false` | no | disables all cells |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the group |
| `mask` | `boolean` | `false` | no | when true, cells use `type="password"` to obscure input |

### Controlled And Uncontrolled

- controlled: `value` plus `valueChange` event
- uncontrolled: `defaultValue`
- value string length is clamped to `length` prop

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no value entered | all cells show empty with border |
| partially filled | some cells have values | filled cells show characters (or dots if masked) |
| complete | all cells filled | all cells populated, complete event fires |
| focus | cell receives focus | focus ring on the active cell |
| disabled | `disabled=true` | all cells non-interactive, reduced opacity |

### Component States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| auto-advance | digit entered in cell | focus moves to next empty cell |
| backspace-retreat | backspace on empty cell | focus moves to previous cell, clears it |
| complete | all `length` cells filled | `complete` event fires with full value string |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | any cell value changes | `{ value: string }` | concatenated string of all cells |
| `complete` | all cells are filled | `{ value: string }` | fires once when last cell is populated |

## 6. Accessibility

### Semantics

- Role: `role="group"` on root container with `aria-label` from prop
- Each cell: `<input>` with `aria-label="Digit {n}"` where n is 1-based position
- When `mask=true`: cells use `type="password"` for native obscuring
- When `mask=false`: cells use `type="text"`

### Keyboard

| Key | Behavior |
|-----|----------|
| digit input | fills current cell, auto-advances focus to next cell |
| `Backspace` | clears current cell; if already empty, moves focus to previous cell and clears it |
| `Arrow Left` | moves focus to previous cell |
| `Arrow Right` | moves focus to next cell |
| `Tab` | exits the pin-input group |

### Focus And Announcement

- focus entry: first empty cell (or first cell) receives visible focus ring
- focus transition: auto-advance moves focus ring to next cell seamlessly
- focus exit: focus ring clears on blur
- live-region behavior: none; completion is signaled via event, not announcement
- GPUI-native accessibility mapping notes: GPUI must expose the group role with per-cell naming, and must handle focus traversal between cells without exposing internal cell count as a tab-stop sequence

## 7. Layout

### Sizing

- Each cell has fixed width and height; no responsive resizing
- Gap between cells is fixed
- Completion does not shift layout

### Composition

- parent expectations: verification flows, 2FA entry, compact code-entry surfaces
- child expectations: none; cells are internally managed
- resizing rules: root is `inline-flex` and sizes to fit its cells

## 8. Token Usage — Exact Values

### Root `.pin-input`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `gap` | `0.375rem` |

### Cell `.pin-input__cell`

| Property | Value |
|----------|-------|
| `width` | `2.25rem` |
| `height` | `2.5rem` |
| `padding` | `0` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `1rem` |
| `line-height` | `1` |
| `text-align` | `center` |

### Cell — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Cell — disabled

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

## 9. Svelte Notes

- Renders `length` individual `<input>` elements, each accepting a single character
- `maxlength="1"` on each cell input
- Auto-advance logic: on input event, if cell has a value, move focus to the next cell
- Backspace logic: if current cell is empty, move focus to previous cell and clear it
- `mask` prop toggles `type` between `"text"` and `"password"` on all cells
- Paste handling: distributes pasted string across cells starting from the focused cell

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::pin_input`
- GPUI must implement per-cell focus traversal with auto-advance semantics
- Masking in GPUI uses platform-native password character rendering
- The group must suppress global keyboard shortcuts while any cell is focused to allow digit entry

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] fixed-length semantics match (length prop controls cell count)
- [ ] auto-advance behavior matches (digit entry moves to next cell)
- [ ] backspace-retreat behavior matches (empty cell backspace goes to previous)
- [ ] ArrowLeft/ArrowRight navigation matches
- [ ] complete event fires when all cells filled
- [ ] mask prop toggles obscured input
- [ ] group role with per-cell aria-label matches

### Tier 2: Visual Parity

- [ ] cell width (2.25rem) and height (2.5rem) match
- [ ] gap between cells (0.375rem) matches
- [ ] code-family font on cells matches
- [ ] focus ring (outline with focusRing color) matches
- [ ] disabled opacity matches

### Tier 3: Implementation Freedom

- [ ] internal cell input implementation (native input vs GPUI text field) stays internal
- [ ] paste distribution strategy is implementation-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| masking visuals may differ | platform text-entry password rendering differs | allowed | keep fixed-length behavior and completion semantics strict |
| paste handling details | clipboard API differs across platforms | allowed | ensure paste populates cells correctly on both platforms |

## 13. Specimen Definitions

### 6-digit Code

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| 6-digit code | `length={6}`, `ariaLabel="Verification code"` | Six empty cells in a row; typing auto-advances focus; displays entered code on completion |

### 4-digit Masked

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| 4-digit masked | `length={4}`, `mask`, `ariaLabel="PIN"` | Four cells with password masking; entered characters display as dots |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `length={6}`, `defaultValue="123"`, `disabled` | Six cells with first three pre-filled, reduced opacity, non-interactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: verification flows, 2FA entry, compact code-entry surfaces
- future follow-up: consider alphanumeric mode (letters + digits) if use cases arise
