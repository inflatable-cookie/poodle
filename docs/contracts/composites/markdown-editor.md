# MarkdownEditor

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `MarkdownEditor`
- Layer: `composites`
- Summary: a rich-text markdown editing surface with formatting toolbar, live preview, and split-view mode
- In scope: markdown toolbar (bold, italic, heading, link, code, quote, list), edit/preview/split view modes, inline markdown-to-HTML rendering, text insertion helpers
- Out of scope: file uploads, image drag-and-drop, syntax highlighting, collaborative editing, plugin system, custom toolbar actions

## 2. Anatomy

```text
[Root]
  ├── [Toolbar]
  │     ├── [Tools]
  │     │     ├── [Tool Button: Bold]
  │     │     ├── [Tool Button: Italic]
  │     │     ├── [Tool Button: Heading]
  │     │     ├── [Tool Button: Link]
  │     │     ├── [Tool Button: Code]
  │     │     ├── [Tool Button: Quote]
  │     │     └── [Tool Button: List]
  │     └── [Mode Switcher]
  │           ├── [Mode Button: Edit]
  │           ├── [Mode Button: Split]
  │           └── [Mode Button: Preview]
  └── [Body]
        ├── [Textarea]     (hidden in preview mode)
        └── [Preview Pane]  (hidden in edit mode)
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | Container with border, radius, surface background |
| toolbar | `<div>` | Flex row, space-between, toolbar buttons and mode switcher |
| tools | `<div>` | Flex row of formatting tool buttons |
| tool-button | `<button>` | Icon button for each formatting action; disabled when `isDisabled` or in preview mode |
| mode-switcher | `<div>` | Segmented group of mode toggle buttons |
| mode-button | `<button>` | Switches between edit, split, preview modes; active state highlighted |
| body | `<div>` | Flex container holding textarea and/or preview pane |
| textarea | `<textarea>` | Markdown input area; monospace font; resizable vertically |
| preview | `<div>` | Rendered HTML preview of markdown content |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | `""` | no | Markdown content (two-way bindable) |
| `placeholder` | `string` | `"Write markdown..."` | no | Placeholder text for the textarea |
| `isDisabled` | `boolean` | `false` | no | Disables editing and toolbar actions |
| `ariaLabel` | `string` | `"Markdown editor"` | no | Accessible label for the textarea |
| `minHeight` | `string` | `"12rem"` | no | CSS min-height for the textarea |
| `mode` | `"edit" \| "preview" \| "split"` | `"edit"` | no | Active view mode |

### Slots

None.

### Controlled / Uncontrolled

`value` supports two-way binding. `mode` is internally managed but can be set from outside.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| default | -- | Normal appearance |
| disabled | `isDisabled=true` | `disabled_opacity` on root, `pointer-events: none` |
| tool-hover | Mouse over tool button | Accent background at 12% opacity, text-primary color |
| tool-focus | Focus-visible on tool button | Focus ring |
| tool-disabled | `isDisabled` or `mode="preview"` | 40% opacity on tool button |
| mode-active | Mode matches current mode | Accent background at 16% opacity, text-primary color |
| mode-hover | Mouse over mode button | Elevated background |

### Component States

| State | Description |
|-------|-------------|
| edit | Only textarea visible |
| preview | Only preview pane visible; toolbar tools disabled |
| split | Both textarea and preview visible side by side |

## 5. Events

| Event | When It Fires | Payload |
|-------|---------------|---------|
| `change` | Value changes (typing or toolbar insertion) | `{ value: string }` |

## 6. Accessibility

### Semantics

- Textarea carries `aria-label` from the `ariaLabel` prop
- Tool buttons each have `aria-label` matching their action label (Bold, Italic, etc.)
- Preview pane has `aria-label="Preview"`

### Keyboard

- Standard textarea keyboard behavior for editing
- Tab navigation through toolbar buttons
- Tool buttons disabled in preview mode are skipped by tab order (native `disabled`)

### Focus

- Tool buttons show focus ring via `focus-visible`: `border-width-focus` solid `accent-focusRing`, offset `0.0625rem`
- Textarea has outline suppressed (border on parent container serves as visual boundary)

## 7. Layout

### Sizing

- Root: full width of container, border `0.0625rem solid border-default`, `radius-surface`
- Toolbar: flex row, space-between, padding `0.375rem 0.5rem`, wraps on narrow widths
- Tool button: `1.75rem x 1.75rem`, `radius-control`
- Mode button: padding `0.1875rem 0.5rem`, font-size `0.6875rem`
- Body: flex row; in split mode, textarea and preview each `flex: 1`
- Textarea: padding `0.75rem`, min-height from prop, resize vertical
- Preview: padding `0.75rem`, overflow-y auto

### Composition

In split mode the textarea gets a right border (`border-subtle`) to visually separate from the preview pane.

## 8. Token Usage And Precise CSS

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-mode` | body `<div>` | `"edit"`, `"preview"`, `"split"` |

