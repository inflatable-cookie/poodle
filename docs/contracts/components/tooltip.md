# Tooltip

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Tooltip`
- Layer: `foundation`
- Summary: a brief non-interactive descriptive overlay tied to a trigger or
  subject element
- In scope: delayed open on hover/focus, descriptive content, placement,
  dismissal, accessible description relationship via aria-describedby
- Out of scope: interactive content, menus, teaching bubbles, persistent help
  panels

## 2. Anatomy

```text
[Root .tooltip]  <span>  role="presentation"
  ├── [Trigger .tooltip__trigger]  <span>  role="button" tabindex="0"
  └── [Bubble .tooltip__bubble]  <span> (conditional, role="tooltip")
        └── [Content text]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | relative positioning host and state owner | position context |
| Trigger | yes | element that owns the tooltip | focus ring |
| Bubble | conditional | floating descriptive shell | surface background, border, elevation, typography |
| Content | yes | short descriptive text | text color, font size |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `content` | `string` | — | yes | tooltip text |
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `delayMs` | `number` | `300` | no | open delay in milliseconds |
| `placement` | `OverlayPlacement` | `"top"` | no | placement hint |

### Type Definitions

```
OverlayPlacement:
  "top" | "top-start" | "top-end" |
  "bottom" | "bottom-start" | "bottom-end" |
  "left" | "left-start" | "left-end" |
  "right" | "right-start" | "right-end"
```

### Slots

| Slot | Purpose |
|------|---------|
| default | trigger element |

### Controlled And Uncontrolled

- controlled: `open` plus `openChange` event
- uncontrolled: `defaultOpen`
- the trigger or subject owns the tooltip relationship and focus context

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | bubble hidden |
| pending | hover or focus delay active | no bubble yet |
| open | delay elapsed or forced open | bubble visible |

### Component States

Closed, pending-open, and open states are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `openChange` | tooltip opens or closes | `{ open: boolean }` | hover, focus, escape, or leave driven |

## 6. Accessibility

### Semantics

- Role: root has `role="presentation"`; trigger has `role="button"` and `tabindex="0"`; bubble has `role="tooltip"`
- Required attributes: trigger has `aria-describedby` pointing to the tooltip
  bubble's `id` (set when tooltip is open); tooltip content supplements a trigger's accessible name
- Optional attributes: none in the baseline contract
- Labeling rules: tooltip content supplements a trigger; it does not replace a
  missing accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| focus navigation | reveals the tooltip when the trigger receives focus |
| `Escape` | dismisses the visible tooltip |

### Focus And Announcement

- focus entry: tooltip itself is not focusable
- focus exit: tooltip dismisses when the trigger loses the relevant focus or
  hover state
- live-region behavior: none; tooltip content should be exposed through the
  `aria-describedby` relationship rather than an unsolicited announcement
- GPUI-native accessibility mapping notes: GPUI must expose tooltip text as
  native help or descriptive metadata where available; it may not create a
  focusable floating window for simple help text

## 7. Layout

### Sizing

- bubble max-width: 16rem
- bubble uses white-space: nowrap for single-line content
- placement adjusts to viewport constraints

### Composition

- parent expectations: icon buttons, dense controls, shell affordances
- child expectations: short non-interactive content only
- resizing rules: tooltip should avoid obscuring the trigger when reasonable

## 8. Token Usage — Exact Values

### Root (.tooltip) — base styles

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |

### Trigger (.tooltip__trigger)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |

### Trigger focus-visible — .tooltip__trigger:focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Bubble (.tooltip__bubble)

| Property | Value |
|----------|-------|
| `position` | `absolute` |
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

### Placement rules — position offsets (0.5rem gap)

| Placement | Properties |
|-----------|------------|
| `top` | `bottom: calc(100% + 0.5rem)`, `left: 50%`, `transform: translateX(-50%)` |
| `bottom` | `top: calc(100% + 0.5rem)`, `left: 50%`, `transform: translateX(-50%)` |
| `left` | `top: 50%`, `right: calc(100% + 0.5rem)`, `transform: translateY(-50%)` |
| `right` | `top: 50%`, `left: calc(100% + 0.5rem)`, `transform: translateY(-50%)` |

### Placement alignment modifiers

| Modifier | Properties |
|----------|------------|
| `*-start` | `left: 0`, `right: auto`, `transform: none` |
| `*-end` | `right: 0`, `left: auto`, `transform: none` |

### Data Attributes

| Attribute | Source |
|-----------|--------|
| `data-placement` | resolved placement value |

## 9. Svelte Notes

- should use tooltip semantics (`role="tooltip"` and `aria-describedby` wiring)
  rather than inventing a menu-like overlay
- interactive content should escalate to `Popover`
- bubble uses a fixed 0.5rem offset gap for all placements
- border-radius is derived from control radius minus 0.125rem for a tighter
  appearance than surfaces
- font-size is 0.6875rem (11px) for compact descriptive text

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::tooltip`
- GPUI implementation must choose native help-text exposure and dismissal
  behavior that preserves non-focusable descriptive semantics
- bubble sizing: max-width 16rem, padding 0.375rem 0.5rem
- border-radius must use the reduced calculation:
  control-radius minus 0.125rem
- typography: 0.6875rem font-size, 1.35 line-height

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] descriptive relationship (aria-describedby) and non-interactive semantics
  match
- [ ] role="tooltip" on bubble element
- [ ] open and dismiss behavior for hover, focus, and escape match
- [ ] tooltip content does not replace the trigger's accessible name
- [ ] delay of 300ms default matches

### Tier 2: Visual Parity

- [ ] bubble padding: 0.375rem 0.5rem
- [ ] border: 0.0625rem solid with 72% opacity border color
- [ ] border-radius: calc(control-radius - 0.125rem)
- [ ] background: 98% elevated mixed with panel
- [ ] box-shadow: elevation-overlay
- [ ] font-size: 0.6875rem, line-height: 1.35
- [ ] color: text-primary
- [ ] white-space: nowrap
- [ ] max-width: 16rem
- [ ] placement offset gap: 0.5rem
- [ ] trigger focus ring matches (focus width, focusRing color, 0.125rem offset)

### Tier 3: Implementation Freedom

- [ ] exact delay timers and animation internals stay internal
- [ ] centering transform strategy stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| precise hover timing may vary slightly | platform event timing differs | allowed | keep descriptive semantics and dismissal strict |
| color-mix transparency blending | GPUI may use direct alpha blending instead of CSS color-mix | allowed | same visual result required |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Default

One tooltip with default placement:

| Trigger | Content | Placement |
|---------|---------|-----------|
| "Hover me" button (secondary variant) | Save your changes | top (default) |

### Placements

Four tooltips showing all cardinal placements, arranged in a 2×2 grid:

| Trigger | Content | Placement |
|---------|---------|-----------|
| "Top" button | Top tooltip | top |
| "Bottom" button | Bottom tooltip | bottom |
| "Left" button | Left tooltip | left |
| "Right" button | Right tooltip | right |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: icon buttons, dense shell actions, compact indicators
- future follow-up: define richer hovercard/help-panel patterns separately if
  product need appears
