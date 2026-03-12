# Switch

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Switch`
- Layer: `foundation`
- Summary: a binary on/off control with switch semantics
- In scope: checked state, optional label, disabled/readonly states
- Out of scope: mixed-state semantics, tri-state membership logic

## 2. Anatomy

```text
[Root]
  ├── [Track]
  │     └── [Thumb]
  └── [Label] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | switch host | spacing, focus ring |
| Track | yes | visible on/off track | background, border |
| Thumb | yes | sliding control indicator | background |
| Label | no | visible text label | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `isChecked` | `boolean` | `false` | no | controlled on/off state |
| `defaultChecked` | `boolean` | `false` | no | uncontrolled initial state |
| `isDisabled` | `boolean` | `false` | no | disables interaction |
| `isReadOnly` | `boolean` | `false` | no | exposes state without allowing mutation |
| `label` | `string \| null` | `null` | no | visible label |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `onCheckedChange` | `(checked: boolean) => void` | none | no | state-change callback |

### Controlled And Uncontrolled

- controlled: `isChecked` plus `onCheckedChange`
- uncontrolled: `defaultChecked`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| off | default | thumb at off position |
| on | `isChecked=true` | thumb at on position |
| focus | focus enters | visible focus ring |
| disabled | `isDisabled=true` | muted non-interactive state |
| readOnly | `isReadOnly=true` | visible state without mutation |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onCheckedChange` | user toggles the switch | next boolean state | suppressed while disabled/readonly |

## 6. Accessibility

### Semantics

- Role: switch
- Required attributes: accessible name from label or `ariaLabel`, checked state
- Optional attributes: readonly state, description relation
- Labeling rules: visible label or programmatic label required

### Keyboard

| Key | Behavior |
|-----|----------|
| `Space` | toggles switch when interactive |
| `Enter` | optional activation parity if platform uses button-like semantics |
| `Tab` | moves focus into or past the switch |

### Focus And Announcement

- focus entry: visible ring on the switch host
- focus exit: ring clears while on/off state remains visible
- live-region behavior: none; checked state is announced through switch
  semantics
- GPUI-native accessibility mapping notes: GPUI must expose switch role,
  checked state, label, and readonly/disabled state explicitly

## 7. Layout

### Sizing

- switch track and thumb proportions follow control-size tokens
- label spacing remains stable regardless of state

### Composition

- parent expectations: settings rows, filter bars, shell toggles
- child expectations: optional visible label
- resizing rules: track/thumb remain fixed proportionally while label can flex

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Track | background and border roles | on/off shell |
| Thumb | background roles | indicator |
| Label | typography and text roles | label styling |
| Focus ring | accent focus roles | focus |
| Disabled | state opacity roles | disabled treatment |

## 9. Svelte Notes

- should prefer native checkbox/switch semantics or headless switch primitives
  that preserve accessible state

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::switch`
- GPUI implementation must preserve keyboard toggle parity and native switch
  semantics instead of exposing the control as a generic button

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] switch role and checked semantics match
- [ ] keyboard toggle behavior matches
- [ ] readonly and disabled behavior matches

### Tier 2: Visual Parity

- [ ] track/thumb proportions and focus treatment use the same token roles

### Tier 3: Implementation Freedom

- [ ] animation and thumb movement internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| thumb animation details may differ | motion internals are runtime-specific | allowed | keep on/off semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings panels, shell toggles
- future follow-up: coordinate with `TriStateSwitch` for ternary semantics

## Next Task

Use `Switch` for binary on/off semantics and reserve ternary membership or
policy semantics for `TriStateSwitch`.
