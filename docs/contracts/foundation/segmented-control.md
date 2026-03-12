# Segmented Control

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `SegmentedControl`
- Layer: `foundation`
- Summary: a single-select control presented as adjacent visual segments
- In scope: exclusive selection, directional keyboard movement, disabled
  segments
- Out of scope: arbitrary content tabs, ternary policy semantics, multi-select

## 2. Anatomy

```text
[Root]
  └── [Segment...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | segmented host | border, radius, background |
| Segment | yes | one selectable option | text, background, selected state |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled selected value |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `options` | `Array<{ value: string; label: string; ariaLabel?: string; isDisabled?: boolean }>` | none | yes | segment list |
| `isDisabled` | `boolean` | `false` | no | disables whole control |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `onValueChange` | `(value: string) => void` | none | no | selection callback |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| unselected | default | neutral segment styling |
| selected | value matches option | selected segment styling |
| focus | current segment focused | visible focus treatment |
| disabled | group or segment disabled | muted non-interactive state |

### Component States

Roving-focus and single-selected-option state are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | selection changes | selected value | one segment selected at a time |

## 6. Accessibility

### Semantics

- Role: exclusive-choice group with radio-like semantics
- Required attributes: group accessible name and per-segment selected state
- Optional attributes: per-segment accessible names when visible labels are
  abbreviated
- Labeling rules: abbreviation or icon-only segments must still expose clear
  accessible names

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` | moves focus and selection |
| `Arrow Up/Down` | optional parity when visually vertical in future variants |
| `Space` or `Enter` | selects focused segment if not already selected |
| `Tab` | enters/exits the control through one tabbable segment |

### Focus And Announcement

- focus entry: one segment participates in the tab order
- focus exit: roving focus remains on the active or last-focused segment
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must expose radiogroup-like
  semantics, selected state, and roving focus explicitly

## 7. Layout

### Sizing

- control height follows shared control-size tokens
- segment widths may auto-fit or distribute evenly, but selection hit targets
  must stay usable

### Composition

- parent expectations: shell tool groups, view mode toggles, compact filters
- child expectations: option list only in this baseline contract
- resizing rules: the control remains visually unified regardless of segment
  count

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | border/background/radius roles | shell |
| Segment | text and background roles | base option styling |
| Selected segment | accent-soft/accent/text roles | current selection |
| Focus ring | accent focus roles | roving focus |
| Disabled | state opacity roles | disabled treatment |

## 9. Svelte Notes

- may wrap a radio-group-like headless primitive or custom control
- do not leak substrate-specific compound API into the public contract

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::segmented_control`
- GPUI implementation must intentionally preserve roving-focus and accessible
  exclusive-choice semantics rather than treating each segment as a free button

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] exclusive-choice semantics match
- [ ] roving focus and directional navigation match
- [ ] abbreviated-label accessible names match

### Tier 2: Visual Parity

- [ ] shell, selected-state, and focus treatment use comparable token roles

### Tier 3: Implementation Freedom

- [ ] segment width strategy and internal focus management stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| equal-width vs intrinsic-width segments may differ | layout mechanics are runtime-specific | allowed | keep selection semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: mode toggles, shell view switches, compact filter strips
- future follow-up: verify first implementations keep `SegmentedControl`
  semantically distinct from `Tabs`

## Next Task

Use `SegmentedControl` for exclusive inline mode switches, and reserve tabs for
larger navigational/content relationships.
