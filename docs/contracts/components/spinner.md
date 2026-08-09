# Spinner

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Spinner`
- Layer: `foundation`
- Summary: a compact animated loading indicator with a standard ring variant, a
  CLI-oriented six-cell grid variant, and a quiet three-dot variant
- In scope: loading animation, variant selection, size scaling, tone/color
  context, decorative or announced usage
- Out of scope: progress percentages, skeleton placeholders, full loading
  overlays, or parent-owned loading copy

## 2. Anatomy

```text
[Root .spinner]
  └── [Visual]
      ├── ring variant: [Ring .spinner__ring]  single wrapper span (rotating ring)
      ├── grid variant: [Grid .spinner__grid]  single wrapper span
      └── dots variant: [Dots .spinner__dots]  wrapper span of three [Dot .spinner__dot]
                          └── 6 [Cell .spinner__cell] span children in a 2x3 matrix
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `span` | yes | inline-flex host carrying variant, size, and tone data |
| Ring visual | `span` (`.spinner__ring`) | conditional | single wrapper span; classic rotating circular loader |
| Grid visual | `span` (`.spinner__grid`) | conditional | single wrapper span containing the cells |
| Grid cell | 6 `span` children (`.spinner__cell`) | conditional | CLI-like square loader cells with staggered fading; children of the grid wrapper |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"ring" \| "grid" \| "dots"` | `"ring"` | no | selects loader visual |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `tone` | `"current" \| "accent" \| "muted"` | `"current"` | no | color source for the indicator |
| `ariaLabel` | `string \| null` | `null` | no | optional announced loading label; decorative when omitted |

### Controlled And Uncontrolled

- Display primitive only. No internal loading state ownership.
- Parent components remain responsible for whether loading is shown.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ring | `variant="ring"` | circular spinner rotates continuously |
| dots | `variant="dots"` | three dots rise in sequence |
| grid | `variant="grid"` | six squares fade in a repeating snake loop: top-left, top-right, mid-right, mid-left, bottom-left, bottom-right, then mid-right and mid-left before reset |
| extra-small | `size="xs"` | compact chrome-sized indicator |
| small | `size="sm"` | compact inline indicator suitable for buttons |
| medium | inherited default or `size="md"` | default inline indicator |
| large | `size="lg"` | emphasized inline indicator |
| extra-large | `size="xl"` | overlay-scale or prominent indicator |
| current tone | `tone="current"` | indicator inherits surrounding currentColor |
| accent tone | `tone="accent"` | indicator uses accent base color |
| muted tone | `tone="muted"` | indicator uses text-secondary color |

### Component States

No internal state. Animation is continuous while the component is mounted.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

### Why Three Variants

`ring` is the default and reads as "this control is busy". `grid` is the
CLI-flavoured one. `dots` is the quietest: it sits beside text as "still
working" rather than as a control that is occupied, which is what an agent's
activity footer wants — the transcript is not blocked, the agent is simply not
finished.

Its animation is a rise rather than a spin, for the same reason: nothing is
rotating, so nothing suggests a wait with a definite end.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive primitive |

## 6. Accessibility

### Semantics

- Root element is non-interactive
- When `ariaLabel` is omitted, spinner is decorative and hidden from assistive
  technology
- When `ariaLabel` is provided, spinner exposes loading meaning through
  `role="status"` and polite announcement behavior

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive |

### Focus And Announcement

- Not focusable by default
- Loading announcement is optional and parent-owned; this primitive only exposes
  a status role when explicitly labeled

## 7. Layout

### Sizing

- Root is `inline-flex`
- Ring sizes:
  - `xs`: `0.625rem`
  - `sm`: `0.75rem`
  - `md`: `1rem`
  - `lg`: `1.5rem`
  - `xl`: `1.875rem`
- Grid sizes:
  - `xs`: `0.375rem` wide, `0.5625rem` tall
  - `sm`: `0.4375rem` wide, `0.6875rem` tall
  - `md`: `0.5625rem` wide, `0.9375rem` tall
  - `lg`: `0.75rem` wide, `1.25rem` tall
  - `xl`: `0.9375rem` wide, `1.5625rem` tall

### Composition

- Parent expectations: buttons, icon buttons, page loading cards, inline busy
  labels, CLI-like status zones
- Resizing rules: spinner stays shrink-wrapped and never stretches to fill
  available width

## 8. Token Usage

### Root `.spinner`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `color` | inherited for `tone="current"` |

### Tone Mapping

| Tone | Value |
|------|-------|
| `current` | `currentColor` |
| `accent` | `var(--poodle-color-accent-base)` |
| `muted` | `var(--poodle-color-text-secondary)` |

### Ring Variant

| Property | Value |
|----------|-------|
| `border` | `0.125rem solid color-mix(in srgb, currentColor 24%, transparent)` |
| `border-top-color` | `currentColor` |
| `border-radius` | `999px` |
| `animation` | `spinner-ring 0.8s linear infinite` |

### Grid Variant

| Property | Value |
|----------|-------|
| `display` | `inline-grid` |
| `grid-template-columns` | `repeat(2, 1fr)` |
| `grid-template-rows` | `repeat(3, 1fr)` |
| `gap` | size-dependent small fixed gap |
| `cell border-radius` | `0.125rem` |
| `cell background` | `currentColor` |
| `cell idle opacity` | `0.2` (rest/baseline floor; `spinner-grid-idle` keyframe holds `0.2` for unphased cells) |
| `cell animation` | `spinner-grid 1.24s linear infinite` with phase-specific opacity keyframes |
| `cell opacity range` | `0.2` (idle floor) → `0.76` (phase peak); each cell ramps within this band |
| `cell order` | top-left, top-right, middle-right, middle-left, bottom-left, bottom-right, middle-right, middle-left |

## 9. Svelte Notes

- Decorative by default; `aria-hidden="true"` is set on the root when no `ariaLabel` is provided, and `role="status"` + `aria-live="polite"` are set only when `ariaLabel` is supplied
- Accepts passthrough props: `class` (merged onto the root), `style`, and `...restProps` spread onto the root span
- The ring variant may use borders or SVG internally as long as the public
  motion and sizing contract matches
- The grid variant should use six cells in a two-column, three-row layout with
  a snake-ordered opacity sequence that revisits the middle pair before reset
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::primitives::spinner`
- GPUI may approximate the ring with an animated SVG rotation
- GPUI grid animation may be implemented as per-cell opacity pulsing derived
  from the same eight-step snake sequence
- GPUI should support color override via host context so buttons can reuse the
  same spinner primitive without losing text-color parity

## 11. Parity Checklist

- [ ] ring variant exists in both runtimes
- [ ] grid variant exists in both runtimes
- [ ] size scaling matches across runtimes
- [ ] accent/current/muted tone mapping matches
- [ ] decorative vs announced semantics match
- [ ] `xs | sm | md | lg | xl` size steps match in both ring and grid variants
