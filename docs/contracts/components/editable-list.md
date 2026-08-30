# EditableList

Status: detailed contract
Updated: 2026-08-30

## 1. Purpose

- Component name: `EditableList`
- Layer: `composites`
- Summary: a unified editable/reorderable list surface that lets users add, remove, and reorder items with optional submit/cancel workflow chrome, large-list guidance, and page-window mode
- In scope: text input for adding items, per-item remove button, drag-and-drop reordering, keyboard reordering, drag handle, item slot, disabled state, submit/cancel workflow, dirty gating, async submit, error surface, info surface, live announcements, long-list guidance, page-window mode, grabbed item visual, max item limit with counter, static (non-reorderable) mode, size and density variants
- Out of scope: inline editing of existing items, rich item content (icons, descriptions), multi-field items, validation beyond empty-string prevention, multi-list transfer, nested lists, horizontal orientation, sortable grids, virtualization

## 2. Anatomy

```text
[Session Root .editable-list-session]  <div>
  ├── [Live Region .editable-list-session__sr]  sr-only, aria-live="polite"
  ├── [Header .editable-list-session__header]  optional workflow chrome
  │     ├── [Cancel Button]  secondary
  │     └── [Submit Button]  primary, disabled unless dirty
  ├── [Error .editable-list-session__error]  optional, role="alert"
  ├── [Info .editable-list-session__info]  optional, role="status"
  ├── [Long List Warning]  optional, role="status" (reuses __info styling)
  ├── [Window Nav .editable-list-session__window-nav]  optional, when windowed
  │     ├── [Previous Button]
  │     ├── [Window Label .editable-list-session__window-label]  "Page X of Y · Items A-B of N"
  │     └── [Next Button]
  ├── [List .editable-list]  <ul> role="listbox"
  │     └── [Item .editable-list__item]  <li> role="option" (repeated)
  │           ├── [Handle .editable-list__handle]  6-dot grip SVG, aria-hidden (when reorderable and not embeddedHandle)
  │           ├── [Content .editable-list__content]  slot or default label text
  │           └── [Remove .editable-list__remove]  <div> wrapping ghost IconButton (icon="x", sizeRole="chrome") (when editable or removable)
  ├── [Add Row .editable-list__add]  (when canAdd)
  │     ├── [Input .editable-list__add-input]  <div> wrapping TextInput primitive
  │     └── [Add Button .editable-list__add-btn]  <div> wrapping primary Button primitive
  └── [Counter .editable-list__count]  <span> "N/M" (when editable and maxItems is set)
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Session Root | `<div>` | yes | Flex column wrapper, wraps in `UiPresentationProvider` |
| Live Region | `<div>` | yes | Visually hidden, `aria-live="polite"`, `aria-atomic="true"` for move announcements |
| Header | `<div>` | conditional | Workflow chrome row with cancel/submit buttons; shown when `onSubmit` or `onCancel` is non-null |
| Error | `<div>` | conditional | Error alert surface, `role="alert"`, danger-styled |
| Info | `<div>` | conditional | Info status surface, `role="status"`, accent-styled |
| Long List Warning | `<div>` | conditional | Same styling as info; shown when item count exceeds `longListThreshold` |
| Window Nav | `<div>` | conditional | Flex row with previous/next buttons and page label; shown when `windowSize` is active |
| Window Label | `<span>` | conditional | "Page X of Y · Items A-B of N" text |
| List | `<ul>` | yes | `role="listbox"`, `aria-label`, flex column with gap |
| Item | `<li>` | yes (per item) | `role="option"`, `tabindex="0"`, `aria-selected="false"`, `data-reorder-index`; reorderable rows register as a paired Poodle drag source/target rather than HTML `draggable` |
| Handle | `<span>` | conditional | 6-dot grip icon, `aria-hidden="true"`, cursor grab; shown when `reorderable` and not `embeddedHandle` |
| Content | `<span>` | yes (per item) | Flex-grow content area, renders slot or `item.label` fallback to `item.id` |
| Remove | `<div>` + IconButton | conditional | Wrapper div (`--danger-on-hover`) around a ghost `IconButton` (icon `x`, `sizeRole="chrome"`); shown when `editable` or `removable`. The X icon is `aria-hidden`; the button carries `aria-label="Remove {label}"` |
| Add Row | `<div>` | conditional | Input + add button row; shown when `canAdd` (editable, not disabled, under max limit) |
| Input | `<div>` + TextInput | conditional | Wrapper div around the `TextInput` primitive for new item entry |
| Add Button | `<div>` + Button | conditional | Wrapper div around a primary `Button` primitive that confirms the add |
| Counter | `<span>` | conditional | "N/M" count display; shown when `editable` and `maxItems` is set |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `T[]` (generic, extends `{ id: string; label?: string }`) | `[]` | no | List items (two-way bindable), keyed by `item.id` |
| `ariaLabel` | `string` | `"Editable list"` | no | Accessible label for the list |
| `disabled` | `boolean` | `false` | no | Disables all interactions |
| `reorderable` | `boolean` | `true` | no | Enables drag-and-drop reordering with drag handles |
| `editable` | `boolean` | `false` | no | Show add-item input and remove buttons |
| `addLabel` | `string` | `"Add item"` | no | Label text for the add button |
| `addPlaceholder` | `string` | `"New item"` | no | Placeholder text for the input |
| `maxItems` | `number \| null` | `null` | no | Maximum number of items; null means unlimited |
| `removable` | `boolean` | `false` | no | Show remove buttons without enabling add input |
| `embeddedHandle` | `boolean` | `false` | no | When true, the item drops its own padding and omits the standalone handle (host renders the grip inside its `item` slot); sets `--embedded-handle` modifiers |
| `dirty` | `boolean` | `false` | no | Enables submit button when true |
| `submitting` | `boolean` | `false` | no | Disables interaction and shows "Saving..." state on submit button |
| `errorMessage` | `string \| null` | `null` | no | Optional workflow error, rendered as alert |
| `infoMessage` | `string \| null` | `null` | no | Optional workflow guidance, rendered as status |
| `longListThreshold` | `number \| null` | `50` | no | Show large-list guidance when item count exceeds the threshold |
| `longListWarningText` | `string \| null` | `null` | no | Custom large-list guidance copy; defaults to generated text |
| `windowSize` | `number \| null` | `null` | no | Optional page window size for very large reorder sessions |
| `submitLabel` | `string` | `"Save Order"` | no | Submit button label when workflow chrome is shown |
| `cancelLabel` | `string` | `"Cancel"` | no | Cancel button label when workflow chrome is shown |
| `showWorkflowChrome` | `boolean` | `true` | no | Gate for the workflow header; chrome shows only when this is true **and** `onSubmit` or `onCancel` is non-null |
| `onSubmit` | `(() => void \| Promise<void>) \| null` | `null` | no | Optional submit callback; when non-null enables workflow chrome |
| `onCancel` | `(() => void) \| null` | `null` | no | Optional cancel callback; when non-null enables workflow chrome |
| `onReorder` | `((items: T[]) => void) \| null` | `null` | no | Callback fired when items are reordered |
| `onAdd` | `((item: T) => void) \| null` | `null` | no | Callback fired when a new item is added |
| `onRemove` | `((id: string) => void) \| null` | `null` | no | Callback fired when an item is removed |
| `onChange` | `((items: T[]) => void) \| null` | `null` | no | Callback fired whenever the list items change |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for row and handle geometry |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for row padding and list spacing |

### Types

```ts
// Generic: T extends { id: string; label?: string }

