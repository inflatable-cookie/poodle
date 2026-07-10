# Code

Status: detailed contract
Updated: 2026-07-10

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
[Inline wrap .code.code--inline-wrap]  <span>
  ├── [Root .code.code--inline]  <code>
  │     └── [Source text]
  └── [Copy button .code__copy.code__copy--inline]  <button> (when showCopyButton)
        └── [Icon SVG]  (copy or check)
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
| Inline wrap | yes (inline) | inline-flex wrapper around the code fragment and optional copy button | display, gap |
| Root (inline) | yes | inline code fragment | background, padding, radius, typography |
| Copy button (inline) | no | compact adjacent copy button (`.code__copy--inline`) | size, icon size |
| Root (block) | yes | block code container | border, radius |
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
| `showCopyButton` | `boolean` | `true` | no | show copy-to-clipboard button; when `inline`, renders a compact adjacent copy button |
| `maxHeight` | `string \| null` | `null` | no | max-height for block container (inline style) |
| `inline` | `boolean` | `false` | no | render as inline `<code>` element |
| `inlineVariant` | `"default" \| "plain"` | `"default"` | no | inline only; `"plain"` drops inline padding, radius, and background |
| `typography` | `"body" \| "inline"` | `"body"` | no | inline only; `"inline"` sets font-size `1em × adjustmentRatio` and `line-height: inherit` so the fragment matches surrounding text |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the code block |

### Controlled And Uncontrolled

- display primitive; internal state limited to copy feedback (2-second timer)

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| inline | `inline=true` | compact inline code fragment |
| inline plain | `inline=true`, `inlineVariant="plain"` | inline fragment with no padding, radius, or background |
| inline typography | `inline=true`, `typography="inline"` | inline fragment at `1em × adjustmentRatio` with inherited line-height |
| block | `inline=false` (default) | full block with optional toolbar |
| copied | user clicks copy button | icon changes to check mark for 2 seconds |
| line-highlighted | line index in `highlightLines` | accent background on that line |

### Component States

- `copied`: internal boolean, set to `true` on copy, reset after 2000ms

### Behavior Machine

Behavior classification: adapter-owned interaction (g11.004 sweep)

Copy-feedback timer only; otherwise presentational. Timer stays adapter-side.

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

### Inline wrap `.code.code--inline-wrap`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.25rem` |

### Inline mode `.code.code--inline`

| Property | Value |
|----------|-------|
| `display` | `inline` |
| `padding` | `0.125rem 0.375rem` |
| `border-radius` | `0.25rem` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 72%, var(--poodle-color-background-elevated))` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `calc(0.8125em × var(--poodle-typography-code-adjustmentRatio))` |
| `line-height` | `1.5` |

### Inline mode — `inlineVariant="plain"` (`[data-inline-variant="plain"]`)

| Property | Value |
|----------|-------|
| `padding` | `0` |
| `border-radius` | `0` |
| `background` | `transparent` |

### Inline mode — `typography="inline"` (`[data-typography="inline"]`)

| Property | Value |
|----------|-------|
| `font-size` | `calc(1em × var(--poodle-typography-code-adjustmentRatio))` |
| `line-height` | `inherit` |

### Block mode `.code.code--block`

The block root carries no background; the visible code surface background lives
on `.code__pre`.

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-direction` | `column` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `overflow` | `hidden` |

`maxHeight` applied as inline style on root when prop is provided.

### Toolbar `.code__toolbar`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `padding` | `0.375rem 0.625rem` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 60%, var(--poodle-color-background-panel))` |
| `border-bottom` | `0.0625rem solid var(--poodle-color-border-subtle)` |

### Language label `.code__language`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `text-transform` | `uppercase` |
| `letter-spacing` | `0.05em` |

### Toolbar actions `.code__toolbar-actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `margin-left` | `auto` |
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
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |
| `transition` | `color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Copy button — hover

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

### Copy button — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |

### Copy button SVG icon

| Property | Value |
|----------|-------|
| `width` | `0.875rem` |
| `height` | `0.875rem` |

Displays check icon when `copied` is true, copy icon otherwise.

### Inline copy button `.code__copy--inline`

| Property | Value |
|----------|-------|
| `width` | `1.25rem` |
| `height` | `1.25rem` |

### Inline copy button SVG icon `.code__copy--inline svg`

| Property | Value |
|----------|-------|
| `width` | `0.75rem` |
| `height` | `0.75rem` |

### Scroll container `.code__scroll`

| Property | Value |
|----------|-------|
| `overflow-x` | `auto` |

### Pre `.code__pre`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `padding` | `0.75rem 1rem` (density-adjusted: compact `0.5rem 0.75rem`, comfortable `1rem 1.25rem`) |
| `background` | `color-mix(in srgb, var(--poodle-color-background-canvas) 92%, black)` |

### Source `.code__source`

| Property | Value |
|----------|-------|
| `display` | `block` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.4` |
| `color` | `var(--poodle-color-text-primary)` |
| `tab-size` | `2` |
| `white-space` | `pre` |

