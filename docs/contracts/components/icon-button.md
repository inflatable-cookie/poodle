# IconButton

Status: detailed contract
Updated: 2026-07-19

## 1. Purpose

- Component name: `IconButton`
- Layer: `foundation`
- Summary: a compact square action trigger whose accessible name comes from a
  label prop rather than visible text content, with variant, size, pressed,
  loading, and disabled states
- In scope: icon-only command triggers, ghost/primary/secondary variants,
  default/danger/success tones, semantic size roles, xs/sm/md/lg/xl sizes,
  pressed/selected state when explicitly configured,
  disabled/loading behavior with shared spinner usage, CSS custom property theming
- Out of scope: toolbar roving-focus behavior, menu-button or toggle-button
  composite semantics beyond explicit opt-in, text-bearing buttons (see Button)

## 2. Anatomy

```text
[Wrapper .icon-button-wrap]  <span>
  └── [Root .icon-button]  <button>
        ├── [Glyph .icon-button__glyph]  <span>  (when not loading; contains default children content for custom glyph content)
        ├── [Spinner .icon-button__spinner]  <span>  (when loading)
        └── [Tooltip .icon-button__tooltip]  <span role="tooltip">  (shown on hover/focus)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Wrapper | yes | outer positioning container for button and tooltip | — |
| Root | yes | compact action surface, square button | background, border, radius, focus ring, size |
| Glyph | yes | visible icon character display; contains default children content for custom glyph content | icon color, icon size, code font |
| Spinner | no | shared `Spinner` primitive replacing glyph | spinner tokens via primitive contract |
| Tooltip | no | built-in tooltip surface shown on hover/focus after 300ms delay | background, text color, radius, padding, placement |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `ButtonVariant` | `"ghost"` | no | semantic appearance |
| `tone` | `ButtonTone` | `"default"` | no | intent modifier; composes with variant for danger/success × primary, secondary, ghost |
| `size` | `ControlSize` | `null` | no | explicit control size override |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `icon` | `string` | — | yes | icon glyph character or registry identifier |
| `ariaLabel` | `string` | — | yes | required accessible name |
| `disabled` | `boolean` | `false` | no | suppresses activation |
| `loading` | `boolean` | `false` | no | shows spinner and suppresses activation |
| `pressed` | `boolean \| null` | `null` | no | optional pressed/toggled state; null omits aria-pressed |
| `defaultPressed` | `boolean \| null` | `null` | no | uncontrolled initial pressed state; toggle mode is enabled when `pressed !== null` or `defaultPressed !== null` |
| `tooltip` | `string \| null` | `null` | no | override text for the built-in tooltip; defaults to `ariaLabel` when null |
| `tooltipPlacement` | `OverlayPlacement` | `"top"` | no | positioning of the tooltip relative to the button |
| `describedBy` | `string \| null` | `null` | no | aria-describedby reference |
| `expanded` | `boolean \| null` | `null` | no | optional `aria-expanded` state when the button triggers a disclosure or popover |
| `controls` | `string \| null` | `null` | no | optional `aria-controls` reference for the controlled surface |
| `type` | `HTMLButtonElement["type"]` | `"button"` | no | HTML button type attribute |

### ButtonVariant

```
type ButtonVariant = "ghost" | "primary" | "secondary"
```

### ButtonTone

```
type ButtonTone = "default" | "danger" | "success"
```

### ControlSize

`ControlSize = "xs" | "sm" | "md" | "lg" | "xl"`

### OverlayPlacement

```
type OverlayPlacement = "top" | "bottom" | "left" | "right"
```

### Derived State

- `isUnavailable` = `disabled || loading` — disables click and sets cursor

### Controlled And Uncontrolled

- Command-only by default
- Optional controlled pressed state through `pressed`; when `pressed`
  is not null, `aria-pressed` is rendered
- `pressed` is bindable for host-owned toggle state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | variant-specific surface with icon centered |
| hover | pointer enters (when available) | elevated background, darkened border; tooltip appears after 300ms delay |
| active | pointer or keyboard activation | further elevated background, slight downward translate; visible tooltip dismisses before the action callback runs |
| focus | keyboard focus | focus ring; tooltip appears after 300ms delay |
| pressed | `pressed=true` | non-primary variants: solid accent-base fill, accent-85%-black border, inverse text, no shadow |
| disabled | `disabled=true` | muted opacity, not-allowed cursor |
| loading | `loading=true` | spinner replaces glyph, activation suppressed, aria-busy |

### Component States

- Variant-driven CSS custom properties set on the root element
- `isUnavailable` combines disabled and loading for interaction gating
- Loading and disabled both suppress click events

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Machine-backed via shared machinery (g11 extraction sweep): the tooltip
path composes `hoverTransition` (300ms open delay, immediate close — the
same machine Tooltip runs). Pressed state is plain controllable state.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onClick` | activation completes | `MouseEvent` | suppressed while disabled or loading |
| `onFocus` | focus enters root | `FocusEvent` | passthrough |
| `onBlur` | focus leaves root | `FocusEvent` | passthrough |
| `onPressedChange` | toggle state changes | `boolean` | fires when the icon button is in toggle mode |

