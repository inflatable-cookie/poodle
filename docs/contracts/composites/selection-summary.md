# SelectionSummary

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `SelectionSummary`
- Layer: `composites`
- Summary: a compact summary of the current selection with removable chips,
  overflow count, and clear-all affordance
- In scope: selected item count/label, removable chips, clear action,
  overflow truncation, single vs multiple display modes
- Out of scope: candidate browsing, confirm/cancel workflow,
  pagination-aware selection semantics

## 2. Anatomy

```text
[Root]
  ├── [Header]
  │     ├── [CountLabel]
  │     └── [ClearButton]  (when items.length > 0)
  └── [Chips]              (when items.length > 0)
        ├── [Chip...]
        │     ├── [ChipLabel]
        │     └── [RemoveIcon]
        └── [Overflow]     (when items.length > maxVisibleItems)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<section>` with `aria-label="Current selection"` | background-panel, radius-surface, border, label-size |
| Header | yes | flex row with count label and clear button | layout only |
| CountLabel | yes | "N selected" (multiple) or "Selected item" / "No selection" (single) | text-primary (inherited bold) |
| ClearButton | no | "Clear" button, visible when items exist | surface background, control radius/height |
| Chips | no | flex-wrap container for removable chip buttons | layout only |
| Chip | no | button showing item label with remove (x) icon | surface background, control radius |
| Overflow | no | "+N more" text for truncated items | text-secondary, surface background |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `Array<{ id: string; label: string }>` | `[]` | no | selected items to display |
| `selectionMode` | `"single" \| "multiple"` | `"multiple"` | no | controls header label text |
| `maxVisibleItems` | `number` | `4` | no | max chips shown before overflow |

### Controlled And Uncontrolled

- display composite; items list is externally driven
- no internal state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | `items` is empty | header shows "No selection" (single) or "0 selected" (multiple); no chips, no clear button |
| populated | `items` has entries | header shows count, clear button visible, chips rendered |
| truncated | `items.length > maxVisibleItems` | only first `maxVisibleItems` chips shown, overflow badge shows "+N more" |

### Component States

No internal state. Visible items and overflow count are derived from props.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `remove` | chip remove button clicked | `{ id: string }` | host removes the item from selection |
| `clear` | clear button clicked | `void` | host clears all selections |

## 6. Accessibility

### Semantics

- Role: `<section>` with `aria-label="Current selection"`
- Chip buttons have `aria-label="Remove {item.label}"`
- Remove icon (x) is `aria-hidden="true"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | navigates between clear button and chip buttons |
| `Enter` / `Space` | activates focused button (clear or remove) |

### Focus And Announcement

- focus entry: clear button or first chip
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must preserve selection summary
  and per-item removal meaning rather than rendering them as decorative tags

## 7. Layout

### Sizing

- fills available width
- gap between header and chips: `--poodle-space-stack-sm`
- chips container uses flex-wrap with gap `--poodle-space-inline-sm`
- chip min-height: 2rem
- chip inline padding: 0.75rem
- chip internal gap: 0.5rem
- overflow badge line-height: 2rem, padding 0 0.625rem

### Composition

- parent expectations: PickerShell selection slot, standalone selection displays
- child expectations: none (self-contained)
- resizing rules: chips wrap to multiple rows

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `--poodle-color-background-panel` | container background (94% alpha mix) |
| Root | `--poodle-radius-surface` | corner radius |
| Root | `--poodle-typography-label-size` | base font size (fallback 0.75rem) |
| Root | `--poodle-space-panel-y` / `--poodle-space-panel-x` | container padding |
| Header | `--poodle-space-inline-md` | gap between label and clear button |
| ClearButton | `--poodle-color-background-surface` | button background (76% alpha mix) |
| ClearButton | `--poodle-radius-control` | button radius |
| ClearButton | `--poodle-size-control-height` | button min-height |
| ClearButton | `--poodle-space-control-x` | button horizontal padding |
| Chip | `--poodle-color-background-surface` | chip background (76% alpha mix) |
| Chip | `--poodle-radius-control` | chip radius |
| Chip | `--poodle-color-text-primary` | chip text color |
| Overflow | `--poodle-color-text-secondary` | overflow text color |
| Overflow | `--poodle-color-background-surface` | overflow background (58% alpha mix) |
| Overflow | `--poodle-radius-control` | overflow radius |
| Root (light) | `--poodle-color-border-default` | light theme outer border (14% alpha mix) |
| Root (light) | `--poodle-color-border-subtle` | light theme inset shadow border |

### Token Usage — Exact CSS Values

#### `.selection-summary` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-sm)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |
| `font-size` | `var(--poodle-typography-label-size, 0.75rem)` |

#### `.selection-summary__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `justify-content` | `space-between` |
| `gap` | `var(--poodle-space-inline-md)` |
| `align-items` | `center` |

#### `.selection-summary__header button`, `.selection-summary__chip` (Shared)

| Property | Value |
|----------|-------|
| `border` | `0.0625rem solid transparent` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |

#### `.selection-summary__header button` (Clear Button)

| Property | Value |
|----------|-------|
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |

#### `.selection-summary__chips`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |

#### `.selection-summary__chip`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `gap` | `0.5rem` |
| `align-items` | `center` |
| `min-height` | `2rem` |
| `padding` | `0 0.75rem` |

#### `.selection-summary__overflow`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `2rem` |
| `padding` | `0 0.625rem` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 58%, transparent)` |

### Light Theme Override: `:global([data-theme="light"]) .selection-summary`

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-border-default) 14%, transparent)` |
| `box-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent), 0 0.375rem 1rem rgba(49, 66, 85, 0.03)` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-theme` | global (`:global([data-theme="light"])`) | light theme override selector |

## 9. Svelte Notes

- uses `createEventDispatcher` for `remove` and `clear` events
- `visibleItems` and `overflowCount` derived reactively from `items` and `maxVisibleItems`
- uses `Icon` primitive for the remove (x) icon with `size="sm"`
- light theme has additional border and inset box-shadow treatment

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::composites::selection_summary`
- render as section with header row and chip container

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] maxVisibleItems truncation behavior matches
- [ ] selectionMode header text matches

### Tier 2: Visual Parity

- [ ] chip styling matches
- [ ] overflow badge styling matches
- [ ] light theme treatment matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Specimen Definitions

### Multiple Items Selected

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Multiple items selected | `selectionMode="multiple"`, five items (Button, Card, Dialog, Table, Tabs), remove and clear handlers wired | Summary with five removable chips and a clear-all affordance |

### Single Item

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Single item | `selectionMode="single"`, one item (`"Primary button"`) | Compact summary showing a single selected item chip |

### Truncated (Max 3 Visible)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Truncated (max 3 visible) | six items (Alpha through Zeta), `maxVisibleItems={3}` | Three visible chips with an overflow count indicating the remaining three |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: RelationPicker, picker shells, selection-heavy workflows
- future follow-up: use `SelectionSummary` inside `RelationPicker`, picker
  shells, and future selection-heavy workflows instead of rebuilding
  selected-chip summaries ad hoc
