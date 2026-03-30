# BlockEditor

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `BlockEditor`
- Layer: `composites`
- Summary: extensible block-based content editor with pluggable block types, reordering via drag-and-drop or buttons, and customisable block rendering via slots
- In scope: block CRUD, type switching, reordering (drag-and-drop + arrow buttons), extensible block type definitions, slotted block content rendering, size and density variants
- Out of scope: rich text editing within blocks, collaborative editing, undo/redo history, block nesting, server persistence

## 2. Anatomy

```text
[Root .block-editor]  <div> aria-label
  └── [Block .block-editor__block]  (repeated, keyed by block.id) role="group"
        ├── [Toolbar .block-editor__toolbar]
        │     ├── [ToolbarLeft .block-editor__toolbar-left]
        │     │     ├── [DragGrip .block-editor__drag-grip]  <span> draggable, Icon grip-vertical
        │     │     └── [TypeSelect .block-editor__type-select]  <select> block type picker
        │     └── [ToolbarRight .block-editor__toolbar-right]
        │           ├── [MoveUpBtn .block-editor__tool-btn]  <button> Icon arrow-up
        │           ├── [MoveDownBtn .block-editor__tool-btn]  <button> Icon arrow-down
        │           ├── [AddBtn .block-editor__tool-btn.block-editor__add-btn]  <button> Icon plus
        │           └── [RemoveBtn .block-editor__tool-btn.block-editor__remove-btn]  <button> Icon x (hidden when 1 block)
        └── [Content .block-editor__content]
              └── [BlockSlot]  named "block" slot; fallback: built-in renderer per type
  └── [AddOverlay .block-editor__add-overlay]  (conditional, when showAddMenu)
        └── [AddMenu .block-editor__add-menu]  grid of block type options
              └── [AddMenuItem .block-editor__add-menu-item]  <button> (repeated per blockType)
                    ├── [MenuIcon .block-editor__add-menu-icon]  Icon
                    └── [MenuLabel]  <span> type label
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Root | `<div>` | yes | Flex column container with `aria-label`, wraps in `UiPresentationProvider` |
| Block | `<div>` | yes (repeated) | `role="group"`, `aria-label="{type} block"`, drag-and-drop target |
| Toolbar | `<div>` | yes (per block) | Flex row, space-between, transparent background |
| DragGrip | `<span>` | yes (per block) | `draggable="true"`, `aria-hidden="true"`, `grab` cursor |
| TypeSelect | `<select>` | yes (per block) | Block type picker, `aria-label="Block type"` |
| MoveUpBtn | `<button>` | yes (per block) | Disabled when first block or editor disabled |
| MoveDownBtn | `<button>` | yes (per block) | Disabled when last block or editor disabled |
| AddBtn | `<button>` | yes (per block) | Opens add menu overlay |
| RemoveBtn | `<button>` | no | Hidden when only 1 block remains |
| Content | `<div>` | yes (per block) | Contains slot or built-in renderer |
| AddOverlay | `<div>` | no | Fixed overlay backdrop for add menu |
| AddMenu | `<div>` | no | Grid of block type buttons |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `blocks` | `EditorBlock[]` | Single empty paragraph | no | Array of block data objects; bind for two-way |
| `blockTypes` | `BlockTypeDefinition[]` | Built-in types (paragraph, heading, code, quote, list, image, divider) | no | Defines available block types for the type selector and add menu |
| `disabled` | `boolean` | `false` | no | Disables all editing controls |
| `ariaLabel` | `string` | `"Block editor"` | no | Accessible label for the root container |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for toolbar chrome and nested controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for shell, toolbar, content, and add-menu spacing |

### Types

```ts
type BlockType = string;

type BlockTypeDefinition = {
  type: BlockType;
  label: string;
  icon: IconProp;
};

