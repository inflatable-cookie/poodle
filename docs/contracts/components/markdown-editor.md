# MarkdownEditor

Status: detailed contract
Updated: 2026-04-09

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
  │     │     ├── [Tool Button: Bold]       Icon: bold
  │     │     ├── [Tool Button: Italic]     Icon: italic
  │     │     ├── [Tool Button: Heading]    Icon: heading
  │     │     ├── [Tool Button: Link]       Icon: link
  │     │     ├── [Tool Button: Code]       Icon: code
  │     │     ├── [Tool Button: Quote]      Icon: quote
  │     │     └── [Tool Button: List]       Icon: list
  │     └── [Mode Switcher]
  │           ├── [Mode Button: Edit]
  │           ├── [Mode Button: Split]
  │           └── [Mode Button: Preview]
  └── [Body]
        ├── [Textarea]     (hidden in preview mode)
        └── [Preview Pane]  (hidden in edit mode)
              ├── [Rendered HTML]  (when value has content)
              └── [Empty Preview]  (when value is empty)
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | Container with border, radius, surface background, class `md-editor` |
| toolbar | `<div>` | Flex row, space-between, toolbar buttons and mode switcher |
| tools | `<div>` | Flex row of formatting tool buttons |
| tool-button | `<button>` | Icon button for each formatting action; disabled when `disabled` or in preview mode |
| mode-switcher | `<div>` | Segmented group of mode toggle buttons, bordered container |
| mode-button | `<button>` | Switches between edit, split, preview modes; active state highlighted |
| body | `<div>` | Flex container holding textarea and/or preview pane |
| textarea | `<textarea>` | Markdown input area; monospace font; resizable vertically |
| preview | `<div>` | Rendered HTML preview of markdown content, `aria-label="Preview"` |
| preview-empty | `<p>` | "Nothing to preview" italic placeholder |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | `""` | no | Markdown content; when supplied, the host owns updates through `onValueChange` |
| `name` | `string \| null` | `null` | no | Optional textarea `name` for native form submission |
| `placeholder` | `string` | `"Write markdown..."` | no | Placeholder text for the textarea |
| `disabled` | `boolean` | `false` | no | Disables editing and toolbar actions |
| `required` | `boolean` | `false` | no | Forwards `required` to the backing textarea for native form validation |
| `ariaLabel` | `string` | `"Markdown editor"` | no | Accessible label for the textarea |
| `minHeight` | `string` | `"12rem"` | no | CSS min-height for the textarea (applied via inline style) |
| `mode` | `"edit" \| "preview" \| "split"` | `"edit"` | no | Active view mode |
| `renderHtml` | `((markdown: string) => string) \| null` | `null` | no | Custom markdown-to-HTML renderer; when provided, replaces the built-in `marked` library fallback |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for toolbar and mode controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for toolbar and pane spacing |
| `onValueChange` | `((value: string) => void) \| null` | `null` | no | Optional callback fired when the markdown content changes |

### Slots

None.

### Controlled / Uncontrolled

`value` is host-owned when supplied and should be updated through `onValueChange`.
`mode` is internally managed but can be set from outside.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| default | -- | Normal appearance |
| disabled | `disabled=true` | `disabled_opacity` on root, `pointer-events: none` |
| tool-hover | Mouse over tool button (not disabled) | Accent background at 12% opacity, text-primary color |
| tool-focus | Focus-visible on tool button | Focus ring: `border-width-focus` solid `accent-focusRing`, offset `0.0625rem` |
| tool-disabled | `disabled` or `mode="preview"` | 40% opacity on tool button, default cursor |
| mode-active | Mode matches current mode | Accent background at 16% opacity, text-primary color |
| mode-hover | Mouse over mode button | Elevated background at 72% blend |

### Component States

| State | Description |
|-------|-------------|
| edit | Only textarea visible; all tools enabled (unless disabled) |
| preview | Only preview pane visible; toolbar tools disabled |
| split | Both textarea and preview visible side by side |

## 5. Callbacks

| Callback | When It Fires | Payload |
|----------|---------------|---------|
| `onValueChange` | Value changes (typing or toolbar insertion) | `string` |

## 6. Accessibility

### Semantics

- Textarea carries `aria-label` from the `ariaLabel` prop
- Tool buttons each have `aria-label` matching their action label ("Bold", "Italic", "Heading", "Link", "Code", "Quote", "List")
- Tool buttons also have `title` matching their action label
- Preview pane has `aria-label="Preview"`
- Disabled tool buttons use native `disabled` attribute

