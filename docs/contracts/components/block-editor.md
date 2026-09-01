# BlockEditor

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `BlockEditor`
- Layer: `composites`
- Summary: a pure shell for block-based content editing — supports both multi-
  block and single-block posture, provides optional block CRUD/reordering/type
  switching infrastructure, and leaves block payload shape and rendering fully
  consumer-owned
- In scope: single or multi posture, optional block CRUD, optional type
  switching via Select or snippet override, optional reordering (drag-and-drop +
  arrow buttons), consumer-provided block type definitions, opaque block
  payloads, snippet-rendered block content, size and density variants
- Out of scope: built-in block types, built-in block rendering, rich text editing within blocks, collaborative editing, undo/redo history, block nesting, server persistence

## 2. Anatomy

```text
[Root .block-editor]  <div> aria-label, wraps in UiPresentationProvider
  └── [Block .block-editor__block]  (repeated, keyed by block.id) role="group"
        ├── [Toolbar .block-editor__toolbar]
        │     ├── [ToolbarLeft .block-editor__toolbar-left]
        │     │     ├── [DragGrip .block-editor__drag-grip]  <span> draggable, Icon grip-vertical
        │     │     └── [TypeSelect .block-editor__type-select]  Select (variant="ghost", menuMinWidth="10rem")
        │     └── [ToolbarRight .block-editor__toolbar-right]
        │           ├── [MoveUpBtn .block-editor__tool-btn]  <button> Icon arrow-up
        │           ├── [MoveDownBtn .block-editor__tool-btn]  <button> Icon arrow-down
        │           ├── [AddSelect .block-editor__add-select]  Select (variant="ghost", menuMinWidth="10rem", trigger slot with plus icon)
        │           └── [RemoveBtn .block-editor__tool-btn.block-editor__remove-btn]  <button> Icon x (hidden when 1 block)
        └── [Content .block-editor__content]
              └── [BlockContent]  `block(...)` snippet; fallback: minimal <textarea>
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Root | `<div>` | yes | Flex column container with `aria-label`, wraps in `UiPresentationProvider` |
| Block | `<div>` | yes (repeated) | `role="group"`, `aria-label="{type} block"`, drag-and-drop target |
| Toolbar | `<div>` | yes (per block) | Flex row, space-between, transparent background |
| DragGrip | `<span>` | yes (per block) | `draggable="true"`, `aria-hidden="true"`, `grab` cursor |
| TypeSelect | Select | yes (per block) | Poodle Select component, `variant="ghost"`, `menuMinWidth="10rem"`, `ariaLabel="Block type"`; gains `--inset` margin-left when reordering is disabled (drag grip hidden) to keep alignment with block content |
| MoveUpBtn | `<button>` | yes (per block) | Disabled when first block or editor disabled |
| MoveDownBtn | `<button>` | yes (per block) | Disabled when last block or editor disabled |
| AddSelect | Select | yes (per block) | Poodle Select with `variant="ghost"`, `menuMinWidth="10rem"`, custom trigger slot showing a plus icon; selecting a type inserts a new block after the current one |
| RemoveBtn | `<button>` | no | Hidden when only 1 block remains |
| Content | `<div>` | yes (per block) | Contains block slot or minimal textarea fallback |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `blocks` | `EditorBlock[]` | `[]` | no | Array of block data objects; host-owned and updated through `onChange` |
| `blockTypes` | `BlockTypeDefinition[]` | `[]` | no | Consumer provides all available block types; drives both the type-switch Select and the add-block Select |
| `blockTypeItems` | `BlockTypeDefinition[] \| BlockTypeGroup[] \| null` | `null` | no | Optional richer type input for built-in pickers; supports flat or grouped Nightfire-style menus |
| `disabled` | `boolean` | `false` | no | Disables all editing controls |
| `ariaLabel` | `string` | `"Block editor"` | no | Accessible label for the root container |
| `mode` | `"single" \| "multi"` | `"multi"` | no | Single posture hides multi-block controls by default; multi posture enables them by default |
| `allowReorder` | `boolean \| null` | `null` | no | Explicit override for drag grip and move controls; defaults from `mode` |
| `allowAdd` | `boolean \| null` | `null` | no | Explicit override for add-block control; defaults from `mode` |
| `allowRemove` | `boolean \| null` | `null` | no | Explicit override for remove control; defaults from `mode` |
| `allowTypeChange` | `boolean \| null` | `null` | no | Explicit override for type-change control; defaults to `true` in both postures |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for toolbar chrome and nested controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for shell, toolbar, content, and input spacing |

### Types

```ts
type BlockType = string;

