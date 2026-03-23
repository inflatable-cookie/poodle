# Button

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Button`
- Layer: `foundation`
- Summary: a general action trigger for commands, confirmations, and view-level
  affordances
- In scope: text buttons, icon-leading buttons, icon-only buttons, loading and
  disabled states, four visual variants, three sizes
- Out of scope: transport controls, DAW-specific command widgets, toggle buttons
  (see Toggle)

## 2. Anatomy

```text
[Root .button]  <button>
  ├── [Spinner .button__spinner] (conditional, when isLoading)
  ├── [Leading Icon .button__icon] (optional, via slot or leadingIcon prop)
  ├── [Label .button__label] (optional, via default slot)
  ├── [Trailing Icon .button__icon] (optional, via slot or trailingIcon prop)
  └── [Chevron .button__chevron] (optional, when chevron=true)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | clickable command surface | background, border, radius, shadow, focus ring |
| Spinner | no | loading indicator (replaces interaction) | currentColor based |
| Leading Icon | no | icon before label | icon size, icon color |
| Label | no | text content | typography, text color |
| Trailing Icon | no | icon after label | icon size, icon color |
| Chevron | no | disclosure indicator after content | opacity, margin |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"primary" \| "secondary" \| "ghost"` | `"secondary"` | no | appearance family |
| `tone` | `"default" \| "danger"` | `"default"` | no | intent modifier; composes with variant for danger×primary, danger×secondary, danger×ghost |
| `size` | `"sm" \| "md" \| "lg"` | `"md"` | no | control size |
| `type` | `"button" \| "submit" \| "reset"` | `"button"` | no | HTML button type |
| `isDisabled` | `boolean` | `false` | no | suppresses activation |
| `isLoading` | `boolean` | `false` | no | shows spinner, suppresses activation |
| `leadingIcon` | `string \| null` | `null` | no | icon registry identifier |
| `trailingIcon` | `string \| null` | `null` | no | icon registry identifier |
| `chevron` | `boolean` | `false` | no | renders trailing disclosure chevron indicator |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `className` | `string` | `""` | no | additional CSS classes |

### Slots

| Slot | Purpose |
|------|---------|
| default | label text content; absence triggers icon-only mode |
| leading | custom leading icon content (overrides leadingIcon prop) |
| trailing | custom trailing icon content (overrides trailingIcon prop) |

### Controlled And Uncontrolled

- command-only component, no persistent value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | variant-specific fill, border, text, shadow |
| hover | pointer enters (not disabled) | `background: button-fill-hover`, `border-color` mix with text-primary 78% |
| active | press (not disabled) | `background: button-fill-active`, `transform: translateY(0.03125rem)` |
| focus | keyboard focus | `outline: border-width-focus solid accent-focusRing`, `outline-offset: 0.125rem` |
| disabled | `isDisabled=true` or `isLoading=true` | `opacity: state-opacity-disabled`, `cursor: not-allowed` |
| loading | `isLoading=true` | spinner visible, button disabled |
| icon-only | no default slot content | square button, no min-width |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `click` | activation completed | `MouseEvent` | suppressed while disabled or loading |
| `focus` | focus enters root | `FocusEvent` | passthrough |
| `blur` | focus leaves root | `FocusEvent` | passthrough |

## 6. Accessibility

### Semantics

- Role: native `<button>` element
- `aria-label`: from prop (required for icon-only)
- `aria-describedby`: from describedBy prop
- `aria-busy`: `"true"` when isLoading
- `disabled`: set when isDisabled or isLoading (`isUnavailable`)
- Icon spans: `aria-hidden="true"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | activates button |
| `Space` | activates button |
| `Tab` | moves focus to next focusable element |
| `Shift+Tab` | moves focus to previous focusable element |

### Focus And Announcement

- focus entry: root receives visible focus ring
- focus exit: focus ring clears immediately
- loading: `aria-busy="true"` signals pending state

## 7. Layout

### Sizing

- Default: `height: control-height`, `min-width: 5rem`, `padding: 0 space-control-x`
- Small: `height: control-height - 0.375rem`, `min-width: 4.25rem`, `padding: 0 (space-control-x - 0.125rem)`, `font-size: 0.75rem`
- Large: `height: control-height + 0.375rem`, `min-width: 5.75rem`, `padding: 0 (space-control-x + 0.125rem)`, `font-size: 0.875rem`
- Icon-only: `min-width: 0`, `padding: 0`, `width: control-height` (adjusted for size)
- Icon padding adjustment: when a leading or trailing icon is present, the padding on that icon's side is reduced by `0.125rem` (2px). This subtly tightens the icon side to balance visual weight against the label side. Applies independently to each side.

### Composition

