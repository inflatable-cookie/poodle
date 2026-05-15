# Popover

Status: detailed contract
Updated: 2026-03-15

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
| `trigger` | trigger element |
| `children` | popover body content |

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

## 5. Callbacks

| Prop | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | popover opens or closes | `boolean` | trigger, outside interact, or escape driven |

## 6. Accessibility

### Semantics

- Trigger: `role="button"`, `tabindex="0"`, `aria-expanded` (true/false), `aria-controls` (surface id when open)
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

- surface min-width: 14rem, max-width: min(24rem, 90vw)
- content sizes to intrinsic needs within these constraints
- anchored width may optionally match the trigger when the use case requires it

### Composition

- parent expectations: field helpers, compact settings panels, shell affordances
- child expectations: informational or interactive content blocks
- resizing rules: placement collision handling should preserve reachability

## 8. Token Usage — Exact Values

### CSS Custom Properties

| Var | Purpose |
|-----|---------|
| `--poodle-popover-offset` | set from `offset` prop (default produces `0.5rem` equivalent at 8px) |

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

### Surface (.popover__surface)

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `z-index` | `var(--poodle-overlay-z-menu)` |
| `min-width` | `14rem` |
| `max-width` | `min(24rem, 90vw)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))` |
| `--poodle-surface` | `color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |

### Placement rules — position offsets by placement value

| Placement prefix | Properties |
|------------------|------------|
| `bottom-*` | `top: calc(100% + var(--poodle-popover-offset))`, `left: 0` |
| `top-*` | `bottom: calc(100% + var(--poodle-popover-offset))`, `left: 0` |
| `right-*` | `top: 0`, `left: calc(100% + var(--poodle-popover-offset))` |
| `left-*` | `top: 0`, `right: calc(100% + var(--poodle-popover-offset))` |

### Placement alignment — end modifier

| Modifier | Properties |
|----------|------------|
| `*-end` | `left: auto`, `right: 0` |

### Data Attributes

| Attribute | Source |
|-----------|--------|
| `data-placement` | resolved placement value |

## 9. Svelte Notes

- may compose headless popover primitives, but the contract owns dismissal and
  focus-restoration semantics
- if the content traps focus, the component should likely be `Dialog` or
  `Drawer` instead
- `--poodle-popover-offset` CSS custom property is set from the `offset` prop,
  enabling placement rules to reference it in `calc()` expressions
- surface uses `position: absolute` relative to the root's `position: relative`

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::popover`
- GPUI implementation must intentionally model anchored overlay behavior,
  outside-dismiss rules, and non-modal focus flow through native window or view
  constructs
- surface sizing constraints must match: min-width 14rem, max-width
  min(24rem, 90vw)
- border uses 72% opacity color-mix for border-default

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] trigger/content relationship and labeling match
- [ ] open, close, outside-dismiss, and escape behavior match
- [ ] focus handoff and restoration match
- [ ] initialFocus strategy (first-focusable vs content) matches

### Tier 2: Visual Parity

- [ ] surface min-width 14rem, max-width min(24rem, 90vw)
- [ ] border: 0.0625rem solid with 72% opacity border color
- [ ] background: 98% elevated mixed with panel
- [ ] border-radius: radius-surface token
- [ ] box-shadow: elevation-overlay
- [ ] padding: panel-y / panel-x
- [ ] trigger focus ring matches (focus width, focusRing color, 0.125rem offset)
- [ ] placement offset uses --poodle-popover-offset custom property

### Tier 3: Implementation Freedom

- [ ] exact collision engine and rendering strategy stay internal
- [ ] CSS custom property vs GPUI prop access stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact placement fallback order may differ | overlay engine internals vary | allowed | keep trigger relation, dismissal, and focus rules strict |
| color-mix transparency blending | GPUI may use direct alpha blending instead of CSS color-mix | allowed | same visual result required |

## 13. Specimen Definitions

### Group: Default (bottom-start)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Bottom-start popover | `<Popover ariaLabel="Quick settings">` with a secondary Button trigger ("Open popover") and content containing a heading and paragraph | Button trigger; clicking opens an elevated surface anchored below-start of trigger with heading and descriptive text; dismisses on outside click or Escape |

### Group: Top placement

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Top popover | `<Popover placement="top" ariaLabel="Help tip">` with a secondary Button trigger ("Show help") and paragraph content | Button trigger; clicking opens an elevated surface anchored above the trigger with descriptive text; offset matches --poodle-popover-offset |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: anchored helpers, compact inspector surfaces, shell
  affordances
- future follow-up: connect richer picker and command-surface composites in
  later milestones

> **Surface elevation**: Popover is a surface creator — see [surface-elevation.md](./surface-elevation.md).
