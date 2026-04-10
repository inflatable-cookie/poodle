# Button

Status: detailed contract
Updated: 2026-04-09

## 1. Purpose

- Component name: `Button`
- Layer: `foundation`
- Summary: a general action trigger for commands, confirmations, and view-level
  affordances
- In scope: text buttons, icon-leading buttons, icon-only buttons, loading and
  disabled states, three visual variants, semantic size roles, and five explicit
  control sizes
- Out of scope: transport controls, DAW-specific command widgets
- Toggle behavior: Button supports `pressed`/`defaultPressed` props and `pressedChange` event for toggle-button use cases

## 2. Anatomy

```text
[Root .button]  <button>
  ├── [Spinner .button__spinner] (conditional, when loading)
  ├── [Leading Icon .button__icon] (optional, via slot or leadingIcon prop)
  ├── [Label .button__label] (optional, via default slot)
  ├── [Trailing Icon .button__icon] (optional, via slot or trailingIcon prop)
  └── [Chevron .button__chevron] (optional, when chevron=true)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | clickable command surface | background, border, radius, shadow, focus ring |
| Spinner | no | shared `Spinner` primitive in `ring` with supporting visual sizing derived from the resolved control size | spinner tokens via primitive contract |
| Leading Icon | no | icon before label | icon size, icon color |
| Label | no | text content | typography, text color |
| Trailing Icon | no | icon after label | icon size, icon color |
| Chevron | no | disclosure indicator after content | opacity, margin |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"primary" \| "secondary" \| "ghost"` | `"secondary"` | no | visual treatment |
| `tone` | `"default" \| "danger"` | `"default"` | no | intent modifier; composes with variant for danger×primary, danger×secondary, danger×ghost |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `type` | `"button" \| "submit" \| "reset"` | `"button"` | no | HTML button type |
| `form` | `string \| null` | `null` | no | external form id to associate with |
| `formaction` | `string \| null` | `null` | no | per-button form submission URL override |
| `formenctype` | `"application/x-www-form-urlencoded" \| "multipart/form-data" \| "text/plain" \| null` | `null` | no | per-button encoding override |
| `formmethod` | `"get" \| "post" \| "dialog" \| null` | `null` | no | per-button form method override |
| `formnovalidate` | `boolean` | `false` | no | skips form validation for this submit action |
| `formtarget` | `"_self" \| "_blank" \| "_parent" \| "_top" \| string \| null` | `null` | no | per-button browsing context override |
| `disabled` | `boolean` | `false` | no | suppresses activation |
| `loading` | `boolean` | `false` | no | shows spinner, suppresses activation |
| `leadingIcon` | `string \| null` | `null` | no | icon registry identifier |
| `trailingIcon` | `string \| null` | `null` | no | icon registry identifier |
| `chevron` | `boolean` | `false` | no | renders trailing disclosure chevron indicator |
| `pressed` | `boolean \| null` | `null` | no | controlled toggle state; when non-null, button acts as a toggle with `aria-pressed` |
| `defaultPressed` | `boolean` | `false` | no | initial pressed state for uncontrolled toggle mode |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible label |
| `ariaExpanded` | `boolean \| null` | `null` | no | disclosure-state hint for menu and accordion triggers |
| `describedBy` | `string \| null` | `null` | no | aria-describedby target |
| `className` | `string` | `""` | no | additional CSS classes |
| `style` | `string \| null` | `null` | no | inline style passthrough for dynamic sizing and CSS-variable overrides |

### Slots

| Slot | Purpose |
|------|---------|
| default | label text content; absence triggers icon-only mode |
| leading | custom leading icon content (overrides leadingIcon prop) |
| trailing | custom trailing icon content (overrides trailingIcon prop) |

### Controlled And Uncontrolled

