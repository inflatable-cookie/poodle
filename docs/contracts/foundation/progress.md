# Progress

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Progress`
- Layer: `foundation`
- Summary: a determinate or indeterminate progress indicator for task
  completion status
- In scope: value range, current progress, indeterminate state, accessible
  status semantics
- Out of scope: stepper workflows, upload-specific shell wrappers

## 2. Anatomy

```text
[Root]
  ├── [Track]
  └── [Indicator]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | progress host | spacing, status context |
| Track | yes | total range shell | background |
| Indicator | yes | completed or active fill | accent/status color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `number \| null` | `null` | no | current progress value |
| `max` | `number` | `100` | no | maximum range |
| `isIndeterminate` | `boolean` | `false` | no | active progress with no fixed value |
| `ariaLabel` | `string \| null` | `null` | no | optional accessible name when context needs it |
| `valueText` | `string \| null` | `null` | no | human-readable progress text |

### Controlled And Uncontrolled

- controlled-only display primitive

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| determinate | `value` present and `isIndeterminate=false` | indicator width reflects progress |
| indeterminate | `isIndeterminate=true` | active motion treatment |
| complete | value reaches max | full completion state |

### Component States

Determinate vs indeterminate state is required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | display primitive only |

## 6. Accessibility

### Semantics

- Role: progress indicator semantics
- Required attributes: current value when determinate, range max when relevant
- Optional attributes: accessible name and value text
- Labeling rules: when progress meaning is unclear from surrounding text, an
  explicit label is required

### Keyboard

| Key | Behavior |
|-----|----------|
| none | not interactive |

### Focus And Announcement

- focus entry: not focusable by default
- live-region behavior: parent-owned unless progress updates must be announced
  explicitly
- GPUI-native accessibility mapping notes: GPUI must expose progress semantics
  and determinate/indeterminate meaning through native accessibility APIs

## 7. Layout

### Sizing

- width is parent-owned
- height remains small but visible

### Composition

- parent expectations: status rows, loading shells, forms, task flows
- child expectations: none
- resizing rules: indicator fill scales with parent width

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Track | background roles | progress shell |
| Indicator | accent/status roles | current progress |
| Motion | motion roles | indeterminate animation |

## 9. Svelte Notes

- can use native progress semantics or a styled wrapper that preserves
  accessible progress meaning

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::progress`
- GPUI implementation must intentionally expose determinate value and
  indeterminate progress semantics rather than presenting only a visual bar

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] determinate/indeterminate meaning matches
- [ ] progress accessibility semantics match

### Tier 2: Visual Parity

- [ ] track and indicator roles use comparable token mappings

### Tier 3: Implementation Freedom

- [ ] animation internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| indeterminate animation details may differ | motion internals are runtime-specific | allowed | keep progress meaning strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: loading states, task indicators
- future follow-up: pair with richer loading wrappers later

## Next Task

Keep progress semantics distinct from skeleton loading placeholders.
