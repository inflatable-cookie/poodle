# OrderBy

Status: detailed contract
Updated: 2026-05-18

## 1. Purpose

- Component name: `OrderBy`
- Layer: `foundation`
- Summary: an anchored dropdown multi-field sort builder that lets users compose an
  ordered list of sort fields with per-field direction toggles, drag reordering
  (via a focusable drag handle + Alt+Arrow keyboard), field removal, and an
  add-field dropdown
- In scope: dropdown trigger with summary text, panel with sort field list,
  drag-and-drop reordering via the drag-handle button, Alt+ArrowUp/ArrowDown
  keyboard reorder, direction toggle buttons, remove buttons, add-field Select
  dropdown, compact summary mode, maxFields cap, legacy `activeSort` compatibility
- Out of scope: column header inline sort controls (see DataTable), custom sort
  comparison functions, server-side sort execution

## 2. Anatomy

```text
[Popover .order-by-popover] <div>  (position: relative wrapper, carries data-size/data-density)
  ├── [Root .order-by] <div role="group">
  │   ├── [Trigger Wrap .order-by__trigger-wrap] <div>
  │   │   └── [Trigger .order-by__trigger] <button aria-expanded aria-controls>
  │   │       ├── [Label .order-by__label] <span>  (hidden when compact)
  │   │       ├── [Summary .order-by__summary] <span>
  │   │       └── [Chevron .order-by__chevron] <span aria-hidden="true">
  │   └── [Reset .order-by__reset] <span> (conditional: showClearButton && value non-empty)
  │       └── IconButton (icon="x", variant="ghost", ariaLabel="Clear sort")
  └── [Surface .order-by__surface > .order-by__panel] <div role="dialog"> (rendered inline when open)
      ├── [List .order-by__list] <div role="list"> (conditional: visible when value non-empty)
      │   └── [Item .order-by__item] <div role="listitem"> (repeated, single flex row)
      │       ├── [Drag Handle .order-by__drag-handle] <button draggable aria-label="Reorder …">
      │       ├── [Item Label .order-by__item-label] <span>
      │       ├── [Direction Toggle] IconButton (arrow-up / arrow-down, size="xs")
      │       └── [Remove] IconButton (x, size="xs")
      ├── [Empty .order-by__empty] <p> (conditional: visible when value empty)
      └── [Add .order-by__add] <div> (conditional: visible when fields remain and maxFields not reached)
          └── Select (placeholder="+ Add field")
```

