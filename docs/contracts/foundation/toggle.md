# Toggle

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Toggle`
- Layer: `foundation`
- Summary: a pressable action control that exposes a persistent pressed or
  selected state, with variant and size options
- In scope: pressed state, disabled state, icon-only or text usage, ghost,
  primary, and secondary variants, three sizes, inline and stack layouts
- Out of scope: mutually exclusive grouped selection (see ToggleGroup), toolbar
  roving-focus, binary on/off switch semantics (see Switch)

## 2. Anatomy

```text
[Root .toggle]  <button aria-pressed>
  └── [Content] (slot — icon, text, or both)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | pressable toggle surface | background, border, radius, focus ring, color |
| Content | yes | slot for text, icon, or combined content | inherits color from root |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `pressed` | `boolean \| null` | `null` | no | controlled pressed state; `null` = uncontrolled |
| `defaultPressed` | `boolean` | `false` | no | uncontrolled initial pressed state |
| `variant` | `"ghost" \| "primary" \| "secondary"` | `"ghost"` | no | appearance family |
| `size` | `"sm" \| "md" \| "lg"` | `"md"` | no | control size |
| `layout` | `"inline" \| "stack"` | `"inline"` | no | inline button vs full-width stacked layout |
| `disabled` | `boolean` | `false` | no | suppresses interaction |
| `ariaLabel` | `string \| null` | `null` | no | required when content is icon-only |
| `className` | `string` | `""` | no | additional CSS classes |

### Slots

| Slot | Purpose |
|------|---------|
| default | toggle content (text, icon, or combined) |

### Controlled And Uncontrolled

- controlled: `pressed` (non-null) plus `pressedChange` event
- uncontrolled: `defaultPressed` with internal state management

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default (unpressed) | resting | variant-specific fill, border, text |
| pressed | `aria-pressed="true"` | accent background, accent border, inverse text |
| focus | keyboard focus | focus ring |
| disabled | `disabled=true` | muted, non-interactive |

### Component States

- `data-pressed`: `"true"` or `"false"` on root
- `data-variant`: variant value on root
- `data-size`: size value on root
- `data-layout`: layout value on root

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `pressedChange` | user toggles pressed state | `{ pressed: boolean }` | suppressed while disabled |

## 6. Accessibility

### Semantics

- Element: native `<button>`
- `aria-pressed`: `"true"` when pressed, `"false"` when not
- `aria-label`: from prop (required for icon-only content)
- `disabled`: set when `disabled=true`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | toggles pressed state |
| `Space` | toggles pressed state |
| `Tab` | moves focus to next focusable element |
| `Shift+Tab` | moves focus to previous focusable element |

### Focus And Announcement

- focus entry: root receives visible focus ring
- focus exit: focus ring clears immediately
- live-region behavior: none; pressed state announced through `aria-pressed`
- GPUI-native accessibility mapping notes: GPUI must expose button role with
  pressed state through the native accessibility tree

## 7. Layout

### Sizing

- Default (inline): `height: control-height`, `padding: 0 space-control-x`
- Small: `height: control-height - 0.375rem`, `padding: 0 (space-control-x - 0.125rem)`
- Large: `height: control-height + 0.375rem`, `padding: 0 (space-control-x + 0.125rem)`
- Stack layout: full-width, auto-height, no explicit padding

### Composition

- parent expectations: toolbars, shell actions, formatting controls, filter bars
- child expectations: text, icon, or combined content via slot
- resizing: inline auto-fits content; stack stretches to parent width

## 8. Token Usage — Exact Values

### Root `.toggle` (base — ghost variant, md size, inline layout)

