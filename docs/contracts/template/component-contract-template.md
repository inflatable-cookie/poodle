# Component Contract Template

Status: active
Updated: 2026-03-11

Use this template for every new Poodle component before implementation begins.

## 1. Purpose

- Component name:
- Layer: `foundation | composites | workstation`
- Summary:
- In scope:
- Out of scope:

## 2. Anatomy

```text
[Root]
  ├── [Part A]
  ├── [Part B]
  └── [Part C]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes/no | | |
| ... | | | |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| | | | | |

### Naming Rules

- use `camelCase` for multi-word props
- use `is*` booleans such as `isDisabled`, `isLoading`
- use `on*` handler props such as `onClick`, `onOpenChange`
- use `variant` for semantic appearance families
- use `size` only when the component opts into shared control sizes

### Controlled And Uncontrolled

Document whether the component supports:

- controlled inputs such as `value` plus `onValueChange`
- uncontrolled inputs such as `defaultValue`
- command-only mode with no persistent value

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | | |
| hover | | |
| focus | | |
| active | | |
| disabled | | |

### Component States

Use a state table when the state shape is small and linear.

Use a state machine diagram when:

- the component has more than one transient interaction mode
- async loading/open/closing behavior exists
- keyboard, pointer, and focus can drive different transitions

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| | | | |

## 6. Accessibility

### Semantics

- Role:
- Required attributes:
- Optional attributes:
- Labeling rules:

### Keyboard

| Key | Behavior |
|-----|----------|
| | |

### Focus And Announcement

- focus entry:
- focus exit:
- live-region or announcement behavior:
- GPUI-native accessibility mapping notes:

## 7. Layout

### Sizing

- minimum size:
- maximum size:
- overflow behavior:

### Composition

- parent expectations:
- child expectations:
- resizing rules:

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| | | |

Components should reference semantic roles by default. Primitive tokens should
only appear when they are the semantic source for a documented rule.

## 9. Svelte Notes

- expected substrate:
- wrapper strategy:
- implementation-only details:
- known browser-specific deltas:

## 10. GPUI Notes

- expected crate/module surface:
- theme access strategy:
- implementation-only details:
- known GPUI-native deltas:

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] semantic inputs have the same meaning
- [ ] state transitions match
- [ ] event timing and payload meaning match
- [ ] accessibility rules and keyboard behavior match
- [ ] accessible name, role, state, and value exposure match
- [ ] focus order and restoration behavior match when relevant

### Tier 2: Visual Parity

- [ ] token roles match
- [ ] spacing and sizing match within platform limits
- [ ] overall proportions and hierarchy match

### Tier 3: Implementation Freedom

- [ ] implementation-only differences are documented
- [ ] no implementation detail leaks into the public contract

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | | | |

## 13. Approval And Adoption Notes

- contract status: `draft | approved | implemented | parity-reviewed`
- approvers:
- downstream adopters:
- future follow-up:

## Next Task

Copy this template into the correct contract layer and fill it out before
starting implementation work for the component.
