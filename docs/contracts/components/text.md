# Text

Status: contract
Updated: 2026-06-12

## 1. Purpose

- Component name: `Text`
- Layer: `display`
- Summary: small text primitive for body, caption, hint, and status copy.
- In scope: semantic element choice, tone, size, line-height, and compact child
  spacing.
- Out of scope: headings, links, rich typography, markdown, and layout shells.

## 2. Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `as` | `"p" \| "span" \| "div"` | `"p"` | rendered element |
| `tone` | `"default" \| "secondary" \| "muted" \| "success" \| "danger" \| "warning"` | `"default"` | color role |
| `size` | `"xs" \| "sm" \| "md"` | `"md"` | text size |
| `weight` | `"normal" \| "medium" \| "semibold" \| "bold"` | `"normal"` | font weight |
| `leading` | `"normal" \| "relaxed"` | `"normal"` | line-height |
| `spacing` | `"none" \| "compact"` | `"none"` | compact grid gap for child paragraphs |
| `clamp` | `"none" \| 1 \| 2 \| 3` | `"none"` | optional line clamp |
| `children` | `Snippet` | none | text/content |

## 3. Visual Rules

- Root margin is always `0`.
- Default color uses `--poodle-color-text-primary`.
- Secondary and muted use `--poodle-color-text-secondary`.
- Status tones use the matching `--poodle-color-status-*` token.
- Sizes:
  - `xs`: `0.75rem`
  - `sm`: `0.8125rem`
  - `md`: `0.875rem`
- Weights:
  - `normal`: inherited/default
  - `medium`: `500`
  - `semibold`: `600`
  - `bold`: `700`
- Normal line-height is `1.5`; relaxed line-height is `1.6`.
- `spacing="compact"` renders a grid with `--poodle-space-stack-sm` gap.
- `clamp={1 | 2 | 3}` applies `-webkit-line-clamp`, `line-clamp`, hidden
  overflow, and vertical box orientation.

## 4. Accessibility

- `Text` does not add ARIA roles.
- Use `as="p"` for paragraphs, `as="span"` for inline phrases, and `as="div"`
  when wrapping multiple text nodes.

## 5. Specimen

- `TextSpecimen.svelte` covers tones, sizes, relaxed leading, inline text, and
  clamped text.
