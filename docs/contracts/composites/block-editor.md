# BlockEditor

Status: seed contract
Updated: 2026-03-19

## 1. Purpose

- Component name: `BlockEditor`
- Layer: `composites`
- Summary: extensible block-based content editor with pluggable block types, reordering via drag-and-drop or buttons, and customisable block rendering via slots
- In scope: block CRUD, type switching, reordering (drag-and-drop + arrow buttons), extensible block type definitions, slotted block content rendering
- Out of scope: rich text editing within blocks, collaborative editing, undo/redo history, block nesting, server persistence

## 2. Anatomy

```
[Root]
  └── [Block] (repeated, keyed by block.id)
        ├── [Toolbar]
        │     ├── [Toolbar Left]
        │     │     ├── [Drag Grip] — draggable handle for reorder
        │     │     └── [Type Select] — dropdown to change block type
        │     └── [Toolbar Right]
        │           ├── [Move Up] — arrow button
        │           ├── [Move Down] — arrow button
        │           ├── [Add] — insert block after this one
        │           └── [Remove] — delete block (hidden when only 1 block)
        └── [Content] — slot-based; falls back to built-in renderers
  └── [Add Menu Overlay] (conditional)
        └── [Add Menu] — grid of block type options
```

## 3. Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `blocks` | `EditorBlock[]` | Single empty paragraph | Array of block data objects |
| `blockTypes` | `BlockTypeDefinition[]` | Built-in types (paragraph, heading, code, quote, list, image, divider) | Defines available block types for the type selector and add menu |
| `isDisabled` | `boolean` | `false` | Disables all editing controls |
| `ariaLabel` | `string` | `"Block editor"` | Accessible label for the root container |

## 4. Types

```ts
type BlockType = string;

type BlockTypeDefinition = {
  type: BlockType;
  label: string;
  icon: string;
};

type EditorBlock = {
  id: string;
  type: BlockType;
  content: string;
  [key: string]: unknown;  // extensible for custom data
};
```

`BlockType` is an open `string` — consumers define their own types via `blockTypes`. The built-in defaults provide paragraph, heading, code, quote, list, image, and divider.

`EditorBlock` uses an index signature so custom block types can store arbitrary data (e.g. `{ type: "embed", content: "", provider: "youtube", videoId: "abc" }`).

## 5. Events

| Event | Detail | Description |
|-------|--------|-------------|
| `change` | `{ blocks: EditorBlock[] }` | Fired after any mutation (add, remove, move, type change, content update) |

## 6. Slots

### `block` slot

Allows consumers to provide custom rendering for block content. Receives:

| Slot prop | Type | Description |
|-----------|------|-------------|
| `block` | `EditorBlock` | The current block data |
| `index` | `number` | Block position in the array |
| `isDisabled` | `boolean` | Whether editing is disabled |
| `update` | `(updates: Partial<EditorBlock>) => void` | Callback to update the block |

When the `block` slot is not provided, the component falls back to built-in renderers for the default block types: paragraph (textarea), heading (text input), code (monospace textarea), quote (italic bordered textarea), list (textarea), image (URL input + preview), and divider (horizontal rule). Unrecognised types fall back to a plain textarea.

## 7. Visual Rules

### Container
- Background: `background-surface`
- Border: `1px solid border-default`, `radius-surface`
- Padding: `0.75rem`
- Layout: flex column, `0.5rem` gap between blocks

### Block
- Background: `background-elevated` at 42% opacity (active: 72%)
- No border — differentiation is via background elevation only
- Border-radius: `radius-control`
- Drag-over: `0.125rem` accent box-shadow ring
- Dragging: 40% opacity

### Toolbar
- Flex row, space-between, `0.25rem 0.375rem` padding
- Transparent background, no border
- Buttons: `1.25rem` square, `text-tertiary`, hover shows accent background
- Remove button: hover shows `status-danger` background and text
- Type select: `0.6875rem` font, subtle border, transparent background
- Drag grip: `grip-vertical` icon, `grab` cursor, `text-tertiary`

### Add Menu
- Centered fixed overlay
- Grid of block type options with icon + label
- `background-elevated`, `border-default`, `elevation-overlay` shadow

### Token Usage — Exact CSS Values

#### `.block-editor` (Root)

| Property | Value |
|----------|-------|
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-surface)` |
| `padding` | `0.75rem` |
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.5rem` |

#### `.block-editor--disabled`

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |
| `pointer-events` | `none` |

#### `.block-editor__block`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `border` | `none` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 42%, transparent)` |
| `transition` | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard), box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.block-editor__block.active`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent)` |

#### `.block-editor__block.drag-over`

| Property | Value |
|----------|-------|
| `box-shadow` | `0 0 0 0.125rem var(--poodle-color-accent-base)` |

#### `.block-editor__block.dragging`

| Property | Value |
|----------|-------|
| `opacity` | `0.4` |

#### `.block-editor__toolbar`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `padding` | `0.25rem 0.375rem` |
| `border-bottom` | `none` |
| `background` | `transparent` |
| `border-radius` | `var(--poodle-radius-control) var(--poodle-radius-control) 0 0` |

#### `.block-editor__toolbar-left`, `.block-editor__toolbar-right`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.125rem` |

