# Box

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Box`
- Layer: `foundation`
- Summary: the neutral layout wrapper for spacing, sizing, positioning, and
  semantic opt-in without imposing directional layout behavior
- In scope: sizing, padding, overflow, semantic role opt-in
- Out of scope: directional layout rules, scrolling ownership, interactive
  behavior

## 2. Anatomy

```text
[Root]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | neutral layout container | spacing, size, radius only when explicitly requested |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `padding` | `"none" \| "sm" \| "md" \| "lg"` | `"none"` | no | semantic interior spacing |
| `width` | `string \| null` | `null` | no | layout-only size value |
| `height` | `string \| null` | `null` | no | layout-only size value |
| `minWidth` | `string \| null` | `null` | no | optional constraint |
| `minHeight` | `string \| null` | `null` | no | optional constraint |
| `overflow` | `"visible" \| "hidden" \| "clip"` | `"visible"` | no | no scrolling semantics |
| `asRole` | `string \| null` | `null` | no | explicit semantic role opt-in |

### Naming Rules

- `Box` remains neutral and does not expose direction-specific props
- semantic opt-in is explicit rather than inferred from styling

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral container |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | layout primitive only |

## 6. Accessibility

### Semantics

- Role: none by default
- Required attributes: none by default
- Optional attributes: role and labeling attributes only when the caller opts
  into semantic usage
- Labeling rules: if `asRole` creates an addressable region, the caller must
  provide the required label relationship

### Keyboard

| Key | Behavior |
|-----|----------|
| none | no keyboard behavior owned by `Box` |

### Focus And Announcement

- focus entry: `Box` is not focusable by default
- focus exit: n/a
- live-region behavior: none

## 7. Layout

### Sizing

- min/max and fixed sizes are caller-owned
- `Box` should not introduce implicit flex or grid behavior

### Composition

- parent expectations: any layout context
- child expectations: arbitrary content
- resizing rules: follows parent and explicit constraints only

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.space.*` | optional padding |
| Root | `semantic.radius.*` | optional rounding when intentionally used as a surface-like shell |

## 9. Svelte Notes

- implemented as a thin wrapper over a semantic HTML element such as `div`
- semantic HTML should be used first when the box is more than neutral layout
- browser accessibility stays HTML-first rather than ARIA-first

## 10. GPUI Notes

- implemented as a neutral GPUI container element
- when semantic role opt-in is requested, the GPUI side must map it into the
  native accessibility tree rather than silently dropping it
- `Box` must not become focusable unless a higher-order contract requires it

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] role opt-in meaning matches
- [ ] focus neutrality matches
- [ ] overflow-without-scroll semantics match

### Tier 2: Visual Parity

- [ ] padding semantics match
- [ ] explicit size constraints match

### Tier 3: Implementation Freedom

- [ ] HTML element choice or GPUI container choice stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: all foundation and composite layers
- future follow-up: clarify polymorphic element support only if needed

## Next Task

Use `Box` as the neutral base when implementing directional layout primitives
such as `Stack`, `Inline`, and `Grid`.