- parent expectations: toolbars, panels, dialogs, form actions
- child expectations: label + optional icons
- resizing: width auto-fits content unless parent stretches

## 8. Token Usage — Exact Values

### Root (base — secondary variant)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `gap` | `0.375rem` (6px) |
| `min-width` | `5rem` |
| `height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-treatment-interactive-radius, var(--poodle-radius-control))` |
| `background` | `var(--poodle-color-background-surface)` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `letter-spacing` | `0.01em` |
| `line-height` | `1` |
| `text-decoration` | `none` |
| `transition` | `background, border-color, color, box-shadow, transform` all at `motion-duration-interaction motion-easing-standard` |

### CSS Custom Properties (variant system)

| Var | Secondary (default) | Primary | Ghost |
|-----|---------------------|---------|-------|
| `--poodle-button-fill` | `background-surface` | `accent-base` | `transparent` |
| `--poodle-button-fill-hover` | `color-mix(fill 84%, background-elevated)` | same formula | same formula |
| `--poodle-button-fill-active` | `color-mix(fill 72%, background-elevated)` | same formula | same formula |
| `--poodle-button-border` | `border-default` | `color-mix(accent-base 84%, black)` | `transparent` |
| `--poodle-button-text` | `text-primary` | `text-inverse` | `text-primary` |
| `--poodle-button-shadow` | `inset 0 0.0625rem 0 color-mix(white 8%, transparent)` | `inset 0 0.0625rem 0 color-mix(white 14%, transparent), 0 0.375rem 1.125rem color-mix(black 18%, transparent)` | `none` |

### Tone: danger

| Selector | `--poodle-button-fill` | `--poodle-button-border` | `--poodle-button-text` | `--poodle-button-shadow` |
|----------|---------------------|-----------------------|---------------------|-----------------------|
| `[data-tone="danger"]` (secondary base) | `color-mix(in srgb, status-danger 16%, background-surface)` | `color-mix(in srgb, status-danger 46%, border-default)` | `text-primary` | default |
| `[data-variant="primary"][data-tone="danger"]` | `status-danger` | `color-mix(in srgb, status-danger 84%, black)` | `text-inverse` | `inset 0 0.0625rem 0 color-mix(white 14%, transparent), 0 0.375rem 1.125rem color-mix(black 18%, transparent)` |
| `[data-variant="ghost"][data-tone="danger"]` | `transparent` | `transparent` | `status-danger` | `none` |

### Size adjustments

| Size | height | min-width | padding | font-size |
|------|--------|-----------|---------|-----------|
| `sm` | `calc(control-height - 0.375rem)` | `4.25rem` | `0 calc(space-control-x - 0.125rem)` | `0.75rem` |
| `md` | `control-height` | `5rem` | `0 space-control-x` | `typography-label-size` |
| `lg` | `calc(control-height + 0.375rem)` | `5.75rem` | `0 calc(space-control-x + 0.125rem)` | `0.875rem` |

### Icon padding adjustments

When a leading icon is present, reduce `padding-left` by `0.125rem` (2px).
When a trailing icon or chevron is present, reduce `padding-right` by `0.125rem` (2px).
Both adjustments apply independently.

| Condition | `padding-left` | `padding-right` |
|-----------|----------------|-----------------|
| No icons | `space-control-x` | `space-control-x` |
| Leading icon only | `space-control-x - 0.125rem` | `space-control-x` |
| Trailing icon only | `space-control-x` | `space-control-x - 0.125rem` |
| Both icons | `space-control-x - 0.125rem` | `space-control-x - 0.125rem` |

### Icon-only adjustments

| Size | width | min-width | padding |
|------|-------|-----------|---------|
| `md` | `control-height` | `0` | `0` |
| `sm` | `calc(control-height - 0.375rem)` | `0` | `0` |
| `lg` | `calc(control-height + 0.375rem)` | `0` | `0` |

### Hover (not disabled)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-button-fill-hover)` |
| `border-color` | `color-mix(in srgb, var(--poodle-button-border) 78%, var(--poodle-color-text-primary))` |

### Active (not disabled)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-button-fill-active)` |
| `transform` | `translateY(0.03125rem)` |

### Focus

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Label

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `white-space` | `nowrap` |

### Icon wrapper

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `var(--poodle-size-icon-md)` |
| `height` | `var(--poodle-size-icon-md)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.875rem` |
| `line-height` | `1` |

### Chevron

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `opacity` | `0.5` |
| `margin-left` | `calc(var(--poodle-space-inline-sm) * -0.25)` |

### Spinner

| Property | Value |
|----------|-------|
| `width` | `0.75rem` |
| `height` | `0.75rem` |
| `border` | `0.125rem solid color-mix(in srgb, currentColor 24%, transparent)` |
| `border-top-color` | `currentColor` |
| `border-radius` | `999px` |
| `animation` | `rotate(360deg) 0.8s linear infinite` |