## 6. Accessibility

### Semantics

- Role: `button` (native)
- Required attributes: `aria-label` from prop (required, always present)
- `aria-busy="true"` when `loading=true`
- `aria-pressed` when `pressed !== null` (renders true or false)
- `aria-expanded` when `expanded !== null`; `aria-controls` when `controls` is provided
- `aria-describedby` links to the built-in tooltip element (auto-generated id);
  when the tooltip is open, the tooltip id replaces any `describedBy` value (not combined)
- The tooltip element uses `role="tooltip"` and is referenced by
  `aria-describedby` on the button
- `disabled` attribute when `disabled=true` or `loading=true`
- Labeling rules: visible icon alone is never treated as sufficient naming;
  `ariaLabel` is always required

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | activates button |
| `Space` | activates button |
| `Tab` | moves focus into or past button |
| `Escape` | dismisses the tooltip if visible (does not blur the button) |

### Focus And Announcement

- focus entry: visible focus ring appears
- focus exit: ring clears with no residual active styling
- `aria-label` is announced on focus
- tooltip text is exposed via `aria-describedby` and announced as a
  description after the label
- tooltip appears on hover (300ms delay) and focus (300ms delay); dismissed
  on activation, mouseleave, blur, or Escape key
- `aria-pressed` state change is announced for toggle buttons
- live-region behavior: none by default
- GPUI-native accessibility mapping notes: icon-only buttons must expose
  role, accessible name, disabled state, and optional pressed state through
  native accessibility APIs

## 7. Layout

### Sizing

- Square surface sized from shared control-height token
- Icon remains centered regardless of loading state
- Size xs: width/height reduced by `0.25rem` from md
- Size sm: width/height reduced by 0.375rem from md
- Size md: width/height equal to `control-height`
- Size lg: width/height increased by 0.375rem from md
- Size xl: width/height increased by `0.5rem` from md
- Five size steps adjust width and height symmetrically

### Composition

- parent expectations: toolbars, shell actions, cards, headers
- child expectations: icon only (no text content; optional default snippet for custom glyph content)
- resizing rules: icon button does not wrap or expand to fit text

## 8. Token Usage — Exact Values

### CSS Custom Properties (Root scope)

| Property | Value |
|----------|-------|
| `--poodle-icon-button-fill` | `transparent` |
| `--poodle-icon-button-fill-hover` | `color-mix(in srgb, var(--poodle-icon-button-fill) 76%, var(--poodle-color-background-elevated))` |
| `--poodle-icon-button-fill-active` | `color-mix(in srgb, var(--poodle-icon-button-fill) 64%, var(--poodle-color-background-elevated))` |
| `--poodle-icon-button-border` | `transparent` |
| `--poodle-icon-button-text` | `var(--poodle-color-text-primary)` |

