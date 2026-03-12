# Button

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Button`
- Layer: `foundation`
- Summary: a general action trigger for commands, confirmations, and view-level
  affordances
- In scope: text buttons, icon-leading buttons, icon-only buttons, loading and
  disabled states
- Out of scope: transport controls, timeline tools, and DAW-specific command
  widgets

## 2. Anatomy

```text
[Root]
  ├── [Leading Icon] (optional)
  ├── [Label or Child Content]
  ├── [Trailing Icon] (optional)
  └── [Loading Indicator] (conditional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | clickable command surface | background, border, radius, focus ring |
| Leading Icon | no | icon before label | icon color, icon size, inline gap |
| Label | no | text label for visible action | text color, typography |
| Trailing Icon | no | icon after label | icon color, icon size, inline gap |
| Loading Indicator | no | replaces or accompanies content while pending | icon color, motion duration |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"primary" \| "secondary" \| "ghost" \| "danger"` | `"secondary"` | no | semantic appearance only |
| `size` | `"sm" \| "md" \| "lg"` | `"md"` | no | maps to shared control-size overlay |
| `isDisabled` | `boolean` | `false` | no | suppresses activation and hover/press affordances |
| `isLoading` | `boolean` | `false` | no | presents pending state while keeping layout stable |
| `leadingIcon` | `string \| null` | `null` | no | icon registry identifier |
| `trailingIcon` | `string \| null` | `null` | no | icon registry identifier |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label exists |
| `onClick` | `(event) => void` | none | no | activation callback |

### Naming Rules

- use `isDisabled` and `isLoading`, not raw HTML-attribute naming, as the
  canonical contract
- use `onClick` as the semantic action hook even if platform adapters translate
  to `onPress`-style internals

### Controlled And Uncontrolled

- no persistent value model
- command-only component

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | base token styling |
| hover | pointer enters | variant-specific hover background or border emphasis |
| focus | keyboard focus | visible focus ring without layout shift |
| active | press/activation in progress | pressed visual state with stable content |
| disabled | `isDisabled=true` | muted opacity and no activation |
| loading | `isLoading=true` | command suppressed and loading indicator visible |

### Component States

The button family uses a state table instead of a state machine diagram because
the interaction model is shallow and command-oriented.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onClick` | activation is completed by pointer or keyboard | framework-native event | suppressed while disabled or loading |
| `onFocus` | focus enters root | framework-native event | optional implementation passthrough |
| `onBlur` | focus leaves root | framework-native event | optional implementation passthrough |

## 6. Accessibility

### Semantics

- Role: `button`
- Required attributes: accessible name via visible label or `ariaLabel`
- Optional attributes: `aria-describedby`, `aria-pressed` when a toggle flavor
  is intentionally documented as a separate variant family
- Labeling rules: icon-only buttons must provide `ariaLabel`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | activates button |
| `Space` | activates button |
| `Tab` | moves focus to next focusable element |
| `Shift+Tab` | moves focus to previous focusable element |

### Focus And Announcement

- focus entry: root receives visible focus indicator for keyboard navigation
- focus exit: focus ring clears with no residual active styling
- live-region behavior: none by default; loading text is app-owned if
  announcement is required
- GPUI-native accessibility mapping notes: button role, disabled/busy state,
  and accessible name must be exposed through native accessibility APIs rather
  than implied only by visible rendering

## 7. Layout

### Sizing

- minimum size: shared control height from `semantic.size.control.height`
- minimum width: `semantic.size.control.minWidth` for labeled variants
- overflow behavior: label truncation is app-owned; button should not wrap by
  default

### Composition

- parent expectations: may live inside toolbars, panels, dialogs, menus, or
  form actions
- child expectations: label plus optional icons
- resizing rules: width may auto-fit content or stretch when parent opts in

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `semantic.color.background.*` | variant fill |
| Root | `semantic.color.border.*` | outline variants and separators |
| Root | `semantic.radius.control` | shape |
| Root | `semantic.size.control.height` | control height |
| Root | `semantic.space.control.x` and `semantic.space.control.y` | internal padding |
| Label | `semantic.typography.label.*` | text styling |
| Label | `semantic.color.text.*` | readable action text |
| Icon | `semantic.icon.size.default` | default icon scale |
| Icon | `semantic.color.icon.*` | icon color |
| Focus ring | `semantic.color.accent.focusRing` and `semantic.border.width.focus` | visible keyboard focus |
| Disabled state | `semantic.state.opacity.disabled` | suppression treatment |

## 9. Svelte Notes

- expected substrate: Bits `Button` primitive or native `<button>` when the
  semantic surface is already sufficient
- wrapper strategy: Pug owns `variant`, `size`, token classes, and data-attrs
- implementation-only details: `class` and `data-*` driven styling remain
  internal; app code should not import Bits
- known browser-specific deltas: `:focus-visible` may provide the focus-ring
  gate instead of a custom input-modality tracker

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::button`
- theme access strategy: button render path reads semantic roles from the
  generated theme helpers exposed through the GPUI token crate
- implementation-only details: GPUI-native pressed and hover handling may use
  entity state rather than DOM state attributes
- known GPUI-native deltas: focus-ring rendering can be GPUI-native but must
  preserve semantic visibility and token color; button semantics, disabled
  state, and accessible naming are not optional native deltas

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `variant`, `size`, `isDisabled`, and `isLoading` mean the same thing
- [ ] activation is suppressed while disabled or loading
- [ ] keyboard activation matches
- [ ] icon-only accessible-name rule matches

### Tier 2: Visual Parity

- [ ] variant token roles match
- [ ] shared control sizes match
- [ ] icon and label spacing stays proportionally aligned

### Tier 3: Implementation Freedom

- [ ] Bits or native button internals stay implementation-only
- [ ] GPUI event/state internals stay implementation-only

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending first implementation | review during parity proof |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Pug foundation, Aura shell actions, Spark shell actions
- future follow-up: split toggle-button semantics into a distinct contract if
  that family becomes parity-critical

## Next Task

Use this button contract as the pattern for the rest of `g01.008` action and
text-entry primitives.
