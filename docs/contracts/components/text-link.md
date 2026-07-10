# TextLink

Status: contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `TextLink`
- Layer: `display`
- Summary: inline text action or navigation link for prose and metadata copy.
- In scope: anchor vs button rendering, disabled state, tone, focus treatment,
  and inline typography inheritance.
- Out of scope: standalone button styling, router ownership, rich link
  previews, and external-link icon policy.

## 2. Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `href` | `string \| null` | `null` | renders an `<a>` when present and not disabled |
| `target` | `string \| null` | `null` | forwarded to anchor links |
| `rel` | `string \| null` | `null` | forwarded to anchor links |
| `ariaLabel` | `string \| null` | `null` | accessible label override |
| `disabled` | `boolean` | `false` | disables activation and renders the button path |
| `tone` | `"accent" \| "inherit" \| "secondary"` | `"accent"` | text color role |
| `className` | `string` | `""` | appended to the root class list |
| `onClick` | `(event: MouseEvent) => void` | `null` | called for enabled activation |
| `children` | `Snippet` | none | inline link content |

## 3. Behavior Rules

- With `href` and `disabled=false`, root renders as an `<a>`.
- Without `href`, root renders as a `<button type="button">`.
- With `disabled=true`, root renders as a disabled button, even when `href` is
  provided.
- Disabled activation calls `preventDefault()` and does not call `onClick`.
- Enabled activation calls `onClick` after native link or button semantics are
  preserved.

## 4. Visual Rules

- Root is inline and inherits font, line-height, and text alignment.
- Root has no padding, border, or background.
- Default tone uses `--poodle-color-accent-base`.
- `tone="inherit"` uses `currentColor`.
- `tone="secondary"` uses `--poodle-color-text-secondary`.
- Text is underlined with a subtle current-color mix.
- Hover and focus-visible strengthen underline color to current color.
- Focus-visible uses `--poodle-border-width-focus` and
  `--poodle-color-accent-focusRing`.
- Disabled state uses `--poodle-state-opacity-disabled` and default cursor.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Accessibility

- Anchor rendering keeps native link semantics.
- Button rendering keeps native button semantics.
- Callers must provide readable child text or `ariaLabel`.
- Disabled links are not rendered as anchors, so they are not reachable as dead
  navigation targets.

## 6. Specimen

- `TextLinkSpecimen.svelte` covers anchor, button, tone, disabled, and inline
  prose usage.