### Root `.icon-button` (base)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `var(--poodle-size-control-height)` |
| `height` | `var(--poodle-size-control-height)` |
| `padding` | `0` |
| `border` | `0.0625rem solid var(--poodle-icon-button-border)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-icon-button-fill)` |
| `box-shadow` | `none` (ghost default); `inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)` (primary/secondary) |
| `color` | `var(--poodle-icon-button-text)` |
| `cursor` | `pointer` |
| `transition` | `background, border-color, box-shadow, transform` at `var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

### Root — Size `sm`

| Property | Value |
|----------|-------|
| `width` | `calc(var(--poodle-size-control-height) - 0.375rem)` |
| `height` | `calc(var(--poodle-size-control-height) - 0.375rem)` |

### Root — Size `xs`

| Property | Value |
|----------|-------|
| `width` | `calc(var(--poodle-size-control-height) - 0.25rem)` |
| `height` | `calc(var(--poodle-size-control-height) - 0.25rem)` |

### Root — Size `lg`

| Property | Value |
|----------|-------|
| `width` | `calc(var(--poodle-size-control-height) + 0.375rem)` |
| `height` | `calc(var(--poodle-size-control-height) + 0.375rem)` |

### Root — Size `xl`

| Property | Value |
|----------|-------|
| `width` | `calc(var(--poodle-size-control-height) + 0.5rem)` |
| `height` | `calc(var(--poodle-size-control-height) + 0.5rem)` |

### Root — Variant `primary`

| Property | Value |
|----------|-------|
| `--poodle-icon-button-fill` | `var(--poodle-color-accent-base)` |
| `--poodle-icon-button-border` | `color-mix(in srgb, var(--poodle-color-accent-base) 84%, black)` |
| `--poodle-icon-button-text` | `var(--poodle-color-text-inverse)` |

### Root — Variant `secondary`

| Property | Value |
|----------|-------|
| `--poodle-icon-button-fill` | `var(--poodle-color-background-surface)` |
| `--poodle-icon-button-border` | `var(--poodle-color-border-default)` |

### Tone: danger

| Selector | `--poodle-icon-button-fill` | `--poodle-icon-button-border` | `--poodle-icon-button-text` |
|----------|--------------------------|----------------------------|--------------------------|
| `[data-tone="danger"]` (base) | `color-mix(in srgb, var(--poodle-color-status-danger) 16%, var(--poodle-color-background-surface))` | `color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default))` | (inherits) |
| `[data-variant="primary"][data-tone="danger"]` | `var(--poodle-color-status-danger)` | `color-mix(in srgb, var(--poodle-color-status-danger) 84%, black)` | `var(--poodle-color-text-inverse)` |
| `[data-variant="ghost"][data-tone="danger"]` | `transparent` | `transparent` | `var(--poodle-color-status-danger)` |

### Tone: success

| Selector | `--poodle-icon-button-fill` | `--poodle-icon-button-border` | `--poodle-icon-button-text` |
|----------|--------------------------|----------------------------|--------------------------|
| `[data-tone="success"]` (base) | `color-mix(in srgb, var(--poodle-color-status-success) 16%, var(--poodle-color-background-surface))` | `color-mix(in srgb, var(--poodle-color-status-success) 46%, var(--poodle-color-border-default))` | (inherits) |
| `[data-variant="primary"][data-tone="success"]` | `var(--poodle-color-status-success)` | `color-mix(in srgb, var(--poodle-color-status-success) 84%, black)` | `var(--poodle-color-text-inverse)` |
| `[data-variant="ghost"][data-tone="success"]` | `transparent` | `transparent` | `var(--poodle-color-status-success)` |

### Ghost danger hover

| Selector | Property | Value |
|----------|----------|-------|
| `.icon-button[data-variant="ghost"][data-tone="danger"]:hover:not(:disabled)` | `--poodle-icon-button-border` | `color-mix(in srgb, var(--poodle-color-status-danger) 46%, var(--poodle-color-border-default))` |
| `.icon-button[data-variant="ghost"][data-tone="danger"]:hover:not(:disabled)` | `background` | `color-mix(in srgb, var(--poodle-color-status-danger) 10%, transparent)` |

### Ghost success hover

| Selector | Property | Value |
|----------|----------|-------|
| `.icon-button[data-variant="ghost"][data-tone="success"]:hover:not(:disabled)` | `--poodle-icon-button-border` | `color-mix(in srgb, var(--poodle-color-status-success) 46%, var(--poodle-color-border-default))` |
| `.icon-button[data-variant="ghost"][data-tone="success"]:hover:not(:disabled)` | `background` | `color-mix(in srgb, var(--poodle-color-status-success) 10%, transparent)` |

### Root — Pressed (`pressed=true`)

Selector: `.icon-button[data-pressed="true"]:not([data-variant="primary"])` — a
solid-accent treatment applied via custom-property overrides for all non-primary
variants (ghost/secondary). Primary variant keeps its own variant styling when
pressed.

| Property | Value |
|----------|-------|
| `--poodle-icon-button-fill` | `var(--poodle-color-accent-base)` |
| `--poodle-icon-button-fill-hover` | `color-mix(in srgb, white 12%, var(--poodle-color-accent-base))` |
| `--poodle-icon-button-border` | `color-mix(in srgb, var(--poodle-color-accent-base) 85%, black)` |
| `--poodle-icon-button-text` | `var(--poodle-color-text-inverse)` |
| `--poodle-icon-button-shadow` | `none` |

### Root — Hover (`:hover:not(:disabled)`)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-icon-button-fill-hover)` |
| `border-color` | `color-mix(in srgb, var(--poodle-icon-button-border) 74%, var(--poodle-color-text-primary))` |
| `box-shadow` | `var(--poodle-recipe-icon-button-shadow, var(--poodle-icon-button-shadow))` |

