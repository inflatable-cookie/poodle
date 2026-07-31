# SelectionSummary

Status: detailed contract
Updated: 2026-07-15

## 1. Purpose

- Component name: `SelectionSummary`
- Layer: `composites`
- Summary: a compact summary of the current selection with removable chips,
  overflow count, and clear-all affordance
- In scope: removable chip buttons, inline clear link, empty placeholder,
  overflow truncation, size and density scaling
- Out of scope: candidate browsing, confirm/cancel workflow,
  pagination-aware selection semantics

## 2. Anatomy

```text
[Root <section>]  aria-label="Current selection"
  └── [Chips .selection-summary__chips]
        ├── [Empty .selection-summary__empty]  <span> (when items.length === 0)
        ├── [Chip <button>...]                 (remove-only mode: onActivate is null)
        │     ├── [ChipLabel]
        │     └── [RemoveIcon]  Icon "x", aria-hidden
        ├── [Chip .selection-summary__chip--split <span>]  (split mode: onActivate set)
        │     ├── [Activate .selection-summary__chip-activate]  <button> containing label
        │     └── [Remove]  IconButton (icon "x", size xs, ghost)
        ├── [Overflow <span>]                  (when items.length > maxVisibleItems)
        └── [ClearLink .selection-summary__clear]  TextLink (when items.length > 0, pushed right via margin-left: auto)
```

The chip renders in one of two modes. Default (remove-only) keeps the whole chip
as a single remove button. When `onActivate` is provided the chip splits into two
independent sibling controls — an activation button (the label) and a remove
IconButton — so consumers like `FilterBuilder` can edit and remove a pill
separately. The two modes never nest interactive elements.

### Parts

| Part | Element | Notes |
|------|---------|-------|
| Root | `<section>` | Class `selection-summary`, `aria-label="Current selection"`, `data-size`, `data-density` |
| Chips | `<div>` | Flex-wrap container; always rendered with `min-height` for reserved space |
| Empty | `<span>` | "No selection" italic placeholder text; shown when `items` is empty |
| Chip | `<button>` | Shows item label with remove (x) icon; `aria-label="Remove {item.label}"` |
| RemoveIcon | `<span>` | Wraps `Icon` with `name="x"`, `aria-hidden="true"` |
| Overflow | `<span>` | "+N more" text for truncated items |
| ClearLink | `TextLink` | Inline "Clear" link-style action; pushed right via `margin-left: auto` |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ id: string; label: string }>` | `[]` | no | Selected items to display |
| `maxVisibleItems` | `number` | `4` | no | Max chips shown before overflow |
| `size` | `ControlSize \| null` | `null` | no | Explicit absolute size override |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic size intent for presentation context resolution |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override |
| `onActivate` | `((id: string) => void) \| null` | `null` | no | When set, each chip splits into a separate activation button (the label) and a remove button (no nested buttons). When `null`, the whole chip is the remove button (default) |
| `onRemove` | `((id: string) => void) \| null` | `null` | no | called when an item is removed from the selection |
| `onClear` | `(() => void) \| null` | `null` | no | called when all items are cleared |

### Slots

None.

### Controlled And Uncontrolled

Display composite. Items list is externally driven. No internal state.
`visibleItems` and `overflowCount` are derived reactively from `items` and
`maxVisibleItems`.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | `items` is empty | Chips container shows "No selection" italic placeholder; no chips, no clear link |
| populated | `items` has entries | Chips rendered with inline "Clear" link pushed right |
| truncated | `items.length > maxVisibleItems` | Only first `maxVisibleItems` chips shown, overflow badge shows "+N more" |

The component always renders (not conditionally hidden when empty). The chips container maintains `min-height` for reserved vertical space regardless of selection state.

No internal component state.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|----------|--------------|---------|-------|
| `onActivate` | chip activation button clicked (split mode) | `string` | host activates/edits the item |
| `onRemove` | chip remove button clicked | `string` | host removes the item from selection |
| `onClear` | clear link clicked | none | host clears all selections |

## 6. Accessibility

- Root is `<section>` with `aria-label="Current selection"`
- Remove-only mode: each chip button has `aria-label="Remove {item.label}"`;
  remove icon (x) is `aria-hidden="true"`
- Split mode (`onActivate` set): the activation button has
  `aria-label="Edit {item.label}"` and the separate remove IconButton has
  `aria-label="Remove {item.label}"` — two independent, non-nested controls
- `Tab` navigates between chip controls and the inline clear link
- `Enter` / `Space` activates the focused button (activate, remove, or clear)
- Focus entry: first chip control or clear link

## 7. Layout

### Sizing

- Root fills available width
- Chips container uses `flex-wrap` with density-aware gap
- Chips container has a size-specific reserved `min-height`
- Chip internal gap follows the same density-aware gap token
- Chip min-height, inline padding, overflow badge line-height, and remove icon
  size all scale with the component's explicit or inherited `size`

### Composition

- Parent expectations: PickerShell selection slot, standalone selection displays
- Child expectations: none (self-contained)
- Resizing rules: chips wrap to multiple rows

## 8. Token Usage

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | Root | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | Root | `"compact"`, `"default"`, `"comfortable"` |

### `.selection-summary` (Root)

| Property | Value |
|----------|-------|
| `font-size` | `var(--poodle-typography-label-size, 0.75rem)` |
| `padding-bottom` | `0.625rem` |

### `.selection-summary__chips`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `gap` | `var(--poodle-selection-summary-gap)` |
| `min-height` | `var(--poodle-selection-summary-chips-min-height)` |

### `.selection-summary__empty`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-tertiary)` |
| `font-style` | `italic` |

