# MetaBar

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `MetaBar`
- Layer: `foundation`
- Summary: an inline wrapping metadata ribbon for page headers and compact
  contextual facts
- In scope: horizontal metadata layout, wrapping, optional visual separators
- Out of scope: value semantics, status treatment, copy affordances, detail-item
  block layout

## 2. Public Props

| Prop | Type | Default |
|------|------|---------|
| `ariaLabel` | `string \| null` | `null` |
| `showSeparators` | `boolean` | `true` |

## 3. Composition

| Snippet | Purpose |
|---------|---------|
| `children()` | Inline metadata items such as `MetaItem`, `Pill`, `Code`, or caller-owned content |

## 4. Behavior

- lays out children in a wrapping inline row (`display: flex`, `flex-wrap: wrap`,
  `gap: 0.5rem`, `line-height: 1.4`)
- keeps metadata visually grouped but distinct from surrounding header copy
- inserts a subtle dot separator before a child when `showSeparators` is true —
  separators are per-child opt-in, not blanket "between adjacent items". A child
  draws a leading separator dot only when it is not the first child **and** it
  carries `data-separator="true"`. `MetaItem` sets `data-separator` from its own
  `separator` prop (default `true`), so callers opt a `MetaItem` out by passing
  `separator={false}`.
- pill suppression: any child whose subtree contains a `.poodle-pill` (e.g. a
  `MetaItem` wrapping a `Pill`, or a bare `Pill`) suppresses its leading
  separator dot and leading padding even when `data-separator="true"`. A
  `MetaItem` that contains a pill additionally collapses its internal gap to `0`
  and hides its own label.
- injects pill typography context via `setPillContext({ size: "md", typography:
  "inherit" })`, so descendant `Pill`s inherit MetaBar typography sizing rather
  than their own default size.
- does not impose any other item-level semantics beyond layout

## 5. Boundary

- use `MetaBar` for compact header or summary metadata
- use `DetailItem` for block-level label/value presentation in detail sections
- callers own item semantics such as copyable IDs, status pills, timestamps,
  and links

## 6. Accessibility

### Semantics

- root carries `data-separators={showSeparators}` and is given `ariaLabel` when
  the metadata ribbon needs a named group
- the root reads each child's `data-separator` attribute (presentational, set by
  the child — `MetaItem` emits it from its `separator` prop) to decide whether to
  draw that child's leading separator dot
- child content is caller-owned and should provide its own semantics where
  needed

### Keyboard

| Key | Behavior |
|-----|----------|
| none | layout-only container; interactive behavior comes from child content |

## 7. Token Usage

### Root `.meta-bar`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `gap` | `0.5rem` |
| `min-width` | `0` |
| `line-height` | `1.4` |

### Separator dot — `> * + [data-separator="true"]::before`

Drawn only when `data-separators="true"`, the child is not the first child, and
the child carries `data-separator="true"` (and is not pill-suppressed).

| Property | Value |
|----------|-------|
| separated child `padding-inline-start` | `1rem` (`0.75rem` under `40rem` viewport) |
| dot `position` | `absolute`, `inset-inline-start: 0.375rem` (`0.25rem` under `40rem`), vertically centered |
| dot `width` / `height` | `0.25rem` |
| dot `border-radius` | `999px` |
| dot `background` | `color-mix(in srgb, var(--poodle-color-text-secondary) 72%, transparent)` |

### Pill suppression — `> *:has(.poodle-pill)[data-separator="true"]`

| Property | Value |
|----------|-------|
| `padding-inline-start` | `0` |
| dot `::before` | `display: none` |
| `.poodle-meta-item:has(.poodle-pill)` gap (`--poodle-meta-item-gap`) | `0` |
| `.poodle-meta-item:has(.poodle-pill) .poodle-meta-item__label` | `display: none` |

### Responsive breakpoint

| Condition | Effect |
|-----------|--------|
| `max-width: 40rem` | separated child `padding-inline-start` → `0.75rem`; dot `inset-inline-start` → `0.25rem` |
