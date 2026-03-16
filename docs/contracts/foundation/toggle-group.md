# Toggle Group

> **Surface elevation**: ToggleGroup is a surface consumer (72% moderate contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `ToggleGroup`
- Layer: `foundation`
- Summary: a grouped toggle surface for single-select or multi-select utility
  actions, presented as a wrapping set of pill-shaped items
- In scope: single and multiple selection modes, grouped labeling, disabled
  items, disabled group, accent-tinted selected state
- Out of scope: tab-panel navigation (see Tabs), segmented shell chrome (see
  SegmentedControl), standalone pressed-state buttons (see Toggle)

## 2. Anatomy

```text
[Root .toggle-group]  <div role="radiogroup"|role="group">
  └── [Item .toggle-group__item...]  <button role="radio"|role="button">
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | group container | gap, flex layout |
| Item | yes | one selectable toggle item | border, radius, background, color, typography, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| string[] \| null` | `null` | no | controlled selected value(s) |
| `defaultValue` | `string \| string[] \| null` | `null` | no | uncontrolled initial value(s) |
| `options` | `ToggleGroupOption[]` | none | yes | item list |
| `selectionMode` | `"single" \| "multiple"` | `"single"` | no | selection behavior |
| `isDisabled` | `boolean` | `false` | no | disables whole group |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |

### ToggleGroupOption

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `value` | `string` | yes | option value |
| `label` | `string` | yes | visible label text |
| `isDisabled` | `boolean` | no | disables individual item |
| `ariaLabel` | `string` | no | accessible name override |

### Controlled And Uncontrolled

- controlled: `value` plus `valueChange` event
- uncontrolled: `defaultValue`
- In single mode, value is `string | null`
- In multiple mode, value is `string[]`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| unselected | default | neutral item styling |
| selected | value matches item | accent-tinted background, accent-tinted border |
| focus | keyboard focus on item | focus ring |
| disabled (group) | `isDisabled=true` | all items muted, non-interactive |
| disabled (item) | option `isDisabled=true` | individual item muted, non-interactive |

### Component States

- Selected items tracked as `string` (single mode) or `string[]` (multiple mode)
- `data-selected` attribute on selected items for styling hooks

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | selection changes | `{ value: string \| string[] }` | payload type matches selectionMode |

## 6. Accessibility

### Semantics

- Single mode: `role="radiogroup"` on root, `role="radio"` with `aria-checked`
  on each item
- Multiple mode: `role="group"` on root, `role="button"` with `aria-pressed`
  on each item
- `aria-label` on root from prop
- Per-item `aria-label` from option when visible labels need override
- `disabled` attribute on disabled items

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters group (focuses first or selected item) then exits |
| `Arrow Left/Right` | moves focus between items (single mode, roving focus) |
| `Space` | toggles selection on focused item |
| `Enter` | toggles selection on focused item |

### Focus And Announcement

- focus entry: first or currently selected item receives focus
- focus exit: focus leaves group entirely
- live-region behavior: none; state changes announced through role semantics
- GPUI-native accessibility mapping notes: GPUI must expose radiogroup/group
  role and per-item checked/pressed state through the accessibility tree

## 7. Layout

### Sizing

- Items use compact pill-shaped sizing with shared control-height minus padding
- Group wraps when items exceed container width

### Composition

- parent expectations: filters, utility bars, formatting controls, toolbars
- child expectations: option items only in this baseline contract
- resizing rules: items wrap using flexbox; group remains cohesive at any width

## 8. Token Usage — Exact Values

### Root `.toggle-group`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `flex-wrap` | `wrap` |
| `gap` | `0.25rem` |

### Item `.toggle-group__item`

| Property | Value |
|----------|-------|
| `min-height` | `calc(var(--pug-size-control-height) - 0.25rem)` |
| `padding` | `0 0.75rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 82%, transparent)` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `color-mix(in srgb, var(--pug-surface) 72%, var(--pug-color-background-elevated))` |
| `color` | `var(--pug-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1` |
| `transition` | `border-color 180ms ease, background 180ms ease, color 180ms ease` |

### Item — selected (`.toggle-group__item.selected`)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 22%, transparent)` |
| `border-color` | `color-mix(in srgb, var(--pug-color-accent-base) 42%, var(--pug-color-border-default))` |

### Item — focus visible (`:focus-visible`)

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Item — disabled (`:disabled`)

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

## 9. Svelte Notes

- Uses native `<button>` elements within a group container
- Single mode: buttons use `role="radio"` and `aria-checked`
- Multiple mode: buttons use `role="button"` and `aria-pressed`
- Root container role switches between `radiogroup` and `group` based on
  `selectionMode`
- `data-selected` attribute drives selected styling
- Selection state managed internally when `value` is `null` (uncontrolled mode)
- Transition uses explicit `180ms ease` timing

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::toggle_group`
- GPUI must switch between radiogroup and group semantics based on selectionMode
- Per-item checked/pressed state must be exposed in the accessibility tree
- The accent-tinted selected state uses color-mix; GPUI must replicate the
  blending formula
- Flex-wrap layout requires GPUI equivalent for multi-row wrapping

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] single vs multiple selection mode meaning matches
- [ ] role switching (radiogroup vs group) matches
- [ ] per-item state exposure (aria-checked vs aria-pressed) matches
- [ ] disabled-item and disabled-group behavior matches
- [ ] valueChange event payload type matches selectionMode
- [ ] keyboard navigation matches

### Tier 2: Visual Parity

- [ ] item border, radius, and background match
- [ ] selected accent-tinted background matches (22% accent-base mix)
- [ ] selected accent-tinted border matches (42% accent-base mix)
- [ ] focus ring appearance matches
- [ ] disabled opacity matches
- [ ] typography matches (family, size, weight)
- [ ] flex-wrap gap matches

### Tier 3: Implementation Freedom

- [ ] roving focus implementation details are platform-owned
- [ ] transition timing is platform-owned
- [ ] data attribute naming is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Focus management details may differ | grouped utility controls may evolve roving focus | allowed | tighten during parity review |
| Transition timing (180ms ease) | GPUI may not support CSS-style transitions | allowed | match where possible |
| Flex-wrap behavior | GPUI wrapping may differ slightly from CSS flexbox | allowed | items must wrap and remain cohesive |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: filters, utility bars, formatting controls, toolbar
  groups
- future follow-up: keep `ToggleGroup` semantically distinct from
  `SegmentedControl` and `Tabs` even when the visual density is similar
