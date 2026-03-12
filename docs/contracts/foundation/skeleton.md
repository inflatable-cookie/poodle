# Skeleton

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Skeleton`
- Layer: `foundation`
- Summary: a non-interactive loading placeholder matching expected content
  shape
- In scope: loading placeholder shape, optional shimmer or pulse treatment
- Out of scope: progress semantics, real content fallback logic

## 2. Anatomy

```text
[Root Placeholder]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Placeholder | yes | placeholder block | background, radius, motion |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `shape` | `"line" \| "block" \| "circle"` | `"line"` | no | placeholder form |
| `width` | `string \| null` | `null` | no | optional layout hint |
| `height` | `string \| null` | `null` | no | optional layout hint |
| `isAnimated` | `boolean` | `true` | no | shimmer/pulse treatment |

### Controlled And Uncontrolled

- controlled display primitive only

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| static | `isAnimated=false` | fixed placeholder |
| animated | `isAnimated=true` | loading motion treatment |

### Component States

No internal state beyond animated/static display.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | display primitive only |

## 6. Accessibility

### Semantics

- Role: none by default
- Required attributes: none
- Labeling rules: skeleton placeholders are decorative loading scaffolds and
  should not be exposed as real content

### Keyboard

| Key | Behavior |
|-----|----------|
| none | not interactive |

### Focus And Announcement

- focus entry: never focusable
- live-region behavior: parent-owned loading state should announce real loading
  context when needed
- GPUI-native accessibility mapping notes: GPUI must keep skeleton placeholders
  out of the accessible tree as decorative scaffolds

## 7. Layout

### Sizing

- placeholder dimensions are caller-owned
- shape and radius determine visual silhouette only

### Composition

- parent expectations: loading states for cards, lists, forms, shells
- child expectations: none
- resizing rules: placeholder follows given width/height constraints

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Placeholder | background roles | placeholder fill |
| Root Placeholder | radius roles | shape |
| Motion | motion roles | shimmer/pulse |

## 9. Svelte Notes

- should remain accessibility-neutral and not receive focus

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::skeleton`
- GPUI implementation must keep placeholders decorative and out of the
  accessible tree

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] decorative/non-semantic loading behavior matches
- [ ] skeletons remain unfocusable and unannounced by default

### Tier 2: Visual Parity

- [ ] shape and motion treatment use comparable token roles

### Tier 3: Implementation Freedom

- [ ] shimmer or pulse implementation details stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| loading animation details may differ | motion internals are runtime-specific | allowed | keep decorative semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: loading cards, lists, forms
- future follow-up: none expected

## Next Task

Use `Skeleton` only for decorative loading placeholders and pair it with real
status messaging when users need announced progress.
