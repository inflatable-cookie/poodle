# Slider

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Slider`
- Layer: `foundation`
- Summary: a single-value continuous or stepped range control
- In scope: current value, min/max bounds, step behavior, keyboard and pointer
  adjustment
- Out of scope: dual-thumb range editing, knob/fader semantics

## 2. Anatomy

```text
[Root]
  ├── [Track]
  ├── [Range Fill]
  └── [Thumb]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | slider host | spacing, focus context |
| Track | yes | full value range | background, border |
| Range Fill | yes | completed value span | accent/status color |
| Thumb | yes | draggable current-value handle | background, border, focus ring |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `number` | none | yes | controlled value |
| `min` | `number` | `0` | no | lower bound |
| `max` | `number` | `100` | no | upper bound |
| `step` | `number` | `1` | no | increment size |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | axis |
| `isDisabled` | `boolean` | `false` | no | disables interaction |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `valueText` | `string \| null` | `null` | no | human-readable value text |
| `onValueChange` | `(value: number) => void` | none | no | live value change |
| `onValueCommit` | `(value: number) => void` | none | no | drag or key interaction commit |

### Controlled And Uncontrolled

- controlled-only in this baseline contract

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral slider |
| focus | thumb focused | visible focus treatment |
| active | thumb dragged or key-adjusted | active interaction state |
| disabled | `isDisabled=true` | muted non-interactive state |

### Component States

Current value state and active drag/keyboard-adjustment state are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | value changes during interaction | current number | live updates |
| `onValueCommit` | interaction finishes | final number | optional commit boundary |

## 6. Accessibility

### Semantics

- Role: slider
- Required attributes: accessible name, current value, min, max
- Optional attributes: human-readable value text, orientation
- Labeling rules: visible labels or `ariaLabel` required

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Down` | decrements value by step |
| `Arrow Right/Up` | increments value by step |
| `Home` | moves to minimum |
| `End` | moves to maximum |
| `Page Up/Down` | optional larger-step movement |
| `Tab` | moves focus to or from thumb |

### Focus And Announcement

- focus entry: thumb is focusable and visibly focused
- focus exit: ring clears while current value remains visible
- live-region behavior: none; value changes are announced through slider
  semantics and optional value text
- GPUI-native accessibility mapping notes: GPUI must expose slider role,
  current value/min/max, value text, and keyboard adjustment semantics

## 7. Layout

### Sizing

- track thickness and thumb size remain usable across orientations
- slider length is parent-owned

### Composition

- parent expectations: forms, inspectors, settings, value controls
- child expectations: none
- resizing rules: orientation changes axis without changing value semantics

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Track | background/border roles | full range |
| Range Fill | accent/status roles | completed range |
| Thumb | background/border roles | handle |
| Focus ring | accent focus roles | thumb focus |
| Disabled | state opacity roles | disabled treatment |

## 9. Svelte Notes

- should prefer headless slider primitives or native range semantics only if
  they preserve the documented keyboard/value behavior

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::slider`
- GPUI implementation must intentionally expose thumb focus, keyboard
  adjustments, and current value semantics through native accessibility APIs

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] value/min/max/step semantics match
- [ ] keyboard adjustment semantics match
- [ ] slider accessibility exposure matches
- [ ] commit semantics match when implemented

### Tier 2: Visual Parity

- [ ] track, fill, thumb, and focus treatment use comparable token roles

### Tier 3: Implementation Freedom

- [ ] pointer/gesture internals and animation details stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| pointer drag feel may differ slightly | input-engine internals are runtime-specific | allowed | keep keyboard and value semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings, inspectors, value controls
- future follow-up: coordinate with `RangeSlider`

## Next Task

Use `Slider` for single-value range adjustment and reserve multi-thumb behavior
for `RangeSlider`.
