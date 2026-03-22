# IconButton

Status: detailed contract
Updated: 2026-03-16

## 1. Purpose

- Component name: `IconButton`
- Layer: `foundation`
- Summary: a compact square action trigger whose accessible name comes from a
  label prop rather than visible text content, with variant, size, pressed,
  loading, and disabled states
- In scope: icon-only command triggers, ghost/primary/secondary/danger
  variants, sm/md/lg sizes, pressed/selected state when explicitly configured,
  disabled/loading behavior with spinner animation, CSS custom property theming
- Out of scope: toolbar roving-focus behavior, menu-button or toggle-button
  composite semantics beyond explicit opt-in, text-bearing buttons (see Button)

## 2. Anatomy

```text
[Wrapper .icon-button-wrap]  <span>
  └── [Root .icon-button]  <button>
        ├── [Glyph .icon-button__glyph]  <span>  (when not loading; contains default slot for custom icon content)
        ├── [Spinner .icon-button__spinner]  <span>  (when loading)
        └── [Tooltip .icon-button__tooltip]  <span role="tooltip">  (shown on hover/focus)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Wrapper | yes | outer positioning container for button and tooltip | — |
| Root | yes | compact action surface, square button | background, border, radius, focus ring, size |
| Glyph | yes | visible icon character display; contains default slot for custom icon content | icon color, icon size, code font |
| Spinner | no | animated loading indicator replacing glyph | border color, animation |
| Tooltip | no | built-in tooltip surface shown on hover/focus after 300ms delay | background, text color, radius, padding, placement |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `ButtonVariant` | `"ghost"` | no | semantic appearance |
| `tone` | `ButtonTone` | `"default"` | no | intent modifier; composes with variant for danger×primary, danger×secondary, danger×ghost |
| `size` | `ControlSize` | `"md"` | no | shared control size |
| `icon` | `string` | — | yes | icon glyph character or registry identifier |
| `ariaLabel` | `string` | — | yes | required accessible name |
| `isDisabled` | `boolean` | `false` | no | suppresses activation |
| `isLoading` | `boolean` | `false` | no | shows spinner and suppresses activation |
| `isPressed` | `boolean \| null` | `null` | no | optional pressed/toggled state; null omits aria-pressed |
| `tooltip` | `string \| null` | `null` | no | override text for the built-in tooltip; defaults to `ariaLabel` when null |
| `tooltipPlacement` | `OverlayPlacement` | `"top"` | no | positioning of the tooltip relative to the button |
| `describedBy` | `string \| null` | `null` | no | aria-describedby reference |
| `type` | `HTMLButtonElement["type"]` | `"button"` | no | HTML button type attribute |

### ButtonVariant

```
type ButtonVariant = "ghost" | "primary" | "secondary"
```

### ButtonTone

```
type ButtonTone = "default" | "danger"
```

### ControlSize

```
type ControlSize = "sm" | "md" | "lg"
```

### OverlayPlacement

```
type OverlayPlacement = "top" | "bottom" | "left" | "right"
```

### Derived State

- `isUnavailable` = `isDisabled || isLoading` — disables click and sets cursor

### Controlled And Uncontrolled

- Command-only by default
- Optional controlled pressed state through `isPressed`; when `isPressed`
  is not null, `aria-pressed` is rendered

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | variant-specific surface with icon centered |
| hover | pointer enters (when available) | elevated background, darkened border; tooltip appears after 300ms delay |
| active | pointer or keyboard activation | further elevated background, slight downward translate |
| focus | keyboard focus | focus ring; tooltip appears after 300ms delay |
| pressed | `isPressed=true` | accent-tinted background and border, inset shadow |
| disabled | `isDisabled=true` | muted opacity, not-allowed cursor |
| loading | `isLoading=true` | spinner replaces glyph, activation suppressed, aria-busy |

### Component States

- Variant-driven CSS custom properties set on the root element
- `isUnavailable` combines disabled and loading for interaction gating
- Loading and disabled both suppress click events

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `click` | activation completes | `MouseEvent` | suppressed while disabled or loading |
| `focus` | focus enters root | `FocusEvent` | passthrough |
| `blur` | focus leaves root | `FocusEvent` | passthrough |

## 6. Accessibility

### Semantics

- Role: `button` (native)
- Required attributes: `aria-label` from prop (required, always present)
- `aria-busy="true"` when `isLoading=true`
- `aria-pressed` when `isPressed !== null` (renders true or false)
- `aria-describedby` links to the built-in tooltip element (auto-generated id);
  when the tooltip is open, the tooltip id replaces any `describedBy` value (not combined)
- The tooltip element uses `role="tooltip"` and is referenced by
  `aria-describedby` on the button
- `disabled` attribute when `isDisabled=true` or `isLoading=true`
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
  on mouseleave, blur, or Escape key
- `aria-pressed` state change is announced for toggle buttons
- live-region behavior: none by default
- GPUI-native accessibility mapping notes: icon-only buttons must expose
  role, accessible name, disabled state, and optional pressed state through
  native accessibility APIs

## 7. Layout

### Sizing

- Square surface sized from shared control-height token
- Icon remains centered regardless of loading state
- Size sm: width/height reduced by 0.375rem from md
- Size lg: width/height increased by 0.375rem from md
- Three size steps adjust width and height symmetrically

### Composition

- parent expectations: toolbars, shell actions, cards, headers
- child expectations: icon only (no text content, no slots)
- resizing rules: icon button does not wrap or expand to fit text

## 8. Token Usage — Exact Values

### CSS Custom Properties (Root scope)

| Property | Value |
|----------|-------|
| `--pug-icon-button-fill` | `transparent` |
| `--pug-icon-button-fill-hover` | `color-mix(in srgb, var(--pug-icon-button-fill) 76%, var(--pug-color-background-elevated))` |
| `--pug-icon-button-fill-active` | `color-mix(in srgb, var(--pug-icon-button-fill) 64%, var(--pug-color-background-elevated))` |
| `--pug-icon-button-border` | `transparent` |
| `--pug-icon-button-text` | `var(--pug-color-text-primary)` |

### Root `.icon-button` (base)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `var(--pug-size-control-height)` |
| `height` | `var(--pug-size-control-height)` |
| `padding` | `0` |
| `border` | `0.0625rem solid var(--pug-icon-button-border)` |
| `border-radius` | `var(--pug-treatment-interactive-solid-radius, var(--pug-radius-control))` |
| `background` | `var(--pug-icon-button-fill)` |
| `box-shadow` | `none` (ghost default); `inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)` (primary/secondary) |
| `color` | `var(--pug-icon-button-text)` |
| `cursor` | `pointer` |
| `transition` | `background, border-color, box-shadow, transform` at `var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard)` |

### Root — Size `sm`

| Property | Value |
|----------|-------|
| `width` | `calc(var(--pug-size-control-height) - 0.375rem)` |
| `height` | `calc(var(--pug-size-control-height) - 0.375rem)` |

### Root — Size `lg`

| Property | Value |
|----------|-------|
| `width` | `calc(var(--pug-size-control-height) + 0.375rem)` |
| `height` | `calc(var(--pug-size-control-height) + 0.375rem)` |

### Root — Variant `primary`

| Property | Value |
|----------|-------|
| `--pug-icon-button-fill` | `var(--pug-color-accent-base)` |
| `--pug-icon-button-border` | `color-mix(in srgb, var(--pug-color-accent-base) 84%, black)` |
| `--pug-icon-button-text` | `var(--pug-color-text-inverse)` |

### Root — Variant `secondary`

| Property | Value |
|----------|-------|
| `--pug-icon-button-fill` | `var(--pug-color-background-surface)` |
| `--pug-icon-button-border` | `var(--pug-color-border-default)` |

### Tone: danger

| Selector | `--pug-icon-button-fill` | `--pug-icon-button-border` | `--pug-icon-button-text` |
|----------|--------------------------|----------------------------|--------------------------|
| `[data-tone="danger"]` (ghost base) | `color-mix(in srgb, var(--pug-color-status-danger) 16%, var(--pug-color-background-surface))` | `color-mix(in srgb, var(--pug-color-status-danger) 46%, var(--pug-color-border-default))` | `var(--pug-color-text-primary)` |
| `[data-variant="primary"][data-tone="danger"]` | `var(--pug-color-status-danger)` | `color-mix(in srgb, var(--pug-color-status-danger) 84%, black)` | `var(--pug-color-text-inverse)` |
| `[data-variant="ghost"][data-tone="danger"]` | `transparent` | `transparent` | `var(--pug-color-status-danger)` |

### Root — Pressed (`isPressed=true`)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 20%, var(--pug-icon-button-fill))` |
| `border-color` | `color-mix(in srgb, var(--pug-color-accent-base) 56%, var(--pug-icon-button-border))` |
| `box-shadow` | `inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent), inset 0 0 0 0.0625rem color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent)` |