| Part | Required | Description |
|------|----------|-------------|
| Popover | yes | `position: relative` wrapper anchoring the surface; carries `data-size`, `data-density` |
| Root | yes | flex container wrapping trigger and reset; `role="group"`, carries `data-disabled`, `data-compact` |
| Trigger Wrap | yes | flex-1 wrapper around the trigger button |
| Trigger | yes | button that opens the dropdown; displays label, summary, and chevron; carries `aria-expanded` / `aria-controls` |
| Label | yes | static "Sort by" uppercase text inside the trigger; hidden when `compact` is true |
| Summary | yes | dynamic text summarizing the active sort fields |
| Chevron | yes | dropdown indicator arrow (`▾`) |
| Reset | no | `IconButton icon="x" variant="ghost"` (aria-label `"Clear sort"`), shown when `showClearButton` and at least one sort field is active |
| Surface | yes | anchored `role="dialog"` dropdown surface (`tabindex="-1"`) containing the sort builder UI |
| List | no | vertical list of active sort items (shown when value is non-empty) |
| Item | no | one active sort field row, a single flex row: drag handle, field label, direction toggle, remove; supports drag reorder |
| Drag Handle | no | focusable `<button>` carrying the braille glyph (`⠿`); `draggable`, owns drag-start and Alt+Arrow keyboard reorder |
| Item Label | no | field name, single line with ellipsis overflow |
| Empty | no | placeholder text ("No sort fields") when value is empty |
| Add | no | wrapper for the Select dropdown that adds a new sort field |

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
| `compact` | `boolean` | `false` | no | when true, hides the static "Sort by" label and, with more than 2 fields active, truncates the summary to first two plus a count badge |
| `showClearButton` | `boolean` | `true` | no | when false, the reset `×` IconButton is never rendered |
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
| empty | no active sort fields | trigger summary shows "Sort by..." in muted text; panel shows "No sort fields"; reset IconButton hidden |
| populated | one or more sort fields active | trigger summary shows field labels with direction arrows; reset IconButton visible (when `showClearButton`) |
| compact | `compact=true` | the static "Sort by" label is hidden; with 3+ fields the summary shows first two then `+N` count (e.g. "Title ↑, Updated ↓ +1") |
| disabled | `disabled=true` | root reduced to disabled opacity; all buttons and controls disabled |
| dropdown open | user clicks trigger | anchored dialog surface appears below the trigger |
| item dragging | user drags a sort item | dragging item reduced to 0.65 opacity |
| item drop target | dragging over a different item | target item gets accent 8% fill and a left accent bar (inset box-shadow) |
| all fields used | every field in `fields` is active | add-field Select hidden |
| maxFields reached | active count equals `maxFields` | add-field Select hidden |

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
| Trigger | `aria-expanded` | `"true"` when open, `"false"` otherwise |
| Trigger | `aria-controls` | the dialog surface id when open |
| Trigger | `disabled` | native disabled attribute when `disabled=true` |
| Dialog surface | `role` | `"dialog"` |
| Dialog surface | `aria-label` | from `ariaLabel` prop |
| Dialog surface | `tabindex` | `"-1"` |
| Reset IconButton | `ariaLabel` | `"Clear sort"` |
| Reset IconButton | `disabled` | when `disabled=true` |
| List | `role` | `"list"` |
| Item | `role` | `"listitem"` |
| Drag handle | element | focusable `<button>` |
| Drag handle | `draggable` | `true` (unless disabled) |
| Drag handle | `aria-label` | `"Reorder {fieldLabel}. Drag or use Alt plus arrow keys."` |
| Chevron | `aria-hidden` | `"true"` |
| Direction toggle IconButton | `ariaLabel` | `"{fieldLabel}: ascending\|descending. Click to toggle."` |
| Direction toggle IconButton | `tooltip` | `"Asc"` or `"Desc"` |
| Direction toggle IconButton | `size` | `"xs"` |
| Remove IconButton | `ariaLabel` | `"Remove {fieldLabel}"` |
| Remove IconButton | `tooltip` | `"Remove"` |
| Remove IconButton | `size` | `"xs"` |
| Add field Select | `ariaLabel` | `"Add sort field"` |
| Add field Select | `placeholder` | `"+ Add field"` |

### Keyboard

Keyboard behavior is inherited from the child components (IconButton, Select).
The dropdown opens on Enter/Space on the trigger. Within the panel, Tab moves
between controls. Escape closes the panel. On open, focus moves to the first
focusable control in the surface. Keyboard reordering is via the focusable drag
handle: **Alt+ArrowUp** moves the field one position earlier, **Alt+ArrowDown**
moves it one position later.

### Focus

- Trigger button receives standard focus ring on `focus-visible`
- Reset IconButton manages its own focus ring
- Panel controls (drag-handle button, IconButton, Select) each manage their own
  focus rings
- The direction-toggle and remove IconButtons in each item are size `xs`,
  variant `ghost`
- Opening the surface auto-focuses its first focusable control

## 7. Layout

### Sizing

- Popover: `position: relative`, `display: flex`, `width: 100%`, `min-width: 0`
- Root: `flex`, `align-items: center`, `width: 100%`
- Trigger: `inline-flex`, `flex: 1`, `width: 100%`, min-height from
  `var(--poodle-size-control-height)` (size-stepped)
- Reset: `inline-flex` wrapper around an `xs`/size-matched IconButton (no bespoke
  square dimensions)
- Surface: anchored `position: absolute`, `min-width: 14rem`,
  `max-width: min(24rem, 90vw)`, `top: calc(100% + 0.5rem)`
- Panel: flex column
- List: flex column
- Item: single flex row, centered items
- Item label: `flex: 1`, ellipsis overflow

### Composition

- Parent expectations: toolbar areas, list headers, filter panels, data table toolbars
- Child composition: uses Select, Button, and IconButton internally
- The dropdown surface is owned locally by `OrderBy`, not by `Popover`
- The trigger width accommodates the summary text with ellipsis overflow

