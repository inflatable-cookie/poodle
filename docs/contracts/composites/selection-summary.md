# SelectionSummary

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `SelectionSummary`
- Layer: `composites`
- Summary: a compact summary of the current selection with removable chips,
  overflow count, and clear-all affordance
- In scope: selected item count/label, removable chip buttons, clear action,
  overflow truncation, single vs multiple display modes, size and density
  scaling, light theme surface treatment
- Out of scope: candidate browsing, confirm/cancel workflow,
  pagination-aware selection semantics

## 2. Anatomy

```text
[Root <section>]  aria-label="Current selection"
  ├── [Header]
  │     ├── [CountLabel <strong>]
  │     └── [ClearButton <button>]  (when items.length > 0)
  └── [Chips]                       (when items.length > 0)
        ├── [Chip <button>...]
        │     ├── [ChipLabel]
        │     └── [RemoveIcon]  Icon "x", aria-hidden
        └── [Overflow <span>]       (when items.length > maxVisibleItems)
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| Root | `<section>` | Class `selection-summary`, `aria-label="Current selection"`, `data-size`, `data-density` |
| Header | `<div>` | Flex row with count label and clear button |
| CountLabel | `<strong>` | "N selected" (multiple) or "Selected item" / "No selection" (single) |
| ClearButton | `<button>` | "Clear" text button, visible only when items exist |
| Chips | `<div>` | Flex-wrap container for removable chip buttons |
| Chip | `<button>` | Shows item label with remove (x) icon; `aria-label="Remove {item.label}"` |
| RemoveIcon | `<span>` | Wraps `Icon` with `name="x"`, `aria-hidden="true"` |
| Overflow | `<span>` | "+N more" text for truncated items |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ id: string; label: string }>` | `[]` | no | Selected items to display |
| `selectionMode` | `"single" \| "multiple"` | `"multiple"` | no | Controls header label text |
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
| empty | `items` is empty | Header shows "No selection" (single) or "0 selected" (multiple); no chips, no clear button |
| populated | `items` has entries | Header shows count, clear button visible, chips rendered |
| truncated | `items.length > maxVisibleItems` | Only first `maxVisibleItems` chips shown, overflow badge shows "+N more" |

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
- `Tab` navigates between clear button and chip buttons
- `Enter` / `Space` activates focused button (clear or remove)
- Focus entry: clear button or first chip

## 7. Layout

### Sizing

- Root fills available width
- Gap between header and chips: `var(--poodle-space-stack-sm)`
- Chips container uses `flex-wrap` with `gap: var(--poodle-space-inline-sm)`
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
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-sm)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |
| `font-size` | `var(--poodle-typography-label-size, 0.75rem)` |

### `.selection-summary__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `justify-content` | `space-between` |
| `gap` | `var(--poodle-space-inline-md)` |
| `align-items` | `center` |

### `.selection-summary__header button` and `.selection-summary__chip` (shared)

| Property | Value |
|----------|-------|
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |

### `.selection-summary__header button` (ClearButton)

| Property | Value |
|----------|-------|
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |

### `.selection-summary__chips`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |

### `.selection-summary__chip`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `gap` | `var(--poodle-space-inline-md)` |
| `align-items` | `center` |
| `min-height` | `calc(var(--poodle-size-control-height) - 0.25rem)` |
| `padding` | `0 0.75rem` |

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
| ClearButton | `min-height` | `1.25rem` |
| ClearButton | `padding` | `0 0.375rem` |
| Chip | `min-height` | `1rem` |
| Chip | `padding` | `0 0.5rem` |
| Chip | `font-size` | `0.6875rem` |
| Overflow | `font-size` | `0.6875rem` |
| Overflow | `line-height` | `1.5rem` |
| Overflow | `padding` | `0 0.375rem` |

#### `[data-size="sm"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.71875rem` |
| ClearButton | `min-height` | `1.375rem` |
| Chip | `min-height` | `1.125rem` |
| Chip | `font-size` | `0.71875rem` |
| Overflow | `font-size` | `0.71875rem` |
| Overflow | `line-height` | `1.625rem` |

#### `[data-size="lg"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.8125rem` |
| ClearButton | `min-height` | `2.125rem` |
| ClearButton | `padding` | `0 0.875rem` |
| Chip | `min-height` | `1.75rem` |
| Chip | `padding` | `0 0.875rem` |
| Chip | `font-size` | `0.8125rem` |
| Overflow | `font-size` | `0.875rem` |
| Overflow | `line-height` | `2.25rem` |
| Overflow | `padding` | `0 0.75rem` |

#### `[data-size="xl"]`

| Part | Property | Value |
|------|----------|-------|
| Root | `font-size` | `0.875rem` |
| ClearButton | `min-height` | `2.25rem` |
| ClearButton | `padding` | `0 1rem` |
| Chip | `min-height` | `2rem` |
| Chip | `padding` | `0 1rem` |
| Chip | `font-size` | `0.875rem` |
| Overflow | `font-size` | `0.9375rem` |
| Overflow | `line-height` | `2.5rem` |
| Overflow | `padding` | `0 0.875rem` |

### Density Variants

| Density | Root `padding` | Root `gap` |
|---------|---------------|------------|
| compact | `0.25rem 0.375rem` | `var(--poodle-space-inline-xs)` |
| comfortable | `0.5rem 0.75rem` | `var(--poodle-space-inline-md)` |

### Light Theme Override

`:global([data-theme="light"]) .selection-summary`

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-border-default) 14%, transparent)` |
| `box-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent), 0 0.375rem 1rem rgba(49, 66, 85, 0.03)` |

## 9. Svelte Notes

- `data-size` resolves via `resolveSemanticControlSize` from presentation context
- `data-density` resolves via presentation context with explicit override
- Uses `createEventDispatcher` for `remove` and `clear` events
- `visibleItems` and `overflowCount` derived reactively from `items` and `maxVisibleItems`
- Uses `Icon` primitive with `name="x"` for the remove icon
- Imports `getUiPresentation` and `resolveSemanticControlSize` from `@poodle/svelte-primitives`

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::selection_summary`
- Render as section with header row and chip container
- Size/density scaling must match CSS variant tables

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] maxVisibleItems truncation behavior matches
- [ ] selectionMode header text matches ("No selection" / "Selected item" vs "N selected")

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
| Multiple items selected | `selectionMode="multiple"`, five items (Button, Card, Dialog, Table, Tabs), remove and clear handlers wired | Summary with five removable chips and a clear-all affordance |

### Single Item

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Single item | `selectionMode="single"`, one item (`"Primary button"`) | Compact summary showing "Selected item" header with a single chip |

### Truncated (Max 3 Visible)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Truncated (max 3 visible) | Six items (Alpha through Zeta), `maxVisibleItems={3}` | Three visible chips with "+3 more" overflow badge |