### Root — Hover (`:hover`)

| Property | Value |
|----------|-------|
| `background` | `var(--pug-icon-button-fill-hover)` |
| `border-color` | `color-mix(in srgb, var(--pug-icon-button-border) 74%, var(--pug-color-text-primary))` |

### Root — Active (`:active`)

| Property | Value |
|----------|-------|
| `background` | `var(--pug-icon-button-fill-active)` |
| `transform` | `translateY(0.03125rem)` |

### Root — Focus Visible (`:focus-visible`)

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Root — Disabled (`:disabled`)

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Glyph `.icon-button__glyph`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `45%` |
| `height` | `45%` |
| `font-family` | `var(--pug-typography-code-family)` |
| `font-size` | `0.875rem` |
| `line-height` | `1` |

Icon SVG inside glyph: `width: 100%; height: 100%` — scales proportionally
with the button surface across all sizes.

### Spinner `.icon-button__spinner`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `var(--pug-size-icon-md)` |
| `height` | `var(--pug-size-icon-md)` |
| `border` | `0.125rem solid color-mix(in srgb, currentColor 24%, transparent)` |
| `border-top-color` | `currentColor` |
| `border-radius` | `999px` |
| `animation` | `icon-button-spinner 0.8s linear infinite` |

### Spinner keyframes `@keyframes icon-button-spinner`