### Root

| Property | Value |
|----------|-------|
| border | `0.0625rem solid var(--poodle-color-border-default)` |
| border-radius | `var(--poodle-radius-surface)` |
| background | `var(--poodle-color-background-surface)` |
| overflow | `hidden` |

### Root (Disabled)

| Property | Value |
|----------|-------|
| opacity | `var(--poodle-state-opacity-disabled)` |
| pointer-events | `none` |

### Toolbar

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `space-between` |
| gap | `0.5rem` |
| padding | `0.375rem 0.5rem` |
| border-bottom | `0.0625rem solid var(--poodle-color-border-subtle)` |
| background | `color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent)` |
| flex-wrap | `wrap` |

### Tools Container

| Property | Value |
|----------|-------|
| display | `flex` |
| gap | `0.125rem` |

### Tool Button

| Property | Value |
|----------|-------|
| display | `inline-flex` |
| align-items | `center` |
| justify-content | `center` |
| width | `1.75rem` |
| height | `1.75rem` |
| padding | `0` |
| border | `0` |
| border-radius | `var(--poodle-radius-control)` |
| background | `transparent` |
| color | `var(--poodle-color-text-secondary)` |
| font-family | `var(--poodle-typography-code-family)` |
| font-size | `0.75rem` |
| font-weight | `600` |
| line-height | `1` |
| transition | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### Tool Button States

| State | Property | Value |
|-------|----------|-------|
| `:hover:not(:disabled)` | background | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |
| `:hover:not(:disabled)` | color | `var(--poodle-color-text-primary)` |
| `:focus-visible` | outline | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `:focus-visible` | outline-offset | `0.0625rem` |
| `:disabled` | opacity | `0.4` |
| `:disabled` | cursor | `default` |

### Mode Switcher

| Property | Value |
|----------|-------|
| display | `flex` |
| gap | `0.125rem` |
| border | `0.0625rem solid var(--poodle-color-border-default)` |
| border-radius | `var(--poodle-radius-control)` |
| overflow | `hidden` |

### Mode Button

