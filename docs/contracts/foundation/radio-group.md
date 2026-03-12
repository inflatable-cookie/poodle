# Radio Group

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `RadioGroup`
- Layer: `foundation`
- Summary: a mutually exclusive selection control composed of radio options
- In scope: selected value, option labeling, roving focus, disabled items
- Out of scope: segmented visual presentation, multi-select behavior

## 2. Anatomy

```text
[Group Root]
  └── [Option...]
        ├── [Indicator]
        └── [Label]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Group Root | yes | radio-group host | spacing, focus context |
| Option | yes | single selectable choice | border, background, spacing |
| Indicator | yes | selected/unselected marker | icon/border tokens |
| Label | yes | visible option text | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled selected value |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial value |
| `options` | `Array<{ value: string; label: string; isDisabled?: boolean }>` | none | yes | option list |
| `orientation` | `"vertical" \| "horizontal"` | `"vertical"` | no | navigation axis |
| `isDisabled` | `boolean` | `false` | no | disables whole group |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible group label exists |
| `descriptionId` | `string \| null` | `null` | no | optional descriptive relation |
| `onValueChange` | `(value: string) => void` | none | no | selection callback |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| unselected | default | empty indicator |
| selected | option value equals selected value | selected marker visible |
| focus | option receives roving focus | visible focus ring |
| disabled | group or option disabled | muted non-interactive state |

### Component States

Roving-focus state and selected-value state are both required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | user changes selected option | selected value | only one option selected at a time |

## 6. Accessibility

### Semantics

- Role: radio group with radio options
- Required attributes: accessible name for the group, selected state for each
  option
- Optional attributes: orientation and description relationships
- Labeling rules: group label and option labels must both be discoverable

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters or exits group through one tabbable option |
| `Arrow Up/Down` | moves selection/focus in vertical groups |
| `Arrow Left/Right` | moves selection/focus in horizontal groups |
| `Space` | selects focused option if not already selected |
| `Home/End` | optional first/last option movement |

### Focus And Announcement

- focus entry: one option is in the tab sequence at a time
- focus exit: roving focus is preserved for re-entry
- live-region behavior: none; selection change must be announced via radio
  semantics
- GPUI-native accessibility mapping notes: GPUI must implement radiogroup
  semantics, per-option selection state, and roving-focus behavior explicitly

## 7. Layout

### Sizing

- group size follows option content and chosen orientation
- option spacing stays consistent regardless of selected state

### Composition

- parent expectations: forms, settings, filter groups
- child expectations: options are group-owned, not arbitrary content in this
  baseline contract
- resizing rules: orientation drives gap axis and navigation axis together

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Group Root | `semantic.space.stack.*` or `semantic.space.inline.*` | option spacing |
| Option | `semantic.color.text.*` and `semantic.color.border.*` | label/indicator styling |
| Indicator | icon and accent/status roles | selected marker |
| Focus ring | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | roving focus treatment |
| Disabled | `semantic.state.opacity.disabled` | disabled state |

## 9. Svelte Notes

- should prefer native radio-group semantics or a headless wrapper that
  preserves roving-focus and selection behavior

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::radio_group`
- GPUI implementation must explicitly maintain one tabbable option, directional
  selection movement, and native radio-group accessibility semantics

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] single-selection semantics match
- [ ] roving-focus behavior matches
- [ ] orientation-based keyboard navigation matches
- [ ] group and option accessibility semantics match

### Tier 2: Visual Parity

- [ ] indicator, spacing, and focus treatment use the same token roles

### Tier 3: Implementation Freedom

- [ ] DOM radio inputs vs GPUI option entities stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings, mutually exclusive control groups
- future follow-up: coordinate segmented presentation with `SegmentedControl`

## Next Task

Use `RadioGroup` when exclusivity matters and `SegmentedControl` when the same
behavior needs a stronger shell-style visual presentation.
