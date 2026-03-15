# StateTile

Status: seed contract
Updated: 2026-03-13

## 1. Purpose

- Component name: `StateTile`
- Layer: `composites`
- Summary: a compact metadata display tile showing a label and its current value,
  used for exposing state signals, configuration keys, or runtime metadata
- In scope: label-value pairs with contained surface treatment, code-style labels
- Out of scope: interactive editing, multi-value display, charts or gauges

## 2. Anatomy

```text
[Root]
  ├── [Label]
  └── [Value]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | contained tile surface | background, radius, border |
| Label | yes | code-style metadata key | typography (code family), secondary color |
| Value | yes | bold display value | typography (body), primary color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `label` | `string` | — | yes | metadata key or signal name |
| `value` | `string` | — | yes | current value to display |
| `ariaLabel` | `string \| null` | `null` | no | overrides computed `label: value` accessible name |

### Controlled And Uncontrolled

- display composite only; value is always externally driven

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | render | contained tile with label above bold value |

### Component States

No internal state. StateTile is a pure display component.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive |

## 6. Accessibility

### Semantics

- Role: generic container (`<div>`)
- Required attributes: none
- Optional attributes: `aria-label` for composite accessible name
- Labeling rules: defaults to `"label: value"` pattern when no explicit
  `ariaLabel` is provided

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive |

### Focus And Announcement

- focus entry: not focusable
- live-region behavior: none
- GPUI-native accessibility mapping notes: expose as labeled value group

## 7. Layout

### Sizing

- fills available width by default (block-level)
- minimum content height driven by label + value stack

### Composition

- parent expectations: grids, stacks, signal panels, workspace surfaces
- child expectations: none (self-contained)
- resizing rules: tile stretches horizontally, value text truncates if needed

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `--pug-color-background-surface` | tile background fill |
| Root | `--pug-radius-surface` | corner radius |
| Label | `--pug-color-text-secondary` | subdued label color |
| Label | `--pug-typography-code-family` | monospace label styling |
| Value | `--pug-color-text-primary` (inherited) | value emphasis |

## 9. Svelte Notes

- simple `<div>` with two styled children
- light theme override uses `--pug-treatment-surface-fill`

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::state_tile`
- render as vertical stack of styled text within contained surface

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] label and value props have the same meaning
- [ ] accessible name computation matches

### Tier 2: Visual Parity

- [ ] tile surface treatment matches
- [ ] typography hierarchy matches (code label, bold value)

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: workspace signals, command state panels, configuration
  displays, runtime metadata
- future follow-up: consider whether a `tone` prop is needed for warning/error
  state tiles, or whether a slot-based value variant is needed for richer content