### Keyboard

- Standard textarea keyboard behavior for editing
- Tab navigation through toolbar buttons
- Tool buttons disabled in preview mode are excluded from tab order (native `disabled`)
- Mode buttons are always keyboard-accessible

### Focus

- Tool buttons show focus ring via `:focus-visible`: `border-width-focus` solid `accent-focusRing`, offset `0.0625rem`
- Textarea has outline suppressed (border on parent container serves as visual boundary)

## 7. Layout

### Sizing

- Root: full width of container, border `0.0625rem solid border-default`, `radius-surface`, `overflow: hidden`
- Toolbar: flex row, space-between, wraps on narrow widths, density-aware padding
- Tool button: semantic control box tied to size scale
- Mode button: semantic label sizing and density-aware horizontal padding
- Body: flex row; in split mode, textarea and preview each `flex: 1`
- Textarea: density-aware pane padding, min-height from prop, resize vertical
- Preview: density-aware pane padding, overflow-y auto

### Composition

In split mode the textarea gets a right border (`border-subtle`) to visually separate from the preview pane. Wraps children in `UiPresentationProvider`.

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-mode` | body `<div>` | `"edit"`, `"preview"`, `"split"` |
| `data-size` | root `<div>` | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | root `<div>` | `"compact"`, `"default"`, `"comfortable"` |

### Root `.md-editor`

| Property | Value |
|----------|-------|
| border | `0.0625rem solid var(--poodle-color-border-default)` |
| border-radius | `var(--poodle-radius-surface)` |
| background | `var(--poodle-color-background-surface)` |
| overflow | `hidden` |

### Root Disabled `.md-editor--disabled`

| Property | Value |
|----------|-------|
| opacity | `var(--poodle-state-opacity-disabled)` |
| pointer-events | `none` |

### Toolbar `.md-editor__toolbar`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `space-between` |
| gap | `0.5rem` |
| padding | `0.375rem 0.5rem` (default) |
| border-bottom | `0.0625rem solid var(--poodle-color-border-subtle)` |
| background | `color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent)` |
| flex-wrap | `wrap` |

### Tools Container `.md-editor__tools`

| Property | Value |
|----------|-------|
| display | `flex` |
| gap | `0.125rem` (default) |

### Tool Button `.md-editor__tool-btn`

| Property | Value |
|----------|-------|
| display | `inline-flex` |
| align-items | `center` |
| justify-content | `center` |
| width | `1.75rem` (default, varies by size) |
| height | `1.75rem` (default, varies by size) |
| padding | `0` |
| border | `0` |
| border-radius | `var(--poodle-radius-control)` |
| background | `transparent` |
| color | `var(--poodle-color-text-secondary)` |
| cursor | `pointer` |
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

### Mode Switcher `.md-editor__modes`

| Property | Value |
|----------|-------|
| display | `flex` |
| gap | `0.125rem` (default, same as tool gap) |
| border | `0.0625rem solid var(--poodle-color-border-default)` |
| border-radius | `var(--poodle-radius-control)` |
| overflow | `hidden` |

### Mode Button `.md-editor__mode-btn`

| Property | Value |
|----------|-------|
| min-height | `calc(tool-size - (toolbar-y * 0.5))` |
| padding | `0.1875rem 0.5rem` (default) |
| border | `0` |
| background | `transparent` |
| color | `var(--poodle-color-text-secondary)` |
| cursor | `pointer` |
| font | `inherit` |
| font-size | `var(--poodle-typography-label-size)` |
| line-height | `1` |
| transition | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### Mode Button States

| State | Property | Value |
|-------|----------|-------|
| `:hover` | background | `color-mix(in srgb, var(--poodle-color-background-elevated) 72%, transparent)` |
| `.active` | background | `color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent)` |
| `.active` | color | `var(--poodle-color-text-primary)` |

### Body `.md-editor__body`

| Property | Value |
|----------|-------|
| display | `flex` |

#### Body Split Mode `[data-mode="split"]`

| Property | Value |
|----------|-------|
| gap | `0` |
| textarea | `flex: 1`, `border-right: 0.0625rem solid var(--poodle-color-border-subtle)` |
| preview | `flex: 1` |

### Textarea `.md-editor__textarea`

| Property | Value |
|----------|-------|
| flex | `1` |
| width | `100%` |
| padding | `0.75rem` (default) |
| border | `0` |
| background | `transparent` |
| color | `var(--poodle-color-text-primary)` |
| font-family | `var(--poodle-typography-code-family)` |
| font-size | `0.8125rem` |
| line-height | `1.6` |
| resize | `vertical` |
| outline | `none` |
| `::placeholder` color | `var(--poodle-color-text-tertiary)` |

### Preview Pane `.md-editor__preview`

| Property | Value |
|----------|-------|
| flex | `1` |
| padding | `0.75rem` (default) |
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
| `ul`, `ol` | margin | `0 0 0.5rem` |
| `ul`, `ol` | padding-left | `1.25rem` |
| `li` | margin | `0 0 0.125rem` |
| `hr` | border | `0` |
| `hr` | border-top | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `hr` | margin | `0.75rem 0` |
| `a` | color | `var(--poodle-color-accent-default, #6366f1)` |
| `a` | text-decoration | `underline` |

### Preview Empty `.md-editor__preview-empty`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-tertiary)` |
| font-style | `italic` |
| margin | `0` |

### Size Adjustments

| Size | Tool button size | Mode horizontal padding |
|------|-----------------|------------------------|
| `xs` | `1.5rem` | `0.375rem` |
| `sm` | `1.75rem` | `0.5rem` (default) |
| `md` | `2rem` | `0.5rem` |
| `lg` | `2.25rem` | `0.625rem` |
| `xl` | `2.5rem` | `0.75rem` |

### Density Adjustments

| Density | Toolbar Y | Toolbar X | Tool gap | Mode Y | Pane X | Pane Y |
|---------|----------|----------|----------|--------|--------|--------|
| `compact` | `0.25rem` | `0.375rem` | `0.0625rem` | `0.125rem` | `0.625rem` | `0.625rem` |
| `default` | `0.375rem` | `0.5rem` | `0.125rem` | `0.1875rem` | `0.75rem` | `0.75rem` |
| `comfortable` | `0.5rem` | `0.625rem` | `0.1875rem` | `0.25rem` | `0.875rem` | `0.875rem` |

## 9. Svelte Notes

- Uses `Icon` primitive for toolbar button icons (bold, italic, heading, link, code, quote, list)
- `insertMarkdown(before, after)` manipulates textarea selection to wrap or prepend markdown syntax
- `insertLine(prefix)` prefixes the current line with markdown syntax (heading, quote, list)
- Built-in markdown rendering uses the `marked` library (`import { marked } from "marked"`)
  called with `marked.parse(value, { async: false })` — tree-shakes out when the
  component is not used in a bundle
- `renderHtml` prop: when provided (non-null), replaces the built-in `marked` renderer;
  the reactive derivation is `previewHtml = renderHtml ? renderHtml(value) : marked.parse(value, { async: false })`
- Preview uses `{@html previewHtml}` for reactive rendering
- `tick()` used after insertion to restore cursor selection
- `mode` is reactive; changing it shows/hides textarea and preview
- `handleInput()` calls `onValueChange` on every textarea input while the host
  owns `value`
- `textareaEl` bound for programmatic selection manipulation
- Wraps content in `UiPresentationProvider` with resolved size and density

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] All props have the same meaning and defaults
- [ ] `onValueChange` callback payload matches
- [ ] Toolbar actions produce correct markdown syntax
- [ ] Edit/Preview/Split mode switching behavior matches
- [ ] Disabled state disables all tools and editing

### Tier 2: Visual Parity

- [ ] Toolbar layout and spacing matches
- [ ] Tool button and mode button styling matches
- [ ] Preview rendered HTML styling matches
- [ ] Size and density adjustments match

### Tier 3: Implementation Freedom

- [ ] Built-in markdown rendering engine may differ (Svelte uses `marked`)
- [ ] `renderHtml` callback allows consumer to override rendering entirely
- [ ] Text insertion mechanics may differ
- [ ] Cursor restoration approach may differ

## 12. Specimen Definitions

### Split View

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Split view | `mode="split"`, pre-filled markdown with headings, bold, code, links | Side-by-side textarea and rendered preview |

### Edit Mode

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Edit mode | `mode="edit"`, empty content, `placeholder="Start writing..."` | Textarea only with placeholder text |

### Preview Mode

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Preview mode | `mode="preview"`, pre-filled markdown content | Rendered HTML only; toolbar tools disabled |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `value="Read-only content"`, `disabled=true` | Reduced opacity, no interaction possible |
