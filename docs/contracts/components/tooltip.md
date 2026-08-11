# Tooltip

Status: detailed contract
Updated: 2026-07-10

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
[Root .tooltip]  <span>  role="presentation"  (display: contents)
  ├── [Anchor]  (the trigger element passed as children; first child is the anchor)
  └── [Bubble .tooltip__bubble]  <span> (conditional, role="tooltip")
        └── [Content text]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `display: contents` state owner; renders the trigger `children` inline with no positioning box of its own | — |
| Anchor | yes | the caller-supplied trigger element; the root's first child element is used as the positioning/`aria-describedby` anchor. No wrapper span is injected; the anchor must be focusable on its own (e.g. a `Button`) | focus ring (owned by the anchor element, not the tooltip) |
| Bubble | conditional | floating descriptive shell, positioned at viewport coordinates | surface background, border, elevation, typography |
| Content | yes | short descriptive text | text color, font size |

There is no `.tooltip__trigger` wrapper part and the tooltip does not inject
`role="button"` or `tabindex` onto the trigger — the first child element is
treated as the anchor and provides its own focusability.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `content` | `string` | — | yes | tooltip text |
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `delayMs` | `number` | `300` | no | open delay in milliseconds |
| `placement` | `OverlayPlacement` | `"top"` | no | placement hint |
| `onOpenChange` | `(open: boolean) => void` | `undefined` | no | called when the tooltip opens or closes |

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
| `children` | trigger element |

### Controlled And Uncontrolled

- controlled: `open` plus `onOpenChange(open)`
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

### Behavior Machine

Behavior classification: machine-backed (`hoverTransition` in
`@inflatable-cookie/poodle-core`)

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
  `resolveOverlayPosition` machinery (collision-aware flip + viewport
  clamp).

Tooltip uses `closeDelayMs = 0` (immediate close on leave/blur/Escape).

## 5. Callbacks

| Prop | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | tooltip opens or closes | `boolean` | hover, focus, escape, or leave driven |

## 6. Accessibility

### Semantics

- Role: root has `role="presentation"`; bubble has `role="tooltip"`. The trigger
  is the caller-supplied anchor element (no `role`/`tabindex` is injected by the
  tooltip); callers must pass an already-focusable trigger
- Required attributes: the anchor (root's first child element) receives
  `aria-describedby` pointing to the tooltip bubble's `id` when open, and the
  attribute is removed on close; tooltip content supplements a trigger's
  accessible name
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
| `display` | `contents` |

The root carries no positioning box; it renders the trigger inline. The bubble is
positioned at viewport coordinates (see Bubble below), not relative to the root.

### Trigger focus ring

The trigger is the caller-supplied anchor element and owns its own focus ring;
the tooltip does not style it. Anchors built from `Button`/icon-button controls
provide the standard control focus treatment
(`var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)`,
offset `0.125rem`).

### Bubble (.tooltip__bubble)

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `top` / `left` | JS-computed viewport pixel coordinates (see Placement below) |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `max-width` | `16rem` |
| `padding` | `0.375rem 0.5rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |
| `background` | `var(--poodle-recipe-tooltip-bubble-fill, color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel)))` |
| `box-shadow` | `var(--poodle-recipe-tooltip-bubble-shadow, 0 0.5rem 1.25rem rgba(0, 0, 0, 0.3), 0 0.125rem 0.375rem rgba(0, 0, 0, 0.2))` |
| `color` | `var(--poodle-recipe-tooltip-bubble-text, var(--poodle-color-text-primary))` |
| `font-size` | `0.6875rem` |
| `line-height` | `1.35` |
| `white-space` | `nowrap` |

Bubble fill and shadow resolve their component Recipe hooks first, then fall
back to the semantic-token forms above. Border and radius resolve directly
from semantic tokens. The box-shadow is a literal two-layer drop shadow, not
the `--poodle-elevation-overlay` token.

### Placement — JS-resolved viewport coordinates

The bubble is portalled to the theme root and positioned in viewport
coordinates by the shared anchored-overlay primitive
(`002-anchored-overlays.md`), which returns the placement that survived
collision resolution. There are no CSS `calc()`/`transform` offset rules; the
requested `placement` and its `-start` / `-end` alignment modifiers are inputs
to the resolver, which positions the bubble at a `0.5rem` gap from the trigger
and exposes the resolved value via `data-placement`.

### Data Attributes

| Attribute | Source |
|-----------|--------|
| `data-placement` | resolved placement value |

## 9. Svelte Notes

- should use tooltip semantics (`role="tooltip"` and `aria-describedby` wiring)
  rather than inventing a menu-like overlay
- root is `display: contents` (no positioning box); the trigger `children` render
  inline and the root's first child element is used as the anchor. No
  `.tooltip__trigger` wrapper is injected
- the bubble is portalled out of the trigger's subtree and positioned in
  viewport coordinates, so no ancestor `overflow`, `transform` or stacking
  context can clip it; the resolver may flip placement to fit the viewport and
  writes the resolved value to `data-placement`
- interactive content should escalate to `Popover`
- bubble uses a fixed 0.5rem offset gap for all placements
- fill and shadow resolve through `--poodle-recipe-tooltip-bubble-*` hooks;
  border and control radius minus 0.125rem resolve from semantic tokens
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
- [ ] background: 98% elevated mixed with panel (via `--poodle-recipe-tooltip-bubble-fill`, with the color-mix as fallback)
- [ ] box-shadow: two-layer drop shadow (`0 0.5rem 1.25rem rgba(0,0,0,0.3), 0 0.125rem 0.375rem rgba(0,0,0,0.2)`)
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
| Jetstream raises no events | the bubble is summoned by hover on a trigger the consumer owns | accepted (by design) | none |
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