type EditorBlock = {
  id: string;
  type: BlockType;
  content: string;
  [key: string]: unknown;  // extensible for custom data
};
```

### Slots

| Slot | Scope | Purpose |
|------|-------|---------|
| `block` | `{ block: EditorBlock, index: number, disabled: boolean, update: (updates: Partial<EditorBlock>) => void }` | Custom rendering for block content; falls back to built-in renderers |

### Default Block Renderers

When the `block` slot is not provided, built-in renderers handle these types:

| Type | Renderer | Notes |
|------|----------|-------|
| `paragraph` | `<textarea>` | Default fallback for unrecognised types |
| `heading` | `<input type="text">` | Bold, larger font |
| `code` | `<textarea>` | Monospace font, elevated background |
| `quote` | `<textarea>` | Italic, left border accent |
| `list` | `<textarea>` | Indented left padding |
| `image` | `<input>` + `<img>` preview | URL input with image preview when content is non-empty |
| `divider` | `<hr>` | Horizontal rule |

### Controlled And Uncontrolled

- `blocks` supports two-way binding (`bind:blocks`)
- Mutations also surfaced via the `change` event

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | blocks present, not disabled | Blocks shown with toolbars |
| disabled | `disabled=true` | Root has `opacity: disabled`, `pointer-events: none` |
| active-block | block receives focus | Block background increases to 72% elevated mix |
| dragging | drag grip held | Source block at 40% opacity |
| drag-over | dragging over another block | Target block shows accent box-shadow ring |
| add-menu-open | add button clicked | Fixed overlay with block type grid |

### Component States

| State | Description |
|-------|-------------|
| `activeBlockId` | ID of the currently focused block |
| `showAddMenu` | Whether the add menu overlay is visible |
| `addMenuIndex` | Index of the block after which to insert |
| `dragSourceIndex` | Index of the block being dragged |
| `dragOverIndex` | Index of the current drop target |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `change` | After any mutation (add, remove, move, type change, content update) | `{ blocks: EditorBlock[] }` | Fires with a shallow copy of the blocks array |

## 6. Accessibility

### Semantics

- Root: `aria-label` from prop (default `"Block editor"`)
- Each block: `role="group"`, `aria-label="{type} block"`
- Type select: `aria-label="Block type"`
- Move buttons: `aria-label="Move up"` / `"Move down"`
- Add button: `aria-label="Add block after this one"`
- Remove button: `aria-label="Remove block"`
- Drag grip: `aria-hidden="true"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | Navigates between toolbar controls and content inputs |
| `Space` / `Enter` | Activates focused buttons |

### Focus

- Tool buttons and type select use standard browser focus
- Newly added blocks receive focus via `tick().then()` after insertion

## 7. Layout

### Sizing

- Root: flex column with density-aware gap and padding
- Blocks: flex column, no explicit border (differentiation via background)
- Toolbar: flex row, space-between
- Content: padded area with min-height `1.5rem`
- Add menu: fixed position, grid layout

### Composition

- Composes: `Icon` primitive, `UiPresentationProvider`
- Parent expectations: content editing surfaces, form sections
- Resizing rules: blocks fill parent width; content areas are flexible

## 8. Token Usage -- Exact Values

### Recipe Custom Properties

| Property | Default |
|----------|---------|
| `--poodle-block-editor-shell-x` | `0.75rem` |
| `--poodle-block-editor-shell-y` | `0.75rem` |
| `--poodle-block-editor-stack-gap` | `0.5rem` |
| `--poodle-block-editor-toolbar-y` | `0.25rem` |
| `--poodle-block-editor-toolbar-x` | `0.375rem` |
| `--poodle-block-editor-toolbar-gap` | `0.125rem` |
| `--poodle-block-editor-control-size` | `1.5rem` |
| `--poodle-block-editor-content-x` | `0.5rem` |
| `--poodle-block-editor-content-y` | `0.375rem` |
| `--poodle-block-editor-input-x` | `0.375rem` |
| `--poodle-block-editor-input-y` | `0.25rem` |
| `--poodle-block-editor-menu-gap` | `0.25rem` |
| `--poodle-block-editor-menu-pad` | `0.5rem` |
| `--poodle-block-editor-menu-item-pad` | `0.5rem` |
| `--poodle-block-editor-menu-min-width` | `16rem` |

#### `.block-editor` (Root)

| Property | Value |
|----------|-------|
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-surface)` |
| `padding` | `var(--poodle-block-editor-shell-y) var(--poodle-block-editor-shell-x)` |
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `var(--poodle-block-editor-stack-gap)` |

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
| `padding` | `var(--poodle-block-editor-toolbar-y) var(--poodle-block-editor-toolbar-x)` |
| `border-bottom` | `none` |
| `background` | `transparent` |
| `border-radius` | `var(--poodle-radius-control) var(--poodle-radius-control) 0 0` |

#### `.block-editor__toolbar-left`, `.block-editor__toolbar-right`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-block-editor-toolbar-gap)` |

#### `.block-editor__drag-grip`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `var(--poodle-block-editor-control-size)` |
| `height` | `var(--poodle-block-editor-control-size)` |
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
| `width` | `var(--poodle-block-editor-control-size)` |
| `height` | `var(--poodle-block-editor-control-size)` |
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
| `min-height` | `var(--poodle-block-editor-control-size)` |
| `padding` | `0.0625rem var(--poodle-block-editor-input-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font` | `inherit` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `cursor` | `pointer` |

#### `.block-editor__content`

| Property | Value |
|----------|-------|
| `padding` | `var(--poodle-block-editor-content-y) var(--poodle-block-editor-content-x)` |
| `min-height` | `1.5rem` |

#### `.block-editor__input` (Default)

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `padding` | `var(--poodle-block-editor-input-y) var(--poodle-block-editor-input-x)` |
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
| `padding` | `calc(var(--poodle-block-editor-input-y) * 2) calc(var(--poodle-block-editor-input-x) * 1.5)` |

