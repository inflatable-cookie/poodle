# OrderBy

Status: detailed contract
Updated: 2026-05-18

## 1. Purpose

- Component name: `OrderBy`
- Layer: `foundation`
- Summary: an anchored dropdown multi-field sort builder that lets users compose an
  ordered list of sort fields with per-field direction toggles, drag reordering,
  move-up/move-down controls, field removal, and an add-field dropdown
- In scope: dropdown trigger with summary text, panel with sort field list,
  drag-and-drop reordering, move up/down buttons, direction toggle buttons,
  remove buttons, add-field Select dropdown, clear-all button, compact summary
  mode, maxFields cap, legacy `activeSort` compatibility
- Out of scope: column header inline sort controls (see DataTable), custom sort
  comparison functions, server-side sort execution

## 2. Anatomy

```text
[Root .order-by] <div role="group">
  ├── [Trigger .order-by__trigger] <button>
  │   ├── [Label .order-by__label] <span>
  │   ├── [Summary .order-by__summary] <span>
  │   └── [Chevron .order-by__chevron] <span aria-hidden="true">
  ├── [Reset .order-by__reset] <button> (conditional: visible when value non-empty)
  └── [Panel .order-by__surface > .order-by__panel] <div> (rendered inline when open)
      ├── [List .order-by__list] <div role="list"> (conditional: visible when value non-empty)
      │   └── [Item .order-by__item] <div role="listitem" draggable> (repeated)
      │       ├── [Item Main .order-by__item-main] <div>
      │       │   ├── [Item Label .order-by__item-label] <span>
      │       │   │   └── [Drag Handle .order-by__drag-handle] <span aria-hidden="true">
      │       │   └── [Item Direction .order-by__item-direction] <span>
      │       └── [Item Actions .order-by__item-actions] <div>
      │           ├── [Direction Toggle] IconButton (arrow-up / arrow-down)
      │           ├── [Move Up] IconButton (chevron-up)
      │           ├── [Move Down] IconButton (chevron-down)
      │           └── [Remove] IconButton (x, tone=danger)
      ├── [Empty .order-by__empty] <p> (conditional: visible when value empty)
      ├── [Add .order-by__add] <div> (conditional: visible when fields remain and maxFields not reached)
      │   └── Select (placeholder="+ Add field")
      └── [Footer .order-by__footer] <div> (conditional: visible when 2+ fields active)
          └── Button ("Clear all", variant=ghost, size=sm)
```

| Part | Required | Description |
|------|----------|-------------|
| Root | yes | inline-flex container wrapping trigger and reset; carries `data-size`, `data-density`, `data-disabled` |
| Trigger | yes | button that opens the dropdown; displays label, summary, and chevron |
| Label | yes | static "Sort by" uppercase text inside the trigger |
| Summary | yes | dynamic text summarizing the active sort fields |
| Chevron | yes | dropdown indicator arrow (`▾`) |
| Reset | no | clear button (`×`) shown when at least one sort field is active |
| Panel | yes | anchored dropdown surface containing the sort builder UI |
| List | no | vertical list of active sort items (shown when value is non-empty) |
| Item | no | one active sort field row: label, direction text, and action buttons; supports drag reorder |
| Item Main | no | left side of an item: field label with drag handle, and direction text |
| Item Label | no | field name with drag handle glyph |
| Drag Handle | no | braille-pattern glyph (`⠿`) for drag initiation |
| Item Direction | no | text label ("Ascending" or "Descending") |
| Item Actions | no | right side of an item: icon buttons for direction toggle, move, and remove |
| Empty | no | placeholder text ("No sort fields selected") when value is empty |
| Add | no | wrapper for the Select dropdown that adds a new sort field |
| Footer | no | wrapper for the "Clear all" button; shown when 2+ fields are active |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `fields` | `SortField[]` | `[]` | yes | available sort fields the user can choose from |
| `value` | `OrderByValue \| undefined` | `undefined` | no | ordered array of active sort fields with directions; when supplied, acts as the controlled multi-field source of truth |
| `activeSort` | `ActiveSort \| null \| undefined` | `undefined` | no | legacy single-field sort; used when `value` is omitted |
| `ariaLabel` | `string` | `"Sort by"` | no | accessible name for root group and trigger |
| `disabled` | `boolean` | `false` | no | disables all interactive controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role for inherited sizing |
| `size` | `ControlSize \| null` | `null` | no | explicit size override (`"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"`) |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override (`"compact"`, `"default"`, `"comfortable"`) |
| `maxFields` | `number \| null` | `null` | no | maximum number of simultaneously active sort fields; `null` means no limit |
| `compact` | `boolean` | `false` | no | when true and more than 2 fields active, summary truncates to first two plus a count badge |
| `onChange` | `(value: OrderByValue) => void \| null` | `null` | no | callback fired on every sort mutation |