## 8. Token Usage -- Exact Values

### Popover wrapper (.order-by-popover)

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `flex` |
| `width` | `100%` |
| `min-width` | `0` |

### Root (.order-by)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `width` | `100%` |
| `min-width` | `0` |

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
| `flex` | `1` |
| `min-width` | `0` |
| `width` | `100%` |
| `max-width` | `100%` |
| `min-height` | `var(--poodle-size-control-height)` (size-stepped) |
| `padding` | `0 var(--poodle-space-control-x)` |
| `box-sizing` | `border-box` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `text-align` | `left` |
| `user-select` | `none` |
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

### Reset (.order-by__reset)

The reset is an `IconButton icon="x" variant="ghost"` (aria-label `"Clear sort"`,
`size` matched to the resolved control size), wrapped in an `inline-flex`,
`flex-shrink: 0` span. It owns its own hover/focus treatment via the IconButton
primitive — there is no bespoke reset button chrome.

### Compact label hide (.order-by[data-compact="true"] .order-by__label)

| Property | Value |
|----------|-------|
| `display` | `none` |

### Surface (.order-by__surface)

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `top` | `calc(100% + 0.5rem)` |
| `left` | `0` |
| `min-width` | `14rem` |
| `max-width` | `min(24rem, 90vw)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-elevated)` |
| `box-shadow` | `var(--poodle-elevation-overlay)` (inset highlight + layered drop shadow) |

### Panel (.order-by__panel)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.375rem` |
| `margin` | `calc(-0.5 * var(--poodle-space-panel-y)) calc(-0.5 * var(--poodle-space-panel-x))` |
| `padding` | `0.375rem` |

### List (.order-by__list)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.25rem` |

### Item (.order-by__item)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `padding` | `0.3125rem 0.5rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.0625rem)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 90%, var(--poodle-color-background-elevated))` |

### Item hover (.order-by__item:hover)

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-border-default) 60%, transparent)` |

### Item dragging (.order-by__item--dragging)

| Property | Value |
|----------|-------|
| `opacity` | `0.65` |

### Item drop target (.order-by__item--drop-target)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 8%, transparent)` |
| `box-shadow` | `inset 0.125rem 0 0 var(--poodle-color-accent-base)` (left accent bar) |

### Item label (.order-by__item-label)

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `min-width` | `0` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |
| `white-space` | `nowrap` |
| `font-size` | `0.8125rem` |
| `color` | `var(--poodle-color-text-primary)` |

### Drag handle (.order-by__drag-handle)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `min-width` | `1.5rem` |
| `min-height` | `1.5rem` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-muted)` |
| `cursor` | `grab` |
| `user-select` | `none` |
| `font-size` | `0.75rem` |
| `flex-shrink` | `0` |

### Empty message (.order-by__empty)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.75rem` |
| `margin` | `0` |
| `padding` | `0.25rem 0` |

### Add (.order-by__add)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |

### Size adjustments

Sizes step the trigger height, the label font-size, and the summary font-size.
`xs`, `lg`, and `xl` also override trigger padding; `sm` and `md` inherit the
default `0 var(--poodle-space-control-x)`.

| Size | Trigger height | Trigger padding | Label font-size | Summary font-size |
|------|----------------|-----------------|-----------------|-------------------|
| `xs` | `1.5rem` | `0 0.5rem` | `0.5625rem` | `0.6875rem` |
| `sm` | `1.75rem` | default | `0.625rem` | `0.8125rem` |
| `md` | `var(--poodle-size-control-height)` | default | `0.75rem` (base) | `0.875rem` (base) |
| `lg` | `2.75rem` | `0 1rem` | `0.8125rem` | `0.9375rem` |
| `xl` | `3.25rem` | `0 1.125rem` | `0.875rem` | `1rem` |

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
- The current drop target gets the `order-by__item--drop-target` class (accent 8% fill + left accent bar via inset box-shadow)

For keyboard-only users, the focusable drag-handle button provides equivalent reordering via **Alt+ArrowUp** (move earlier) and **Alt+ArrowDown** (move later).

## 11. Internal Sub-Components

