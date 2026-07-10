# Field Set

Status: active
Updated: 2026-07-10

- Component name: `FieldSet`
- Layer: `foundation`

## 1. Purpose

FieldSet groups related form controls under an optional legend and provides a
simple multi-column layout for coordinated form fields without taking over
validation or submission ownership.

## 2. Anatomy

```text
fieldset (root)
├── legend (optional)
├── description (optional)
└── fields grid
    └── child field content
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| `root` | yes | Native `<fieldset>` wrapper | spacing, layout |
| `legend` | no | Optional grouping label | label typography, spacing |
| `description` | no | Optional `<p>` description rendered between legend and fields | body typography, secondary color, spacing |
| `fields` | yes | Grid wrapper for child content | gap spacing |

## 3.1 Composition

| Snippet | Purpose |
|---------|---------|
| `children()` | Child `Field` components and other grouped form content |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `legend` | `string \| null` | `null` | no | Optional field-group label |
| `description` | `string \| null` | `null` | no | Optional description rendered as a `<p>` between legend and fields |
| `columns` | `number` | `1` | no | Grid column count |
| `gap` | `SpaceScale` | `"md"` | no | Space token used between fields |
| `span` | `number \| "full" \| null` | `null` | no | Optional parent-grid span |

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| single-column | `columns=1` | Fields stack in one column |
| multi-column | `columns>1` | Fields render in a responsive grid |
| legendless | `legend=null` | No legend is rendered |
| described | `description!=null` | A `<p>` description renders between legend and fields |
| spanned | `span!=null` | Root sets `grid-column: span <n>` (or `1 / -1` when `span="full"`) within a parent grid |

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Accessibility

### Semantics

- Role: native `<fieldset>` semantics
- Required attributes: none
- Optional attributes: none
- Labeling rules: provide `legend` whenever the grouped controls need a shared
  accessible name

### Keyboard

FieldSet does not change keyboard behavior for its children.

### Focus And Announcement

- focus entry: delegated to child controls
- focus exit: delegated to child controls
- live-region or announcement behavior: none
- GPUI-native accessibility mapping notes: native renderers should preserve the
  grouping relationship between the legend and grouped controls

## 6. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| `legend` | `--poodle-color-text-secondary` | Muted grouping label color |
| `legend` | `--poodle-typography-label-family` | Grouping label family |
| `legend` | `0.6875rem` (fixed) | Legend font-size — eyebrow scale, fixed value (not `typography-label-size`) |
| `legend` | `font-weight: 600` | Legend weight |
| `legend` | `letter-spacing: 0.12em` | Legend tracking |
| `legend` | `line-height: 1.5` | Legend line height |
| `legend` | `text-transform: uppercase` | Legend casing |
| `legend` | `--poodle-space-stack-sm` | `margin-bottom` below the legend |
| `description` | `--poodle-typography-body-size` (`0.875rem`) | Description font-size |
| `description` | `--poodle-typography-body-lineHeight` (`1.5`) | Description line height |
| `description` | `--poodle-color-text-secondary` | Description color |
| `description` | `--poodle-space-stack-md` | `margin-bottom` below the description (top margin pulls up by `space-stack-sm * -0.5`) |
| `fields` | Poodle space scale (`column-gap`) | Column gap between grouped fields = `scaleToSpace(gap)` |
| `fields` | Poodle space scale + `0.5rem` (`row-gap`) | Row gap = `scaleToSpace(gap) + 0.5rem` (asymmetric, larger than column gap) |
