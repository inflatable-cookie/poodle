# CollapseToggle

Status: seed contract
Updated: 2026-03-17

## 1. Purpose

- Component name: `CollapseToggle`
- Layer: `primitive`
- Summary: a standalone collapse/expand toggle button with directional chevron
  icon
- In scope: toggle interaction, collapsed/expanded visual state, directional
  icon, accessible labeling
- Out of scope: what gets collapsed (determined by host), collapse animation,
  layout reflow policy

## 2. Anatomy

```text
[CollapseToggle]
  └── [Toggle Button]
        └── [Direction Icon]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Toggle Button | yes | interactive collapse/expand trigger | icon button roles |
| Direction Icon | yes | chevron or arrow indicating collapse direction | icon roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `isCollapsed` | `boolean` | `false` | no | current state |
| `direction` | `"left" \| "right" \| "up" \| "down"` | `"left"` | no | collapse direction |
| `isDisabled` | `boolean` | `false` | no | suppresses interaction |
| `ariaLabel` | `string \| null` | `null` | no | accessible label (defaults to "Collapse"/"Expand") |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| expanded | `isCollapsed=false` | icon points toward collapse direction |
| collapsed | `isCollapsed=true` | icon points toward expand direction |
| disabled | `isDisabled=true` | muted, non-interactive |

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `toggle` | `{ isCollapsed: boolean }` | user clicks or presses Enter/Space |

## 6. Accessibility

### Semantics

- Role: `button` with `aria-expanded`
- Label: descriptive ("Collapse left dock" / "Expand left dock")

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | toggle collapse state |

## 7. Layout

### Sizing

- compact button sized to icon
- positioned by host at region boundary

### Composition

- parent expectations: any collapsible region boundary, divider, panel header, or sidebar
- child expectations: none

## 8. Token Usage — Exact Values

### Toggle Button

| Property | Value |
|----------|-------|
| `padding` | `0.125rem` |
| `border-radius` | `var(--poodle-radius-sm, 0.25rem)` |
| `color` | `var(--poodle-color-text-muted)` |

### Toggle Button — hover

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-surface-hover)` |
| `color` | `var(--poodle-color-text-default)` |

### Toggle Button — disabled

| Property | Value |
|----------|-------|
| `opacity` | `0.4` |

### Toggle Button — focus-visible

| Property | Value |
|----------|-------|
| `outline-offset` | `0.0625rem` |

## 9. Svelte Notes

- thin wrapper around IconButton with collapse-specific defaults
- icon rotates based on direction and collapsed state

## 10. GPUI Notes

- equivalent thin wrapper in GPUI
- icon rotation via transform

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] toggle semantics match
- [ ] ARIA expanded state matches

### Tier 2: Visual Parity

- [ ] icon direction and rotation match

### Tier 3: Implementation Freedom

- [ ] icon component internals stay renderer-specific

## 12. Specimen Definitions

### Group: Directions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Left | `direction="left"`, `isCollapsed=false` (toggleable) | Chevron pointing left when expanded; toggles on click; label shows "(collapsed)" or "(expanded)" |
| Right | `direction="right"`, `isCollapsed=false` (toggleable) | Chevron pointing right when expanded; toggles on click; label shows state |
| Up | `direction="up"`, `isCollapsed=false` (toggleable) | Chevron pointing up when expanded; toggles on click; label shows state |
| Down | `direction="down"`, `isCollapsed=false` (toggleable) | Chevron pointing down when expanded; toggles on click; label shows state |

### Group: Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled left | `direction="left"`, `isDisabled=true` | Muted toggle, non-interactive, left-pointing chevron |
| Disabled right | `direction="right"`, `isDisabled=true` | Muted toggle, non-interactive, right-pointing chevron |

## 13. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none anticipated | — | — | — |

## Next Task

None — implemented as `CollapseToggle` primitive.
