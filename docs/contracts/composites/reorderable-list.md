# ReorderableList

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `ReorderableList`
- Layer: `composites`
- Summary: a vertical reorder surface with optional submit/cancel workflow chrome for controlled reorder sessions, including large-list guidance and optional page-window mode
- In scope: drag-and-drop reordering, keyboard reordering, drag handle, item slot, disabled state, submit/cancel workflow, dirty gating, async submit, error surface, info surface, live announcements, long-list guidance, page-window mode, grabbed item visual
- Out of scope: multi-list transfer, nested lists, horizontal orientation, sortable grids, virtualization

## 2. Anatomy

```text
[Session Root]
  ├── [Live Region]  sr-only, aria-live="polite"
  ├── [Header]  optional workflow chrome
  │     ├── [Cancel Button]  secondary
  │     └── [Submit Button]  primary, disabled unless dirty
  ├── [Error]   optional, role="alert"
  ├── [Info]    optional, role="status"
  ├── [Long List Warning]  optional, role="status"
  ├── [Window Nav]  optional, when windowed
  │     ├── [Previous Button]
  │     ├── [Window Label]  "Page X of Y - Items A-B of N"
  │     └── [Next Button]
  └── [List]  <ul> role="listbox"
        └── [Item...]  <li> role="option"
              ├── [Handle]  6-dot grip SVG, aria-hidden
              └── [Content]  slot or default label text
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| session-root | `<div>` | Flex column wrapper, class `reorderable-list-session` |
| live-region | `<div>` | Visually hidden, `aria-live="polite"`, `aria-atomic="true"` for move announcements |
| header | `<div>` | Workflow chrome row with cancel/submit buttons, bottom border |
| error | `<div>` | Error alert surface, `role="alert"`, danger-styled |
| info | `<div>` | Info status surface, `role="status"`, accent-styled |
| long-list-warning | `<div>` | Same styling as info, shows when item count exceeds `longListThreshold` |
| window-nav | `<div>` | Flex row with previous/next buttons and page label |
| window-label | `<span>` | "Page X of Y - Items A-B of N" text |
| list | `<ul>` | `role="listbox"`, `aria-label`, flex column with gap |
| item | `<li>` | `role="option"`, `tabindex="0"`, `draggable`, `aria-selected="false"`, `data-reorder-index` |
| handle | `<span>` | 6-dot grip icon, `aria-hidden="true"`, cursor grab |
| content | `<span>` | Flex-grow content area, renders slot or `item.label` fallback to `item.id` |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `T[]` (generic, extends `{ id: string; label?: string }`) | `[]` | no | List items (two-way bindable), keyed by `item.id` |
| `ariaLabel` | `string` | `"Reorderable list"` | no | Accessible label for the list |
| `disabled` | `boolean` | `false` | no | Disables drag, keyboard reorder, and all interaction |
| `dirty` | `boolean` | `false` | no | Enables submit button when true |
| `submitting` | `boolean` | `false` | no | Disables interaction and shows "Saving..." state on submit button |
| `errorMessage` | `string \| null` | `null` | no | Optional workflow error, rendered as alert |
| `infoMessage` | `string \| null` | `null` | no | Optional workflow guidance, rendered as status |
| `longListThreshold` | `number \| null` | `50` | no | Show large-list guidance when item count exceeds the threshold |
| `longListWarningText` | `string \| null` | `null` | no | Custom large-list guidance copy; defaults to generated text |
| `windowSize` | `number \| null` | `null` | no | Reorder large lists in page windows of this size |
| `submitLabel` | `string` | `"Save Order"` | no | Submit button label |
| `cancelLabel` | `string` | `"Cancel"` | no | Cancel button label |
| `onsubmit` | `(() => void \| Promise<void>) \| null` | `null` | no | Optional submit callback; when non-null enables workflow chrome |
| `oncancel` | `(() => void) \| null` | `null` | no | Optional cancel callback; when non-null enables workflow chrome |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for row and handle geometry |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for row padding and list spacing |

### Types

```ts
// Generic: T extends { id: string; label?: string }