### SortField Type

```typescript
type SortField = {
  label: string;
  value?: string;
  key?: string;
  disabled?: boolean;
  defaultDirection?: "asc" | "desc";
};
```

Fields are normalized internally: `key` is resolved as `field.key ?? field.value ?? ""`, and fields with empty keys are filtered out. `value` is an alias for `key` for backward compatibility.

### OrderByFieldDefinition Type (internal)

```typescript
type OrderByFieldDefinition = {
  key: string;
  label: string;
  disabled?: boolean;
  defaultDirection?: "asc" | "desc";
};
```

### OrderByField Type

```typescript
type OrderByField = {
  key: string;
  direction: "asc" | "desc";
};
```

### OrderByValue Type

```typescript
type OrderByValue = OrderByField[];
```

An ordered array where index 0 is the primary sort, index 1 is the secondary sort, and so on.

### ActiveSort Type (legacy)

```typescript
type ActiveSort = {
  field: string;
  direction: "asc" | "desc";
};
```

When `value` is empty but `activeSort` is provided, the component treats it as a one-element value: `[{ key: activeSort.field, direction: activeSort.direction }]`. The `activeSort` prop is also updated on every mutation to reflect the first element of the value array.

### Controlled And Uncontrolled

- Controlled multi-field mode: provide `value`; the component mirrors edits
  through `onChange`
- Controlled legacy mode: provide `activeSort` without `value`; the component
  treats it as a one-item sort list and mirrors edits back through `activeSort`
- Uncontrolled fallback: when neither `value` nor `activeSort` is provided, the
  component owns its local sort state
- When both are present, `value` is the source of truth and `activeSort` is
  mirrored from the first active sort item

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | no active sort fields | trigger summary shows "Sort by..." in muted text; panel shows "No sort fields selected"; reset button hidden |
| populated | one or more sort fields active | trigger summary shows field labels with direction arrows; reset button visible |
| compact populated | `compact=true` and 3+ fields | summary shows first two fields then `+N` count (e.g. "Title ↑, Updated ↓ +1") |
| disabled | `disabled=true` | root reduced to disabled opacity; all buttons and controls disabled |
| dropdown open | user clicks trigger | anchored panel appears below the trigger |
| item dragging | user drags a sort item | dragging item reduced to 0.65 opacity |
| item drop target | dragging over a different item | target item gets accent border and glow shadow |
| all fields used | every field in `fields` is active | add-field Select hidden |
| maxFields reached | active count equals `maxFields` | add-field Select hidden |
| single field active | exactly one sort field | footer with "Clear all" hidden (only shown at 2+) |

### Summary Text Logic

