# HoverCard

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `HoverCard`
- Layer: `foundation`
- Summary: a hover-triggered overlay that displays supplementary content
  anchored to a trigger element, with separate open and close delay timers
- In scope: delayed open and close behavior, anchored preview surface, trigger
  and surface hover continuity, controlled/uncontrolled open state, placement
  positioning, dialog semantics on surface, keyboard dismissal
- Out of scope: click-to-open (use Popover), nested hover cards, arrow
  indicators, command menus

## 2. Anatomy

```text
[Root .hover-card]  <span>
  ├── [Trigger .hover-card__trigger]  <span>  role="button"  tabindex="0"
  │     └── {trigger snippet}
  └── [Surface .hover-card__surface]  <span>  role="dialog"  tabindex="-1"  (when open)
        └── {children snippet}
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
| `onOpenChange` | `(open: boolean) => void` | `undefined` | no | called when the open state changes |

### Type Definitions

```
OverlayPlacement:
  "top" | "top-start" | "top-end" |
  "bottom" | "bottom-start" | "bottom-end" |
  "left" | "left-start" | "left-end" |
  "right" | "right-start" | "right-end"
```

### Snippets

| Snippet | Purpose |
|------|---------|
| `trigger` | trigger element content |
| `children` | surface preview content |

### Controlled And Uncontrolled

- controlled: `open` prop plus `onOpenChange(open)`
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

### Behavior Machine

Behavior classification: machine-backed (`hoverTransition` in
`@poodle/headless`)

Hover-intent machine shared by Tooltip and HoverCard.

- States: `closed` | `opening` (open delay pending) | `open` | `closing`
  (close delay pending)
- Context: `openDelayMs`, `closeDelayMs`; open state controllable
- Events: `ENTER` (pointer enter / focus in), `LEAVE` (pointer leave /
  focus out), `TIMER_FIRE`, `DISMISS` (Escape), `SET_OPEN` (programmatic)
- Transitions: `ENTER` from closed/opening starts the open timer; from
  open/closing it cancels a pending close and stays open. `LEAVE` with a
  zero close delay closes immediately; otherwise enters `closing` with the
  close timer. `TIMER_FIRE` resolves the pending state; stale fires are
  inert. `DISMISS` closes immediately from any non-closed state.
- Effects: `startTimer(ms)` / `clearTimer` (adapter owns the timer handle),
  `emitOpenChange(open)`. Immediate closes emit `emitOpenChange(false)` even
  if the surface never became visible, matching pre-machine behavior.
- Machinery dependencies: adapter timer; anchor positioning via the core
  `resolveOverlayPosition` machinery. Delta: near viewport edges the
  surface now flips to the opposite side instead of clamp-only (strictly
  avoids covering the trigger).

HoverCard uses both delays (defaults 180/120ms). Delta from pre-machine
behavior: `LEAVE` while fully closed is now inert instead of scheduling a
redundant close callback.

## 5. Callbacks

| Prop | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | open state changes | `boolean` | fires on hover intent, focus, escape, or leave |

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
- Surface: portalled to the theme root and positioned in viewport coordinates
  (`002-anchored-overlays.md`)
- Surface min-width: `14rem`, max-width: `min(22rem, 90vw)`

### Placement Positioning

Position is calculated in JS using the trigger's viewport coordinates:
- `top`: centered above trigger with 8px gap
- `bottom`: centered below trigger with 8px gap
- `left`: centered left of trigger with 8px gap
- `right`: centered right of trigger with 8px gap
- `*-start` suffix: aligned to trigger's start edge
- `*-end` suffix: aligned to trigger's end edge

### Viewport Clamping

After initial placement, the surface is clamped to stay within 8px of all
viewport edges. This prevents cards from being clipped when triggers are near
screen boundaries or inside scrollable containers.

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
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Surface `.hover-card__surface`

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `min-width` | `14rem` |
| `max-width` | `min(22rem, 90vw)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))` |
| `--poodle-surface` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |

### Surface Positioning

Placement offsets (8px gap) and viewport clamping (8px padding) are computed
in JS after render. The `left` and `top` properties are set via inline style.
No `data-placement` attribute is emitted — positioning is entirely JS-driven.

## 9. Svelte Notes

- Root and trigger and surface are `<span>` elements (not `<div>`) for inline context
- Surface is portalled out of the trigger's subtree and positioned in viewport
  coordinates by the shared anchored-overlay primitive, which also repositions
  it on scroll and resize and hides it when the trigger scrolls out of view
- Module-level `nextHoverCardId` counter for unique IDs across instances
- Separate `openTimer` and `closeTimer` cleared via `clearTimers()` helper
- Timers cleared on `onDestroy`
- Surface mouseenter cancels close timer; mouseleave schedules close
- Escape keydown on trigger or surface calls `clearTimers()` then `setOpen(false)`
- Snippets: named `trigger` snippet, `children` snippet for surface content
- Surface uses fixed positioning with 8px gap from trigger and 8px viewport padding
- Max-width is 22rem (narrower than Popover's 24rem)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::hover_card`
- Spec struct: `HoverCardSpec` in primitives crate holds placement + delay config
- Component struct: `PoodleHoverCard` in components crate renders via `IntoElement`
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

## 13. Specimen Definitions

### Group: Default (top placement)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| User preview hover card | `<HoverCard ariaLabel="User preview">` with inline trigger text ("@clay") and content containing a name heading and bio paragraph | Underlined accent-colored trigger text; hovering (after 180ms delay) opens an elevated surface above the trigger showing a name and description; leaving trigger/surface closes after 120ms delay |

### Group: Bottom placement

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Repository info hover card | `<HoverCard placement="bottom" ariaLabel="Repository info">` with inline trigger text ("poodle/svelte-primitives") and content containing a repo name and stats | Underlined accent-colored trigger text; hovering opens an elevated surface below the trigger showing repository name and summary; surface has dialog role with aria-label |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: identity previews, asset summaries, compact profile
  surfaces, data table cells, link previews
- future follow-up: arrow indicator, animation, keep click-owned or action
  surfaces in Popover/Menu

> **Surface elevation**: HoverCard is a surface creator — see [surface-elevation.md](./surface-elevation.md).