### Line `.code__line`

| Property | Value |
|----------|-------|
| `display` | `block` |

### Line highlighted `.code__line--highlighted`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |
| `margin` | `0 -1rem` |
| `padding` | `0 1rem` |

### Line number `.code__line-number`

| Property | Value |
|----------|-------|
| `display` | `inline-block` |
| `width` | `2.5rem` |
| `padding-right` | `1rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `text-align` | `right` |
| `user-select` | `none` |
| `font-variant-numeric` | `tabular-nums` |

### Size adjustments

Inline code font sizing uses `var(--poodle-typography-code-adjustmentRatio)` so
body-context monospace scale can be tuned in one place when the configured code
face reads larger or smaller than the default sans family.

| Size | source font-size | inline font-size |
|------|-----------------|-----------------|
| `xs` | `0.6875rem` | `0.6875em × var(--poodle-typography-code-adjustmentRatio)` |
| `sm` | `0.75rem` | `0.75em × var(--poodle-typography-code-adjustmentRatio)` |
| `md` | `0.8125rem` | `0.8125em × var(--poodle-typography-code-adjustmentRatio)` |
| `lg` | `0.875rem` | `0.875em × var(--poodle-typography-code-adjustmentRatio)` |
| `xl` | `0.9375rem` | `0.9375em × var(--poodle-typography-code-adjustmentRatio)` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- Inline mode renders a single `<code>` element with class `code code--inline`
- Block mode renders a `<div>` wrapper with `<pre><code>` inside
- Toolbar rendered conditionally when `language` or `showCopyButton` is truthy
- Lines split from `source` by newline; each wrapped in a `<span class="code__line">`
- Copy uses `navigator.clipboard.writeText(source)` with a 2-second `copied`
  boolean for icon swap feedback
- No external events dispatched

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::code`
- Inline mode: GPUI renders as a styled text run within a parent text element
- Block mode: GPUI uses a scrollable container with monospace text rendering
- Copy button: GPUI must use platform clipboard API
- color-mix mappings:
  - inline background `color-mix(in srgb, panel 72%, elevated)` maps to `panel.blend(elevated, 0.72)`
  - pre background `color-mix(in srgb, canvas 92%, black)` maps to `canvas.blend(black, 0.92)`
  - toolbar background `color-mix(in srgb, elevated 60%, panel)` maps to `elevated.blend(panel, 0.60)`
  - line highlight `color-mix(in srgb, accent-base 12%, transparent)` maps to `accent.opacity(accent.a * 0.12)`
  - block border / toolbar border-bottom use the plain `border-subtle` token (no mix)
- Line highlighting: GPUI must extend background highlight by 1rem on each side
- text-transform uppercase: GPUI must uppercase language label programmatically

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] inline vs block mode selection via `inline` prop
- [ ] copy button clipboard behavior with 2s feedback
- [ ] line numbering display and alignment
- [ ] highlighted line indices match `highlightLines` prop

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] inline padding 0.125rem 0.375rem matches
- [ ] inline border-radius 0.25rem matches
- [ ] inline background color-mix (panel 72%, elevated) matches
- [ ] inline font-size 0.8125em matches
- [ ] block border 0.0625rem solid border-subtle (plain token) matches
- [ ] block border-radius uses `--poodle-radius-surface`
- [ ] pre background color-mix (canvas 92%, black) matches
- [ ] toolbar padding 0.375rem 0.625rem matches
- [ ] toolbar background color-mix (elevated 60%, panel) matches
- [ ] toolbar border-bottom 0.0625rem solid border-subtle (plain token) matches
- [ ] language label font-size 0.6875rem, uppercase, letter-spacing 0.05em
- [ ] copy button 1.5rem square, icon 0.875rem square; inline copy 1.25rem square, icon 0.75rem square
- [ ] source font-size 0.8125rem, line-height 1.4, tab-size 2
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
