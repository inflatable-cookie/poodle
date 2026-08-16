# Popover

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `Popover`
- Layer: `foundation`
- Summary: an anchored non-modal overlay for contextual interactive or rich
  informational content
- In scope: trigger/content relationship, anchored placement, outside dismissal,
  optional initial focus, placement via CSS custom property
- Out of scope: modal flows, menu-specific item semantics, long-lived pinned
  panels

## 2. Anatomy

```text
[Root .popover]  <div>
  ├── [Trigger .popover__trigger]  <div>
  └── [Surface .popover__surface]  <div> (conditional)
        └── [Content] (children snippet)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | relative positioning host and state owner | position context |
| Trigger | yes | invokes the popover | focus ring |
| Surface | conditional | anchored floating shell | surface background, border, radius, elevation |
| Content | yes | informational or interactive content | caller-owned |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `placement` | `OverlayPlacement` | `"bottom-start"` | no | placement hint |
| `offset` | `number` | `8` | no | trigger gap in pixels, set as CSS custom property |
| `dismissOnOutsideInteract` | `boolean` | `true` | no | outside dismissal |
| `initialFocus` | `"first-focusable" \| "content" \| "none"` | `"first-focusable"` | no | initial focus strategy |
| `ariaLabel` | `string \| null` | `null` | no | optional label when no internal heading exists |
| `block` | `boolean` | `false` | no | makes the trigger and root expand to available width |
| `disabled` | `boolean` | `false` | no | disables the trigger — blocks `setOpen`, sets `data-disabled`/`aria-disabled="true"`, `tabindex=-1`, and `cursor: not-allowed` |
| `surfaceWidth` | `"content" \| "trigger"` | `"content"` | no | surface width strategy; `"trigger"` makes the surface `width: 100%` / `min-width: 100%` of the trigger (emits `data-surface-width`) |
| `surfaceMinWidth` | `string \| null` | `null` | no | overrides `--poodle-popover-surface-min-width` (default `14rem`); the portable value is a rem length — see §12 |
| `surfaceMaxWidth` | `string \| null` | `null` | no | overrides `--poodle-popover-surface-max-width` (default `min(24rem, 90vw)`); the portable value is a rem length — see §12 |
| `onOpenChange` | `(open: boolean) => void` | `undefined` | no | called when the open state changes |
| `onSurfaceGeometryChange` | `(change: OverlaySurfaceGeometryChange) => void` | `undefined` | no | web-only, host-neutral viewport snapshot stream for the private built-in surface |

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
| `trigger` | trigger element |
| `children` | popover body content |

Web adapters also accept `triggerIsInteractive`. Set it when `trigger` is
already an interactive control such as `IconButton`; the wrapper then observes
clicks without adding a second button role or keyboard handler. Native adapters
compose their trigger directly and do not need this DOM-only switch.

### Controlled And Uncontrolled

- controlled: `open` plus `onOpenChange(open)`
- uncontrolled: `defaultOpen`
- popover content state remains external to the primitive

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | surface hidden |
| open | open state true | surface visible and anchored |
| focus-within | content receives focus | visible focus context |

### Component States

Open/closed state and placement state are required.

### Behavior Machine

Behavior classification: machine-backed

Hard-case pilot: exercises the dismissable-layer, focus, and (future) anchor
positioning machinery.

#### Context

| Field | Type | Initial | Controllable | Meaning |
|-------|------|---------|--------------|---------|
| `open` | `boolean` | `defaultOpen` | yes | overlay visibility |
| `disabled` | `boolean` | `false` | input | trigger inert; open blocked in all directions |
| `dismissOnOutsideInteract` | `boolean` | `true` | input | guard for outside dismissal |
| `initialFocus` | `"first-focusable" \| "content" \| "none"` | `"first-focusable"` | input | focus strategy on open |

#### States

| State | Description |
|-------|-------------|
| `closed` | surface unmounted |
| `open` | surface mounted, anchored to trigger, registered on the dismissable layer |

#### Events

| Event | Payload | Source |
|-------|---------|--------|
| `TOGGLE` | — | trigger click, or Enter/Space on trigger |
| `OPEN` / `CLOSE` | — | programmatic |
| `ESCAPE` | — | keyboard, document-level while open |
| `OUTSIDE_INTERACT` | target | pointerdown outside root while open |

#### Transitions

| State | Event | Guard | Target | Actions / Effects |
|-------|-------|-------|--------|-------------------|
| `closed` | `TOGGLE` / `OPEN` | `!disabled` | `open` | `onOpenChange(true)`; effects `focusOnOpen`, `registerDismissLayer` |
| `open` | `TOGGLE` / `CLOSE` | `!disabled` | `closed` | `onOpenChange(false)`; effect `restoreTriggerFocus` |
| `open` | `ESCAPE` | — | `closed` | as close above (preventDefault on the key) |
| `open` | `OUTSIDE_INTERACT` | `dismissOnOutsideInteract` | `closed` | as close above |
| any | any open-direction event | `disabled` | unchanged | none |

Current-implementation notes the machine must preserve: focus returns to the
trigger on every close path (explicit, escape, and outside dismiss); escape
and outside listeners are document-level and active only while open.

#### Effects

| Effect | What It Does | Cleanup |
|--------|--------------|---------|
| `focusOnOpen` | after render: `first-focusable` focuses first focusable element in surface; `content` focuses the surface itself; `none` does nothing | none |
| `restoreTriggerFocus` | focuses the trigger element on close | none |
| `registerDismissLayer` | attaches document `mousedown` + `keydown(Escape)` handlers; in core this becomes registration on the shared dismissable-layer stack so nested overlays dismiss innermost-first | detach/unregister on close and on unmount |
| `positionSurface` | anchors the surface per `placement`/`offset` through the shared anchored-overlay primitive: portalled to the theme root, placed in viewport coordinates by the collision-aware resolver, repositioned on scroll and resize (`002-anchored-overlays.md`) | dispose positioning subscription on close |

#### Part Attribute Output

| Part | Attribute | Value |
|------|-----------|-------|
| root | `data-scope` / `data-part` | `popover` / `root` |
| root | `data-state` | `open` \| `closed` |
| root | `data-block` | `block` |
| trigger | `data-part` | `trigger` |
| trigger | `role` / `tabindex` | `"button"` / `0` (`-1` when `disabled`) |
| trigger | `aria-expanded` | `"true"` \| `"false"` |
| trigger | `aria-controls` | surface id while open |
| trigger | `aria-disabled` / `data-disabled` | `"true"` when `disabled` |
| trigger | `data-state` | `open` \| `closed` |
| surface | `data-part` / `id` | `surface` / generated instance id |
| surface | `role` | `"dialog"` |
| surface | `aria-label` | `ariaLabel` |
| surface | `tabindex` | `0` when `initialFocus="content"`, else `-1` |
| surface | `data-state` / `data-placement` / `data-surface-width` | `open` / resolved placement / resolved width strategy |

Note: `data-scope`/`data-part`/`data-state` are added during the core swap
(additive); existing attributes above match the current implementation.

#### Machinery Dependencies

Dismissable-layer stack, focus (focusable-element query + restore), anchor
positioning (core `resolveOverlayPosition` exists; Popover deliberately
keeps CSS anchoring as a documented delta — converting to JS positioning
is a behavior change deferred), id
wiring, presence (if open/close animation is added later).

## 5. Callbacks

| Prop | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | popover opens or closes | `boolean` | trigger, outside interact, or escape driven |
| `onSurfaceGeometryChange` | surface is positioned, moves, changes visibility, or unmounts | `OverlaySurfaceGeometryChange` | immutable CSS-viewport numbers; no element or selector exposure |

## 6. Accessibility

### Semantics

- Trigger: `role="button"`, `tabindex="0"` (`-1` when `disabled`), `aria-expanded` (true/false), `aria-controls` (surface id when open)
- Disabled trigger: `data-disabled="true"`, `aria-disabled="true"`, `tabindex=-1`; click/keydown are ignored and the popover cannot open
- Surface: `role="dialog"`, `tabindex` set to `0` when `initialFocus="content"` or `-1` otherwise
- Required attributes: trigger-to-content relationship via `aria-controls` and accessible naming
  when the content acts as a meaningful region
- Optional attributes: `aria-label` on surface, description relation and heading association
- Labeling rules: if content is interactive or long-lived, it must have a
  stable accessible label

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` or `Space` | opens from an interactive trigger when appropriate |
| `Escape` | closes the popover and restores focus to the trigger |
| `Tab` | moves through focusable content without trapping the user |