#### `.block-editor__input--quote`

| Property | Value |
|----------|-------|
| `border-left` | `0.1875rem solid var(--poodle-color-border-default)` |
| `padding-left` | `calc(var(--poodle-block-editor-input-x) + 0.25rem)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-style` | `italic` |

#### `.block-editor__input--list`

| Property | Value |
|----------|-------|
| `padding-left` | `calc(var(--poodle-block-editor-input-x) + 0.625rem)` |

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
| `gap` | `var(--poodle-block-editor-content-y)` |

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
| `gap` | `var(--poodle-block-editor-menu-gap)` |
| `padding` | `var(--poodle-block-editor-menu-pad)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-elevated)` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |
| `min-width` | `var(--poodle-block-editor-menu-min-width)` |

#### `.block-editor__add-menu-item`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-block-editor-content-y)` |
| `padding` | `var(--poodle-block-editor-menu-item-pad)` |
| `border` | `0` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `font-size` | `var(--poodle-typography-label-size)` |
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
| `width` | `var(--poodle-block-editor-control-size)` |
| `text-align` | `center` |

### Size Adjustments

| Size | `control-size` | `menu-min-width` |
|------|---------------|-----------------|
| `xs` | `1.25rem` | `13rem` |
| `sm` | `1.5rem` | (default) |
| `md` | `1.75rem` | (default) |
| `lg` | `2rem` | `18rem` |
| `xl` | `2.25rem` | `20rem` |

### Density Adjustments

| Density | `shell-x` | `shell-y` | `stack-gap` | `toolbar-y` | `toolbar-x` | `content-x` | `content-y` | `input-x` | `input-y` | `menu-pad` | `menu-item-pad` |
|---------|-----------|-----------|-------------|-------------|-------------|-------------|-------------|-----------|-----------|-----------|----------------|
| `compact` | `0.625rem` | `0.625rem` | `0.375rem` | `0.1875rem` | `0.25rem` | `0.375rem` | `0.25rem` | `0.25rem` | `0.1875rem` | `0.375rem` | `0.375rem` |
| `default` | `0.75rem` | `0.75rem` | `0.5rem` | `0.25rem` | `0.375rem` | `0.5rem` | `0.375rem` | `0.375rem` | `0.25rem` | `0.5rem` | `0.5rem` |
| `comfortable` | `1rem` | `1rem` | `0.625rem` | `0.3125rem` | `0.5rem` | `0.625rem` | `0.5rem` | `0.5rem` | `0.3125rem` | `0.625rem` | `0.625rem` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-type` | `.block-editor__block` | Block type identifier (paragraph, heading, code, etc.) |
| `data-size` | `.block-editor` root | Drives size variant custom properties |
| `data-density` | `.block-editor` root | Drives density variant custom properties |

### CSS Classes Used for State Selectors

| Class | Element | Purpose |
|-------|---------|---------|
| `.active` | `.block-editor__block` | Currently focused/active block |
| `.drag-over` | `.block-editor__block` | Drop target highlight during drag |
| `.dragging` | `.block-editor__block` | Source block during drag |
| `.block-editor--disabled` | `.block-editor` | Disabled state on root |

## 9. Svelte Notes

- Uses `createEventDispatcher` for `change` event
- Wraps in `UiPresentationProvider` to propagate resolved size and density to child controls
- Composes `Icon` primitive from `@poodle/svelte-primitives`
- New block IDs generated with `crypto.randomUUID()`
- Add menu positioned via `getBoundingClientRect()` of the add button
- Native HTML drag-and-drop for block reordering
- `focusin` event on blocks tracks `activeBlockId`
- After adding a block, uses `tick().then()` to set `activeBlockId` on the new block

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::block_editor`
- Drag-and-drop reordering may use move-up/move-down button approach
- Block slot rendering needs GPUI component composition pattern

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event name and payload matches
- [ ] EditorBlock and BlockTypeDefinition types are identical
- [ ] block CRUD operations produce same results
- [ ] move up/down boundary checks match
- [ ] remove disabled when single block remaining

### Tier 2: Visual Parity

- [ ] block background opacity (42% / 72%) matches
- [ ] drag-over ring and dragging opacity match
- [ ] toolbar button sizing and spacing match
- [ ] add menu grid layout matches
- [ ] disabled state opacity matches

### Tier 3: Implementation Freedom

- [ ] drag-and-drop mechanics stay internal
- [ ] add menu positioning stays internal

## 12. Specimen Definitions

### Default Block Types

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Default block types | Uses built-in `blockTypes` and default slot rendering | Shows heading, paragraph, quote, code, list, divider, and paragraph blocks with toolbars |

### Custom Block Types with Slot Rendering

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Custom block types | Consumer provides custom `blockTypes` (text, callout, embed) and a `block` slot with custom rendering | Callout has accent left border, embed uses monospace URL input |
