# Callout

Status: detailed contract
Updated: 2026-03-25

## 1. Purpose

- Component name: `Callout`
- Layer: `foundation`
- Summary: contextual messaging block with tone-specific coloring, optional
  dismissal, action slots, and ARIA announcement support. Consolidates the
  former Banner and Callout components into a single primitive.
- In scope: neutral, info, success, warning, danger, and pending tones;
  inline contextual content; dismissible messaging; action slots; optional
  icon override; ARIA live-region announcements; title, message prop, and
  body slot
- Out of scope: toast notifications, alert dialogs, full-width page banners

## 2. Anatomy

```text
[Root .callout]  <section>
  ├── [Body .callout__body]
  │     ├── [Icon .callout__icon]  (slot or default based on tone)
  │     └── [Content .callout__content]
  │           ├── <strong>  (title, optional)
  │           ├── <p>  (message prop, optional)
  │           └── <slot>  (body, default slot)
  ├── [Actions .callout__actions]  (named slot, optional)
  └── [Dismiss .callout__dismiss]  (button, optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | contextual message shell with 3-column grid | background, border, radius, padding, color |
| Body | yes | icon + content container | grid layout, gap |
| Icon | no | tone-specific glyph or shared pending spinner in a circular badge | background, radius, font, color |
| Content | yes | title, message, and body container | typography, text color, gap |
| Actions | no | action buttons area | flex layout, gap |
| Dismiss | no | close button | size, border-radius, color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `StatusTone \| "neutral"` | `"neutral"` | no | semantic tone and coloring |
| `title` | `string \| null` | `null` | no | bold heading text rendered as `<strong>` |
| `message` | `string \| null` | `null` | no | body text rendered as `<p>` |
| `ariaLabel` | `string \| null` | `null` | no | optional accessible label for the callout region |
| `announceMode` | `CalloutAnnounceMode` | `"none"` | no | ARIA live-region behavior |
| `dismissible` | `boolean` | `false` | no | shows dismiss button |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `dismissLabel` | `string` | `"Dismiss message"` | no | accessible label for dismiss button |

### CalloutAnnounceMode

```
type CalloutAnnounceMode = "none" | "polite" | "assertive"
```

### StatusTone (with neutral extension)

```
type CalloutTone = "neutral" | "info" | "success" | "warning" | "danger" | "pending"
```

### Slots

| Slot | Purpose |
|------|---------|
| default | body content rendered inside `.callout__content` |
| icon | named slot to override the default tone-based icon |
| actions | named slot for action buttons (e.g. Resolve, Inspect) |

### Controlled And Uncontrolled

- Dismiss state is uncontrolled; parent handles via `on:dismiss` event

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral | `tone="neutral"` (default) | subtle panel background and border |
| info | `tone="info"` | info-status-tinted fill and border (dedicated blue `--poodle-color-status-info` token) |
| success | `tone="success"` | success-tinted fill and border |
| warning | `tone="warning"` | warning-tinted fill and border |
| danger | `tone="danger"` | danger-tinted fill and border |
| pending | `tone="pending"` | lighter accent-tinted fill and border |

### Component States

- dismissible: shows close button when `dismissible` is true

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `dismiss` | dismiss button clicked | `void` | only available when `dismissible` is true |

## 6. Accessibility

### Semantics

- Root element: `<section>`
- `aria-label` from prop when provided
- ARIA role derived from `announceMode`:
  - `"assertive"` → `role="alert"`, `aria-live="assertive"`
  - `"polite"` → `role="status"`, `aria-live="polite"`
  - `"none"` → no role or aria-live (default)

### Icon Mapping

| Tone | Default Indicator |
|------|-------------------|
| `neutral` | `info` icon |
| `info` | `info` icon |
| `success` | `check` icon |
| `warning` | `triangle-alert` icon |
| `danger` | `circle-x` icon |
| `pending` | shared `Spinner` primitive in `ring` + `sm` + `accent` configuration |

### Keyboard

| Key | Behavior |
|-----|----------|
| Tab | focuses dismiss button (when dismissible) |
| Enter/Space | activates dismiss button |

### Focus And Announcement

- focus entry: dismiss button is focusable when present
- live-region behavior: controlled by `announceMode` prop
- GPUI-native accessibility mapping: GPUI should expose callouts as
  grouped informational content; when announceMode is assertive, expose
  as an alert

## 7. Layout

### Sizing

- Width follows parent container
- Height grows with content

### Responsive

- At `max-width: 45rem`, grid collapses to single column; actions align start

### Composition

- parent expectations: forms, inspectors, cards, settings sections, dialogs
- child expectations: informative text, action buttons, optional inline content
- resizing rules: content wraps naturally; icon column remains fixed-width

## 8. Token Usage — Exact Values

### Root `.callout`

| Property | Value |
|----------|-------|
| `--poodle-callout-fill` | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |
| `--poodle-callout-border` | `color-mix(in srgb, var(--poodle-color-border-subtle) 88%, transparent)` |
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto auto` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-md)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid var(--poodle-callout-border)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-callout-fill)` |
| `--poodle-surface` | `var(--poodle-callout-fill)` |
| `color` | `var(--poodle-color-text-primary)` |

### Root — tone: info

| Property | Value |
|----------|-------|
| `--poodle-callout-fill` | `color-mix(in srgb, var(--poodle-color-status-info, #3b82f6) 10%, var(--poodle-color-background-panel))` |
| `--poodle-callout-border` | `color-mix(in srgb, var(--poodle-color-status-info, #3b82f6) 34%, var(--poodle-color-border-default))` |

### Root — tone: success

| Property | Value |
|----------|-------|
| `--poodle-callout-fill` | `color-mix(in srgb, var(--poodle-color-status-success) 10%, var(--poodle-color-background-panel))` |
| `--poodle-callout-border` | `color-mix(in srgb, var(--poodle-color-status-success) 34%, var(--poodle-color-border-default))` |

### Root — tone: warning

| Property | Value |
|----------|-------|
| `--poodle-callout-fill` | `color-mix(in srgb, var(--poodle-color-status-warning) 10%, var(--poodle-color-background-panel))` |
| `--poodle-callout-border` | `color-mix(in srgb, var(--poodle-color-status-warning) 34%, var(--poodle-color-border-default))` |

### Root — tone: danger

| Property | Value |
|----------|-------|
| `--poodle-callout-fill` | `color-mix(in srgb, var(--poodle-color-status-danger) 10%, var(--poodle-color-background-panel))` |
| `--poodle-callout-border` | `color-mix(in srgb, var(--poodle-color-status-danger) 34%, var(--poodle-color-border-default))` |

### Root — tone: pending

| Property | Value |
|----------|-------|
| `--poodle-callout-fill` | `color-mix(in srgb, var(--poodle-color-accent-base) 8%, var(--poodle-color-background-panel))` |
| `--poodle-callout-border` | `color-mix(in srgb, var(--poodle-color-accent-base) 26%, var(--poodle-color-border-default))` |

### Body `.callout__body`

| Property | Value |
|----------|-------|
| `align-self` | `start` |

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `auto minmax(0, 1fr)` |
| `gap` | `var(--poodle-space-inline-md)` |
| `min-width` | `0` |

### Icon `.callout__icon`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.375rem` |
| `height` | `1.375rem` |
| `border-radius` | `999px` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 78%, transparent)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `700` |
| `line-height` | `1` |

Pending tone uses the shared [`Spinner`](./spinner.md) primitive with
`variant="ring"`, `size="sm"`, and `tone="accent"` inside the icon badge when
no icon slot override is provided.

### Content `.callout__content`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.25rem` |
| `min-width` | `0` |

### Content — `p` (global within content)

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.5` |

### Content — `strong`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `line-height` | `var(--poodle-typography-label-lineHeight)` |

### Actions `.callout__actions`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `align-items` | `center` |
| `justify-content` | `flex-end` |

### Dismiss `.callout__dismiss`

| Property | Value |
|----------|-------|
| `width` | `1.75rem` |
| `height` | `1.75rem` |
| `min-height` | `0` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.0625rem)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |
| `margin-right` | `calc(-0.5 * var(--poodle-space-panel-x))` |
| `font` | `inherit` |

### Dismiss — focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Size adjustments

| Size | icon badge size | title font-size | message font-size | dismiss button size |
|------|----------------|----------------|-------------------|-------------------|
| `xs` | `1rem` | `0.6875rem` | `0.6875rem` | `1.25rem` |
| `sm` | `1.1875rem` | `0.75rem` | `0.75rem` | `1.5rem` |
| `md` | `1.375rem` | `typography-label-size` | `0.8125rem` | `1.75rem` |
| `lg` | `1.5625rem` | `0.9375rem` | `0.875rem` | `2rem` |
| `xl` | `1.75rem` | `1rem` | `0.9375rem` | `2.25rem` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- Root element is a `<section>` with optional `aria-label`, `role`, and `aria-live`
- Icon slot allows overriding the default tone-based indicator
- Default indicator is rendered based on tone mapping when no icon slot content
  is provided; pending uses the shared spinner primitive instead of a loader icon
- `data-tone` attribute on root for CSS tone targeting
- Custom properties `--poodle-callout-fill` and `--poodle-callout-border` are set
  per tone variant
- Content slot projects into `.callout__content` after the optional title
- Actions slot and dismiss button conditionally rendered
- Consolidates former Banner component — all Banner consumers should migrate
  to Callout

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::callout`
- When announceMode is assertive, expose as an alert; otherwise keep
  informational and non-announcing
- Icon badge uses circular border-radius (999px)
- Custom property pattern for fill/border can be flattened in GPUI to
  direct color assignments per tone variant
- color-mix values must be replicated using GPUI's color blending or
  equivalent
- GPUI still has separate BannerSpec and CallOutSpec — consolidation
  can follow in a future pass

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all six tone values produce distinct visual treatments
- [ ] announceMode controls ARIA live-region behavior
- [ ] dismissible state shows/hides close button
- [ ] dismiss event fires on close button click
- [ ] aria-label passthrough matches
- [ ] default indicator mapping per tone matches
- [ ] icon slot override behavior matches
- [ ] actions slot renders action buttons

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] root 3-column grid layout matches (1fr auto auto)
- [ ] neutral tone uses custom property defaults (94% panel, 88% border)
- [ ] info/success/warning/danger tones use 10%/34% color-mix pattern
- [ ] pending tone uses 8%/26% color-mix pattern (distinct from others)
- [ ] icon badge size (1.375rem), circular radius (999px), and background match
- [ ] icon typography matches (code-family, 0.75rem, weight 700)
- [ ] content gap (0.25rem) matches
- [ ] title typography uses label token family/size/lineHeight
- [ ] paragraph color uses text-secondary
- [ ] responsive collapse at 45rem breakpoint

