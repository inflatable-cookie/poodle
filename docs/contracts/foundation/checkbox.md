# Checkbox

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Checkbox`
- Layer: `foundation`
- Summary: a binary or mixed-state selection control for independent options
- In scope: checked, unchecked, mixed state, label association, disabled and
  readonly semantics
- Out of scope: mutually exclusive group selection, range selection logic

## 2. Anatomy

```text
[Root]
  ├── [Indicator Box]
  │     └── [Mark] (conditional)
  └── [Label] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | selection control host | spacing, focus ring |
| Indicator Box | yes | visible checkbox boundary | border, background, radius |
| Mark | no | check or mixed glyph | icon color |
| Label | no | visible option label | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `isChecked` | `boolean` | `false` | no | controlled checked state |
| `defaultChecked` | `boolean` | `false` | no | uncontrolled initial state |
| `isMixed` | `boolean` | `false` | no | tri-state visual/assistive state |
| `isDisabled` | `boolean` | `false` | no | suppresses interaction |
| `isReadOnly` | `boolean` | `false` | no | exposes state without allowing mutation |
| `label` | `string \| null` | `null` | no | visible label |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `descriptionId` | `string \| null` | `null` | no | optional descriptive relation |
| `onCheckedChange` | `(checked: boolean) => void` | none | no | change callback |

### Controlled And Uncontrolled

- controlled: `isChecked` plus `onCheckedChange`
- uncontrolled: `defaultChecked`
- `isMixed` is controlled and visual/assistive only until the next user toggle
  resolves the state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| unchecked | default | empty indicator |
| checked | `isChecked=true` | check mark visible |
| mixed | `isMixed=true` | mixed glyph visible |
| focus | focus enters control | visible focus ring |
| disabled | `isDisabled=true` | muted non-interactive state |
| readOnly | `isReadOnly=true` | visible state without mutation |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onCheckedChange` | user toggles control | next checked state | mixed state resolves according to contract policy |

## 6. Accessibility

### Semantics

- Role: native checkbox or equivalent checkbox semantics
- Required attributes: accessible name from label or `ariaLabel`
- Optional attributes: description relation, readonly state
- Labeling rules: visible label or programmatic label required

### Keyboard

| Key | Behavior |
|-----|----------|
| `Space` | toggles checked state when interactive |
| `Enter` | optional activation parity if platform exposes button-like behavior |
| `Tab` | moves focus into or past the control |

### Focus And Announcement

- focus entry: visible focus ring on the checkbox host
- focus exit: ring clears while state remains visible
- live-region behavior: none; checked and mixed state changes must be announced
  through control semantics
- GPUI-native accessibility mapping notes: GPUI must expose checkbox role,
  accessible name, checked/mixed state, and readonly/disabled state through the
  native accessibility tree

## 7. Layout

### Sizing

- indicator box remains readable at shared control sizing
- label may wrap or truncate according to parent layout policy

### Composition

- parent expectations: forms, settings, filters, tables, selection rows
- child expectations: optional visible label
- resizing rules: label spacing stays stable regardless of state glyph

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Indicator Box | `semantic.color.border.*` and `semantic.color.background.*` | boundary/fill |
| Indicator Box | `semantic.radius.control` | shape |
| Mark | `semantic.color.icon.*` | glyph color |
| Label | `semantic.typography.label.*` and `semantic.color.text.*` | text styling |
| Focus ring | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | focus |
| Disabled | `semantic.state.opacity.disabled` | disabled treatment |

## 9. Svelte Notes

- should prefer native checkbox semantics or a Bits-backed checkbox wrapper
- mixed state must be reflected in both DOM state and assistive state

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::checkbox`
- GPUI implementation must intentionally expose mixed state and maintain
  keyboard toggle parity without requiring pointer interaction

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] checked and mixed semantics match
- [ ] accessible name and checkbox state exposure match
- [ ] keyboard toggling matches
- [ ] readonly and disabled behavior matches

### Tier 2: Visual Parity

- [ ] indicator, glyph, and label spacing use the same token roles

### Tier 3: Implementation Freedom

- [ ] native input internals vs GPUI control internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings, filters, selection lists
- future follow-up: coordinate mixed-state parent/child patterns with composite
  tree or table contracts later

## Next Task

Use `Checkbox` for independent boolean selection and reserve grouped
single-choice semantics for `RadioGroup`.