### Root — Active (`:active`)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-icon-button-fill-active)` |
| `transform` | `translateY(0.03125rem)` |

### Root — Focus Visible (`:focus-visible`)

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Root — Disabled (`:disabled`)

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Glyph `.icon-button__glyph`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `45%` |
| `height` | `45%` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.875rem` |
| `line-height` | `1` |

Icon SVG inside glyph: `width: 100%; height: 100%` — scales proportionally
with the button surface across all sizes.

### Spinner `.icon-button__spinner`

The loading indicator is the shared [`Spinner`](./spinner.md) primitive with
`variant="ring"`, `size="sm"`, and `tone="current"`, centered inside the
button-owned wrapper.

### Tooltip `.icon-button__tooltip`

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `max-width` | `16rem` |
| `padding` | `0.375rem 0.5rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-size` | `0.6875rem` |
| `line-height` | `1.35` |
| `white-space` | `nowrap` |
| `pointer-events` | `none` |

## 9. Svelte Notes

- Uses native `<button>` element with `type` from prop (default `"button"`)
- `aria-label` always rendered from required prop
- `aria-pressed` rendered only when `pressed !== null`
- `aria-busy` rendered when `loading=true`
- `disabled` attribute set when `disabled` or `loading` is true
  (`isUnavailable` derived state)
- Glyph and spinner are mutually exclusive; spinner replaces glyph during
  loading
- Custom properties on root element enable variant overrides without
  duplicating transition declarations
- `data-variant`, `data-tone`, `data-size`, and `data-density` attributes on root for CSS targeting
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- Icon glyph is `aria-hidden` as the accessible name comes from `ariaLabel`
- border-radius uses `--poodle-radius-control` with
  fallback to `--poodle-radius-control`
- Built-in tooltip renders as a `<span role="tooltip">` with auto-generated
  `id`, linked to the button via `aria-describedby`
- Tooltip displays `tooltip` prop text when provided, otherwise falls back
  to `ariaLabel`
- Tooltip appears after 300ms hover/focus delay and dismisses on mouseleave,
  blur, or Escape key
- Tooltip placement controlled by `tooltipPlacement` prop (default `"top"`)
- Events forwarded directly from native button element

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::icon_button`
- Spec struct: `IconButtonSpec` in primitives crate holds variant + size + icon
- Component struct: `PoodleIconButton` in components crate renders via `IntoElement`
- GPUI must not rely on tooltip text as the only accessible name
- Custom property pattern for fill/border/hover/active can be flattened in
  GPUI to direct computed values per variant
- GPUI must model `color-mix` as `token.opacity(token.a * multiplier)` since GPUI has no CSS color-mix
- Ghost fill opacity: 58% on background-surface
- Ghost border opacity: 76% on border-subtle
- Primary border: 84% accent-base mixed with black
- Danger tone (secondary base) fill: 16% status-danger mixed with background-surface
- Danger tone (secondary base) border: 46% status-danger mixed with border-default
- Hover border: 74% border mixed with text-primary
- Pressed accent mix: 20% on fill, 56% on border, 18% inset ring
- Spinner animation must be replicated using GPUI's animation system
- The `translateY(0.03125rem)` active press effect should be preserved

## 10a. Jetstream Notes

