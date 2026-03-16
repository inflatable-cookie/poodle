# Eyebrow

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Eyebrow`
- Layer: `foundation`
- Summary: a small uppercase typographic label used to introduce or categorize
  a section, card, or content block
- In scope: short category labels, section intros, metadata headers
- Out of scope: contained pill/badge styling, interactive labels

## 2. Anatomy

```text
[Root .eyebrow]
  └── [Text content (default slot)]
```

| Part | Element | Required | Description |
|------|---------|----------|-------------|
| Root | `<p>` | yes | paragraph element with uppercase label styling |
| Text content | slot | yes | short label text via default slot |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `ariaLabel` | `string \| null` | `null` | no | optional explicit accessible name |

### Controlled And Uncontrolled

- Display primitive only; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | render | small uppercase secondary-color label |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive |

## 6. Accessibility

### Semantics

- Role: paragraph text (`<p>`)
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

### Composition

- parent expectations: section headers, cards, page headers, panel surfaces
- child expectations: short text content only

## 8. Token Usage

### Root `.eyebrow`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--pug-color-text-secondary)` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `600` |
| `letter-spacing` | `0.12em` |
| `line-height` | `1.5` |
| `text-transform` | `uppercase` |

## 9. Svelte Notes

- Renders as a simple styled `<p>` element with a default slot
- No wrapper overhead; the root element is the `<p>` itself
- `margin: 0` resets browser default paragraph margins

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::eyebrow`
- render as styled text with uppercase transform and letter-spacing
- GPUI text rendering should apply the equivalent of `text-transform: uppercase`
  either by transforming the string or via a text style flag

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] non-interactive text semantics match
- [ ] renders as paragraph-level element

### Tier 2: Visual Parity

- [ ] font-size matches 0.6875rem
- [ ] font-weight matches 600
- [ ] letter-spacing matches 0.12em
- [ ] text-transform uppercase applied
- [ ] line-height matches 1.5
- [ ] color matches `--pug-color-text-secondary`

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: section headers, card intros, metadata labels, page
  headers
- future follow-up: consider whether Eyebrow should accept a `tone` prop for
  accent-colored variants