### Focus And Announcement

- focus entry: opening may move focus into the content according to
  `initialFocus`
- focus exit: non-modal popovers do not trap focus; leaving the content may
  dismiss according to implementation policy
- focus restoration: explicit close returns focus to the trigger
- live-region behavior: none by default; content semantics should carry the
  meaning
- GPUI-native accessibility mapping notes: GPUI must expose popover ownership,
  focus handoff, and restoration without confusing the popover with a modal
  window

## 7. Layout

### Sizing

- surface min-width: `var(--poodle-popover-surface-min-width, 14rem)`, max-width: `var(--poodle-popover-surface-max-width, min(24rem, 90vw))` — both overridable via the `surfaceMinWidth` / `surfaceMaxWidth` props
- content sizes to intrinsic needs within these constraints
- anchored width matches the trigger when `surfaceWidth="trigger"` (`width: 100%` / `min-width: 100%`); otherwise it is content-driven (`surfaceWidth="content"`, the default)

### Composition

- parent expectations: field helpers, compact settings panels, shell affordances
- child expectations: informational or interactive content blocks
- resizing rules: placement collision handling should preserve reachability

## 8. Token Usage — Exact Values

### CSS Custom Properties

| Var | Purpose |
|-----|---------|
| `--poodle-popover-surface-min-width` | set from `surfaceMinWidth` prop when provided; defaults to `14rem` |
| `--poodle-popover-surface-max-width` | set from `surfaceMaxWidth` prop when provided; defaults to `min(24rem, 90vw)` |

