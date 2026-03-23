# Code

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Code`
- Layer: `foundation`
- Summary: a code display primitive supporting inline code snippets and
  multi-line code blocks with optional line numbers, line highlighting, and a
  copy-to-clipboard button
- In scope: inline code, block code with toolbar, line numbers, line
  highlighting, copy button with feedback, scrollable overflow, language label
- Out of scope: syntax highlighting/tokenization, code editing, live code
  execution

## 2. Anatomy

### Inline mode

```text
[Root .code.code--inline]  <code>
  └── [Source text]
```

### Block mode

```text
[Root .code.code--block]  <div>
  ├── [Toolbar .code__toolbar]  (conditional: language or showCopyButton)
  │   ├── [Language label .code__language]  <span>
  │   └── [Actions .code__toolbar-actions]  <div>
  │       └── [Copy button .code__copy]  <button>
  │           └── [Icon SVG]  (copy or check)
  └── [Scroll .code__scroll]  <div>
      └── [Pre .code__pre]  <pre>
          └── [Source .code__source]  <code>
              └── [Line .code__line] ...  <span>
                  ├── [Line number .code__line-number]  <span> (optional)
                  └── [Line content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root (inline) | yes | inline code fragment | background, padding, radius, typography |
| Root (block) | yes | block code container | border, radius, background |
| Toolbar | no | header with language label and actions | border-bottom, padding |
| Language label | no | uppercase language identifier | typography, color |
| Copy button | no | clipboard copy trigger | color, radius, transition |
| Scroll | yes (block) | horizontal scroll container | overflow |
| Pre | yes (block) | preformatted text wrapper | margin, padding |
| Source | yes | code text element | typography, color, tab-size |
| Line | yes (block) | individual code line | display |
| Line (highlighted) | no | accent-highlighted line | background, padding |
| Line number | no | gutter line number | width, color, alignment |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `source` | `string` | `""` | no | code text content |
| `language` | `string \| null` | `null` | no | language label shown in toolbar |
| `showLineNumbers` | `boolean` | `false` | no | display line numbers in gutter |
| `highlightLines` | `number[]` | `[]` | no | 1-based line indices to highlight |
| `showCopyButton` | `boolean` | `true` | no | show copy-to-clipboard button |
| `maxHeight` | `string \| null` | `null` | no | max-height for block container (inline style) |
| `inline` | `boolean` | `false` | no | render as inline `<code>` element |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the code block |

### Controlled And Uncontrolled

- display primitive; internal state limited to copy feedback (2-second timer)

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| inline | `inline=true` | compact inline code fragment |
| block | `inline=false` (default) | full block with optional toolbar |
| copied | user clicks copy button | icon changes to check mark for 2 seconds |
| line-highlighted | line index in `highlightLines` | accent background on that line |

### Component States

- `copied`: internal boolean, set to `true` on copy, reset after 2000ms

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | copy uses `navigator.clipboard.writeText` internally with 2s visual feedback; no external event |

## 6. Accessibility

### Semantics

- Inline: renders `<code>` element, no role needed
- Block: outer `<div>` with `aria-label` from prop if provided
- Copy button: `<button>` with accessible label (e.g., "Copy code" / "Copied")
- Pre/code: standard `<pre><code>` semantics

### Keyboard

| Key | Behavior |
|-----|----------|
| Tab | focuses copy button (if present) |
| Enter/Space | activates copy button |

### Focus And Announcement

- Copy button is the only focusable element
- `focus-visible` outline uses focus ring token
- Copy feedback should announce state change to screen readers

## 7. Layout

### Sizing

- Inline: `display: inline`, sizes to content
- Block: `display: flex, flex-direction: column`, full width of container
- Block respects `maxHeight` prop as inline style with overflow scroll
- Horizontal overflow scrolls via `.code__scroll`

### Composition

- parent expectations: documentation, form help text, detail views, articles
- child expectations: plain text source code (no child components)
- resizing rules: block fills container width; inline flows with text

## 8. Token Usage — Exact Values

### Inline mode `.code.code--inline`

| Property | Value |
|----------|-------|
| `display` | `inline` |
| `padding` | `0.125rem 0.375rem` |
| `border-radius` | `0.25rem` |
| `background` | `color-mix(in srgb, var(--flint-color-background-panel) 72%, var(--flint-color-background-elevated))` |
| `color` | `var(--flint-color-text-primary)` |
| `font-family` | `var(--flint-typography-code-family)` |
| `font-size` | `0.8125em` |
| `line-height` | `1.5` |

### Block mode `.code.code--block`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 42%, transparent)` |
| `border-radius` | `var(--flint-radius-surface)` |
| `background` | `color-mix(in srgb, var(--flint-color-background-panel) 92%, var(--flint-color-background-elevated))` |
| `overflow` | `hidden` |

`maxHeight` applied as inline style on root when prop is provided.

### Toolbar `.code__toolbar`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `padding` | `0.375rem 0.625rem` |
| `border-bottom` | `0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 32%, transparent)` |

### Language label `.code__language`

| Property | Value |
|----------|-------|
| `font-family` | `var(--flint-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `var(--flint-typography-label-weight)` |
| `color` | `var(--flint-color-text-secondary)` |
| `text-transform` | `uppercase` |
| `letter-spacing` | `0.05em` |

### Toolbar actions `.code__toolbar-actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `gap` | `0.25rem` |

### Copy button `.code__copy`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.5rem` |
| `height` | `1.5rem` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `0.25rem` |
| `background` | `transparent` |
| `color` | `var(--flint-color-text-secondary)` |
| `cursor` | `pointer` |
| `transition` | `color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard)` |

### Copy button — hover

| Property | Value |
|----------|-------|
| `color` | `var(--flint-color-text-primary)` |

### Copy button — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing)` |

### Copy button SVG icon

| Property | Value |
|----------|-------|
| `width` | `0.875rem` |
| `height` | `0.875rem` |

Displays check icon when `copied` is true, copy icon otherwise.

### Scroll container `.code__scroll`

| Property | Value |
|----------|-------|
| `overflow-x` | `auto` |

### Pre `.code__pre`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `padding` | `0.75rem 1rem` |

### Source `.code__source`

| Property | Value |
|----------|-------|
| `display` | `block` |
| `font-family` | `var(--flint-typography-code-family)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.4` |
| `color` | `var(--flint-color-text-primary)` |
| `tab-size` | `2` |
| `white-space` | `pre` |

### Line `.code__line`

| Property | Value |
|----------|-------|
| `display` | `block` |

### Line highlighted `.code__line--highlighted`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--flint-color-accent-base) 12%, transparent)` |
| `margin` | `0 -1rem` |
| `padding` | `0 1rem` |

### Line number `.code__line-number`

| Property | Value |
|----------|-------|
| `display` | `inline-block` |
| `width` | `2.5rem` |
| `padding-right` | `1rem` |
| `color` | `var(--flint-color-text-secondary)` |
| `text-align` | `right` |
| `user-select` | `none` |
| `font-variant-numeric` | `tabular-nums` |

## 9. Svelte Notes

- Inline mode renders a single `<code>` element with class `code code--inline`
- Block mode renders a `<div>` wrapper with `<pre><code>` inside
- Toolbar rendered conditionally when `language` or `showCopyButton` is truthy
- Lines split from `source` by newline; each wrapped in a `<span class="code__line">`
- Copy uses `navigator.clipboard.writeText(source)` with a 2-second `copied`
  boolean for icon swap feedback
- No external events dispatched

## 10. GPUI Notes

- expected crate/module surface: `flint_gpui::primitives::code`
- Inline mode: GPUI renders as a styled text run within a parent text element
- Block mode: GPUI uses a scrollable container with monospace text rendering
- Copy button: GPUI must use platform clipboard API
- color-mix mappings:
  - `color-mix(in srgb, X 72%, Y)` maps to `X.blend(Y, 0.72)`
  - `color-mix(in srgb, X 92%, Y)` maps to `X.blend(Y, 0.92)`
  - `color-mix(in srgb, X 42%, transparent)` maps to `X.opacity(X.a * 0.42)`
  - `color-mix(in srgb, X 12%, transparent)` maps to `X.opacity(X.a * 0.12)`
  - `color-mix(in srgb, X 32%, transparent)` maps to `X.opacity(X.a * 0.32)`
- Line highlighting: GPUI must extend background highlight by 1rem on each side
- text-transform uppercase: GPUI must uppercase language label programmatically

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] inline vs block mode selection via `inline` prop
- [ ] copy button clipboard behavior with 2s feedback
- [ ] line numbering display and alignment
- [ ] highlighted line indices match `highlightLines` prop