type BlockTypeDefinition = {
  type: BlockType;
  label: string;
  icon: IconProp;
};

type BlockTypeGroup = {
  label: string;
  options: BlockTypeDefinition[];
};

type EditorBlock = {
  id: string;
  type: BlockType;
  version?: string | number;
  hash?: string | null;
  data?: unknown;
  content?: string;  // legacy fallback convenience, not required
  [key: string]: unknown;  // opaque payload remains consumer-owned
};
```

### Snippets

| Snippet | Scope | Purpose |
|---------|-------|---------|
| `block` | `{ block: EditorBlock, index: number, disabled: boolean, update: (updates: Partial<EditorBlock>) => void }` | Custom rendering for block content; when not provided, falls back to a minimal `<textarea>` |
| `typePicker` | `{ block: EditorBlock, index: number, disabled: boolean, options: SelectOption[], groupedOptions: SelectItems, changeType: (type: BlockType) => void }` | Override the built-in type-change control while keeping the shell |
| `addPicker` | `{ block: EditorBlock, index: number, disabled: boolean, options: SelectOption[], groupedOptions: SelectItems, addBlock: (type: BlockType) => void }` | Override the built-in add-block control while keeping the shell |

### Fallback Block Renderer

When the `block` snippet is not provided, the only fallback is a minimal `<textarea>` with `placeholder="Type something..."` and `rows="1"`. That fallback reads and writes `block.content` only for legacy convenience. There are no built-in type-specific renderers (no heading, code, quote, list, image, or divider rendering). Consumers are expected to provide a `block` snippet for meaningful rendering and should treat the full block object as opaque, consumer-owned data.

### Controlled And Uncontrolled

- `blocks` are host-owned and updated through `onChange`
- there is no bindable `blocks` surface in the current Svelte implementation

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | blocks present, not disabled | Blocks shown with toolbars |
| disabled | `disabled=true` | Root has `opacity: disabled`, `pointer-events: none` |
| single-posture | `mode="single"` | Reorder, add, and remove controls hidden unless explicitly re-enabled |
| active-block | block receives focus | Block background increases to 72% elevated mix |
| dragging | the block is the substrate's active drag source | Source block at 40% opacity |
| drag-over | the substrate accepts an intent on another block | Target block shows accent box-shadow ring |

### Component States

| State | Description |
|-------|-------------|
| `activeBlockId` | ID of the currently focused block |

Drag posture is not component state. The dragged block and the current drop
target are read from the drag substrate's session snapshot, keyed by the
block's own registration ids. There is no `dragSourceIndex`, `dragOverIndex`,
`draggable` attribute, or `DataTransfer` payload.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|----------|--------------|---------|-------|
| `onChange` | after any mutation (add, remove, move, type change, content update) | `EditorBlock[]` | runs with a shallow copy of the blocks array |

One accepted reorder emits `onChange` exactly once, carrying the complete next
block order. A drop whose source or target block is no longer present is
rejected rather than committed against a stale index.

## 6. Accessibility

### Semantics

- Root: `aria-label` from prop (default `"Block editor"`)
- Each block: `role="group"`, `aria-label="{type} block"`
- Type-switch Select: `ariaLabel="Block type"`
- Move buttons: `aria-label="Move up"` / `"Move down"`
- Add-block Select: `ariaLabel="Add block after this one"`
- Remove button: `aria-label="Remove block"`
- Drag grip: `aria-hidden="true"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | Navigates between toolbar controls and content inputs |
| `Space` / `Enter` | Activates focused buttons |