- `IconButton::from_spec(spec, theme).on_click(...)`, identical to `Button`.
- Loading is treated as disabled for interaction.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] required accessible-name rule matches (ariaLabel always present)
- [ ] aria-pressed semantics match when pressed !== null
- [ ] aria-busy semantics match when loading
- [ ] disabled/loading suppression of click events matches
- [ ] keyboard activation (Enter, Space) matches
- [ ] tooltip uses `role="tooltip"` and is linked via `aria-describedby`
- [ ] tooltip shows `ariaLabel` by default, `tooltip` prop when provided
- [ ] tooltip appears on hover/focus with 300ms delay
- [ ] tooltip dismisses on mouseleave, blur, and Escape key
- [ ] describedBy passthrough matches (tooltip id replaces describedBy when open)

### Tier 2: Visual Parity

- [ ] ghost variant custom properties match (58% surface fill, 76% border)
- [ ] primary variant fill/border/text match (accent-base, 84% darkened border, inverse text)
- [ ] secondary variant fill/border match (surface, border-default)
- [ ] danger tone color-mix values match (16% danger fill, 46% danger border for secondary base; solid danger for primary; transparent for ghost)
- [ ] success tone color-mix values match (16% success fill, 46% success border for secondary base; solid success for primary; transparent for ghost)
- [ ] pressed state (non-primary) solid accent-base fill, accent-85%-black border, inverse text, no shadow match
- [ ] hover state fill-hover and darkened border (74% mix) match
- [ ] active state fill-active and translateY(0.03125rem) match
- [ ] size sm/md/lg width/height adjustments match (0.375rem delta)
- [ ] focus ring appearance matches (border-width-focus, focusRing color, 0.125rem offset)
- [ ] disabled opacity matches (state-opacity-disabled)
- [ ] glyph sizing matches (icon-md, code-family, 0.875rem)
- [ ] shared spinner sizing, tone, and centering match
- [ ] inset box-shadow on default state matches (white 8%)

### Tier 3: Implementation Freedom

- [ ] tooltip visual styling is platform-owned (delay, animation, arrow)
- [ ] icon rendering mechanism is platform-owned
- [ ] spinner animation implementation is platform-owned
- [ ] custom property vs direct color assignment is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Jetstream has no `onFocus` / `onBlur` | the runtime raises pointer events, not focus ones | accepted, tracked | arrives with focus plumbing |
| Jetstream has no `onPressedChange` | `pressed` is a spec input there; the host derives the change from `on_click` | accepted | none |
| Color-mix blending | GPUI may approximate color-mix differently | allowed | visual result must be comparable |
| Custom property pattern | GPUI may use direct values instead of CSS custom properties | allowed | final computed colors must match |
| Spinner animation | GPUI may use different animation primitives | allowed | visual effect must match (rotating arc) |
| translateY active effect | GPUI may not support sub-pixel transforms | allowed | press feedback must be perceptible |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Variants

Three icon buttons in a horizontal row with 8px gap:

| Icon | Variant | aria-label |
|------|---------|------------|
| plus | primary | Add |
| settings | secondary | Settings |
| x | ghost | Close |

### Danger tone

Three icon buttons with danger tone in a horizontal row:

| Icon | Variant | Tone | aria-label |
|------|---------|------|------------|
| trash-2 | primary | danger | Delete |
| trash-2 | secondary | danger | Delete |
| trash-2 | ghost | danger | Delete |

### Sizes

Three icon buttons at different sizes:

| Icon | Size | aria-label |
|------|------|------------|
| star | sm | Favorite |
| star | md | Favorite |
| star | lg | Favorite |

### States

Four icon button states:

| Icon | State | Props | aria-label |
|------|-------|-------|------------|
| map-pin | pressed | `pressed: true` | Pin |
| ban | disabled | `disabled: true` | Block |
| refresh-cw | loading | `loading: true` | Refresh |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: Aura shell controls, Spark shell controls, generic
  toolbars, card actions, header actions, dialog close buttons, table row actions
- future follow-up: split `ToggleIconButton` only if parity review shows it
  deserves its own contract; coordinate with toolbar roving-focus work;
  badge overlay, icon-button group
- Glyph icon and loading spinner both resolve through the shared supporting-size mapping rather than a fixed absolute icon size