- Empty: `"Sort by..."`
- Non-empty: field labels joined with `, `, each suffixed with `↑` (asc) or `↓` (desc)
- Compact mode with 3+ fields: first two items shown, then ` +N` where N is the remaining count

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onChange` | any sort mutation (add, remove, reorder, direction toggle, clear all) | `OrderByValue` | parent-owned sort state should be updated from this callback |

## 6. Accessibility

### Semantics

| Element | Attribute | Value |
|---------|-----------|-------|
| Root | `role` | `"group"` |
| Root | `aria-label` | from `ariaLabel` prop (default `"Sort by"`) |
| Root | `data-disabled` | `"true"` when disabled |
| Trigger | `aria-label` | from `ariaLabel` prop |
| Trigger | `disabled` | native disabled attribute when `disabled=true` |
| Panel surface | `aria-label` | from `ariaLabel` prop |
| Reset button | `aria-label` | `"Clear sort"` |
| Reset button | `disabled` | native disabled attribute when `disabled=true` |
| List | `role` | `"list"` |
| Item | `role` | `"listitem"` |
| Item | `draggable` | `true` (unless disabled) |
| Drag handle | `aria-hidden` | `"true"` |
| Chevron | `aria-hidden` | `"true"` |
| Direction toggle IconButton | `ariaLabel` | `"Toggle {fieldLabel} direction"` |
| Direction toggle IconButton | `tooltip` | `"Ascending"` or `"Descending"` |
| Move up IconButton | `ariaLabel` | `"Move {fieldLabel} earlier"` |
| Move up IconButton | `tooltip` | `"Move up"` |
| Move down IconButton | `ariaLabel` | `"Move {fieldLabel} later"` |
| Move down IconButton | `tooltip` | `"Move down"` |
| Remove IconButton | `ariaLabel` | `"Remove {fieldLabel} from sort"` |
| Remove IconButton | `tooltip` | `"Remove"` |
| Add field Select | `ariaLabel` | `"Add sort field"` |
| Add field Select | `placeholder` | `"+ Add field"` |

### Keyboard

Keyboard behavior is inherited from the child components (Button,
IconButton, Select). The dropdown opens on Enter/Space on the trigger.
Within the panel, Tab moves between controls. Escape closes the panel.
Drag reordering is supplemented by the move-up/move-down icon buttons for
keyboard users.

### Focus

- Trigger button receives standard focus ring on `focus-visible`
- Reset button receives standard focus ring on `focus-visible`
- Panel controls (IconButton, Select, Button) each manage their own focus rings
- All icon buttons in item actions are size `sm`, variant `ghost`
- Move-up disabled on first item; move-down disabled on last item

## 7. Layout

### Sizing

- Root: `inline-flex`, `align-items: center`
- Trigger: `inline-flex`, min-width `12rem`, max-width `min(28rem, 75vw)`, min-height `2rem`
- Reset button: `1.75rem` square
- Panel: flex column
- List: flex column
- Item: flex row, space-between, centered items
- Item actions: inline-flex row

### Composition

- Parent expectations: toolbar areas, list headers, filter panels, data table toolbars
- Child composition: uses Select, Button, and IconButton internally
- The dropdown surface is owned locally by `OrderBy`, not by `Popover`
- The trigger width accommodates the summary text with ellipsis overflow

## 8. Token Usage -- Exact Values

### Root (.order-by)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |

### Root disabled (.order-by[data-disabled="true"])

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Trigger (.order-by__trigger)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.5rem` |
| `min-width` | `12rem` |
| `max-width` | `min(28rem, 75vw)` |
| `min-height` | `2rem` |
| `padding` | `0 0.75rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `text-align` | `left` |
| `transition` | `background, border-color` at `var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Trigger hover (.order-by__trigger:hover:not(:disabled))

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 84%, var(--poodle-color-background-elevated))` |

### Trigger focus (.order-by__trigger:focus-visible)

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

### Label (.order-by__label)

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `text-transform` | `uppercase` |
| `letter-spacing` | `0.05em` |
| `white-space` | `nowrap` |

### Summary (.order-by__summary)

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `min-width` | `0` |
| `font-size` | `0.875rem` |
| `white-space` | `nowrap` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |

### Summary placeholder (.order-by__summary[data-placeholder="true"])

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-muted)` |

### Chevron (.order-by__chevron)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

### Reset button (.order-by__reset)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.75rem` |
| `height` | `1.75rem` |
| `border` | `0` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |

### Reset button hover (.order-by__reset:hover:not(:disabled))

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 68%, var(--poodle-color-background-elevated))` |

### Reset button focus (.order-by__reset:focus-visible)

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

### Panel (.order-by__panel)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.75rem` |