## 9. Svelte Notes

- Uses CSS custom properties (`--poodle-button-fill`, etc.) for the variant system
- `data-variant`, `data-tone`, `data-size`, `data-icon-only`, `data-loading`, `data-has-leading`, `data-has-trailing` data attributes
- `data-tone` only emits when tone is not `"default"` (omitted otherwise)
- `data-loading` always emits (even as `"false"`)
- `data-has-leading` and `data-has-trailing` emit presence-only (value is truthy or attribute is omitted)
- `isUnavailable = isDisabled || isLoading` — both disable the native button
- Icon component rendered at size="sm" for leadingIcon/trailingIcon props
- Supports named slots `leading` and `trailing` for custom icon content
- Treatment token: `--poodle-treatment-interactive-radius` with fallback to `--poodle-radius-control`
- Chevron renders `chevron-down` icon from registry at size `sm`, positioned after all other content

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::button`
- Spec struct: `ButtonSpec` in primitives crate
- Component struct: `PoodleButton` in components crate
- GPUI must replicate the hover/active color-mix chains
- The treatment radius fallback can be modeled as: use treatment token if set, else radius-control
- Active translateY(0.03125rem) — half a pixel press-down — may be omitted in GPUI (known delta)
- Spinner animation: GPUI should use a rotating element or skip spinner for initial implementation

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] variant, size, isDisabled, isLoading mean the same thing
- [ ] activation suppressed while disabled or loading
- [ ] keyboard activation matches
- [ ] icon-only accessible-name rule matches
- [ ] aria-busy on loading matches

### Tier 2: Visual Parity

- [ ] all three variant color schemes plus danger tone combinations match exactly
- [ ] all three sizes match (height, min-width, padding, font-size)
- [ ] icon-only square sizing matches
- [ ] hover border-color mix matches (78% with text-primary)
- [ ] hover/active background formulas match
- [ ] box-shadow per variant matches
- [ ] focus ring matches
- [ ] disabled opacity matches
- [ ] spinner appearance matches

### Tier 3: Implementation Freedom

- [ ] active translateY may differ (known delta)
- [ ] spinner implementation details are platform-owned
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| active translateY(0.03125rem) may be omitted in GPUI | sub-pixel transform, GPUI limitation | allowed | revisit if GPUI gains sub-pixel transforms |
| CSS transition timing | GPUI may not support CSS-style transitions | allowed | match where possible |
| Treatment radius fallback chain | CSS var fallback vs Rust conditional | allowed | same visual result |
| box-shadow omitted in GPUI | GPUI lacks CSS box-shadow support | allowed | revisit if GPUI adds shadow primitives |
| letter-spacing omitted in GPUI | GPUI text rendering has no letter-spacing API | allowed | minor visual impact |
| Spinner uses loader icon in GPUI | GPUI cannot animate CSS border spinners | allowed | same semantic meaning, visual delta accepted |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Variants

Three buttons in a horizontal row with 8px gap:

| Label | Variant | Tone |
|-------|---------|------|
| Primary | primary | default |
| Secondary | secondary | default |
| Ghost | ghost | default |

### Danger tone

Three buttons in a horizontal row with 8px gap:

| Label | Variant | Tone |
|-------|---------|------|
| Danger primary | primary | danger |
| Danger secondary | secondary | danger |
| Danger ghost | ghost | danger |

### With icons

Three buttons in a horizontal row with 8px gap:

| Label | Variant | Leading icon | Trailing icon |
|-------|---------|-------------|--------------|
| Create | secondary | plus | — |
| Open | secondary | — | external-link |
| Save | secondary | save | check |

### With chevron

Three buttons in a horizontal row with 8px gap:

| Label | Variant | Chevron | Leading icon |
|-------|---------|---------|-------------|
| Options | secondary | true | — |
| Actions | primary | true | — |
| Filter | secondary | true | filter |

### Sizes

Three buttons in a horizontal row with 8px gap, bottom-aligned:

| Label | Variant | Size |
|-------|---------|------|
| Small | primary | sm |
| Medium | primary | md |
| Large | primary | lg |

### States

Three buttons in a horizontal row with 8px gap:

| Label | Variant | Props |
|-------|---------|-------|
| Disabled | primary | `isDisabled: true` |
| Loading | primary | `isLoading: true` |
| Disabled secondary | secondary | `isDisabled: true` |

### Click counter

Text element below all specimens showing "Clicks: {n}" that increments when any interactive button above is clicked. Text uses `text.secondary` color at `xs` size.

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: all action surfaces, toolbars, dialogs, form actions
- future follow-up: toggle-button variant family may get separate contract
