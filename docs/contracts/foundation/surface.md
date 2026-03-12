# Surface

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Surface`
- Layer: `foundation`
- Summary: a semantic visual container for backgrounds, borders, elevation, and
  optional region semantics
- In scope: background tiers, border presence, elevation, optional labeling
- Out of scope: panel-header chrome, dock behavior, scroll ownership

## 2. Anatomy

```text
[Root Surface]
  └── [Children...]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Surface | yes | background and boundary container | background, border, radius, elevation |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `"canvas" \| "panel" \| "elevated"` | `"panel"` | no | semantic surface tier |
| `border` | `"none" \| "subtle" \| "default"` | `"subtle"` | no | shell boundary level |
| `padding` | `"none" \| "sm" \| "md" \| "lg"` | `"md"` | no | interior spacing |
| `isElevated` | `boolean` | `false` | no | stronger elevation treatment |
| `asRole` | `"region" \| "group" \| null` | `null` | no | semantic opt-in only |
| `label` | `string \| null` | `null` | no | required when the surface is an addressable region without visible title |

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | panel-tier shell |
| canvas | `tone="canvas"` | background matches canvas |
| elevated | `tone="elevated"` or `isElevated=true` | stronger elevation and contrast |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | container only |

## 6. Accessibility

### Semantics

- Role: none by default; `group` or `region` only by explicit opt-in
- Required attributes: accessible label when `asRole="region"` and no visible
  heading is associated externally
- Optional attributes: `aria-labelledby`, `aria-describedby` or native
  equivalents
- Labeling rules: decorative surfaces must stay accessibility-neutral

### Keyboard

| Key | Behavior |
|-----|----------|
| none | no intrinsic keyboard behavior |

### Focus And Announcement

- focus entry: surface itself is not focusable by default
- live-region behavior: none

## 7. Layout

### Sizing

- follows parent constraints and child content
- does not own scroll behavior unless paired with `ScrollShell`

### Composition

- parent expectations: any layout primitive or shell region
- child expectations: arbitrary content or nested primitives
- resizing rules: visual chrome scales with size but does not change semantics

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Surface | `semantic.color.background.canvas/panel/elevated` | fill tier |
| Root Surface | `semantic.color.border.*` | shell boundary |
| Root Surface | `semantic.radius.surface` | rounding |
| Root Surface | `semantic.elevation.surface` and `semantic.elevation.overlay` | elevation |
| Root Surface | `semantic.space.panel.*` | interior spacing |

## 9. Svelte Notes

- implemented as semantic HTML container plus token styling
- if the surface is a true region or section, prefer matching HTML semantics
  first and use ARIA only when needed

## 10. GPUI Notes

- implemented as a styled GPUI container plus native accessibility-node mapping
  when semantic opt-in is requested
- region/group semantics must map into platform accessibility APIs rather than
  being lost because GPUI lacks HTML semantics

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] tone and border semantics match
- [ ] region/group opt-in meaning matches
- [ ] decorative surfaces remain accessibility-neutral

### Tier 2: Visual Parity

- [ ] background tiers and elevation hierarchy match
- [ ] spacing and boundary weight match

### Tier 3: Implementation Freedom

- [ ] CSS shadows vs GPUI drawing details stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: cards, panels, dialogs, shell sections
- future follow-up: add high-contrast appearance guidance during accessibility
  hardening

## Next Task

Build `PanelSurface` and future card-like composites on top of `Surface`
instead of re-documenting basic container semantics.
