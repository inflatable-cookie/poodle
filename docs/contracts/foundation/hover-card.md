# HoverCard

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `HoverCard`
- Layer: `foundation`
- Summary: a hover-triggered overlay that displays supplementary content
  anchored to a trigger element, with separate open and close delay timers
- In scope: delayed open and close behavior, anchored preview surface, trigger
  and surface hover continuity, controlled/uncontrolled open state, placement
  positioning, dialog semantics on surface, keyboard dismissal
- Out of scope: click-to-open (use Popover), nested hover cards, arrow
  indicators, viewport collision detection, command menus

## 2. Anatomy

```text
[Root .hover-card]  <span>
  ├── [Trigger .hover-card__trigger]  <span>  role="button"  tabindex="0"
  │     └── {trigger slot}
  └── [Surface .hover-card__surface]  <span>  role="dialog"  tabindex="-1"  (when open)
        └── {default slot}
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | inline-flex span, position context | `position: relative` |
| Trigger | yes | hover/focus target, inline-flex span | focus ring, aria-expanded, aria-controls |
| Surface | conditional | overlay panel, shown when open | border, background, elevation, padding, z-index |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial open state |
| `openDelayMs` | `number` | `180` | no | milliseconds before opening on hover |
| `closeDelayMs` | `number` | `120` | no | milliseconds before closing on leave |
| `placement` | `OverlayPlacement` | `"top"` | no | surface positioning relative to trigger |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the surface dialog |

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
| trigger | trigger element content |
| default | surface preview content |

### Controlled And Uncontrolled

- controlled: `open` prop plus `openChange` event
- uncontrolled: `defaultOpen` — internal state tracks visibility
- module-level `nextHoverCardId` counter for unique IDs across instances
- separate `openTimer` and `closeTimer` managed internally, cleared on component destroy

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default / hover leave after delay | surface hidden |
| pending-open | hover/focus starts open delay timer | no surface yet |
| open | open delay elapsed | surface visible and anchored |
| pending-close | hover/focus leaves trigger | close delay timer running; re-entering trigger or surface cancels timer |
| trigger-focus | keyboard focus on trigger | focus ring on trigger |

### Component States

- Open state: controlled or uncontrolled boolean tracking
- Delay timers: `openTimer` and `closeTimer` — entering trigger starts open timer, leaving trigger starts close timer
- Surface hover: mouseenter on surface cancels close timer, mouseleave schedules close
- Hover entering the surface must cancel the close timer to maintain continuity

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `openChange` | open state changes | `{ open: boolean }` | fires on hover intent, focus, escape, or leave |

## 6. Accessibility

### Semantics

- Root: no role (span container)
- Trigger: `role="button"`, `tabindex="0"`, `aria-expanded` (boolean), `aria-controls` (links to surface id)
- Surface: `role="dialog"`, `tabindex="-1"`, `aria-label` from prop

### Keyboard

| Key | Behavior |
|-----|----------|
| `Escape` | clears all timers and closes the surface |
| `Tab` | natural focus movement; trigger is focusable |

### Focus And Announcement

- Trigger receives focus via `tabindex="0"`
- Surface has `tabindex="-1"` for programmatic focus if needed
- Opening on trigger focus shows surface; blur schedules close
- live-region behavior: none; content semantics carry the meaning
- GPUI-native accessibility mapping notes: GPUI must expose the hover card as
  an accessible dialog-like preview surface with trigger relationship

## 7. Layout

### Sizing

- Root: `display: inline-flex`, `position: relative`
- Trigger: `display: inline-flex`
- Surface: absolutely positioned relative to root, placement-dependent offsets
- Surface min-width: `14rem`, max-width: `min(22rem, 90vw)`

### Placement Positioning

- `top`: bottom: calc(100% + 0.5rem), left: 50%, transform: translateX(-50%)
- `bottom`: top: calc(100% + 0.5rem), left: 50%, transform: translateX(-50%)
- `left`: right: calc(100% + 0.5rem), top: 50%, transform: translateY(-50%)
- `right`: left: calc(100% + 0.5rem), top: 50%, transform: translateY(-50%)
- `*-start` suffix: left: 0, right: auto, transform: none
- `*-end` suffix: right: 0, left: auto, transform: none

### Composition

- parent expectations: inline content, data cells, labels needing supplementary detail
- child expectations: surface receives arbitrary content via default slot
- trigger content provided via named `trigger` slot
- resizing rules: placement collision handling should preserve reachability

## 8. Token Usage — Exact Values

### Root `.hover-card`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |

### Trigger `.hover-card__trigger`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |

### Trigger — Focus (`:focus-visible`)

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Surface `.hover-card__surface`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `z-index` | `var(--pug-overlay-z-menu)` |
| `min-width` | `14rem` |
| `max-width` | `min(22rem, 90vw)` |
| `padding` | `var(--pug-space-panel-y) var(--pug-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--pug-radius-surface)` |
| `background` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| `--pug-surface` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| `box-shadow` | `var(--pug-elevation-overlay)` |

### Surface — Placement `top`

| Property | Value |
|----------|-------|
| `bottom` | `calc(100% + 0.5rem)` |
| `left` | `50%` |
| `transform` | `translateX(-50%)` |

### Surface — Placement `bottom`

| Property | Value |
|----------|-------|
| `top` | `calc(100% + 0.5rem)` |
| `left` | `50%` |
| `transform` | `translateX(-50%)` |

### Surface — Placement `left`

| Property | Value |
|----------|-------|
| `right` | `calc(100% + 0.5rem)` |
| `top` | `50%` |
| `transform` | `translateY(-50%)` |

### Surface — Placement `right`

| Property | Value |
|----------|-------|
| `left` | `calc(100% + 0.5rem)` |
| `top` | `50%` |
| `transform` | `translateY(-50%)` |

### Surface — Placement `*-start` suffix

| Property | Value |
|----------|-------|
| `left` | `0` |
| `right` | `auto` |
| `transform` | `none` |

### Surface — Placement `*-end` suffix

| Property | Value |
|----------|-------|
| `right` | `0` |
| `left` | `auto` |
| `transform` | `none` |

### Data Attributes

| Attribute | Source |
|-----------|--------|
| `data-placement` | resolved placement value |

## 9. Svelte Notes

- Root and trigger and surface are `<span>` elements (not `<div>`) for inline context
- Uses `data-placement` data attribute for positioning styles
- Module-level `nextHoverCardId` counter for unique IDs across instances
- Separate `openTimer` and `closeTimer` cleared via `clearTimers()` helper
- Timers cleared on `onDestroy`
- Surface mouseenter cancels close timer; mouseleave schedules close
- Escape keydown on trigger or surface calls `clearTimers()` then `setOpen(false)`
- Slots: named `trigger` slot, default slot for surface content
- Surface uses same placement rules as Tooltip (0.5rem offset with centering)
- Max-width is 22rem (narrower than Popover's 24rem)

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::hover_card`
- Spec struct: `HoverCardSpec` in primitives crate holds placement + delay config
- Component struct: `PugHoverCard` in components crate renders via `IntoElement`
- GPUI must model `color-mix` as `token.opacity(token.a * multiplier)` since GPUI has no CSS color-mix
- Border opacity: 72% on border-default
- Background: 98% elevated mixed with panel (approximate as 0.98 multiplier on elevated)
- Delay timers map to GPUI async tasks with cancellation
- Placement offset gap is 0.5rem (same as Tooltip)
- GPUI must expose the hover card as an accessible dialog-like preview surface

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] controlled/uncontrolled open state resolution matches
- [ ] open/close delay timing matches (180ms / 120ms defaults)
- [ ] surface mouseenter cancels close, mouseleave schedules close
- [ ] Escape key clears timers and closes
- [ ] dialog role on surface with tabindex="-1" matches
- [ ] trigger role="button" with tabindex="0" matches
- [ ] aria-expanded on trigger matches
- [ ] aria-controls on trigger links to surface id
- [ ] trigger and preview reachability match

