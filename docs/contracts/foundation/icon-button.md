# Icon Button

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `IconButton`
- Layer: `foundation`
- Summary: a compact action trigger whose accessible name comes from a label
  prop rather than visible text content
- In scope: icon-only command triggers, pressed/selected state when explicitly
  configured, disabled/loading behavior
- Out of scope: toolbar roving-focus behavior, menu-button or toggle-button
  composite semantics beyond explicit opt-in

## 2. Anatomy

```text
[Root]
  ├── [Icon]
  └── [Loading Indicator] (conditional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | compact action surface | background, border, radius, focus ring |
| Icon | yes | visible glyph for the action | icon color, icon size |
| Loading Indicator | no | pending replacement or overlay | icon color, motion duration |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"primary" \| "secondary" \| "ghost" \| "danger"` | `"ghost"` | no | semantic appearance |
| `size` | `"sm" \| "md" \| "lg"` | `"md"` | no | shared control size |
| `icon` | `string` | none | yes | icon registry id |
| `ariaLabel` | `string` | none | yes | required accessible name |
| `isDisabled` | `boolean` | `false` | no | suppresses activation |
| `isLoading` | `boolean` | `false` | no | suppresses activation and shows progress affordance |
| `isPressed` | `boolean \| null` | `null` | no | optional pressed/toggled state exposure |
| `onClick` | `(event) => void` | none | no | activation callback |

### Controlled And Uncontrolled

- command-only by default
- optional controlled pressed state through `isPressed`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | icon-only command surface |
| hover | pointer enters | variant-specific emphasis |
| focus | keyboard focus | visible focus ring |
| active | pointer or keyboard activation | pressed treatment |
| pressed | `isPressed=true` | selected/toggled treatment |
| disabled | `isDisabled=true` | muted and non-interactive |
| loading | `isLoading=true` | progress treatment with activation suppressed |

### Component States

State table is sufficient.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onClick` | activation completes | framework-native event | suppressed while disabled/loading |
| `onFocus` | focus enters root | framework-native event | optional passthrough |
| `onBlur` | focus leaves root | framework-native event | optional passthrough |

## 6. Accessibility

### Semantics

- Role: `button`
- Required attributes: accessible name via `ariaLabel`
- Optional attributes: `aria-pressed` when toggle state is exposed,
  `aria-describedby` for extra context
- Labeling rules: visible icon alone is never treated as sufficient naming

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | activates button |
| `Space` | activates button |
| `Tab` | moves focus into or past button |

### Focus And Announcement

- focus entry: visible focus ring appears
- focus exit: ring clears with no residual active styling
- live-region behavior: none by default
- GPUI-native accessibility mapping notes: icon-only buttons must expose role,
  accessible name, disabled state, and optional pressed state through native
  accessibility APIs

## 7. Layout

### Sizing

- square or near-square surface sized from shared control sizes
- icon remains centered regardless of loading state

### Composition

- parent expectations: toolbars, shell actions, cards, headers
- child expectations: icon only
- resizing rules: icon button does not wrap or expand to fit text

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.size.control.height` | control size baseline |
| Root | `semantic.radius.control` | shape |
| Root | `semantic.color.background.*` and `semantic.color.border.*` | variant styling |
| Icon | `semantic.icon.size.default` | icon scale |
| Icon | `semantic.color.icon.*` | icon color |
| Focus ring | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | focus treatment |
| Disabled | `semantic.state.opacity.disabled` | disabled state |

## 9. Svelte Notes

- may wrap Bits button behavior or native `<button>` behavior
- visible glyph should be `aria-hidden` when it is redundant with the accessible
  name

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::icon_button`
- GPUI implementation must not rely on tooltip text as the only accessible name
- global shortcut routing must respect focused icon buttons without swallowing
  activation keys meant for the control

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] required accessible-name rule matches
- [ ] pressed state semantics match when used
- [ ] disabled/loading suppression matches
- [ ] keyboard activation matches

### Tier 2: Visual Parity

- [ ] icon centering and control sizing match
- [ ] pressed and hover emphasis use the same token roles

### Tier 3: Implementation Freedom

- [ ] tooltip mechanics and icon rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Aura shell controls, Spark shell controls, generic toolbars
- future follow-up: split `ToggleIconButton` only if parity review shows it
  deserves its own contract

## Next Task

Treat `IconButton` as part of the button family while `g01.009` and `g01.010`
add richer pressed/selected composite controls.
