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

## 8. Token Usage

| Property | Token |
|----------|-------|
| Root border | `color-border-default` |
| Root background | `color-background-surface` |
| Root radius | `radius-surface` |
| Toolbar background | `color-background-elevated` at 72% mix |
| Toolbar bottom border | `color-border-subtle` |
| Tool button text | `color-text-secondary` |
| Tool button hover bg | `color-accent-base` at 12% |
| Tool button hover text | `color-text-primary` |
| Tool button focus ring | `color-accent-focusRing`, `border-width-focus` |
| Tool button radius | `radius-control` |
| Tool button font | `typography-code-family` |
| Mode button text | `color-text-secondary` |
| Mode button active bg | `color-accent-base` at 16% |
| Mode button active text | `color-text-primary` |
| Mode switcher border | `color-border-default` |
| Mode switcher radius | `radius-control` |
| Textarea color | `color-text-primary` |
| Textarea font | `typography-code-family` |
| Textarea placeholder | `color-text-tertiary` |
| Preview font | `typography-body-family` |
| Preview text | `color-text-primary` |
| Preview code bg | `color-background-elevated` at 72% |
| Preview code font | `typography-code-family` |
| Preview blockquote border | `color-border-default` |
| Preview blockquote text | `color-text-secondary` |
| Preview hr | `color-border-subtle` |
| Preview link color | `color-accent-default` |
| Disabled opacity | `state-opacity-disabled` |
| Split divider | `color-border-subtle` |
| Motion duration | `motion-duration-interaction` |
| Motion easing | `motion-easing-standard` |

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
