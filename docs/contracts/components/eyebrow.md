# Eyebrow

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Eyebrow`
- Layer: `foundation`
- Summary: a small uppercase typographic label used to introduce or categorize
  a section, card, or content block
- In scope: short category labels, section intros, metadata headers, semantic
  section headings using the same visual treatment
- Out of scope: contained pill/badge styling, interactive labels

## 2. Anatomy

```text
[Root .eyebrow]
  └── [Text content]
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `<span>`, `<p>`, `<h2>`, `<h3>`, or `<h4>` | yes | text element with uppercase label styling |
| Text content | snippet | yes | short label text via `children()` |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `as` | `"span" \| "p" \| "h2" \| "h3" \| "h4"` | `"span"` | no | rendered element |
| `ariaLabel` | `string \| null` | `null` | no | optional explicit accessible name |
| `size` | `"xs" \| "sm" \| "md"` | `"sm"` | no | visual size; `sm` preserves the historical default |
| `spacing` | `"none" \| "bottom"` | `"none"` | no | optional bottom margin for heading-like use |

### Controlled And Uncontrolled

- Display primitive only; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | render | small uppercase secondary-color label |
| semantic heading | `as="h2"`, `as="h3"`, or `as="h4"` | same visual treatment on a heading element |

### Component States

No internal state.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive |

## 6. Accessibility

### Semantics

- Role: inline text by default (`<span>`); heading semantics when rendered as
  `h2`, `h3`, or `h4`
- Required attributes: none
- Optional attributes: `aria-label` when visible text is abbreviated
- Labeling rules: purely decorative typography; not focusable

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive |

### Focus And Announcement

- focus entry: not focusable
- live-region behavior: none
- GPUI-native accessibility mapping notes: render as styled text label

## 7. Layout

### Sizing

- Sizes to content width
- Single-line by default, no wrapping expected
- Heading elements may use `spacing="bottom"` when followed by related content

### Composition

- parent expectations: section headers, cards, page headers, panel surfaces
- child expectations: short text content only

## 8. Token Usage

### Root `.eyebrow`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.12em` |
| `line-height` | `1.5` |
| `text-transform` | `uppercase` |

### Size and spacing variants

| Selector | Property | Value |
|----------|----------|-------|
| `[data-size="xs"]` | `font-size` | `0.6875rem` |
| `[data-size="md"]` | `font-size` | `0.85rem` |
| `[data-size="md"]` | `letter-spacing` | `0.04em` |
| `[data-spacing="bottom"]` | `margin-bottom` | `0.5rem` |
| `[data-size="xs"][data-spacing="bottom"]` | `margin-bottom` | `0.35rem` |

## 9. Svelte Notes

- Renders as a simple styled element with `children()`
- No wrapper overhead; the root element is the `<span>` itself
- `margin: 0` resets browser default margins

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::eyebrow`
- render as styled text with uppercase transform and letter-spacing
- GPUI text rendering should apply the equivalent of `text-transform: uppercase`
  either by transforming the string or via a text style flag

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] non-interactive text semantics match
- [ ] renders as inline-level element by default (`<span>`)
- [ ] heading element semantics are preserved when `as` is a heading

### Tier 2: Visual Parity

- [ ] font-size matches 0.6875rem
- [ ] font-weight matches 600
- [ ] letter-spacing matches 0.12em
- [ ] text-transform uppercase applied
- [ ] line-height matches 1.5
- [ ] color matches `--poodle-color-text-secondary`

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Specimen Definitions

### Section Label

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Section label | text content: "Section label", followed by descriptive paragraph | Small uppercase secondary-color label above content |

### Primitive Category

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Primitive | text content: "Primitive", followed by heading "Button" and description | Eyebrow categorizing a component as primitive type |

### Composite Category

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Composite | text content: "Composite", followed by heading "DataTable" and description | Eyebrow categorizing a component as composite type |

### Semantic Section Heading

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Semantic section heading | `as="h3"`, `size="md"`, `spacing="bottom"` | Heading element with uppercase muted label styling and bottom spacing |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: section headers, card intros, metadata labels, page
  headers
- future follow-up: consider whether Eyebrow should accept a `tone` prop for
  accent-colored variants