### `.selection-summary__chip`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `gap` | `var(--poodle-selection-summary-gap)` |
| `align-items` | `center` |
| `min-height` | `var(--poodle-selection-summary-chip-min-height)` |
| `padding` | `0 var(--poodle-selection-summary-chip-padding-x)` |
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-surface, var(--poodle-color-background-surface)) 60%, var(--poodle-color-background-elevated))` |
| `box-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 70%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |

### `.selection-summary__clear`

| Property | Value |
|----------|-------|
| `margin-left` | `auto` |
| `font-size` | `var(--poodle-selection-summary-clear-font-size)` |

### `.selection-summary__clear:hover`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

### `.selection-summary__overflow`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `var(--poodle-selection-summary-overflow-font-size)` |
| `line-height` | `var(--poodle-selection-summary-overflow-line-height)` |
| `padding` | `0 var(--poodle-selection-summary-overflow-padding-x)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-surface, var(--poodle-color-background-surface)) 68%, var(--poodle-color-background-elevated))` |
| `box-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 60%, transparent)` |

### Size Variants

#### `[data-size="xs"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.6875rem` |
| Chips | `min-height` | `0.875rem` |
| Chip | `min-height` | `1rem` |
| Chip | `padding` | `0 0.5rem` |
| Chip | `font-size` | `0.6875rem` |

#### `[data-size="sm"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.71875rem` |
| Chips | `min-height` | `1rem` |
| Chip | `min-height` | `1.125rem` |
| Chip | `font-size` | `0.71875rem` |

#### `[data-size="md"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.75rem` |
| Chips | `min-height` | `1.25rem` |
| Chip | `min-height` | `1.5rem` |
| Chip | `padding` | `0 0.75rem` |
| Chip | `font-size` | `0.75rem` |

#### `[data-size="lg"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.8125rem` |
| Chips | `min-height` | `1.5rem` |
| Chip | `min-height` | `1.75rem` |
| Chip | `padding` | `0 0.875rem` |
| Chip | `font-size` | `0.8125rem` |

#### `[data-size="xl"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.875rem` |
| Chips | `min-height` | `1.75rem` |
| Chip | `min-height` | `2rem` |
| Chip | `padding` | `0 1rem` |
| Chip | `font-size` | `0.875rem` |

### Density Variants

| Density | Gap | Bottom padding | Chip X padding | Overflow X padding |
|---------|-----|----------------|----------------|--------------------|
| `compact` | `0.375rem` | `0.5rem` | `0.625rem` | `0.5rem` |
| `default` | `var(--poodle-space-inline-sm)` | `0.625rem` | `0.75rem` | `0.625rem` |
| `comfortable` | `var(--poodle-space-inline-md)` | `0.75rem` | `0.875rem` | `0.75rem` |

## 9. Svelte Notes

- `data-size` resolves via `resolveSemanticControlSize` from presentation context
- `data-density` resolves via presentation context with explicit override
- Uses callback props for remove and clear actions
- `visibleItems` and `overflowCount` derived reactively from `items` and `maxVisibleItems`
- Uses `Icon` primitive with `name="x"` for the remove icon
- Uses `TextLink` for the trailing clear action
- Imports `getUiPresentation` and `resolveSemanticControlSize` from `@poodle/svelte`

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::selection_summary`
- Render as section with header row and chip container
- Size/density scaling must match CSS variant tables

## 10a. Jetstream Notes

- `SelectionSummary::from_spec(spec, theme).on_remove(...).on_clear(...)`.
  `on_remove` carries the removed item's id.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] maxVisibleItems truncation behavior matches
- [ ] empty state shows "No selection" italic placeholder
- [ ] clear link appears inline with chips, pushed right

### Tier 2: Visual Parity

- [ ] chip styling matches
- [ ] overflow badge styling matches
- [ ] light theme border and box-shadow treatment matches
- [ ] size variant scaling matches all 5 sizes
- [ ] density variant padding and gap match

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Specimen Definitions

### Multiple Items Selected

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Multiple items selected | Five items (Button, Card, Dialog, Table, Tabs), remove and clear handlers wired | Chips with inline "Clear" link pushed right |

### Empty State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty state | `items=[]` | Reserved-height container showing "No selection" italic placeholder |

### Truncated (Max 3 Visible)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Truncated (max 3 visible) | Six items (Alpha through Zeta), `maxVisibleItems={3}` | Three visible chips with "+3 more" overflow badge and inline "Clear" link |
