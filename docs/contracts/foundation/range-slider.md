# Range Slider

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `RangeSlider`
- Layer: `foundation`
- Summary: a dual-thumb range control representing lower and upper numeric
  bounds
- In scope: lower/upper value pair, min/max bounds, stepped adjustment,
  separate thumb focus
- Out of scope: histogram overlays, arbitrary multi-thumb editing

## 2. Anatomy

```text
[Root]
  ├── [Track]
  ├── [Selected Range Fill]
  ├── [Lower Thumb]
  └── [Upper Thumb]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | range slider host | spacing, focus context |
| Track | yes | full available range | background, border |
| Selected Range Fill | yes | active selected window | accent/status color |
| Lower Thumb | yes | lower bound handle | handle/focus tokens |
| Upper Thumb | yes | upper bound handle | handle/focus tokens |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `[number, number]` | none | yes | controlled lower/upper pair |
| `min` | `number` | `0` | no | lower bound |
| `max` | `number` | `100` | no | upper bound |
| `step` | `number` | `1` | no | increment size |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | axis |
| `isDisabled` | `boolean` | `false` | no | disables interaction |
| `ariaLabel` | `string \| null` | `null` | no | group label when no visible label exists |
| `lowerValueText` | `string \| null` | `null` | no | optional lower value text |
| `upperValueText` | `string \| null` | `null` | no | optional upper value text |
| `onValueChange` | `(value: [number, number]) => void` | none | no | live range updates |
| `onValueCommit` | `(value: [number, number]) => void` | none | no | committed range updates |

### Controlled And Uncontrolled

- controlled-only in this baseline contract

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | lower and upper thumbs visible |
| focus-lower | lower thumb focused | lower-thumb focus treatment |
| focus-upper | upper thumb focused | upper-thumb focus treatment |
| active | a thumb is dragged or key-adjusted | active interaction state |
| disabled | `isDisabled=true` | muted non-interactive state |

### Component States

Lower value, upper value, and active-thumb state are all required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | either thumb changes value | `[lower, upper]` | lower <= upper invariant preserved |
| `onValueCommit` | interaction finishes | `[lower, upper]` | optional commit boundary |

## 6. Accessibility

### Semantics

- Role: two related slider controls representing a bounded range
- Required attributes: accessible group label plus per-thumb current value/min/max
- Optional attributes: per-thumb value text
- Labeling rules: each thumb must remain individually focusable and
  distinguishable to assistive technology as lower or upper bound

### Keyboard

| Key | Behavior |
|-----|----------|
| arrow keys | adjust the focused thumb by step |
| `Home/End` | move the focused thumb to min/max within allowed constraints |
| `Tab` | moves between thumbs and out of the control |

### Focus And Announcement

- focus entry: one thumb at a time is focusable
- focus exit: lower/upper values remain visible and accessible
- live-region behavior: none; per-thumb slider semantics announce value changes
- GPUI-native accessibility mapping notes: GPUI must expose lower and upper
  thumbs as distinct value controls with clear accessible naming and bound
  semantics

## 7. Layout

### Sizing

- track length is parent-owned
- thumb overlap and crossing behavior must preserve the lower<=upper invariant

### Composition

- parent expectations: filters, inspectors, range constraints
- child expectations: none
- resizing rules: orientation changes axis without changing pair semantics

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Track | background/border roles | full range |
| Selected Range Fill | accent/status roles | selected window |
| Thumbs | background/border roles | handles |
| Focus ring | accent focus roles | active thumb focus |
| Disabled | state opacity roles | disabled treatment |

## 9. Svelte Notes

- should use a headless dual-thumb range implementation rather than ad hoc drag
  math whenever possible

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::range_slider`
- GPUI implementation must intentionally expose two separately focusable value
  handles with distinct accessibility names and keyboard adjustment behavior

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] lower/upper semantics and invariant handling match
- [ ] per-thumb accessibility and focus semantics match
- [ ] keyboard adjustment behavior matches

### Tier 2: Visual Parity

- [ ] track, selected fill, thumb, and focus treatment use comparable token roles

### Tier 3: Implementation Freedom

- [ ] pointer/gesture internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| pointer overlap handling may differ slightly | gesture internals are runtime-specific | allowed | keep keyboard and value semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: filters, inspectors, bounded-range editors
- future follow-up: consider a value-label wrapper once forms/composites deepen

## Next Task

Keep multi-thumb range editing separate from future visual data or graph
overlays.