### Tier 2: Visual Parity

- [ ] inline padding 0.125rem 0.375rem matches
- [ ] inline border-radius 0.25rem matches
- [ ] inline background color-mix (panel 72%, elevated) matches
- [ ] inline font-size 0.8125em matches
- [ ] block border 0.0625rem color-mix (border-subtle 42%) matches
- [ ] block border-radius uses `--flint-radius-surface`
- [ ] block background color-mix (panel 92%, elevated) matches
- [ ] toolbar padding 0.375rem 0.625rem matches
- [ ] toolbar border-bottom 0.0625rem color-mix (border-subtle 32%) matches
- [ ] language label font-size 0.6875rem, uppercase, letter-spacing 0.05em
- [ ] copy button 1.5rem square, icon 0.875rem square
- [ ] source font-size 0.8125rem, line-height 1.625, tab-size 2
- [ ] line highlight background accent-base 12%, margin/padding +/-1rem
- [ ] line number width 2.5rem, tabular-nums, right-aligned

### Tier 3: Implementation Freedom

- [ ] syntax highlighting/tokenization approach is platform-owned
- [ ] scroll container implementation details stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| syntax highlighting not specified | tokenization is implementation-specific | allowed | may add token color contracts later |
| clipboard API differs per platform | GPUI uses native clipboard vs navigator.clipboard | allowed | keep 2s feedback timing |

## 13. Specimen Definitions

### Block With Language Label

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Block with language label | `source` = TypeScript import + function snippet, `language="typescript"` | Block code container with toolbar showing "TYPESCRIPT" label and copy button |

### With Line Numbers And Highlight

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With line numbers and highlight | same TypeScript source, `language="ts"`, `showLineNumbers`, `highlightLines={[3, 4]}` | Block code with line number gutter; lines 3 and 4 have accent background highlight |

### CSS With Max Height

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| CSS with max height | `source` = CSS rule block, `language="css"`, `maxHeight="6rem"` | Block code constrained to 6rem height with vertical scroll if content overflows |

### Inline Code

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Inline code | `source="npm install"`, `inline` | Compact inline code fragment within surrounding paragraph text |

### No Copy Button

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| No copy button | `source="echo 'hello world'"`, `language="bash"`, `showCopyButton={false}` | Block code with language label but no copy button in toolbar |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: documentation pages, code examples, API reference,
  configuration displays
- future follow-up: syntax highlighting token color contracts if needed