- Command actions: no persistent value model for click behavior
- Toggle mode (controlled): set `pressed` prop; listen to `pressedChange` to update
- Toggle mode (uncontrolled): set `defaultPressed`; component manages internal state; `pressedChange` fires on each toggle

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | variant-specific fill, border, text, shadow |
| hover | pointer enters (not disabled) | `background: button-fill-hover`, `border-color` mix with text-primary 78% |
| active | press (not disabled) | `background: button-fill-active`, `transform: translateY(0.03125rem)` |
| focus | keyboard focus | `outline: border-width-focus solid accent-focusRing`, `outline-offset: 0.125rem` |
| disabled | `disabled=true` or `loading=true` | `opacity: state-opacity-disabled`, `cursor: not-allowed` |
| loading | `loading=true` | spinner visible, button disabled |
| icon-only | no default slot content | square button, no min-width |
| pressed | `pressed=true` or uncontrolled toggle active | non-primary variants get accent fill, accent border, inverse text; `aria-pressed="true"` |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `click` | activation completed | `MouseEvent` | suppressed while disabled or loading |
| `focus` | focus enters root | `FocusEvent` | passthrough |
| `blur` | focus leaves root | `FocusEvent` | passthrough |
| `pressedChange` | toggle state changes | `{ pressed: boolean }` | fires when button is in toggle mode (`pressed` non-null or `defaultPressed` set) |

## 6. Accessibility

### Semantics

- Role: native `<button>` element
- Native attributes not modeled as explicit props, such as `role`, `aria-checked`,
  `name`, and `value`, pass through via standard rest attributes
- Native form override attrs: `form`, `formaction`, `formenctype`, `formmethod`, `formnovalidate`, and `formtarget` pass through directly when provided
- `aria-label`: from prop (required for icon-only)
- `aria-expanded`: set from `ariaExpanded` when the button acts as a disclosure trigger
- `aria-describedby`: from describedBy prop
- `style`: passes through directly to the native `<button>` when instance-level sizing or CSS-variable overrides are required
- `aria-pressed`: `"true"` or `"false"` when button is in toggle mode (pressed non-null or defaultPressed set); omitted for non-toggle buttons
- `aria-busy`: `"true"` when loading
- `disabled`: set when disabled or loading (`isUnavailable`)
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

- Extra-small: `height: control-height - 0.5rem`, `min-width: 3.75rem`, `padding: 0 (space-control-x - 0.125rem)`, `font-size: 0.6875rem`
- Small: `height: control-height - 0.375rem`, `min-width: 4.25rem`, `padding: 0 (space-control-x - 0.125rem)`, `font-size: 0.75rem`
- Default: `height: control-height`, `min-width: 5rem`, `padding: 0 space-control-x`
- Large: `height: control-height + 0.375rem`, `min-width: 5.75rem`, `padding: 0 (space-control-x + 0.125rem)`, `font-size: 0.875rem`
- Extra-large: `height: control-height + 0.5rem`, `min-width: 6.5rem`, `padding: 0 (space-control-x + 0.1875rem)`, `font-size: 0.9375rem`
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
| `background` | `var(--poodle-button-fill)` (see CSS Custom Properties table for per-variant values) |
| `box-shadow` | `var(--poodle-button-shadow)` (see CSS Custom Properties table for per-variant values) |
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

The secondary variant uses **elevation stacking** via `color-mix` toward `text-primary` (not toward `background-elevated`). This produces a surface that is slightly elevated relative to its container by mixing a small percentage of the text color into the surface color.

| Var | Secondary (default) | Primary | Ghost |
|-----|---------------------|---------|-------|
| `--poodle-button-fill` | `var(--poodle-treatment-interactive-fill, color-mix(in srgb, var(--poodle-surface, var(--poodle-color-background-surface)) 88%, var(--poodle-color-text-primary)))` | `accent-base` | `transparent` |
| `--poodle-button-fill-hover` | `var(--poodle-treatment-interactive-fill-active, color-mix(in srgb, var(--poodle-surface, ...) 80%, var(--poodle-color-text-primary)))` | `color-mix(in srgb, white 12%, accent-base)` | inherits generic hover formula |
| `--poodle-button-fill-active` | `color-mix(in srgb, var(--poodle-surface, ...) 84%, var(--poodle-color-text-primary))` | `color-mix(in srgb, accent-base 88%, black)` | inherits generic active formula |
| `--poodle-button-border` | `var(--poodle-treatment-interactive-border, var(--poodle-color-border-default))` | `color-mix(in srgb, accent-base 84%, black)` | `transparent` |
| `--poodle-button-border-hover` | `var(--poodle-treatment-interactive-border-active, color-mix(in srgb, var(--poodle-button-border) 78%, var(--poodle-color-text-primary)))` | inherits generic hover formula | `transparent` |
| `--poodle-button-text` | `text-primary` | `text-inverse` | `text-primary` |
| `--poodle-button-shadow` | `var(--poodle-treatment-interactive-shadow, inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent))` | `inset 0 0.0625rem 0 color-mix(white 14%, transparent), 0 0.375rem 1.125rem color-mix(black 18%, transparent)` | `none` |