type EditableListItem = {
  id: string;
  label: string;
};

/** @deprecated Use EditableListItem */
type ReorderableItem = EditableListItem;
```

### Slots

| Slot | Props | Notes |
|------|-------|-------|
| `item` | `T` (the item) | Custom content for each list item; falls back to `item.label ?? item.id` |

### Controlled / Uncontrolled

`items` is intentionally bindable. Reorder, add, and remove operations produce
new arrays rather than mutating the existing one in place, then surface the
result through the bindable prop plus `onReorder` and `onChange`. The windowed
view uses `windowStart` offset for correct global indexing.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| empty | `items` is empty | No list items shown; only add row (if canAdd) |
| populated | `items` has entries | List shown with items, add row below if canAdd |
| at-max | `items.length >= maxItems` | Add row hidden; counter shows "N/N" |
| default | -- | Transparent background, transparent border |
| hover | Mouse over item | `elevated 82%` over transparent background blend |
| focus | Focus-visible on item | `accent-focusRing` border-color + `box-shadow` ring (`border-width-focus`) |
| dragging | Item being dragged via the Poodle web drag substrate | 40% opacity on the dragged item |
| drop-target | Dragging over another item | `accent-base 56%` border, `accent-base 10%` tinted background |
| grabbed | Item grabbed via keyboard (Space/Enter) | Same visual as drop-target (`--grabbed`/`--last-moved` share it) |
| disabled | `disabled=true` or `submitting=true` | `state-opacity-disabled` on list, `pointer-events: none` via `tabindex="-1"` |
| add-disabled | input is empty or whitespace | Add button is disabled (cursor: not-allowed, reduced opacity) |

### Component States

| State | Description |
|-------|-------------|
| idle | No drag in progress, no grabbed item |
| dragging | An item is being dragged via mouse; source item faded, drop targets highlighted |
| keyboard-grabbed | An item is grabbed via keyboard; arrow keys move it, Escape cancels |
| submitting | `submitting=true`; all interaction disabled, submit button shows "Saving..." |
| windowed | `windowSize` active; list shows a page slice with navigation controls |
| `newItemText` (internal) | Current input value |
| `canAdd` (derived) | `editable && !disabled && (maxItems === null || items.length < maxItems)` |

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Machine-backed via core machinery: pointer, touch, and keyboard reorder
share one drop path. Web runtimes register each enabled reorderable row as
a Poodle drag source/target and commit through `applyReorder`. Keyboard
pickup, intent, drop, and Escape use that same session. Announcements,
focus return, and row composition stay adapter-side. HTML `DataTransfer`
is not session authority.

## 5. Callbacks

| Callback | When It Fires | Payload |
|----------|---------------|---------|
| `onReorder` | Items reordered via drag-and-drop or keyboard | `T[]` |
| `onAdd` | New item added via the input | `T` |
| `onRemove` | Item removed via the remove button | `string` |
| `onChange` | Item added, removed, or reordered | `T[]` |
| `onSubmit` | Submit action triggered | none |
| `onCancel` | Cancel action triggered | none |

## 6. Accessibility

### Semantics

- Root list is `<ul>` with `role="listbox"` and `aria-label`
- Each item is `<li>` with `role="option"`, `aria-selected="false"`, and detailed `aria-label`: "Reorder {label}. Position {n} of {total}. Press space to grab, then arrow keys to move."
- Handle is `aria-hidden="true"` (decorative)
- Items have `data-reorder-index` for programmatic focus management
- Live region announces all moves: "Moved {label} to position {n} of {total}."
- Remove buttons: `aria-label="Remove {item.label}"`
- Remove icon SVG: `aria-hidden="true"`
- Add button and input: standard form control semantics
- Error surface has `role="alert"`
- Info and long-list warning surfaces have `role="status"`

### Keyboard

| Key | Action |
|-----|--------|
| `Space` / `Enter` | Pick up the focused reorderable item, or drop it on the current intent |
| `ArrowUp` | Move drop intent to the previous row while dragging, or focus the previous item when idle |
| `ArrowDown` | Move drop intent to the next row while dragging, or focus the next item when idle |
| `Escape` | Cancel the active drag session |
| `Enter` (in input) | Adds the current text as a new item (prevents default form submission) |
| `Tab` | Navigates between items, input, add button, and remove buttons |

After keyboard reorder, focus follows the moved item to its new position via `requestAnimationFrame` and `querySelector`.

When `windowSize` is enabled, `ensureIndexVisible()` adjusts the page window so the moved item remains visible, allowing continuous keyboard reordering across page boundaries.

### Focus

- Items have `tabindex="0"` (or `-1` when disabled/submitting)
- Focus ring: `accent-focusRing` border-color plus a `box-shadow` ring at `border-width-focus`
- After adding an item, input is cleared but retains focus

## 7. Layout

### Sizing

- Session root: flex column, gap `0.75rem`
- Header: flex row, end-justified, gap `0.5rem`, bottom border
- Error/info panels: padding `0.75rem`, `radius-surface`, `font-size: 0.875rem`
- Window nav: flex row, end-justified, gap `0.5rem`
- Window label: min-width `13rem`, centered, secondary color
- List: flex column with semantic density-driven gap, no list styling, no margin/padding
- Item: flex row, centered, semantic density-driven gap and padding, border `0.0625rem solid transparent`, `radius-control`
- Handle: semantic size-driven square grip, flex-shrink 0
- Content: flex 1, min-width 0
- Remove button: semantic size-driven square control
- Remove icon SVG: `0.75rem` square
- Add row: flex, `align-items: center`, gap `0.5rem`
- Input: wrapper is `flex: 1 1 auto`, min-width 0; the composed `TextInput` owns its control-height/padding/border/focus
- Add button: wrapper is `flex-shrink: 0`; the composed primary `Button` owns geometry/typography/states
- Counter: label-size, secondary color, aligned to flex-end

### Composition

Standalone component. Uses `Button` for workflow chrome (header and window nav). Items can be customized via the `item` slot. Wraps children in `UiPresentationProvider`.

## 8. Token Usage -- Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-disabled` | session root and list | `"true"`, `"false"` |
| `data-reorder-index` | item `<li>` | numeric index (global, not local) |
| `data-size` | session root and list | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | session root and list | `"compact"`, `"default"`, `"comfortable"` |

