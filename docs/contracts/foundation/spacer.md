# Spacer

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Spacer`
- Layer: `foundation`
- Summary: a flex-style expansion primitive for distributing remaining space
- In scope: consuming remaining space in stack or inline layouts
- Out of scope: arbitrary margin hacks, semantic grouping, focus behavior

## 2. Anatomy

```text
[Root]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | expansion-only layout node | none by default |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `grow` | `number` | `1` | no | relative flex growth |
| `minSize` | `string \| null` | `null` | no | optional lower bound |

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | invisible expansion node |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | layout primitive only |

## 6. Accessibility

### Semantics

- Role: none
- Required attributes: none
- Optional attributes: none
- Labeling rules: `Spacer` must not expose itself to the accessibility tree as
  meaningful content

### Keyboard

| Key | Behavior |
|-----|----------|
| none | no keyboard behavior |

### Focus And Announcement

- focus entry: never focusable
- live-region behavior: none

## 7. Layout

### Sizing

- consumes available space based on `grow`
- optional `minSize` may reserve space without implying visible content

### Composition

- parent expectations: flex-like stack or inline layout
- child expectations: none
- resizing rules: grows and shrinks with siblings according to layout engine

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | none by default | structural expansion only |

## 9. Svelte Notes

- implemented as a flex child with grow/shrink rules
- should remain aria-hidden or semantically absent

## 10. GPUI Notes

- implemented as a layout spacer or weighted empty node
- must not appear as a named or focusable accessibility node

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] remains accessibility-neutral in both runtimes
- [ ] growth semantics match

### Tier 2: Visual Parity

- [ ] space distribution feels equivalent

### Tier 3: Implementation Freedom

- [ ] DOM flex item vs GPUI layout weight stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: shell headers, toolbar rows, layout groups
- future follow-up: none expected

## Next Task

Use `Spacer` only where expansion semantics are clearer than ad hoc margins or
empty wrappers.
