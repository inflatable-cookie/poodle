# Avatar

Status: contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Avatar`
- Layer: `display`
- Summary: image or initials avatar for user identity surfaces.
- In scope: image rendering, initials fallback, size variants, circle/rounded
  shape, neutral/accent tone, and decorative mode.
- Out of scope: presence status, upload workflows, user menus, and profile
  editing.

## 2. Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `src` | `string \| null` | `null` | image URL |
| `alt` | `string \| null` | `null` | image alt text and fallback accessible label |
| `initials` | `string \| null` | `null` | fallback text, trimmed to three uppercase characters |
| `ariaLabel` | `string \| null` | `null` | accessible label for initials fallback |
| `decorative` | `boolean` | `false` | hides avatar from assistive tech when adjacent text names the user |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `"md"` | avatar size |
| `shape` | `"circle" \| "rounded"` | `"circle"` | avatar shape |
| `tone` | `"neutral" \| "accent"` | `"neutral"` | fallback background tone |

## 3. Visual Rules

- Root is an inline-flex square with centered content and hidden overflow.
- Images fill the square and use `object-fit: cover`.
- Circle shape uses `border-radius: 50%`.
- Rounded shape uses `--poodle-radius-control`.
- Size scale:
  - `xs`: `1.5rem`
  - `sm`: `2rem`
  - `md`: `2.75rem`
  - `lg`: `4.5rem`
  - `xl`: `6rem`
- Initials font scale:
  - `xs`: `0.625rem`
  - `sm`: `0.75rem`
  - `md`: `1rem`
  - `lg`: `1.5rem`
  - `xl`: `2rem`

## 4. Accessibility

- With `src`, the image receives `alt` unless `decorative=true`.
- Without `src`, the root uses `role="img"` and `aria-label` unless
  `decorative=true`.
- With `decorative=true`, root receives `aria-hidden="true"` and image alt is
  empty.
- Use decorative mode when visible adjacent copy already names the user.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Specimen

- `AvatarSpecimen.svelte` covers initials, size scale, tone, shape, and image
  rendering.