### Session Root `.editable-list-session`

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-direction | `column` |
| gap | `0.75rem` |

### Live Region `.editable-list-session__sr`

| Property | Value |
|----------|-------|
| position | `absolute` |
| width | `1px` |
| height | `1px` |
| padding | `0` |
| margin | `-1px` |
| overflow | `hidden` |
| clip | `rect(0, 0, 0, 0)` |
| white-space | `nowrap` |
| border | `0` |

### Header `.editable-list-session__header`

| Property | Value |
|----------|-------|
| display | `flex` |
| justify-content | `flex-end` |
| align-items | `center` |
| gap | `0.5rem` |
| padding-bottom | `0.5rem` |
| border-bottom | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 76%, transparent)` |

### Error `.editable-list-session__error`

| Property | Value |
|----------|-------|
| padding | `0.75rem` |
| border-radius | `var(--poodle-radius-surface)` |
| font-size | `0.875rem` |
| border | `0.0625rem solid color-mix(in srgb, var(--poodle-color-status-danger) 40%, transparent)` |
| background | `color-mix(in srgb, var(--poodle-color-status-danger) 8%, var(--poodle-color-background-surface))` |
| color | `var(--poodle-color-status-danger)` |

### Info `.editable-list-session__info`

| Property | Value |
|----------|-------|
| padding | `0.75rem` |
| border-radius | `var(--poodle-radius-surface)` |
| font-size | `0.875rem` |
| border | `0.0625rem solid color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent)` |
| background | `color-mix(in srgb, var(--poodle-color-accent-base) 6%, var(--poodle-color-background-surface))` |
| color | `var(--poodle-color-text-primary)` |

### Window Nav `.editable-list-session__window-nav`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `flex-end` |
| gap | `0.5rem` |

### Window Label `.editable-list-session__window-label`

| Property | Value |
|----------|-------|
| min-width | `13rem` |
| text-align | `center` |
| font-size | `0.875rem` |
| color | `var(--poodle-color-text-secondary)` |

### Recipe Custom Properties

| Property | Default |
|----------|---------|
| `--poodle-editable-list-gap` | `0.125rem` |
| `--poodle-editable-list-item-gap` | `0.5rem` |
| `--poodle-editable-list-item-x` | `0.625rem` |
| `--poodle-editable-list-item-y` | `0.5rem` |
| `--poodle-editable-list-handle-size` | `1rem` |
| `--poodle-editable-list-font-size` | `0.8125rem` |

### List `.editable-list`

| Property | Value |
|----------|-------|
| list-style | `none` |
| margin | `0` |
| padding | `0` |
| display | `flex` |
| flex-direction | `column` |
| gap | `var(--poodle-editable-list-gap)` |

#### List Disabled `[data-disabled="true"]`

| Property | Value |
|----------|-------|
| opacity | `var(--poodle-state-opacity-disabled)` |

### Item `.editable-list__item`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `var(--poodle-editable-list-item-gap)` |
| padding | `var(--poodle-editable-list-item-y) var(--poodle-editable-list-item-x)` |
| border | `0.0625rem solid transparent` |
| border-radius | `var(--poodle-radius-control)` |
| background | `transparent` |
| outline | `none` |
| transition | `background-color, border-color, box-shadow, opacity` at `120ms ease` |

#### Item States

| State | Property | Value |
|-------|----------|-------|
| `:hover` | background | `color-mix(in srgb, var(--poodle-color-background-elevated) 82%, transparent)` |
| `:focus-visible` | border-color | `var(--poodle-color-accent-focusRing)` |
| `:focus-visible` | box-shadow | `0 0 0 var(--poodle-border-width-focus) var(--poodle-color-accent-focusRing)` |
| `--dragging` | opacity | `0.4` |
| `--drop-target` / `--grabbed` / `--last-moved` | border-color | `color-mix(in srgb, var(--poodle-color-accent-base) 56%, transparent)` |
| `--drop-target` / `--grabbed` / `--last-moved` | background | `color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent)` |
| `--embedded-handle` | padding | `0` |

### Handle `.editable-list__handle`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| flex-shrink | `0` |
| width | `var(--poodle-editable-list-handle-size)` |
| height | `var(--poodle-editable-list-handle-size)` |
| color | `var(--poodle-color-text-secondary)` |
| cursor | `grab` |
| SVG | `width: 100%; height: 100%` |
| SVG icon | 6-dot grip pattern (`viewBox="0 0 16 16"`, circles at cx=5/11, cy=4/8/12, r=1.25) |

### Content `.editable-list__content`

| Property | Value |
|----------|-------|
| flex | `1` |
| min-width | `0` |
| font-family | `var(--poodle-typography-body-family)` |
| font-size | `var(--poodle-editable-list-font-size)` |
| color | `var(--poodle-color-text-primary)` |

### Remove `.editable-list__remove` (wrapper around ghost `IconButton`)

The wrapper is layout-only; geometry/typography come from the composed `IconButton` primitive (`icon="x"`, `variant="ghost"`, `sizeRole="chrome"`). The wrapper carries `--danger-on-hover` styling overrides.

| Property | Value |
|----------|-------|
| flex-shrink | `0` |
| inner `.poodle-icon-button` color | `var(--poodle-color-text-secondary)` |

#### `--danger-on-hover` hover/focus-visible (`:not(:disabled)`)

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-status-danger)` |
| border-color | `color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default))` |
| background | `color-mix(in srgb, var(--poodle-color-status-danger) 10%, transparent)` |