type ReorderableItem = {
  id: string;
  label: string;
};
```

### Slots

| Slot | Props | Notes |
|------|-------|-------|
| `item` | `T` (the item) | Custom content for each list item; falls back to `item.label ?? item.id` |

### Controlled / Uncontrolled

`items` supports two-way binding. The component mutates the array order internally (splice and re-insert) and dispatches `reorder` with the updated array. The windowed view uses `windowStart` offset for correct global indexing.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| default | -- | Surface background, transparent border |
| hover | Mouse over item | Elevated background blend |
| focus | Focus-visible on item | Focus ring: `border-width-focus` solid `accent-focusRing`, offset `-0.0625rem` |
| dragging | Item being dragged (HTML5 DnD) | 40% opacity on the dragged item |
| drop-target | Dragging over another item | `accent-base` border color, accent-tinted background |
| grabbed | Item grabbed via keyboard (Space/Enter) | Same visual as drop-target (accent border + tinted background) |
| disabled | `disabled=true` or `submitting=true` | `state-opacity-disabled` on list, `pointer-events: none` via `tabindex="-1"` |

### Component States

| State | Description |
|-------|-------------|
| idle | No drag in progress, no grabbed item |
| dragging | An item is being dragged via mouse; source item faded, drop targets highlighted |
| keyboard-grabbed | An item is grabbed via keyboard; arrow keys move it, Escape cancels |
| submitting | `submitting=true`; all interaction disabled, submit button shows "Saving..." |
| windowed | `windowSize` active; list shows a page slice with navigation controls |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `reorder` | Items reordered via drag-and-drop or keyboard | `{ items: T[] }` |
| `submit` | Submit action triggered (before callback) | `void` |
| `cancel` | Cancel action triggered (before callback) | `void` |

## 6. Accessibility

### Semantics

- Root list is `<ul>` with `role="listbox"` and `aria-label`
- Each item is `<li>` with `role="option"`, `aria-selected="false"`, and detailed `aria-label`: "Reorder {label}. Position {n} of {total}. Press space to grab, then arrow keys to move."
- Handle is `aria-hidden="true"` (decorative)
- Items have `data-reorder-index` for programmatic focus management
- Live region announces all moves: "Moved {label} to position {n} of {total}."
- Error surface has `role="alert"`
- Info and long-list warning surfaces have `role="status"`

### Keyboard

| Key | Action |
|-----|--------|
| `Space` / `Enter` | Toggle grab state on focused item (grab or drop) |
| `ArrowUp` | Move grabbed item up one position, or focus previous item |
| `ArrowDown` | Move grabbed item down one position, or focus next item |
| `Escape` | Cancel keyboard grab mode |
| `Tab` | Navigate between items |

After keyboard reorder, focus follows the moved item to its new position via `requestAnimationFrame` and `querySelector`.

When `windowSize` is enabled, `ensureIndexVisible()` adjusts the page window so the moved item remains visible, allowing continuous keyboard reordering across page boundaries.

### Focus

- Items have `tabindex="0"` (or `-1` when disabled/submitting)
- Focus ring: `border-width-focus` solid `accent-focusRing`, offset `-0.0625rem`

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

### Composition

Standalone component. Uses `Button` for workflow chrome (header and window nav). Items can be customized via the `item` slot. Wraps children in `UiPresentationProvider`.

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-disabled` | session root and list | `"true"`, `"false"` |
| `data-reorder-index` | item `<li>` | numeric index (global, not local) |
| `data-size` | session root and list | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | session root and list | `"compact"`, `"default"`, `"comfortable"` |

### Session Root `.reorderable-list-session`

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-direction | `column` |
| gap | `0.75rem` |

### Live Region `.reorderable-list-session__sr`

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

### Header `.reorderable-list-session__header`

| Property | Value |
|----------|-------|
| display | `flex` |
| justify-content | `flex-end` |
| align-items | `center` |
| gap | `0.5rem` |
| padding-bottom | `0.5rem` |
| border-bottom | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 76%, transparent)` |

### Error `.reorderable-list-session__error`

| Property | Value |
|----------|-------|
| padding | `0.75rem` |
| border-radius | `var(--poodle-radius-surface)` |
| font-size | `0.875rem` |
| border | `0.0625rem solid color-mix(in srgb, var(--poodle-color-status-danger) 40%, transparent)` |
| background | `color-mix(in srgb, var(--poodle-color-status-danger) 8%, var(--poodle-color-background-surface))` |
| color | `var(--poodle-color-status-danger)` |

### Info `.reorderable-list-session__info`

| Property | Value |
|----------|-------|
| padding | `0.75rem` |
| border-radius | `var(--poodle-radius-surface)` |
| font-size | `0.875rem` |
| border | `0.0625rem solid color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent)` |
| background | `color-mix(in srgb, var(--poodle-color-accent-base) 6%, var(--poodle-color-background-surface))` |
| color | `var(--poodle-color-text-primary)` |

### Window Nav `.reorderable-list-session__window-nav`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `flex-end` |
| gap | `0.5rem` |

### Window Label `.reorderable-list-session__window-label`

| Property | Value |
|----------|-------|
| min-width | `13rem` |
| text-align | `center` |
| font-size | `0.875rem` |
| color | `var(--poodle-color-text-secondary)` |

### List `.reorderable-list`

| Property | Value |
|----------|-------|
| list-style | `none` |
| margin | `0` |
| padding | `0` |
| display | `flex` |
| flex-direction | `column` |
| gap | `0.125rem` (default) |

#### List Disabled `[data-disabled="true"]`

| Property | Value |
|----------|-------|
| opacity | `var(--poodle-state-opacity-disabled)` |

### Item `.reorderable-list__item`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.5rem` (default) |
| padding | `0.5rem 0.625rem` (default) |
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
| `--grabbed` | border-color | `var(--poodle-color-accent-base)` |
| `--grabbed` | background | `color-mix(in srgb, var(--poodle-color-accent-base) 8%, var(--poodle-color-background-surface))` |