#### `.block-editor__drag-grip`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.25rem` |
| `height` | `1.25rem` |
| `color` | `var(--poodle-color-text-tertiary)` |
| `cursor` | `grab` |
| `border-radius` | `var(--poodle-radius-control)` |
| `transition` | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.block-editor__drag-grip:hover`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |
| `color` | `var(--poodle-color-text-secondary)` |

#### `.block-editor__drag-grip:active`

| Property | Value |
|----------|-------|
| `cursor` | `grabbing` |

#### `.block-editor__tool-btn`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.25rem` |
| `height` | `1.25rem` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-tertiary)` |
| `cursor` | `pointer` |
| `font-size` | `0.75rem` |
| `line-height` | `1` |
| `transition` | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.block-editor__tool-btn:hover`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |

#### `.block-editor__tool-btn:disabled`

| Property | Value |
|----------|-------|
| `opacity` | `0.3` |
| `cursor` | `default` |

#### `.block-editor__tool-btn:disabled:hover`

| Property | Value |
|----------|-------|
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-tertiary)` |

#### `.block-editor__remove-btn:hover:not(:disabled)`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-status-danger) 16%, transparent)` |
| `color` | `var(--poodle-color-status-danger)` |

#### `.block-editor__type-select`

| Property | Value |
|----------|-------|
| `padding` | `0.0625rem 0.25rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font` | `inherit` |
| `font-size` | `0.6875rem` |
| `cursor` | `pointer` |

#### `.block-editor__content`

| Property | Value |
|----------|-------|
| `padding` | `0.375rem 0.5rem` |
| `min-height` | `1.5rem` |

#### `.block-editor__input` (Default)

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `padding` | `0.25rem 0.375rem` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `0.875rem` |
| `line-height` | `1.6` |
| `outline` | `none` |
| `resize` | `vertical` |

#### `.block-editor__input--heading`

| Property | Value |
|----------|-------|
| `font-size` | `1.125rem` |
| `font-weight` | `700` |

#### `.block-editor__input--code`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.8125rem` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `padding` | `0.5rem` |

#### `.block-editor__input--quote`

| Property | Value |
|----------|-------|
| `border-left` | `0.1875rem solid var(--poodle-color-border-default)` |
| `padding-left` | `0.625rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-style` | `italic` |

#### `.block-editor__input--list`

| Property | Value |
|----------|-------|
| `padding-left` | `1rem` |

#### `.block-editor__input--image-url`

| Property | Value |
|----------|-------|
| `font-size` | `0.75rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-code-family)` |

#### `.block-editor__divider`

| Property | Value |
|----------|-------|
| `border` | `0` |
| `border-top` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `margin` | `0.5rem 0` |

#### `.block-editor__image-block`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.375rem` |

#### `.block-editor__image-preview img`

| Property | Value |
|----------|-------|
| `max-width` | `100%` |
| `max-height` | `16rem` |
| `border-radius` | `var(--poodle-radius-control)` |
| `object-fit` | `contain` |

#### `.block-editor__add-overlay`

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `inset` | `0` |
| `z-index` | `var(--poodle-overlay-z-menu, 100)` |

#### `.block-editor__add-menu`

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `display` | `grid` |
| `grid-template-columns` | `repeat(auto-fill, minmax(6rem, 1fr))` |
| `gap` | `0.25rem` |
| `padding` | `0.5rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-elevated)` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |
| `min-width` | `16rem` |

#### `.block-editor__add-menu-item`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `padding` | `0.5rem` |
| `border` | `0` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `font-size` | `0.8125rem` |
| `text-align` | `left` |
| `transition` | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.block-editor__add-menu-item:hover`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |

#### `.block-editor__add-menu-icon`

| Property | Value |
|----------|-------|
| `font-size` | `0.875rem` |
| `width` | `1.25rem` |
| `text-align` | `center` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-type` | `.block-editor__block` | block type identifier (paragraph, heading, code, etc.) |

### CSS Classes Used for State Selectors

| Class | Element | Purpose |
|-------|---------|---------|
| `.active` | `.block-editor__block` | currently focused/active block |
| `.drag-over` | `.block-editor__block` | drop target highlight during drag |
| `.dragging` | `.block-editor__block` | source block during drag |
| `.block-editor--disabled` | `.block-editor` | disabled state on root |

## 8. Reordering

Blocks can be reordered two ways:

1. **Arrow buttons**: up/down buttons in the toolbar. First block disables up, last block disables down.
2. **Drag-and-drop**: drag grip handle in the toolbar. Uses native HTML drag-and-drop. Drop target shows accent ring highlight. Source block shows reduced opacity while dragging.

## 9. Accessibility

- Root: `aria-label` from prop
- Each block: `role="group"`, `aria-label="{type} block"`
- Type select: `aria-label="Block type"`
- Move buttons: `aria-label="Move up"` / `"Move down"`
- Add button: `aria-label="Add block after this one"`
- Remove button: `aria-label="Remove block"`

## 10. Specimen Definitions

### Default Block Types

Uses built-in `blockTypes` and default slot rendering. Shows heading, paragraph, quote, code, list, divider, and paragraph blocks.

### Custom Block Types with Slot Rendering

Demonstrates extensibility: consumer provides custom `blockTypes` (text, callout, embed) and a `block` slot with custom rendering for each type. Callout has an accent left border, embed uses monospace URL input.

## 11. Next Task

Use `BlockEditor` as the content editing surface where block-structured content is needed. For rich text within individual blocks, compose with a dedicated rich-text component inside the block slot.
