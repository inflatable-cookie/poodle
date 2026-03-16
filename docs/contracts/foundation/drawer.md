# Drawer

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Drawer`
- Layer: `foundation`
- Summary: an edge-anchored overlay surface for contextual tasks, inspectors,
  and secondary workflows
- In scope: edge anchoring (left, right, top, bottom), modal or non-modal
  posture, backdrop behavior, focus management, dismissal, title/content
  structure, body scroll locking
- Out of scope: full resizable dock layouts, persistent sidebars, sheet stacks

## 2. Anatomy

```text
[Root .drawer]  <div>
  ├── [Backdrop .drawer__backdrop]  <button> (conditional, modal only)
  └── [Surface .drawer__surface]  <div>
        ├── [Header .drawer__header]  <div> (conditional)
        │     ├── [Title]  <strong>
        │     └── [Description]  <p>
        ├── [Body] (slot)
        └── [Actions .drawer__actions]  <div> (conditional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | fixed overlay positioning host | overlay z-index, flex alignment |
| Backdrop | no | modal background block (only when isModal) | overlay background color |
| Surface | yes | edge-anchored container | surface background, border, elevation, padding |
| Header | no | title and description region | typography, spacing |
| Body | yes | primary content area (default slot) | caller-owned |
| Actions | no | footer action row | flex layout, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean \| null` | `null` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `edge` | `DrawerEdge: "left" \| "right" \| "top" \| "bottom"` | `"right"` | no | anchored edge |
| `isModal` | `boolean` | `true` | no | whether background becomes inert |
| `title` | `string \| null` | `null` | no | visible title when present |
| `description` | `string \| null` | `null` | no | visible supporting text |
| `dismissOnEscape` | `boolean` | `true` | no | escape dismissal |
| `dismissOnBackdrop` | `boolean` | `true` | no | backdrop dismissal when modal |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible title exists |

### Type Definitions

```
DrawerEdge: "left" | "right" | "top" | "bottom"
```

### Slots

| Slot | Purpose |
|------|---------|
| default | primary body content |
| actions | footer action buttons |

### Controlled And Uncontrolled

- controlled: `open` plus `openChange` event
- uncontrolled: `defaultOpen`
- modality is governed by `isModal`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | drawer hidden, pointer-events: none on root |
| open | open state true | drawer visible, surface and backdrop become interactive |
| modal | `isModal=true` | backdrop visible, focus trapped, body scroll locked |
| non-modal | `isModal=false` | no backdrop, no focus trap, no scroll lock |

### Component States

A small state machine is appropriate: closed, opening, open, closing, with
modal vs non-modal posture.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `openChange` | drawer opens or closes | `{ open: boolean }` | state ownership callback |
| `requestClose` | user attempts dismissal | void | escape, backdrop, or explicit close intent |

## 6. Accessibility

### Semantics

- Role: dialog-like overlay semantics with modal state depending on `isModal`
- Required attributes: accessible title from `title` prop or `ariaLabel`,
  description when present, drawer modality state
- Optional attributes: close button label and helper relations
- Labeling rules: non-modal drawers still need a stable accessible label

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through focusable content; traps only when modal |
| `Shift+Tab` | reverse navigation; traps only when modal |
| `Escape` | requests dismissal when `dismissOnEscape` is true |

### Focus And Announcement

- focus entry: opening may place focus inside the drawer or on the drawer
  surface depending on content
- focus trap: required when `isModal=true`; disallowed when `isModal=false`
- focus restoration: explicit close returns focus to the invoking control or
  reasonable fallback
- body scroll lock: applied only when `isModal=true`
- live-region behavior: none beyond dialog-like announcement of the drawer's
  title and content context
- GPUI-native accessibility mapping notes: GPUI must preserve the difference
  between modal and non-modal drawers, including background inertness only when
  modal and deterministic focus restoration in both cases

## 7. Layout

### Sizing

- left/right edges: surface width is `min(28rem, 100vw)`, height is `100vh`
- top/bottom edges: surface width is `100vw`, height is `min(24rem, 100vh)`
- large content should compose with `ScrollShell`

### Composition

- parent expectations: inspectors, secondary settings, focused side tasks
- child expectations: structured header/body/footer content
- resizing rules: edge anchoring remains stable while content scrolls within
  the drawer body

## 8. Token Usage — Exact Values

### Root (.drawer) — base styles

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `inset` | `0` |
| `z-index` | `var(--pug-overlay-z-dialog)` |
| `display` | `flex` |
| `pointer-events` | `none` |

### Root edge variants — data-edge attribute

| Selector | Property | Value |
|----------|----------|-------|
| `.drawer[data-edge="left"]` | `justify-content` | `flex-start` |
| `.drawer[data-edge="right"]` | `justify-content` | `flex-end` |
| `.drawer[data-edge="top"]` | `align-items` | `flex-start` |
| `.drawer[data-edge="bottom"]` | `align-items` | `flex-end` |

### Backdrop (.drawer__backdrop) — `<button>` element

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `0` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `var(--pug-color-background-overlay)` |
| `pointer-events` | `auto` |
| `cursor` | `default` |

### Surface (.drawer__surface)

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `z-index` | `1` |
| `pointer-events` | `auto` |
| `width` | `min(28rem, 100vw)` |
| `height` | `100vh` |
| `overflow` | `auto` |
| `padding` | `var(--pug-space-panel-y) var(--pug-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 78%, transparent)` |
| `border-radius` | `0` |
| `background` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| `--pug-surface` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| `box-shadow` | `var(--pug-elevation-dialog)` |

### Surface — top/bottom edge override

| Selector | Property | Value |
|----------|----------|-------|
| `.drawer[data-edge="top"] .drawer__surface` | `width` | `100vw` |
| `.drawer[data-edge="top"] .drawer__surface` | `height` | `min(24rem, 100vh)` |
| `.drawer[data-edge="bottom"] .drawer__surface` | `width` | `100vw` |
| `.drawer[data-edge="bottom"] .drawer__surface` | `height` | `min(24rem, 100vh)` |

### Header (.drawer__header)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `0.375rem` |
| `margin-bottom` | `var(--pug-space-stack-md)` |

### Header title — .drawer__header strong

| Property | Value |
|----------|-------|
| `font-family` | `var(--pug-typography-heading-family)` |
| `font-size` | `1rem` |
| `line-height` | `1.2` |

### Header description — .drawer__header p

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `color` | `var(--pug-color-text-secondary)` |

### Actions (.drawer__actions)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--pug-space-inline-sm)` |
| `justify-content` | `flex-end` |
| `margin-top` | `var(--pug-space-stack-md)` |

### Data Attributes

| Attribute | Source |
|-----------|--------|
| `data-edge` | `edge` prop |

## 9. Svelte Notes

- should share dialog-grade accessibility infrastructure when modal, while still
  allowing non-modal posture when configured
- edge positioning should not compromise semantic naming or focus restoration
- backdrop rendered as a `<button>` element for accessible click handling
- body scroll lock applied only in modal posture
- surface overflow set to `auto` for built-in scroll support

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::drawer`
- GPUI implementation must explicitly handle edge-anchored overlay stacking,
  modal-vs-non-modal focus rules, background inertness, and restoration
- surface sizing must match: left/right use min(28rem, 100vw) x 100vh;
  top/bottom use 100vw x min(24rem, 100vh)
- border-radius is 0 (no rounding on drawer surfaces)

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] drawer naming, description, and modality semantics match
- [ ] focus trapping behavior matches based on `isModal`
- [ ] escape, backdrop, and restoration behavior match
- [ ] body scroll lock applies only in modal posture

### Tier 2: Visual Parity

- [ ] edge anchoring, backdrop emphasis, and shell hierarchy use comparable
  token roles
- [ ] surface dimensions match (28rem left/right, 24rem top/bottom)
- [ ] border: 0.0625rem solid with 78% opacity border color
- [ ] background: 98% elevated mixed with panel
- [ ] box-shadow matches elevation-dialog
- [ ] border-radius is 0
- [ ] header typography matches (heading-family, 1rem, 1.2 line-height)

### Tier 3: Implementation Freedom

- [ ] slide mechanics and shell-host integration stay internal
- [ ] portal or layer strategy stays internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact edge motion curve may differ | runtime animation systems differ | allowed | keep modality and restoration semantics strict |
| color-mix transparency blending | GPUI may use direct alpha blending instead of CSS color-mix | allowed | same visual result required |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: inspectors, secondary settings panels, shell task flows
- future follow-up: connect persistent workstation shell sidebars and inspector
  composites in later milestones

> **Surface elevation**: Drawer is a surface creator — see [surface-elevation.md](./surface-elevation.md).