### Tone: danger

Danger tone hover and active states stay within the red/danger color family rather than using the generic hover variable system. Each variant x danger combination defines its own fill, border, and text across idle, hover, and active states.

#### Secondary danger `[data-tone="danger"]`

| Var | Idle | Hover | Active |
|-----|------|-------|--------|
| `--poodle-button-fill` | `color-mix(in srgb, status-danger 16%, surface)` | `color-mix(in srgb, status-danger 24%, surface)` | `color-mix(in srgb, status-danger 32%, surface)` |
| `--poodle-button-border` | `border-default` | `color-mix(in srgb, status-danger 62%, border-default)` | (inherits hover) |
| `--poodle-button-text` | `text-primary` | (inherits) | (inherits) |

Note: The secondary danger idle border uses `border-default` (matching normal secondary), **not** a danger-tinted border. The danger border tint only appears on hover.

#### Primary danger `[data-variant="primary"][data-tone="danger"]`

| Var | Idle | Hover | Active |
|-----|------|-------|--------|
| `--poodle-button-fill` | `status-danger` | `color-mix(in srgb, white 12%, status-danger)` | `color-mix(in srgb, status-danger 88%, black)` |
| `--poodle-button-border` | `color-mix(in srgb, status-danger 84%, black)` | `color-mix(in srgb, status-danger 72%, black)` | (inherits hover) |
| `--poodle-button-text` | `text-inverse` | (inherits) | (inherits) |
| `--poodle-button-shadow` | `inset 0 0.0625rem 0 color-mix(white 14%, transparent), 0 0.375rem 1.125rem color-mix(black 18%, transparent)` | (inherits) | (inherits) |

#### Ghost danger `[data-variant="ghost"][data-tone="danger"]`

| Var | Idle | Hover | Active |
|-----|------|-------|--------|
| `--poodle-button-fill` | `transparent` | `color-mix(in srgb, status-danger 12%, transparent)` | `color-mix(in srgb, status-danger 18%, transparent)` |
| `--poodle-button-border` | `transparent` | `color-mix(in srgb, status-danger 28%, transparent)` | (inherits hover) |
| `--poodle-button-text` | `status-danger` | (inherits) | (inherits) |
| `--poodle-button-shadow` | `none` | (inherits) | (inherits) |

### Size adjustments

| Size | height | min-width | padding | font-size |
|------|--------|-----------|---------|-----------|
| `xs` | `calc(control-height - 0.5rem)` | `3.75rem` | `0 calc(space-control-x - 0.125rem)` | `0.6875rem` |
| `sm` | `calc(control-height - 0.375rem)` | `4.25rem` | `0 calc(space-control-x - 0.125rem)` | `0.75rem` |
| `md` | `control-height` | `5rem` | `0 space-control-x` | `typography-label-size` |
| `lg` | `calc(control-height + 0.375rem)` | `5.75rem` | `0 calc(space-control-x + 0.125rem)` | `0.875rem` |
| `xl` | `calc(control-height + 0.5rem)` | `6.5rem` | `0 calc(space-control-x + 0.1875rem)` | `0.9375rem` |

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
| `xs` | `calc(control-height - 0.5rem)` | `0` | `0` |
| `md` | `control-height` | `0` | `0` |
| `sm` | `calc(control-height - 0.375rem)` | `0` | `0` |
| `lg` | `calc(control-height + 0.375rem)` | `0` | `0` |
| `xl` | `calc(control-height + 0.5rem)` | `0` | `0` |

### Hover (not disabled)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-button-fill-hover)` |
| `border-color` | `var(--poodle-button-border-hover)` |
| `box-shadow` | `var(--poodle-treatment-interactive-shadow-active, var(--poodle-button-shadow))` |

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

