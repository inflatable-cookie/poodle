# Eyebrow

Status: seed contract
Updated: 2026-03-13

## 1. Purpose

- Component name: `Eyebrow`
- Layer: `foundation`
- Summary: a small uppercase typographic label used to introduce or categorize
  a section, card, or content block
- In scope: short category labels, section intros, metadata headers
- Out of scope: contained pill/badge styling, interactive labels

## 2. Anatomy

```text
[Root]
  └── [Text content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<p>` element with uppercase label styling | typography, text color |
| Text content | yes | short label text via default slot | none (inherits) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `ariaLabel` | `string \| null` | `null` | no | optional explicit accessible name |

### Controlled And Uncontrolled

- display primitive only

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

- sizes to content width
- single-line by default, no wrapping expected

### Composition

- parent expectations: section headers, cards, page headers, panel surfaces
- child expectations: short text content only

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `--pug-color-text-secondary` | label color |
| Root | `--pug-typography-label-family` | font family |

## 9. Svelte Notes

- simple styled `<p>` with slot
- no wrapper overhead

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::eyebrow`
- render as styled text with uppercase transform and letter-spacing

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] non-interactive text semantics match

### Tier 2: Visual Parity

- [ ] typography treatment matches (size, weight, spacing, transform)

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: section headers, card intros, metadata labels, page
  headers
- future follow-up: consider whether Eyebrow should accept a `tone` prop for
  accent-colored variants