The move up / move down buttons are the keyboard reorder route. Each issues a
keyboard drop command against the same source and target registrations the
pointer uses, so both routes share eligibility, drop-time revalidation, the
single commit, and the `onChange` payload. A button never edits the block array
behind the substrate's back.

### Focus

- Tool buttons and block text input: `border-width-focus` solid `accent-focusRing`, offset `0.125rem`
- Newly added blocks receive focus via `tick().then()` after insertion

## 7. Layout

### Sizing

- Root: flex column with density-aware gap; no padding, border, or radius on the root
- Blocks: flex column, no explicit border (differentiation via background)
- Toolbar: flex row, space-between
- Content: padded area with min-height `1.5rem`

### Composition

- Composes: `Icon`, `Select`, `UiPresentationProvider` from `@inflatable-cookie/poodle-svelte`
- Both the type-switch and add-block controls use Select with `variant="ghost"` and `native={false}`
- `blockTypeItems` can drive those built-in controls with either flat or grouped
  options; `blockTypes` remains the flat convenience input
- The add-block Select uses a custom trigger slot containing a plus icon styled as a tool button
- Consumers may override the built-in type-change and add-block controls via
  slots while reusing the same shell chrome
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

#### `.block-editor` (Root)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-background-surface)` |
| `padding` | `0` |
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `var(--poodle-block-editor-stack-gap)` |

The root carries no border, no border-radius, and no shell padding — block
differentiation is via per-block background only. The `--poodle-block-editor-shell-x`
/ `--poodle-block-editor-shell-y` custom properties are declared on the root but
are currently unused by the root itself.

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
| `flex-shrink` | `0` |

#### `.block-editor__type-select--inset` (applied when `!canReorder`)

| Property | Value |
|----------|-------|
| `margin-left` | `calc(content-x + input-x − toolbar-x)` (aligns the type select with the block content when the drag grip is hidden) |

#### `.block-editor__add-select`

| Property | Value |
|----------|-------|
| `flex-shrink` | `0` |

#### `.block-editor__content`

| Property | Value |
|----------|-------|
| `padding` | `var(--poodle-block-editor-content-y) var(--poodle-block-editor-content-x)` |
| `min-height` | `1.5rem` |

#### `.block-editor__input` (Fallback textarea)

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

### Size Adjustments

| Size | `control-size` |
|------|---------------|
| `xs` | `1.25rem` |
| `sm` | `1.5rem` |
| `md` | `1.75rem` |
| `lg` | `2rem` |
| `xl` | `2.25rem` |

### Density Adjustments