### Tier 2: Visual Parity

- [ ] surface border color-mix 72% on border-default matches
- [ ] surface background color-mix 98% elevated with panel matches
- [ ] surface elevation shadow matches
- [ ] surface padding matches panel tokens (panel-y / panel-x)
- [ ] placement positioning offsets match (0.5rem gap)
- [ ] focus ring style matches (border-width-focus, focusRing color, 0.125rem offset)
- [ ] surface min-width 14rem and max-width min(22rem, 90vw) match
- [ ] surface border-radius uses radius-surface token
- [ ] surface z-index uses overlay-z-menu token

### Tier 3: Implementation Freedom

- [ ] exact delay timing implementation stays internal
- [ ] centering transform strategy stays internal
- [ ] hover event handling mechanism stays internal
- [ ] ID generation scheme is implementation-owned
- [ ] animation/transition on open/close is implementation-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact delay timing may differ slightly | runtime timer behavior differs | allowed | keep open and dismiss meaning strict |
| GPUI uses opacity multiplication instead of CSS color-mix | platform capability | allowed | visual result must match |
| GPUI timer cancellation uses async tasks instead of clearTimeout | platform capability | allowed | delay behavior must match |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: identity previews, asset summaries, compact profile
  surfaces, data table cells, link previews
- future follow-up: viewport collision detection, arrow indicator, animation,
  keep click-owned or action surfaces in Popover/Menu

> **Surface elevation**: HoverCard is a surface creator — see [surface-elevation.md](./surface-elevation.md).