Loading spinner uses the shared [`Spinner`](./spinner.md) contract with
`variant="ring"`, `size="sm"`, and `tone="current"` inside the
`.button__spinner` wrapper.

### Pressed/toggle state `[data-pressed="true"]:not([data-variant="primary"])`

When a button is in toggle mode and pressed, non-primary variants receive accent treatment (primary is already accent-colored, so pressed is purely semantic via `aria-pressed`):

| Var | Value |
|-----|-------|
| `--poodle-button-fill` | `var(--poodle-color-accent-base)` |
| `--poodle-button-fill-hover` | `color-mix(in srgb, white 12%, var(--poodle-color-accent-base))` |
| `--poodle-button-fill-active` | `color-mix(in srgb, var(--poodle-color-accent-base) 88%, black)` |
| `--poodle-button-border` | `color-mix(in srgb, var(--poodle-color-accent-base) 85%, black)` |
| `--poodle-button-text` | `var(--poodle-color-text-inverse)` |
| `--poodle-button-shadow` | `none` |

Toggle mode is activated when `pressed` is non-null OR `defaultPressed` is set. The `data-pressed` attribute reflects the current pressed state. `aria-pressed` is set to `"true"` or `"false"` accordingly. The `pressedChange` event fires on every toggle.

## 9. Svelte Notes

- Uses CSS custom properties (`--poodle-button-fill`, etc.) for the variant system
- `data-variant`, `data-tone`, `data-size`, `data-density`, `data-icon-only`, `data-loading`, `data-has-leading`, `data-has-trailing`, `data-pressed` data attributes
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- `data-tone` only emits when tone is not `"default"` (omitted otherwise)
- `data-loading` always emits (even as `"false"`)
- `data-has-leading` and `data-has-trailing` emit presence-only (value is truthy or attribute is omitted)
- `isUnavailable = disabled || loading` — both disable the native button
- Icon and spinner supporting visuals resolve through the shared supporting-size mapping rather than a fixed absolute size
- Supports named slots `leading` and `trailing` for custom icon content
- Treatment token: `--poodle-treatment-interactive-radius` with fallback to `--poodle-radius-control`
- Secondary variant uses elevation stacking: `color-mix` toward `var(--poodle-color-text-primary)` rather than toward a separate elevated background token; the surface color (`--poodle-surface` with fallback to `--poodle-color-background-surface`) is mixed at 88% idle / 80% hover / 84% active with text-primary
- Danger tone defines all three interaction states (idle, hover, active) for fill and border inline in the danger CSS custom properties, keeping hover/active within the red family rather than deferring to generic `--poodle-button-fill-hover`/`--poodle-button-border-hover`
- `data-pressed` emits the current pressed boolean when button is in toggle mode; omitted entirely for non-toggle buttons
- `isToggle` derived from `pressed !== null || defaultPressed`; controlled mode when `pressed !== null`, uncontrolled otherwise
- `pressedChange` event dispatches on every toggle activation, before the `click` event
- Chevron renders `chevron-down` icon from registry at size `sm`, positioned after all other content

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::button`
- Spec struct: `ButtonSpec` in primitives crate
- Component struct: `PoodleButton` in components crate
- GPUI must replicate the hover/active color-mix chains
- The treatment radius fallback can be modeled as: use treatment token if set, else radius-control
- Active translateY(0.03125rem) — half a pixel press-down — may be omitted in GPUI (known delta)
- GPUI uses the shared `Spinner` primitive rather than a button-owned loader treatment

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] variant, size, disabled, loading mean the same thing
- [ ] activation suppressed while disabled or loading
- [ ] keyboard activation matches
- [ ] icon-only accessible-name rule matches
- [ ] aria-busy on loading matches

### Tier 2: Visual Parity

- [ ] all three variant color schemes plus danger tone combinations match exactly
- [ ] all five sizes match (height, min-width, padding, font-size)
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
| Disabled | primary | `disabled: true` |
| Loading | primary | `loading: true` |
| Disabled secondary | secondary | `disabled: true` |

### Click counter

Text element below all specimens showing "Clicks: {n}" that increments when any interactive button above is clicked. Text uses `text.secondary` color at `xs` size.

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: all action surfaces, toolbars, dialogs, form actions
- future follow-up: toggle-button variant family may get separate contract