### Add Row `.editable-list__add`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.5rem` |

### Add Input `.editable-list__add-input` (wrapper around `TextInput`)

Layout-only wrapper; input geometry/typography/focus styling come from the composed `TextInput` primitive (which carries its own control-height, control-x padding, border, radius, and focus ring).

| Property | Value |
|----------|-------|
| flex | `1 1 auto` |
| min-width | `0` |

### Add Button `.editable-list__add-btn` (wrapper around primary `Button`)

Layout-only wrapper; the composed primary `Button` (disabled when the input is empty/whitespace or `!canAdd`) owns geometry, typography, hover, and disabled styling.

| Property | Value |
|----------|-------|
| flex-shrink | `0` |

### Counter `.editable-list__count`

| Property | Value |
|----------|-------|
| font-size | `var(--poodle-typography-label-size)` |
| color | `var(--poodle-color-text-secondary)` |
| font-variant-numeric | `tabular-nums` |
| align-self | `flex-end` |

### Size Adjustments

| Size | Handle size | Item X padding | Item Y padding | Font size |
|------|-----------|----------------|----------------|-----------|
| `xs` | `0.875rem` | `0.5rem` | `0.375rem` | `0.6875rem` |
| `sm` | `1rem` | `0.625rem` | `0.4375rem` | `0.75rem` |
| `md` (default) | `1rem` | `0.625rem` | `0.5rem` | `0.8125rem` |
| `lg` | `1.125rem` | `0.75rem` | `0.5625rem` | `0.875rem` |
| `xl` | `1.25rem` | `0.875rem` | `0.625rem` | `0.9375rem` |