### List (.order-by__list)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.5rem` |

### Item (.order-by__item)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `gap` | `0.75rem` |
| `padding` | `0.625rem 0.75rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 88%, var(--poodle-color-background-elevated))` |

### Item dragging (.order-by__item--dragging)

| Property | Value |
|----------|-------|
| `opacity` | `0.65` |

### Item drop target (.order-by__item--drop-target)

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-base)` |
| `box-shadow` | `0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent)` |

### Item main (.order-by__item-main)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.125rem` |
| `min-width` | `0` |

### Item label (.order-by__item-label)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `font-weight` | `var(--poodle-typography-body-strong-weight, 600)` |

### Drag handle (.order-by__drag-handle)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-muted)` |
| `cursor` | `grab` |
| `user-select` | `none` |

### Item direction (.order-by__item-direction)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.875rem` |

### Empty message (.order-by__empty)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.875rem` |

### Item actions (.order-by__item-actions)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.25rem` |
| `flex-shrink` | `0` |

### Add / Footer (.order-by__add, .order-by__footer)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `flex-start` |

### Size adjustments (trigger min-height)

| Size | Trigger min-height | Trigger padding |
|------|-------------------|-----------------|
| `xs` | `1.625rem` | `0 0.5rem` |
| `sm` | `1.75rem` | `0 0.75rem` (default) |
| `md` | `2rem` (default) | `0 0.75rem` (default) |
| `lg` | `2.25rem` | `0 0.75rem` (default) |
| `xl` | `2.5rem` | `0 0.75rem` (default) |

Note: `xs` also reduces the label font-size to `0.625rem`. Only `xs` overrides trigger padding.

### Density adjustments (trigger gap)

| Density | Trigger gap |
|---------|-------------|
| `compact` | `0.375rem` |
| `default` | `0.5rem` (default) |
| `comfortable` | `0.625rem` |

## 9. Data Attributes

