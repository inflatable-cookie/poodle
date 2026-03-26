# ReorderableList

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `ReorderableList`
- Layer: `composites`
- Summary: a vertical list of items that can be reordered via drag-and-drop or keyboard shortcuts
- In scope: drag-and-drop reordering, keyboard reordering (Alt+Arrow), drag handle, item slot, disabled state
- Out of scope: multi-list transfer, nested lists, horizontal orientation, sortable grids, virtualization

## 2. Anatomy

```text
[Root]  <ul> role="listbox"
  └── [Item...]  <li> role="option"
        ├── [Handle]  drag grip icon, aria-hidden
        └── [Content]  slot or default label text
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<ul>` | `role="listbox"`, `aria-label`, flex column with gap |
| item | `<li>` | `role="option"`, `tabindex="0"`, `draggable`, visual states for dragging/drop-target |
| handle | `<span>` | 6-dot grip icon, `aria-hidden="true"`, cursor grab |
| content | `<span>` | Flex-grow content area, renders slot or `item.label` |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `ReorderableItem[]` | `[]` | no | List items (two-way bindable) |
| `ariaLabel` | `string` | `"Reorderable list"` | no | Accessible label for the list |
| `disabled` | `boolean` | `false` | no | Disables drag, keyboard reorder, and interaction |

### Types

```ts
type ReorderableItem = {
  id: string;
  label: string;
};
```

### Slots

| Slot | Props | Notes |
|------|-------|-------|
| `item` | `{ item: ReorderableItem, index: number }` | Custom content for each list item; falls back to `item.label` |

### Controlled / Uncontrolled

`items` supports two-way binding. The component mutates the array order internally and dispatches `reorder` with the updated array.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| default | -- | Surface background, transparent border |
| hover | Mouse over item | Slightly elevated background blend |
| focus | Focus-visible on item | Focus ring: `border-width-focus` solid `accent-focusRing` |
| dragging | Item being dragged | 40% opacity on the dragged item |
| drop-target | Dragging over another item | `accent-base` border color, accent-tinted background |
| disabled | `disabled=true` | `state-opacity-disabled`, `pointer-events: none` |

### Component States

| State | Description |
|-------|-------------|
| idle | No drag in progress |
| dragging | An item is being dragged; source item faded, drop targets highlighted |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `reorder` | Items reordered via drag-and-drop or keyboard | `{ items: ReorderableItem[] }` |

## 6. Accessibility

### Semantics

- Root is `<ul>` with `role="listbox"` and `aria-label`
- Each item is `<li>` with `role="option"` and `aria-selected="false"`
- Handle is `aria-hidden="true"` (decorative)
- Items have `data-reorder-index` for programmatic focus management

### Keyboard

| Key | Action |
|-----|--------|
| `Alt+ArrowUp` | Move focused item up one position |
| `Alt+ArrowDown` | Move focused item down one position |
| `Tab` | Navigate between items |

After keyboard reorder, focus follows the moved item to its new position.

### Focus

- Items have `tabindex="0"` (or `-1` when disabled)
- Focus ring: `border-width-focus` solid `accent-focusRing`, offset `-0.0625rem`

## 7. Layout

### Sizing

- Root: flex column, gap `0.125rem`, no list styling, no margin/padding
- Item: flex row, centered, gap `0.5rem`, padding `0.5rem 0.625rem`, border `0.0625rem solid transparent`, `radius-control`
- Handle: `1rem x 1rem`, flex-shrink 0
- Content: flex 1, min-width 0

### Composition

Standalone component. Items can be customized via the `item` slot.

## 8. Token Usage And Precise CSS

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-disabled` | root `<ul>` | `"true"`, `"false"` |
| `data-reorder-index` | item `<li>` | numeric index |

### Root

| Property | Value |
|----------|-------|
| list-style | `none` |
| margin | `0` |
| padding | `0` |
| display | `flex` |
| flex-direction | `column` |
| gap | `0.125rem` |

#### Root Disabled (`[data-disabled="true"]`)

| Property | Value |
|----------|-------|
| opacity | `var(--poodle-state-opacity-disabled)` |
| pointer-events | `none` |

### Item

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.5rem` |
| padding | `0.5rem 0.625rem` |
| border | `0.0625rem solid transparent` |
| border-radius | `var(--poodle-radius-control)` |
| background | `var(--poodle-color-background-surface)` |
| cursor | `grab` |
| transition | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard), border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### Item States

| State | Property | Value |
|-------|----------|-------|
| `:hover` | background | `color-mix(in srgb, var(--poodle-color-background-elevated) 52%, var(--poodle-color-background-surface))` |
| `:focus-visible` | outline | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `:focus-visible` | outline-offset | `-0.0625rem` |
| `--dragging` | opacity | `0.4` |
| `--drop-target` | border-color | `var(--poodle-color-accent-base)` |
| `--drop-target` | background | `color-mix(in srgb, var(--poodle-color-accent-base) 8%, var(--poodle-color-background-surface))` |

### Handle

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| flex-shrink | `0` |
| width | `1rem` |
| height | `1rem` |
| color | `var(--poodle-color-text-secondary)` |
| cursor | `grab` |
| SVG icon | 6-dot grip pattern (circles at cx=5/11, cy=4/8/12, r=1.25) |

### Content

| Property | Value |
|----------|-------|
| flex | `1` |
| min-width | `0` |
| font-family | `var(--poodle-typography-body-family)` |
| font-size | `var(--poodle-typography-body-size)` |
| color | `var(--poodle-color-text-primary)` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Uses native HTML5 drag-and-drop API (`dragstart`, `dragover`, `drop`, `dragend`)
- `event.dataTransfer.effectAllowed = "move"` for correct drag cursor
- `moveItem()` splices and re-inserts; dispatches `reorder` after mutation
- Keyboard reordering uses `requestAnimationFrame` to focus the moved item after DOM update
- Items keyed by `item.id` for stable Svelte `{#each}` rendering

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

| Feature | Svelte | GPUI | Jetstream |
|---------|--------|------|-----------|
| Drag-and-drop reorder | Yes | -- | -- |
| Keyboard reorder (Alt+Arrow) | Yes | -- | -- |
| Drag handle icon | Yes | -- | -- |
| Item slot | Yes | -- | -- |
| Disabled state | Yes | -- | -- |
| Drop target highlight | Yes | -- | -- |
| Focus management after reorder | Yes | -- | -- |

## 12. Known Deltas

None yet (single implementation).

## 13. Specimen Definitions

### Drag To Reorder

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Drag to reorder | 5 items, `ariaLabel="Reorderable items"`, reorder event logs new order | List with drag handles; drag or Alt+Arrow to reorder |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | 2 items, `disabled=true` | Reduced opacity, no drag handles active, no interaction |

## 14. Approval And Adoption Notes

Use `ReorderableList` for user-controlled ordering of small to medium lists (settings panels, priority lists, step sequences). For large lists requiring virtualization, consider a custom implementation. The component supports custom item rendering via the `item` slot for rich content beyond simple labels.
