# Region

Status: active
Updated: 2026-03-23

- Component name: `Region`
- Layer: `foundation`

## 1. Purpose

Region is a presentational placeholder block that designates an area where
content could exist but currently does not. It is useful in documentation,
wireframes, layout specimens, and empty shell states where structure matters
more than the final payload.

## 2. Anatomy

```text
region (root)
└── label (text)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| `root` | yes | Dashed placeholder container | border, radius, padding |
| `label` | yes | Centered uppercase placeholder copy | label color, label typography |

## 3. Composition

Region does not accept child content. It renders only the optional `label`
inside the placeholder block.

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `label` | `string` | `""` | no | Placeholder copy shown in the centre of the region |
| `color` | `string \| null` | `null` | no | Optional custom border and label color via `--region-color` |
| `minHeight` | `string` | `"4rem"` | no | Minimum height for the placeholder region |

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no custom color | Dashed border and muted text use Poodle semantic tokens |
| custom-color | `color` set | Border and label both adopt the supplied custom color |

## 5. Accessibility

### Semantics

- Role: `presentation`
- Required attributes: none
- Optional attributes: none
- Labeling rules: the visible label is for sighted layout review only

### Keyboard

Region is non-interactive and should not be keyboard-focusable.

### Focus And Announcement

- focus entry: not applicable
- focus exit: not applicable
- live-region or announcement behavior: none
- GPUI-native accessibility mapping notes: preserve decorative-only behavior in
  native renderers

## 6. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| `root` | `--poodle-color-border-default` | Default dashed outline color |
| `root` | `--poodle-radius-surface` | Shared surface rounding |
| `root` | `--poodle-space-inline-md` | Interior spacing |
| `label` | `--poodle-color-text-tertiary` | Muted placeholder label color |
| `label` | `--poodle-typography-label-family` | Label family |
| `label` | `--poodle-typography-label-size` | Label size |