### Tier 3: Implementation Freedom

- [ ] icon rendering mechanism is platform-owned
- [ ] slot projection mechanism is platform-owned
- [ ] custom property vs direct color assignment is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Color-mix blending | GPUI may approximate color-mix differently | allowed | visual result must be comparable |
| Custom property pattern | GPUI may use direct values instead of CSS custom properties | allowed | final computed colors must match |
| GPUI still has separate Banner/CallOut | consolidation deferred | allowed | align in future GPUI pass |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Tones

Five callouts stacked vertically, one per tone with body content:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Neutral callout | `tone="neutral"`, `title="Neutral callout"`, body slot content | Subtle panel background and border with info icon badge, bold title, secondary-color body text |
| Info | `tone="info"`, `title="Info"`, body slot content | Info-tinted fill and border with info icon badge |
| Success | `tone="success"`, `title="Success"`, body slot content | Success-tinted fill and border with check icon badge |
| Warning | `tone="warning"`, `title="Warning"`, body slot content | Warning-tinted fill and border with triangle-alert icon badge |
| Error | `tone="danger"`, `title="Error"`, body slot content | Danger-tinted fill and border with circle-x icon badge |
| Pending | `tone="pending"`, `title="Pending"`, body slot content | Accent-tinted fill and border with shared ring spinner in the icon badge |

### Message prop

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Information | `tone="info"`, `title="Information"`, `message="This is an informational callout using the message prop instead of slot content."` | Info-tinted callout with title and message rendered as paragraph text |

### Dismissible

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Dismissible callout | `tone="info"`, `title="Dismissible callout"`, `message="This callout can be dismissed by the user."`, `dismissible=true` | Info-tinted callout with a visible dismiss (close) button |

### Without title

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| (no title) | `tone="info"`, body slot content only | Info-tinted callout with icon and body text but no bold title |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: contextual messaging in forms, inspectors, cards,
  settings sections, dialogs, command palettes
- migration note: former Banner consumers should use Callout with
  `announceMode`, `dismissible`, and `actions` slot as needed

> **Surface elevation**: Callout is a surface creator — see [surface-elevation.md](./surface-elevation.md).