### Root (.popover) — base styles

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |

### Trigger (.popover__trigger)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |

### Trigger focus-visible — .popover__trigger:focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |
| `border-radius` | `var(--poodle-radius-control)` — without it the ring cornered against the rounded surface the trigger sits on |

### Trigger disabled — .popover__trigger[data-disabled="true"]

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |

### Surface (.popover__surface)

| Property | Value |
|----------|-------|
| `position` | `fixed` (from `anchored-surface.css`; `top` / `left` are written by the primitive) |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `min-width` | `var(--poodle-popover-surface-min-width, 14rem)` |
| `max-width` | `var(--poodle-popover-surface-max-width, min(24rem, 90vw))` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-recipe-popover-surface-fill, var(--poodle-color-background-elevated))` |
| `--poodle-surface` | `var(--poodle-color-background-elevated)` |
| `box-shadow` | `var(--poodle-recipe-popover-surface-shadow, inset highlight + two drop shadows)` |

### Surface — trigger width `.popover__surface[data-surface-width="trigger"]`

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `min-width` | `100%` |
| `box-sizing` | `border-box` |

### Placement

The surface carries no placement CSS. `placement` and `offset` are passed to
the anchored-overlay primitive, which measures the trigger and the surface,
picks the first candidate that clears the viewport, and writes `top` / `left`
in viewport coordinates. See `002-anchored-overlays.md`.

### Data Attributes

| Attribute | Source |
|-----------|--------|
| `data-placement` | placement in effect after collision resolution |
| `data-poodle-anchored` | `true` — the surface is portalled and viewport-positioned |
| `data-anchor-hidden` | `true` when the trigger has scrolled out of its clipping ancestor |
| `data-surface-width` | resolved `surfaceWidth` value (`content` / `trigger`) |
| `data-disabled` | `true` when `disabled` (on trigger) |
| `data-block` | `true` when `block` (on root and trigger) |

## 9. Svelte Notes

- may compose headless popover primitives, but the contract owns dismissal and
  focus-restoration semantics
- if the content traps focus, the component should likely be `Dialog` or
  `Drawer` instead
- `offset` is passed to the anchored-overlay primitive as a px gap; there is no
  `--poodle-popover-offset` custom property, because the surface no longer
  positions itself in CSS
- the surface is portalled out of the root and positioned in viewport
  coordinates (`002-anchored-overlays.md`), so no ancestor `overflow`,
  `transform` or stacking context can clip it
- `surfaceWidth="trigger"` is honoured by measuring the trigger, not by
  `width: 100%`
- `onSurfaceGeometryChange` reports the portalled surface after positioning,
  then removes its opaque surface id on close or destruction

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::popover`
- GPUI implementation must intentionally model anchored overlay behavior,
  outside-dismiss rules, and non-modal focus flow through native window or view
  constructs