```
from { transform: rotate(0deg); }
to   { transform: rotate(360deg); }
```

## 9. Svelte Notes

- Uses native `<button>` element with `type` from prop (default `"button"`)
- `aria-label` always rendered from required prop
- `aria-pressed` rendered only when `isPressed !== null`
- `aria-busy` rendered when `isLoading=true`
- `disabled` attribute set when `isDisabled` or `isLoading` is true
  (`isUnavailable` derived state)
- Glyph and spinner are mutually exclusive; spinner replaces glyph during
  loading
- Custom properties on root element enable variant overrides without
  duplicating transition declarations
- `data-variant`, `data-tone`, and `data-size` attributes on root for CSS targeting
- Icon glyph is `aria-hidden` as the accessible name comes from `ariaLabel`
- border-radius uses `--pug-treatment-interactive-solid-radius` with
  fallback to `--pug-radius-control`
- Built-in tooltip renders as a `<span role="tooltip">` with auto-generated
  `id`, linked to the button via `aria-describedby`
- Tooltip displays `tooltip` prop text when provided, otherwise falls back
  to `ariaLabel`
- Tooltip appears after 300ms hover/focus delay and dismisses on mouseleave,
  blur, or Escape key
- Tooltip placement controlled by `tooltipPlacement` prop (default `"top"`)
- Events forwarded directly from native button element

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::icon_button`
- Spec struct: `IconButtonSpec` in primitives crate holds variant + size + icon
- Component struct: `PugIconButton` in components crate renders via `IntoElement`
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

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] required accessible-name rule matches (ariaLabel always present)
- [ ] aria-pressed semantics match when isPressed !== null
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
- [ ] pressed state accent-tinted background and double inset shadow match
- [ ] hover state fill-hover and darkened border (74% mix) match
- [ ] active state fill-active and translateY(0.03125rem) match
- [ ] size sm/md/lg width/height adjustments match (0.375rem delta)
- [ ] focus ring appearance matches (border-width-focus, focusRing color, 0.125rem offset)
- [ ] disabled opacity matches (state-opacity-disabled)
- [ ] glyph sizing matches (icon-md, code-family, 0.875rem)
- [ ] spinner border/animation matches (0.125rem, 24% currentColor, 0.8s linear)
- [ ] inset box-shadow on default state matches (white 8%)

### Tier 3: Implementation Freedom

- [ ] tooltip visual styling is platform-owned (delay, animation, arrow)
- [ ] icon rendering mechanism is platform-owned
- [ ] spinner animation implementation is platform-owned
- [ ] custom property vs direct color assignment is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
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
| map-pin | pressed | `isPressed: true` | Pin |
| ban | disabled | `isDisabled: true` | Block |
| refresh-cw | loading | `isLoading: true` | Refresh |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: Aura shell controls, Spark shell controls, generic
  toolbars, card actions, header actions, dialog close buttons, table row actions
- future follow-up: split `ToggleIconButton` only if parity review shows it
  deserves its own contract; coordinate with toolbar roving-focus work;
  badge overlay, icon-button group
