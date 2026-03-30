# EditableList

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `EditableList`
- Layer: `composites`
- Summary: an autonomous list that lets users add, remove, and optionally reorder text items -- suitable for tag lists, simple ordered collections, and other user-managed string arrays
- In scope: text input for adding items, per-item remove button, optional drag-and-drop reordering (via ReorderableList), max item limit with counter, disabled state, static (non-reorderable) mode, size and density variants
- Out of scope: inline editing of existing items, rich item content (icons, descriptions), multi-field items, validation beyond empty-string prevention

## 2. Anatomy

```text
[Root .autonomous-list]  <div>
  ├── [ItemList]  (when items.length > 0)
  │     ├── [ReorderableList]  (when reorderable)
  │     │     └── [ItemRow .autonomous-list__item-row]  (repeated, via snippet)
  │     │           ├── [ItemText .autonomous-list__item-text]  <span>
  │     │           └── [RemoveButton .autonomous-list__remove]  <button>
  │     │                 └── [RemoveIcon]  <svg> x icon, aria-hidden
  │     └── [StaticList .autonomous-list__static]  <ul role="list"> (when !reorderable)
  │           └── [StaticItem .autonomous-list__static-item]  <li> (repeated)
  │                 ├── [ItemText .autonomous-list__item-text]  <span>
  │                 └── [RemoveButton .autonomous-list__remove]  <button>
  │                       └── [RemoveIcon]  <svg> x icon, aria-hidden
  ├── [AddRow .autonomous-list__add]  (when canAdd)
  │     ├── [Input .autonomous-list__input]  <input type="text">
  │     └── [AddButton .autonomous-list__add-btn]  <button>
  └── [Counter .autonomous-list__count]  <span> (when maxItems is set)
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Root | `<div>` | yes | Flex column container, wraps in `UiPresentationProvider` |
| ItemList | varies | conditional | Either `ReorderableList` or static `<ul>`; shown when items exist |
| ItemRow | `<span>` | yes (per item) | Flex row with text and remove button |
| ItemText | `<span>` | yes (per item) | Truncated label text with text-overflow ellipsis |
| RemoveButton | `<button>` | yes (per item) | Icon-only button to remove item |
| RemoveIcon | `<svg>` | yes (per item) | X icon, `aria-hidden="true"` |
| AddRow | `<div>` | conditional | Input + add button row; shown when under max limit and not disabled |
| Input | `<input>` | conditional | Text input for new item entry |
| AddButton | `<button>` | conditional | Button to confirm adding the new item |
| Counter | `<span>` | conditional | "N/M" count display; shown when `maxItems` is set |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `ReorderableItem[]` | `[]` | no | Current list items; bind for two-way |
| `addLabel` | `string` | `"Add item"` | no | Label text for the add button |
| `placeholder` | `string` | `"New item"` | no | Placeholder text for the input |
| `maxItems` | `number \| null` | `null` | no | Maximum number of items; null means unlimited |
| `disabled` | `boolean` | `false` | no | Disables all interactions |
| `ariaLabel` | `string` | `"List"` | no | Accessible label for the list region |
| `reorderable` | `boolean` | `true` | no | Enables drag-and-drop reordering via ReorderableList |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override |

### Types

```ts
type ReorderableItem = {
  id: string;
  label: string;
};
```

### Slots

None.

### Controlled And Uncontrolled

- `items` supports two-way binding (`bind:items`)
- Items are also surfaced via the `change` event

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| empty | `items` is empty | No list shown, only add row (if canAdd) |
| populated | `items` has entries | List shown with items, add row below if canAdd |
| at-max | `items.length >= maxItems` | Add row hidden; counter shows "N/N" |
| disabled | `disabled` is true | Entire component has reduced opacity; input, add button, and remove buttons are disabled |
| add-disabled | input is empty or whitespace | Add button is disabled (cursor: not-allowed, reduced opacity) |

### Component States

| State | Description |
|-------|-------------|
| `newItemText` (internal) | Current input value |
| `canAdd` (derived) | `!disabled && (maxItems === null || items.length < maxItems)` |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `change` | Item added, removed, or reordered | `{ items: ReorderableItem[] }` | Fires after the items array is updated |

## 6. Accessibility

### Semantics

- Reorderable mode: accessibility delegated to ReorderableList composite
- Static mode: `<ul role="list">` with `aria-label`
- Remove buttons: `aria-label="Remove {item.label}"`
- Remove icon SVG: `aria-hidden="true"`
- Add button and input: standard form control semantics

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | In the input field, adds the current text as a new item (prevents default form submission) |
| `Tab` | Navigates between input, add button, and remove buttons |
| (drag keys) | Delegated to ReorderableList when reorderable |

### Focus And Announcement

- After adding an item, input is cleared but retains focus
- Remove buttons are standard `<button>` elements with native focus

## 7. Layout

### Sizing

- Root: flex column with density-driven gap
- Static list: flex column with density-driven gap
- Static item: density-aware row padding, border-radius from token
- Item text: `flex: 1`, `min-width: 0`, text-overflow ellipsis
- Remove button: semantic size-driven square control
- Remove icon SVG: `0.75rem` square
- Add row: flex with density-aware gap
- Input: `flex: 1`, height from control-height token, horizontal padding from control-x token
- Add button: height from control-height token, semantic horizontal padding
- Counter: semantic label size, aligned to flex-end

### Composition

- Composes: `ReorderableList` composite (when reorderable), `UiPresentationProvider`
- Parent expectations: form fields, settings panels, tag editors
- Resizing rules: fills parent width; items stack vertically

## 8. Token Usage -- Exact Values

### Recipe Custom Properties

| Property | Default |
|----------|---------|
| `--poodle-autonomous-list-gap` | `0.5rem` |
| `--poodle-autonomous-list-static-gap` | `0.125rem` |
| `--poodle-autonomous-list-item-y` | `0.5rem` |
| `--poodle-autonomous-list-item-x` | `0.625rem` |
| `--poodle-autonomous-list-remove-size` | `1.25rem` |
| `--poodle-autonomous-list-add-gap` | `0.375rem` |
| `--poodle-autonomous-list-add-x` | `0.75rem` |

#### `.autonomous-list` (Root)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `var(--poodle-autonomous-list-gap)` |

#### `.autonomous-list[data-disabled="true"]`

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

#### `.autonomous-list__static`

| Property | Value |
|----------|-------|
| `list-style` | `none` |
| `margin` | `0` |
| `padding` | `0` |
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `var(--poodle-autonomous-list-static-gap)` |

#### `.autonomous-list__static-item`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `padding` | `var(--poodle-autonomous-list-item-y) var(--poodle-autonomous-list-item-x)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |

#### `.autonomous-list__item-row`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `flex` | `1` |
| `min-width` | `0` |

#### `.autonomous-list__item-text`

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `min-width` | `0` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |
| `white-space` | `nowrap` |

#### `.autonomous-list__remove`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `flex-shrink` | `0` |
| `width` | `var(--poodle-autonomous-list-remove-size)` |
| `height` | `var(--poodle-autonomous-list-remove-size)` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `0.25rem` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |
| `transition` | `color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.autonomous-list__remove:hover:not(:disabled)`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-status-danger)` |

#### `.autonomous-list__remove svg`

| Property | Value |
|----------|-------|
| `width` | `0.75rem` |
| `height` | `0.75rem` |

#### `.autonomous-list__add`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `gap` | `var(--poodle-autonomous-list-add-gap)` |

#### `.autonomous-list__input`

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `min-width` | `0` |
| `height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `outline` | `none` |

#### `.autonomous-list__input:focus`

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-focusRing)` |
| `box-shadow` | `0 0 0 var(--poodle-border-width-focus) color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)` |

#### `.autonomous-list__input::placeholder`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

#### `.autonomous-list__add-btn`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-autonomous-list-add-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `transition` | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard), border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.autonomous-list__add-btn:hover:not(:disabled)`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 84%, var(--poodle-color-background-elevated))` |

#### `.autonomous-list__add-btn:disabled`

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

#### `.autonomous-list__count`

| Property | Value |
|----------|-------|
| `font-size` | `var(--poodle-typography-label-size)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-variant-numeric` | `tabular-nums` |
| `align-self` | `flex-end` |

### Size Adjustments

| Size | `remove-size` | `item-x` | `add-x` |
|------|--------------|----------|---------|
| `xs` | `1rem` | `0.5rem` | `0.625rem` |
| `sm` | `1.125rem` | (default) | (default) |
| `md` | (default `1.25rem`) | (default `0.625rem`) | (default `0.75rem`) |
| `lg` | `1.375rem` | `0.75rem` | `0.875rem` |
| `xl` | `1.5rem` | `0.875rem` | `1rem` |

### Density Adjustments

| Density | `gap` | `static-gap` | `item-y` | `add-gap` |
|---------|-------|-------------|---------|----------|
| `compact` | `0.375rem` | `0.0625rem` | `0.375rem` | `0.25rem` |
| `default` | `0.5rem` | `0.125rem` | `0.5rem` | `0.375rem` |
| `comfortable` | `0.625rem` | `0.1875rem` | `0.625rem` | `0.5rem` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-disabled` | `.autonomous-list` root | Targets disabled state opacity |
| `data-size` | `.autonomous-list` root | Drives size variant custom properties |
| `data-density` | `.autonomous-list` root | Drives density variant custom properties |

## 9. Svelte Notes

- Uses `createEventDispatcher` for `change` event
- Wraps in `UiPresentationProvider` to propagate resolved size and density
- Composes `ReorderableList` composite for drag-and-drop reordering
- New item IDs generated with `Date.now()` + random suffix
- Input `keydown` handler prevents default on Enter to avoid form submission
- Remove button uses `stopPropagation` in reorderable mode to prevent drag initiation
- Uses Svelte 5 `{#snippet}` syntax for ReorderableList item rendering

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::editable_list`
- Reordering behavior may need a simplified drag-and-drop or move-up/move-down button approach
- Text input and add button compose from primitives

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event name and payload matches
- [ ] ReorderableItem type is identical
- [ ] maxItems enforcement matches (hides add row, shows counter)
- [ ] Enter-to-add behavior matches

### Tier 2: Visual Parity

- [ ] item row styling matches
- [ ] remove button color and hover matches
- [ ] input and add button styling matches
- [ ] counter positioning and styling matches
- [ ] size and density variant spacing matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal
- [ ] reorder mechanism may differ (drag-and-drop vs buttons)

## 12. Specimen Definitions

### Reorderable With Add/Remove

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Reorderable with add/remove | three initial items ("svelte", "typescript", "design-system"), `ariaLabel="Tags"`, `placeholder="Add a tag..."`, `addLabel="Add"` | Draggable list with remove buttons and add row below |

### With Max Items (5)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With max items (5) | two initial items ("Item A", "Item B"), `maxItems={5}`, `ariaLabel="Limited list"`, `placeholder="Add item..."` | List with two items, add row, and "2/5" counter |

### Non-Reorderable

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Non-reorderable | one item ("Static item"), `reorderable={false}`, `ariaLabel="Static list"`, `placeholder="Add item..."` | Static list (no drag handles) with remove button and add row |