- surface sizing constraints must match: min-width 14rem, max-width
  min(24rem, 90vw) (both overridable via `surfaceMinWidth`/`surfaceMaxWidth`)
- border uses a 74% opacity color-mix for `border-subtle`

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] trigger/content relationship and labeling match
- [ ] open, close, outside-dismiss, and escape behavior match
- [ ] focus handoff and restoration match
- [ ] initialFocus strategy (first-focusable vs content) matches

### Tier 2: Visual Parity

- [ ] surface min-width 14rem, max-width min(24rem, 90vw)
- [ ] border: 0.0625rem solid with `border-subtle` at 74%
- [ ] background: plain `background-elevated`
- [ ] border-radius: `radius-surface`
- [ ] box-shadow: 3-layer stack (inset highlight + two drop shadows)
- [ ] padding: panel-y / panel-x
- [ ] trigger focus ring matches (focus width, focusRing color, 0.125rem offset)
- [ ] `offset` is honoured as the gap between trigger and surface

### Tier 3: Implementation Freedom

- [ ] exact collision engine and rendering strategy stay internal
- [ ] CSS custom property vs GPUI prop access stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Jetstream raises no events | the component renders the panel; the trigger and the open state belong to the consumer, so there is nothing here to click | accepted (by design) | none |
| `onSurfaceGeometryChange` is Svelte/React-only | it reports CSS viewport geometry for a web surface; native hosts own their renderer geometry | accepted | do not copy browser geometry callbacks into native specs |
| exact placement fallback order may differ | overlay engine internals vary | allowed | keep trigger relation, dismissal, and focus rules strict |
| color-mix transparency blending | GPUI may use direct alpha blending instead of CSS color-mix | allowed | same visual result required |
| width bounds are rem lengths in the portable surface | the contract's `surfaceMinWidth`/`surfaceMaxWidth` accept any CSS length on the web shell, but the portable surface cannot interpret arbitrary CSS values; the portable subset is a rem length (`Nrem`) — `PopoverSpec::surface_min_width`/`surface_max_width` parse exactly that and an unsupported value is an authoring error, never a silent default — and arbitrary CSS lengths remain a web-shell extension beside the adapters | accepted by g14.005 (contract contradiction reported in the batch log) | keep the portable rem subset validated; revisit only with a portable CSS-length parser |

## 13. Specimen Definitions

### Group: Default (bottom-start)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Bottom-start popover | `<Popover ariaLabel="Quick settings">` with a secondary Button trigger ("Open popover") and content containing a heading and paragraph | Button trigger; clicking opens an elevated surface anchored below-start of trigger with heading and descriptive text; dismisses on outside click or Escape |

### Group: Top placement

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Top popover | `<Popover placement="top" ariaLabel="Help tip">` with a secondary Button trigger ("Show help") and paragraph content | Button trigger; clicking opens an elevated surface anchored above the trigger with descriptive text; the gap matches `offset` |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: anchored helpers, compact inspector surfaces, shell
  affordances
- future follow-up: connect richer picker and command-surface composites in
  later milestones

> **Surface elevation**: Popover is a surface creator — see [surface-elevation.md](./surface-elevation.md).