### Density Adjustments

| Density | List gap | Item gap |
|---------|----------|----------|
| `compact` | `0.0625rem` | `0.375rem` |
| `default` | `0.125rem` | `0.5rem` |
| `comfortable` | `0.1875rem` | `0.625rem` |

## 9. Svelte Notes

- Generic component: `<script lang="ts" generics="T extends { id: string; label?: string }">`
- Wraps in `UiPresentationProvider` to propagate resolved size and density
- Uses the Poodle web drag substrate (`DragDropProvider` plus per-row source/target registration). Pointer Events and the keyboard sensor own the session; HTML `dragstart` / `DataTransfer` are not used
- A dedicated handle is the pointer sensor when present; embedded-handle rows use the whole row and ignore interactive descendants (remove buttons, inputs)
- `moveItem(fromIndex, toIndex)` splices and re-inserts; calls `onReorder` and `onChange`; calls `ensureIndexVisible()` for windowed mode; announces move via live region
- Keyboard reordering uses `requestAnimationFrame` to focus the moved item after DOM update via `querySelector('[data-reorder-index="${targetIndex}"]')`
- Keyboard grab posture is the substrate dragging snapshot (`--grabbed`); drop-target and dragging classes follow provider source/target ids
- `isUnavailable` computed from `disabled || submitting`
- `showWorkflowChrome` computed from `onSubmit !== null || onCancel !== null`
- `isLongList` computed from `longListThreshold` and item count
- Windowed mode: `visibleItems = items.slice(windowStart, windowEnd)`, global index = `windowStart + localIndex`
- Items keyed by `item.id` for stable Svelte `{#each}` rendering
- `handleSubmit()` awaits `onSubmit()` when dirty and available
- `handleCancel()` calls `onCancel()`
- New item IDs generated with `Date.now()` + random suffix
- Input `keydown` handler prevents default on Enter to avoid form submission
- Remove button uses `stopPropagation` to prevent drag initiation
- `item` slot uses Svelte 5 `Snippet` type

