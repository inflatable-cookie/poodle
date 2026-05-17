# Field Set

Status: active
Updated: 2026-03-23

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
└── fields grid
    └── child field content
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| `root` | yes | Native `<fieldset>` wrapper | spacing, layout |
| `legend` | no | Optional grouping label | label typography, spacing |
| `fields` | yes | Grid wrapper for child content | gap spacing |

## 3.1 Composition

| Snippet | Purpose |
|---------|---------|
| `children()` | Child `Field` components and other grouped form content |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `legend` | `string \| null` | `null` | no | Optional field-group label |
| `columns` | `number` | `1` | no | Grid column count |
| `gap` | `SpaceScale` | `"md"` | no | Space token used between fields |
| `span` | `number \| "full" \| null` | `null` | no | Optional parent-grid span |

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| single-column | `columns=1` | Fields stack in one column |
| multi-column | `columns>1` | Fields render in a responsive grid |
| legendless | `legend=null` | No legend is rendered |

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
| `fields` | Poodle space scale | Grid gap between grouped fields |
