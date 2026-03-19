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