| Property | Value |
|----------|-------|
| padding | `0.1875rem 0.5rem` |
| border | `0` |
| background | `transparent` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.6875rem` |
| line-height | `1` |
| transition | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### Mode Button States

| State | Property | Value |
|-------|----------|-------|
| `:hover` | background | `color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent)` |
| `.active` | background | `color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent)` |
| `.active` | color | `var(--poodle-color-text-primary)` |

### Body

| Property | Value |
|----------|-------|
| display | `flex` |

#### Body Split Mode (`[data-mode="split"]`)

| Property | Value |
|----------|-------|
| gap | `0` |
| textarea flex | `1`, right border `0.0625rem solid var(--poodle-color-border-subtle)` |
| preview flex | `1` |

### Textarea

| Property | Value |
|----------|-------|
| flex | `1` |
| width | `100%` |
| padding | `0.75rem` |
| border | `0` |
| background | `transparent` |
| color | `var(--poodle-color-text-primary)` |
| font-family | `var(--poodle-typography-code-family)` |
| font-size | `0.8125rem` |
| line-height | `1.6` |
| resize | `vertical` |
| outline | `none` |
| `::placeholder` color | `var(--poodle-color-text-tertiary)` |

### Preview Pane

| Property | Value |
|----------|-------|
| flex | `1` |
| padding | `0.75rem` |
| font-family | `var(--poodle-typography-body-family)` |
| font-size | `0.875rem` |
| line-height | `1.6` |
| color | `var(--poodle-color-text-primary)` |
| overflow-y | `auto` |

### Preview Rendered Elements

| Element | Property | Value |
|---------|----------|-------|
| `h1` | font-size | `1.25rem` |
| `h1` | font-weight | `700` |
| `h1` | margin | `0 0 0.5rem` |
| `h2` | font-size | `1.0625rem` |
| `h2` | font-weight | `600` |
| `h2` | margin | `0 0 0.375rem` |
| `h3` | font-size | `0.9375rem` |
| `h3` | font-weight | `600` |
| `h3` | margin | `0 0 0.25rem` |
| `p` | margin | `0 0 0.5rem` |
| `strong` | font-weight | `700` |
| `code` | padding | `0.125rem 0.25rem` |
| `code` | border-radius | `var(--poodle-radius-control)` |
| `code` | background | `color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent)` |
| `code` | font-family | `var(--poodle-typography-code-family)` |
| `code` | font-size | `0.8125rem` |
| `blockquote` | margin | `0 0 0.5rem` |
| `blockquote` | padding | `0.375rem 0.75rem` |
| `blockquote` | border-left | `0.1875rem solid var(--poodle-color-border-default)` |
| `blockquote` | color | `var(--poodle-color-text-secondary)` |
| `li` | margin | `0 0 0.125rem` |
| `li` | padding-left | `0.25rem` |
| `li` | list-style | `disc inside` |
| `hr` | border | `0` |
| `hr` | border-top | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `hr` | margin | `0.75rem 0` |
| `a` | color | `var(--poodle-color-accent-default, #6366f1)` |
| `a` | text-decoration | `underline` |

### Preview Empty

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-tertiary)` |
| font-style | `italic` |
| margin | `0` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Uses `Icon` primitive for toolbar button icons (bold, italic, heading, link, code, quote, list)
- `insertMarkdown()` manipulates textarea selection to wrap or prepend markdown syntax
- `insertLine()` prefixes the current line with markdown syntax (heading, quote, list)
- `renderMarkdown()` is a safe subset HTML renderer (escapes HTML, no raw passthrough)
- `tick()` used after insertion to restore cursor selection
- `mode` is reactive; changing it shows/hides textarea and preview

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

| Feature | Svelte | GPUI | Jetstream |
|---------|--------|------|-----------|
| Toolbar formatting buttons | Yes | -- | -- |
| Edit/Preview/Split modes | Yes | -- | -- |
| Live markdown preview | Yes | -- | -- |
| Disabled state | Yes | -- | -- |
| Keyboard toolbar navigation | Yes | -- | -- |
| Focus ring on tool buttons | Yes | -- | -- |

## 12. Known Deltas

None yet (single implementation).

## 13. Specimen Definitions

### Split View

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Split view | `mode="split"`, pre-filled markdown content | Side-by-side textarea and rendered preview |

### Edit Mode

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Edit mode | `mode="edit"`, empty content, `placeholder="Start writing..."` | Textarea only with placeholder text |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `value="Read-only content"`, `isDisabled=true` | Reduced opacity, no interaction |

## 14. Approval And Adoption Notes

Use `MarkdownEditor` for content authoring fields where markdown formatting is needed. For block-based rich content editing with multiple block types, use `BlockEditor` instead. The built-in markdown renderer is intentionally a safe subset; applications needing full CommonMark or GFM support should provide a custom renderer.