### Handle `.reorderable-list__handle`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| flex-shrink | `0` |
| width | `1rem` (default, varies by size) |
| height | `1rem` (default, varies by size) |
| color | `var(--poodle-color-text-secondary)` |
| cursor | `grab` |
| SVG | `width: 100%; height: 100%` |
| SVG icon | 6-dot grip pattern (`viewBox="0 0 16 16"`, circles at cx=5/11, cy=4/8/12, r=1.25) |

### Content `.reorderable-list__content`

| Property | Value |
|----------|-------|
| flex | `1` |
| min-width | `0` |
| font-family | `var(--poodle-typography-body-family)` |
| font-size | `var(--poodle-typography-body-size)` |
| color | `var(--poodle-color-text-primary)` |

### Size Adjustments

| Size | Handle size | Item X padding |
|------|-----------|----------------|
| `xs` | `0.875rem` | `0.5rem` |
| `sm` | `1rem` | `0.625rem` |
| `md` (default) | `1rem` | `0.625rem` |
| `lg` | `1.125rem` | `0.75rem` |
| `xl` | `1.25rem` | `0.875rem` |

### Density Adjustments

| Density | List gap | Item gap | Item Y padding |
|---------|----------|----------|---------------|
| `compact` | `0.0625rem` | `0.375rem` | `0.375rem` |
| `default` | `0.125rem` | `0.5rem` | `0.5rem` |
| `comfortable` | `0.1875rem` | `0.625rem` | `0.625rem` |

## 9. Svelte Notes

- Generic component: `<script lang="ts" generics="T extends { id: string; label?: string }">`
- Uses native HTML5 drag-and-drop API (`dragstart`, `dragover`, `drop`, `dragend`)
- `event.dataTransfer.effectAllowed = "move"` for correct drag cursor
- `moveItem(fromIndex, toIndex)` splices and re-inserts; dispatches `reorder` after mutation; calls `ensureIndexVisible()` for windowed mode; announces move via live region
- Keyboard reordering uses `requestAnimationFrame` to focus the moved item after DOM update via `querySelector('[data-reorder-index="${targetIndex}"]')`
- `grabbedIndex` tracks keyboard grab state; visual class `--grabbed` applied
- `isUnavailable` computed from `disabled || submitting`
- `showWorkflowChrome` computed from `onsubmit !== null || oncancel !== null`
- `isLongList` computed from `longListThreshold` and item count
- Windowed mode: `visibleItems = items.slice(windowStart, windowEnd)`, global index = `windowStart + localIndex`
- Items keyed by `item.id` for stable Svelte `{#each}` rendering
- `handleSubmit()` dispatches `submit` event then awaits `onsubmit()` callback
- `handleCancel()` dispatches `cancel` event then calls `oncancel()` callback, resets `grabbedIndex`
- `item` slot uses Svelte 5 `Snippet` type
- Wraps content in `UiPresentationProvider` with resolved size and density

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] All props have the same meaning and defaults
- [ ] Event names and payloads match
- [ ] Drag-and-drop reorder behavior matches
- [ ] Keyboard reorder behavior matches (grab, move, drop, cancel)
- [ ] Workflow chrome visibility gated on `onsubmit`/`oncancel`
- [ ] Dirty gating on submit button
- [ ] Windowed mode paging and navigation matches

### Tier 2: Visual Parity

- [ ] Item styling matches (default, hover, focus, dragging, drop-target, grabbed)
- [ ] Handle icon and sizing matches
- [ ] Error/info panel styling matches
- [ ] Size and density adjustments match

### Tier 3: Implementation Freedom

- [ ] Internal state management approach may differ
- [ ] Drag-and-drop API may differ across platforms
- [ ] Live region announcement timing may differ

## 12. Specimen Definitions

### Drag To Reorder

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Drag to reorder | 5 items, `ariaLabel="Reorderable items"`, reorder event logs new order | List with drag handles; drag or keyboard (Space then Arrow) to reorder |

### Workflow Chrome

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With workflow | 5 items, `onsubmit` and `oncancel` provided, `dirty=false` initially | Header with disabled Save Order button and Cancel button; submit enables after reorder |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | 3 items, `disabled=true` | Reduced opacity, no drag handles active, no interaction |

### Windowed Mode

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Windowed | 25 items, `windowSize=10` | Window nav with Previous/Next buttons and page label; 10 items visible per page |

### Error State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Error | 5 items, `errorMessage="Failed to save order"` | Danger-styled error panel above list |