The panel uses the following internal component instances:

| Component | Usage | Props |
|-----------|-------|-------|
| local dialog surface | wraps the sort-builder UI when open | `role="dialog"`, `aria-label` from prop, `tabindex="-1"` |
| `IconButton` (reset) | clears all sort fields | `icon="x"`, `variant="ghost"`, `ariaLabel="Clear sort"`, `size` from resolved size |
| drag-handle `<button>` | initiates drag + Alt+Arrow reorder | `draggable`, `aria-label="Reorder {field}. Drag or use Alt plus arrow keys."` |
| `IconButton` (direction toggle) | toggles asc/desc per field | `icon="arrow-up"` or `"arrow-down"`, `size="xs"`, `variant="ghost"`, `tooltip="Asc"`/`"Desc"` |
| `IconButton` (remove) | removes field from sort | `icon="x"`, `size="xs"`, `variant="ghost"`, `tooltip="Remove"` (no danger tone) |
| `Select` | add-field dropdown | `placeholder="+ Add field"`, `ariaLabel="Add sort field"`, items from available (unused) fields |

## 12. Svelte Notes

- The component owns its open state and anchored panel surface directly
- Size resolves from `size` prop or from inherited presentation context via `resolveSemanticControlSize`
- Density resolves from `density` prop or from inherited presentation context
- The `activeSort` prop provides backward compatibility: when `value` is empty, `activeSort` is converted to a one-element value; on every mutation, `activeSort` is updated to reflect the first value element
- CSS classes `order-by__item--dragging` and `order-by__item--drop-target` are toggled via Svelte's `class:` directive
- The add-field Select uses its value-change callback to call `addField(key)`, then resets its own value to `""` to allow re-selection
- Clearing all sort fields is done via the reset `×` IconButton only — there is no footer or "Clear all" Button
- The reset `×` IconButton handler uses `stopPropagation` and `preventDefault` to avoid toggling the dropdown
- The surface is a `role="dialog"` element with `tabindex="-1"`; on open the component auto-focuses the first focusable control, and closes on outside-click or Escape

## 13. Parity Checklist

### Tier 1: Strict Parity

- [ ] dropdown opens on trigger click below the trigger
- [ ] adding a field appends to value array with field's `defaultDirection` (or `"asc"`)
- [ ] removing a field splices it from value array
- [ ] direction toggle flips between `"asc"` and `"desc"`
- [ ] Alt+ArrowUp/ArrowDown on the drag handle swaps adjacent items in value array
- [ ] drag reorder moves item from source index to target index
- [ ] `onChange` fires with the full `value` array on every mutation
- [ ] `activeSort` legacy bridging: value-to-activeSort and activeSort-to-value
- [ ] reset IconButton clears all sort fields (value to `[]`, activeSort to `null`)
- [ ] `showClearButton=false` suppresses the reset IconButton
- [ ] maxFields enforced: add-field hidden when limit reached
- [ ] disabled state suppresses all interactions
- [ ] summary text format: `"FieldName ↑"` / `"FieldName ↓"` joined by `", "`
- [ ] compact mode truncation at 2+ fields shown

### Tier 2: Visual Parity

- [ ] trigger dimensions match (flex:1, width 100%, min-height from control-height)
- [ ] trigger border, radius, background, color match
- [ ] trigger hover background matches (surface 84% / elevated)
- [ ] trigger focus ring matches
- [ ] label typography matches (0.75rem, uppercase, 0.05em spacing, secondary color)
- [ ] label hidden in compact mode
- [ ] summary font-size matches (0.875rem)
- [ ] summary placeholder color matches (muted)
- [ ] chevron color matches (secondary)
- [ ] reset renders as a ghost IconButton (no bespoke square chrome)
- [ ] item layout matches (single row, padding 0.3125rem 0.5rem, border-subtle, radius control-0.0625rem, surface 90%/elevated bg)
- [ ] item label is single-line with ellipsis (0.8125rem, primary color)
- [ ] drag handle is a button (muted, grab cursor, 1.5rem min square)
- [ ] dragging item opacity matches (0.65)
- [ ] drop target accent 8% fill + left accent bar match
- [ ] item gap matches (0.375rem)
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