## 9a. React Notes

React matches the Svelte web contract through the same provider-scoped
controller. Rows use `useDragSource` / `useDropTarget`. The public
`onReorder` / `onChange` payload remains the complete next item order.

## 10. GPUI Notes

A GPUI composite exists (`poodle_gpui::composites::editable_list`). It is currently presentational: it renders rows, handle, remove, add row, and workflow chrome, but does not yet implement callbacks, drag-and-drop, keyboard grab/move, the live region, or `<ul>`/`role` semantics. Reordering may use a simplified drag or move-up/move-down approach. Text input and add button compose from primitives.

## 10a. Jetstream Notes

- `EditableList::from_spec(spec, theme).on_remove(...).on_submit(...).on_cancel(...)`.
- `on_remove` carries the row's index, matching the contract's own index-based
  `onRemove`.
- No `on_add`: the add button renders disabled, because the draft field is typed
  and this runtime cannot know it has content — a handler on a control that can
  never be pressed would be dead by construction.
- No `on_reorder` (drag-with-payload) or `on_change` (typed).

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] All props have the same meaning and defaults
- [ ] Callback names and payloads match
- [ ] EditableListItem type is identical
- [ ] Drag-and-drop reorder behavior matches
- [ ] Keyboard reorder behavior matches (grab, move, drop, cancel)
- [ ] maxItems enforcement matches (hides add row, shows counter)
- [ ] Enter-to-add behavior matches
- [ ] Workflow chrome visibility gated on `onSubmit`/`onCancel`
- [ ] Dirty gating on submit button
- [ ] Windowed mode paging and navigation matches

### Tier 2: Visual Parity

- [ ] Item styling matches (default, hover, focus, dragging, drop-target, grabbed)
- [ ] Handle icon and sizing matches
- [ ] Remove button color and hover matches
- [ ] Input and add button styling matches
- [ ] Counter positioning and styling matches
- [ ] Error/info panel styling matches
- [ ] Size and density adjustments match

### Tier 3: Implementation Freedom

- [ ] Internal state management approach may differ
- [ ] Drag-and-drop API may differ across platforms
- [ ] Live region announcement timing may differ
- [ ] Reorder mechanism may differ (drag-and-drop vs buttons)

## 12. Specimen Definitions

### Editable With Add/Remove

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Editable + reorderable | three initial items ("svelte", "typescript", "design-system"), `editable`, `ariaLabel="Tags"`, `addPlaceholder="Add a tag..."`, `addLabel="Add"` | Draggable list with remove buttons and add row below |

### With Max Items (5)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With max items (5) | two initial items ("Item A", "Item B"), `editable`, `maxItems={5}`, `ariaLabel="Limited list"`, `addPlaceholder="Add item..."` | List with two items, add row, and "2/5" counter |

### Removable Only

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Removable only | two items, `reorderable={false}`, `removable`, `ariaLabel="Static list"` | Static list (no drag handles) with remove buttons, no add row |

### Drag To Reorder

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Drag to reorder | 5 items, `ariaLabel="Reorderable items"`, `onReorder` logs new order | List with drag handles; drag or keyboard (Space then Arrow) to reorder |

### Workflow Chrome

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With workflow | 5 items, `onSubmit` and `onCancel` provided, `dirty=false` initially | Header with disabled Save Order button and Cancel button; submit enables after reorder |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | items, `disabled=true` | Reduced opacity, no drag handles active, no interaction |

### Windowed Mode

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Windowed | 12 items, `windowSize=5`, `longListThreshold=8` | Window nav with Previous/Next buttons and page label; 5 items visible per page |

### Semantic Presentation

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Semantic presentation | Compact density, sm size via `UiPresentationProvider` | Tighter spacing and smaller handle/text |
