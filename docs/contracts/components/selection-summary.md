# SelectionSummary

Status: detailed contract
Updated: 2026-03-30

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
        ├── [Chip <button>...]                 (when items.length > 0)
        │     ├── [ChipLabel]
        │     └── [RemoveIcon]  Icon "x", aria-hidden
        ├── [Overflow <span>]                  (when items.length > maxVisibleItems)
        └── [ClearLink .selection-summary__clear]  <button> (when items.length > 0, pushed right via margin-left: auto)
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| Root | `<section>` | Class `selection-summary`, `aria-label="Current selection"`, `data-size`, `data-density` |
| Chips | `<div>` | Flex-wrap container; always rendered with `min-height` for reserved space |
| Empty | `<span>` | "No selection" italic placeholder text; shown when `items` is empty |
| Chip | `<button>` | Shows item label with remove (x) icon; `aria-label="Remove {item.label}"` |
| RemoveIcon | `<span>` | Wraps `Icon` with `name="x"`, `aria-hidden="true"` |
| Overflow | `<span>` | "+N more" text for truncated items |
| ClearLink | `<button>` | Inline "Clear" link-style button; pushed right via `margin-left: auto` |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ id: string; label: string }>` | `[]` | no | Selected items to display |
| `maxVisibleItems` | `number` | `4` | no | Max chips shown before overflow |
| `size` | `ControlSize \| null` | `null` | no | Explicit absolute size override |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic size intent for presentation context resolution |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override |

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

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `remove` | Chip remove button clicked | `{ id: string }` | Host removes the item from selection |
| `clear` | Clear button clicked | `void` | Host clears all selections |

## 6. Accessibility

- Root is `<section>` with `aria-label="Current selection"`
- Each chip button has `aria-label="Remove {item.label}"`
- Remove icon (x) is `aria-hidden="true"`
- `Tab` navigates between chip buttons and the inline clear link
- `Enter` / `Space` activates focused button (clear or remove)
- Focus entry: first chip or clear link

## 7. Layout

### Sizing

- Root fills available width
- Chips container uses `flex-wrap` with `gap: var(--poodle-space-inline-sm)`
- Chips container has `min-height: calc(var(--poodle-size-control-height) - 0.5rem)` for reserved space
- Chip internal gap: `var(--poodle-space-inline-md)`
- Chip min-height: `calc(var(--poodle-size-control-height) - 0.25rem)`
- Chip inline padding: `0 0.75rem`
- Overflow badge line-height: `2rem`, padding `0 0.625rem`

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
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-height` | `calc(var(--poodle-size-control-height) - 0.5rem)` |

### `.selection-summary__empty`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-tertiary)` |
| `font-style` | `italic` |

### `.selection-summary__chip`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `gap` | `var(--poodle-space-inline-md)` |
| `align-items` | `center` |
| `min-height` | `calc(var(--poodle-size-control-height) - 0.25rem)` |
| `padding` | `0 0.75rem` |
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |

### `.selection-summary__clear`

| Property | Value |
|----------|-------|
| `margin-left` | `auto` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `font-size` | `var(--poodle-typography-label-size, 0.75rem)` |

### `.selection-summary__clear:hover`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

### `.selection-summary__overflow`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `2rem` |
| `padding` | `0 0.625rem` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 58%, transparent)` |

### Size Variants

#### `[data-size="xs"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.6875rem` |
| Chip | `min-height` | `1rem` |
| Chip | `padding` | `0 0.5rem` |
| Chip | `font-size` | `0.6875rem` |

#### `[data-size="sm"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.71875rem` |
| Chip | `min-height` | `1.125rem` |
| Chip | `font-size` | `0.71875rem` |

#### `[data-size="lg"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.8125rem` |
| Chip | `min-height` | `1.75rem` |
| Chip | `padding` | `0 0.875rem` |
| Chip | `font-size` | `0.8125rem` |

#### `[data-size="xl"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.875rem` |
| Chip | `min-height` | `2rem` |
| Chip | `padding` | `0 1rem` |
| Chip | `font-size` | `0.875rem` |

No density variants or light theme overrides in the current implementation.

## 9. Svelte Notes

- `data-size` resolves via `resolveSemanticControlSize` from presentation context
- `data-density` resolves via presentation context with explicit override
- Uses `createEventDispatcher` for `remove` and `clear` events
- `visibleItems` and `overflowCount` derived reactively from `items` and `maxVisibleItems`
- Uses `Icon` primitive with `name="x"` for the remove icon
- Imports `getUiPresentation` and `resolveSemanticControlSize` from `@poodle/svelte`

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::selection_summary`
- Render as section with header row and chip container
- Size/density scaling must match CSS variant tables

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