| Attribute | Element | Values | Purpose |
|-----------|---------|--------|---------|
| `data-disabled` | Root | `"true"` / `"false"` | reflects disabled state |
| `data-size` | Root | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` | resolved control size |
| `data-density` | Root | `"compact"`, `"default"`, `"comfortable"` | resolved density |
| `data-placeholder` | Summary | `"true"` / `"false"` | whether the summary is showing placeholder text (empty state) |

## 10. Drag-And-Drop Behavior

Items in the sort list support native HTML drag-and-drop reordering:

1. **Drag start**: sets `dragIndex` to the source item index
2. **Drag enter**: as the dragged item enters another item, `dragOverIndex` updates to highlight the drop target
3. **Drop**: the dragged item is spliced out of its original position and inserted at the drop target index; the value array is synced
4. **Drag end**: clears all drag state regardless of whether a drop occurred

Visual feedback during drag:
- The source item gets the `order-by__item--dragging` class (0.65 opacity)
- The current drop target gets the `order-by__item--drop-target` class (accent border + glow shadow)

For keyboard-only users, the move-up and move-down IconButtons provide equivalent reordering functionality.

## 11. Internal Sub-Components

The panel uses the following internal component instances:

| Component | Usage | Props |
|-----------|-------|-------|
| local panel surface | wraps the sort-builder UI when open | `role="dialog"`, `aria-label` from prop |
| `IconButton` (direction toggle) | toggles asc/desc per field | `icon="arrow-up"` or `"arrow-down"`, `size="sm"`, `variant="ghost"` |
| `IconButton` (move up) | moves field one position earlier | `icon="chevron-up"`, `size="sm"`, `variant="ghost"`, disabled on first item |
| `IconButton` (move down) | moves field one position later | `icon="chevron-down"`, `size="sm"`, `variant="ghost"`, disabled on last item |
| `IconButton` (remove) | removes field from sort | `icon="x"`, `size="sm"`, `variant="ghost"`, `tone="danger"` |
| `Select` | add-field dropdown | `placeholder="+ Add field"`, `ariaLabel="Add sort field"`, items from available (unused) fields |
| `Button` | clear-all in footer | `variant="ghost"`, `tone="default"`, `size="sm"` |

## 12. Svelte Notes

- The component owns its open state and anchored panel surface directly
- Size resolves from `size` prop or from inherited presentation context via `resolveSemanticControlSize`
- Density resolves from `density` prop or from inherited presentation context
- The `activeSort` prop provides backward compatibility: when `value` is empty, `activeSort` is converted to a one-element value; on every mutation, `activeSort` is updated to reflect the first value element
- CSS classes `order-by__item--dragging` and `order-by__item--drop-target` are toggled via Svelte's `class:` directive
- The add-field Select uses its value-change callback to call `addField(key)`, then resets its own value to `""` to allow re-selection
- The "Clear all" button and footer are only shown when 2 or more fields are active
- The reset `×` button in the trigger area uses `stopPropagation` and `preventDefault` to avoid toggling the dropdown

## 13. Parity Checklist

### Tier 1: Strict Parity

- [ ] dropdown opens on trigger click below the trigger
- [ ] adding a field appends to value array with field's `defaultDirection` (or `"asc"`)
- [ ] removing a field splices it from value array
- [ ] direction toggle flips between `"asc"` and `"desc"`
- [ ] move up/down swaps adjacent items in value array
- [ ] drag reorder moves item from source index to target index
- [ ] `onChange` fires with the full `value` array on every mutation
- [ ] `onChange` callback called with `value` on every mutation
- [ ] `activeSort` legacy bridging: value-to-activeSort and activeSort-to-value
- [ ] clear-all resets value to `[]` and activeSort to `null`
- [ ] reset button clears all sort fields
- [ ] maxFields enforced: add-field hidden when limit reached
- [ ] disabled state suppresses all interactions
- [ ] summary text format: `"FieldName ↑"` / `"FieldName ↓"` joined by `", "`
- [ ] compact mode truncation at 2+ fields shown

### Tier 2: Visual Parity

- [ ] trigger dimensions match (min-width 12rem, max-width min(28rem, 75vw), min-height 2rem)
- [ ] trigger border, radius, background, color match
- [ ] trigger hover background matches
- [ ] trigger focus ring matches
- [ ] label typography matches (0.75rem, uppercase, 0.05em spacing, secondary color)
- [ ] summary font-size matches (0.875rem)
- [ ] summary placeholder color matches (muted)
- [ ] chevron color matches (secondary)
- [ ] reset button dimensions match (1.75rem square)
- [ ] reset button hover matches
- [ ] item layout matches (padding, border, radius, background)
- [ ] item label font-weight matches (body-strong-weight)
- [ ] item direction text matches (secondary, 0.875rem)
- [ ] drag handle color matches (muted)
- [ ] dragging item opacity matches (0.65)
- [ ] drop target border and shadow match
- [ ] item actions gap matches (0.25rem)
- [ ] all five sizes visually match per size table
- [ ] all three densities visually match per density table
- [ ] disabled opacity matches

### Tier 3: Implementation Freedom

- [ ] dropdown implementation details (animation, portal behavior) are platform-owned
- [ ] drag-and-drop implementation mechanism is platform-owned
- [ ] transition timing is platform-owned

## 14. Specimen Definitions

### Multi-field sort builder

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Multi-field sort builder | `fields`: Title, Kind, Updated (defaultDirection desc), Created (defaultDirection desc), Visibility (disabled); `value`: Updated desc + Title asc; `compact` | Trigger showing compact summary, dropdown panel with two sort items, add-field dropdown with remaining fields |

### Sizes

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Sizes | Same fields, single value `[{key: "title", direction: "asc"}]`, each size from xs to xl | Five triggers at increasing heights |

### Densities

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Densities | Same fields, single value, density set to compact / default / comfortable | Three triggers with varying internal gap |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | Same fields, single value, `disabled=true` | Trigger at reduced opacity, all controls non-interactive |
