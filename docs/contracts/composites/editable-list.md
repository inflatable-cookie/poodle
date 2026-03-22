# EditableList

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `EditableList`
- Layer: `composites`
- Summary: an autonomous list that lets users add, remove, and optionally
  reorder text items — suitable for tag lists, simple ordered collections,
  and other user-managed string arrays
- In scope: text input for adding items, per-item remove button, optional
  drag-and-drop reordering (via ReorderableList), max item limit with counter,
  disabled state, static (non-reorderable) mode
- Out of scope: inline editing of existing items, rich item content (icons,
  descriptions), multi-field items, validation beyond empty-string prevention

## 2. Anatomy

```text
[Root .autonomous-list]  <div>
  ├── [ItemList]  (when items.length > 0)
  │     ├── [ReorderableList]  (when isReorderable)
  │     │     └── [ItemRow .autonomous-list__item-row]  (repeated, via slot)
  │     │           ├── [ItemText .autonomous-list__item-text]  <span>
  │     │           └── [RemoveButton .autonomous-list__remove]  <button>
  │     │                 └── [RemoveIcon]  <svg> (x icon)
  │     └── [StaticList .autonomous-list__static]  <ul> (when !isReorderable)
  │           └── [StaticItem .autonomous-list__static-item]  <li> (repeated)
  │                 ├── [ItemText .autonomous-list__item-text]  <span>
  │                 └── [RemoveButton .autonomous-list__remove]  <button>
  │                       └── [RemoveIcon]  <svg> (x icon)
  ├── [AddRow .autonomous-list__add]  (when canAdd)
  │     ├── [Input .autonomous-list__input]  <input type="text">
  │     └── [AddButton .autonomous-list__add-btn]  <button>
  └── [Counter .autonomous-list__count]  <span> (when maxItems is set)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | flex column container | gap, disabled opacity |
| ItemList | conditional | either ReorderableList or static `<ul>`; shown when items exist | delegates to ReorderableList or list styling |
| ItemRow | yes (per item) | flex row with text and remove button | flex layout |
| ItemText | yes (per item) | truncated label text | text overflow, flex |
| RemoveButton | yes (per item) | icon-only button to remove item | size, color, hover color, border-radius |
| RemoveIcon | yes (per item) | SVG x icon, `aria-hidden="true"` | size, stroke |
| AddRow | conditional | input + add button row; shown when under max limit | gap |
| Input | conditional | text input for new item entry | height, padding, border, radius, background, font, focus ring |
| AddButton | conditional | button to confirm adding the new item | height, padding, border, radius, background, font, hover, disabled opacity |
| Counter | conditional | "N/M" count display; shown when maxItems is set | font-size, color, tabular-nums |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `ReorderableItem[]` | `[]` | no | current list items; bind for two-way |
| `addLabel` | `string` | `"Add item"` | no | label text for the add button |
| `placeholder` | `string` | `"New item"` | no | placeholder text for the input |
| `maxItems` | `number \| null` | `null` | no | maximum number of items; null means unlimited |
| `isDisabled` | `boolean` | `false` | no | disables all interactions |
| `ariaLabel` | `string` | `"List"` | no | accessible label for the list region |
| `isReorderable` | `boolean` | `true` | no | enables drag-and-drop reordering via ReorderableList |

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
| empty | `items` is empty | no list shown, only add row (if canAdd) |
| populated | `items` has entries | list shown with items, add row below if canAdd |
| at-max | `items.length >= maxItems` | add row hidden; counter shows "N/N" |
| disabled | `isDisabled` is true | entire component has reduced opacity; input, add button, and remove buttons are disabled |
| add-disabled | input is empty or whitespace | add button is disabled |

### Component States

- `newItemText` (internal string): current input value
- `canAdd` (derived): `!isDisabled && (maxItems === null || items.length < maxItems)`

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `change` | item added, removed, or reordered | `{ items: ReorderableItem[] }` | fires after the items array is updated |

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
| `Enter` | in the input field, adds the current text as a new item |
| `Tab` | navigates between input, add button, and remove buttons |
| (drag keys) | delegated to ReorderableList when reorderable |

### Focus And Announcement

- After adding an item, input is cleared but retains focus
- Remove buttons are standard `<button>` elements with native focus

## 7. Layout

### Sizing

- Root: flex column, gap `0.5rem` (8px)
- Static list: flex column, gap `0.125rem` (2px)
- Static item: padding `0.5rem 0.625rem`, border-radius from token
- Item text: `flex: 1`, `min-width: 0`, text-overflow ellipsis
- Remove button: `1.25rem` square, `0.25rem` border-radius
- Remove icon SVG: `0.75rem` square
- Add row: flex, gap `0.375rem` (6px)
- Input: `flex: 1`, height from control-height token, horizontal padding from control-x token
- Add button: height from control-height token, padding `0 0.75rem`
- Counter: font-size `0.6875rem`, aligned to flex-end

### Composition

- Parent expectations: form fields, settings panels, tag editors
- Child expectations: ReorderableList composite (when reorderable)
- Resizing rules: fills parent width; items stack vertically

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root (disabled) | `--pug-state-opacity-disabled` | disabled opacity |
| StaticItem | `--pug-radius-control` | item border-radius |
| StaticItem | `--pug-color-background-surface` | item background |
| RemoveButton | `--pug-color-text-secondary` | default icon color |
| RemoveButton (hover) | `--pug-color-status-danger` | hover icon color |
| RemoveButton (transition) | `--pug-motion-duration-interaction` | color transition duration |
| RemoveButton (transition) | `--pug-motion-easing-standard` | color transition easing |
| Input | `--pug-size-control-height` | input height |
| Input | `--pug-space-control-x` | input horizontal padding |
| Input | `--pug-color-border-default` | input border color |
| Input | `--pug-radius-control` | input border-radius |
| Input | `--pug-color-background-surface` | input background |
| Input | `--pug-color-text-primary` | input text color |
| Input | `--pug-typography-body-family` | input font family |
| Input | `--pug-typography-body-size` | input font size |
| Input (focus) | `--pug-color-accent-focusRing` | focus border and shadow color |
| Input (focus) | `--pug-border-width-focus` | focus shadow width |
| Input (placeholder) | `--pug-color-text-secondary` | placeholder text color |
| AddButton | `--pug-size-control-height` | button height |
| AddButton | `--pug-color-border-default` | button border color |
| AddButton | `--pug-radius-control` | button border-radius |
| AddButton | `--pug-color-background-surface` | button background |
| AddButton | `--pug-color-text-primary` | button text color |
| AddButton | `--pug-typography-label-family` | button font family |
| AddButton | `--pug-typography-label-size` | button font size |
| AddButton | `--pug-typography-label-weight` | button font weight |
| AddButton (hover) | `--pug-color-background-surface` / `--pug-color-background-elevated` | hover background (84% mix) |
| AddButton (disabled) | `--pug-state-opacity-disabled` | disabled opacity |
| Counter | `--pug-color-text-secondary` | counter text color |

## 9. Svelte Notes

- Uses `createEventDispatcher` for `change` event
- Composes `ReorderableList` composite for drag-and-drop reordering
- New item IDs generated with `Date.now()` + random suffix
- Input `keydown` handler prevents default on Enter to avoid form submission
- Remove button uses `stopPropagation` in reorderable mode to prevent drag initiation

## 10. GPUI Notes

- Expected crate/module surface: `pug_gpui::composites::editable_list`
- Reordering behavior may need a simplified drag-and-drop or move-up/move-down
  button approach
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

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal
- [ ] reorder mechanism may differ (drag-and-drop vs buttons)

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Specimen Definitions

### Reorderable With Add/Remove

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Reorderable with add/remove | three initial items ("svelte", "typescript", "design-system"), `ariaLabel="Tags"`, `placeholder="Add a tag..."`, `addLabel="Add"` | draggable list with remove buttons and add row below |

### With Max Items (5)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With max items (5) | two initial items ("Item A", "Item B"), `maxItems={5}`, `ariaLabel="Limited list"`, `placeholder="Add item..."` | list with two items, add row, and "2/5" counter |

### Non-Reorderable

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Non-reorderable | one item ("Static item"), `isReorderable={false}`, `ariaLabel="Static list"`, `placeholder="Add item..."` | static list (no drag handles) with remove button and add row |

## 14. Approval And Adoption Notes

- Contract status: `seed contract`
- Approvers: pending
- Downstream adopters: tag editors, simple list management forms, settings panels
- Future follow-up: consider inline editing support; consider custom item
  rendering via slot; consider validation callback for new items