| Density | `shell-x` | `shell-y` | `stack-gap` | `toolbar-y` | `toolbar-x` | `content-x` | `content-y` | `input-x` | `input-y` |
|---------|-----------|-----------|-------------|-------------|-------------|-------------|-------------|-----------|-----------|
| `compact` | `0.625rem` | `0.625rem` | `0.375rem` | `0.1875rem` | `0.25rem` | `0.375rem` | `0.25rem` | `0.25rem` | `0.1875rem` |
| `default` | `0.75rem` | `0.75rem` | `0.5rem` | `0.25rem` | `0.375rem` | `0.5rem` | `0.375rem` | `0.375rem` | `0.25rem` |
| `comfortable` | `1rem` | `1rem` | `0.625rem` | `0.3125rem` | `0.5rem` | `0.625rem` | `0.5rem` | `0.5rem` | `0.3125rem` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-type` | `.block-editor__block` | Block type identifier (consumer-defined string) |
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
- Composes `Icon`, `Select`, and `UiPresentationProvider` from `@inflatable-cookie/poodle-svelte`
- Block type options are derived from `blockTypes` prop, mapped to `SelectOption[]` with `value`, `label`, and `icon`
- Type-switch per block: `Select` with `variant="ghost"`, `native={false}`, `menuMinWidth="10rem"`, bound to `block.type`
- Add-block: `Select` with `variant="ghost"`, `native={false}`, `menuMinWidth="10rem"`, `value={null}`, and a trigger slot containing a plus icon styled as a tool button; selecting a value calls `addBlock(value, index)`
- New block IDs generated with `crypto.randomUUID()`
- Block reordering runs on the common drag-and-drop substrate (architecture
  011, spec 069): the block registers a drag source whose handle selector is
  `.poodle-block-editor__drag-grip`, and every reorderable block is a drop
  target. Registration ids and the subject kind
  (`poodle.reorder-item:block-editor:{instance}`) are scoped to the editor
  instance, so two mounted editors holding the same block ids under one ambient
  provider can never cross-drop. The editor joins an ambient
  `DragDropProvider` when one exists and owns an isolated controller otherwise
- A block's own toolbar controls (type select, add, remove, move, textarea) are
  reachable during and outside a drag: the drag source is the grip, not the
  block body
- `focusin` event on blocks tracks `activeBlockId`
- After adding a block, uses `tick().then()` to set `activeBlockId` on the new block

## 10. GPUI Notes

- Implemented: `BlockEditorSpec`
  (`packages/contracts/components/src/block_editor.rs`),
  `poodle_render::block_editor` / `block_editor_with_children` taking
  `BlockEditorHandlers`, GPUI specimen
  `packages/gpui/preview/src/specimens/block_editor_specimen.rs`.
- `BlockEditorHandlers::new(instance_id)` carries `on_change`, which fires with
  the **complete next block order** — the renderer-neutral mirror of the web
  `onChange` reorder payload. `on_type_change`, `on_add`, and `on_remove` carry
  the block id (and the chosen type) for the other toolbar intents.
- Reordering runs on the renderer-neutral substrate: the grip registers a
  `NodeDragSource` and every reorderable block a `NodeDropTarget`, both scoped
  to `instance_id`. The band rule is
  `crate::drag_drop::vertical_band_resolver`, a block dropped onto itself is
  rejected, and the drop is revalidated against the spec's live block list.
- Move up / move down reach the same emitter as a drop, so the native keyboard
  and pointer routes produce one identical order payload.
- A grip is drawn only when the editor can actually reorder; a spec with no
  reorder handler draws no grip and no move buttons rather than a dead
  affordance.
- Block slot rendering uses `block_editor_with_children`, whose builders run in
  the component's own presentation scope.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults (`blocks=[]`, `blockTypes=[]`)
- [ ] event name and payload matches
- [ ] EditorBlock and BlockTypeDefinition types are identical
- [ ] block CRUD operations produce same results
- [ ] move up/down boundary checks match
- [ ] remove disabled when single block remaining
- [ ] type-switch and add-block both use Select with ghost variant

### Tier 2: Visual Parity

- [ ] block background opacity (42% / 72%) matches
- [ ] drag-over ring and dragging opacity match
- [ ] toolbar button sizing and spacing match
- [ ] disabled state opacity matches

### Tier 3: Implementation Freedom

- [ ] drag-and-drop mechanics stay internal
- [ ] Select menu positioning delegated to Select primitive

## 12. Specimen Definitions

### Custom Block Types with Slot Rendering

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Custom block types | Consumer provides custom `blockTypes` (text, callout, embed) and a `block` slot with custom rendering | Blocks shown with Select-based type switcher and add-block menu; callout has accent left border, embed uses monospace URL input |

### Minimal (No Block Slot)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Minimal fallback | Consumer provides `blockTypes` but no `block` slot | Each block renders a plain textarea fallback with "Type something..." placeholder |