| Property | Value |
|----------|-------|
| `--poodle-toggle-fill` | `color-mix(in srgb, var(--poodle-color-background-surface) 86%, transparent)` |
| `--poodle-toggle-border` | `color-mix(in srgb, var(--poodle-color-border-subtle) 78%, transparent)` |
| `--poodle-toggle-text` | `var(--poodle-color-text-primary)` |
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-width` | `2.25rem` |
| `height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-toggle-border)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-toggle-fill)` |
| `color` | `var(--poodle-toggle-text)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1` |

### Size adjustments

| Size | Property | Value |
|------|----------|-------|
| `sm` (`[data-size="sm"]`) | `height` | `calc(var(--poodle-size-control-height) - 0.375rem)` |
| `sm` | `padding` | `0 calc(var(--poodle-space-control-x) - 0.125rem)` |
| `lg` (`[data-size="lg"]`) | `height` | `calc(var(--poodle-size-control-height) + 0.375rem)` |
| `lg` | `padding` | `0 calc(var(--poodle-space-control-x) + 0.125rem)` |

### Stack layout (`[data-layout="stack"]`)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `width` | `100%` |
| `min-width` | `0` |
| `height` | `auto` |
| `padding` | `0` |
| `justify-content` | `stretch` |
| `justify-items` | `stretch` |
| `align-content` | `start` |
| `text-align` | `left` |
| `line-height` | `1.3` |

### CSS Custom Properties (variant system)

| Var | Ghost (default) | Primary | Secondary |
|-----|-----------------|---------|-----------|
| `--poodle-toggle-fill` | `color-mix(in srgb, background-surface 86%, transparent)` | `color-mix(in srgb, accent-base 18%, background-surface)` | `background-surface` |
| `--poodle-toggle-border` | `color-mix(in srgb, border-subtle 78%, transparent)` | `color-mix(in srgb, accent-base 38%, border-default)` | `border-default` |
| `--poodle-toggle-text` | `text-primary` | `text-primary` | `text-primary` |

### Pressed state (`[data-pressed="true"]`)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-accent-base)` |
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 78%, black)` |
| `color` | `var(--poodle-color-text-inverse)` |

### Focus visible (`:focus-visible`)

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Disabled (`:disabled`)

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

## 9. Svelte Notes

- Uses native `<button>` with `aria-pressed` attribute
- CSS custom properties (`--poodle-toggle-fill`, `--poodle-toggle-border`,
  `--poodle-toggle-text`) drive the variant system
- Data attributes: `data-pressed`, `data-variant`, `data-size`, `data-layout`
- Pressed state override uses direct property values, not custom property
  reassignment
- Internal state managed when `isPressed` is `null` (uncontrolled mode)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::toggle`
- GPUI must expose button role with `aria-pressed` semantics
- The variant system maps to three fill/border/text color sets
- Stack layout requires GPUI grid or flex-column equivalent
- Pressed state accent coloring must override all variant fills

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] pressed semantics match (`aria-pressed`)
- [ ] keyboard activation matches (Enter, Space)
- [ ] icon-only accessible name rule matches
- [ ] disabled behavior matches
- [ ] pressedChange event payload matches
- [ ] controlled and uncontrolled modes match

### Tier 2: Visual Parity

- [ ] all three variant color schemes match
- [ ] all three sizes match (height, padding)
- [ ] pressed state accent background and inverse text match
- [ ] pressed state border-color mix matches (78% with black)
- [ ] focus ring appearance matches
- [ ] disabled opacity matches
- [ ] stack layout behavior matches
- [ ] label typography matches (family, size, weight)

### Tier 3: Implementation Freedom

- [ ] CSS custom property system vs Rust styling is internal
- [ ] transition timing is platform-owned
- [ ] data attribute naming is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| CSS custom property system vs Rust styling | different styling mechanisms per platform | allowed | same visual result required |
| Stack layout grid details | GPUI grid may differ from CSS grid | allowed | width: 100% and start-aligned content must match |
| Transition timing | GPUI may not support CSS-style transitions | allowed | match where possible |

## 13. Specimen Definitions

### Ghost Variant (default)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Bold | `variant="ghost"` (default), `ariaLabel="Bold"`, icon-only (bold icon) | Ghost-styled toggle button with icon, unpressed state |
| Italic | `variant="ghost"` (default), `ariaLabel="Italic"`, icon-only (italic icon) | Ghost-styled toggle button with icon, unpressed state |

### Primary Variant

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Pinned | `variant="primary"`, `defaultPressed=true`, `ariaLabel="Pinned"`, pin icon + text | Primary-styled toggle in pressed state with accent background, inverse text |

### Secondary Variant

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Favorite | `variant="secondary"`, `ariaLabel="Favorite"`, star icon + text | Secondary-styled toggle in unpressed state |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Locked | `defaultPressed=true`, `isDisabled=true`, `ariaLabel="Disabled toggle"`, text-only | Ghost-styled toggle in pressed+disabled state, reduced opacity, not-allowed cursor |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: mode toggles, formatting toggles, compact shell actions,
  filter controls
- future follow-up: coordinate with ToggleGroup for grouped toggle patterns
