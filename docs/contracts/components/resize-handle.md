# ResizeHandle

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `ResizeHandle`
- Layer: `foundation`
- Summary: a standalone resize interaction primitive for adjusting the size of
  adjacent regions via mouse drag or keyboard input, rendering as a thin visual
  line with an invisible, larger grab overlay that costs no layout space
- In scope: horizontal and vertical orientation, mouse drag interaction with
  window-level move/up listeners, keyboard resize with arrow keys and Home/End,
  visual affordance line, hover/active/disabled states, ARIA separator role with
  value semantics
- Out of scope: layout policy (min/max sizes are host-provided), animation,
  persistence of sizes, touch/pointer events (uses mouse events), snap points

## 2. Anatomy

```text
[Root .resize-handle]  <div role="separator" aria-orientation="..." tabindex="...">
  ├── [Hit .resize-handle__hit]  <span aria-hidden="true">
  └── [Line .resize-handle__line]  <span aria-hidden="true">
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<div>` with separator role; focus receiver. Its layout footprint is exactly the line's thickness | position, display, flex-shrink, cursor, width/height per orientation |
| Hit | yes | `<span>` grab area: absolutely positioned overlay, centred on the line and wider than it. Contributes nothing to layout, so it overlaps the adjacent regions instead of widening the divider | position, inset, z-index |
| Line | yes | `<span>` visual affordance line; fills the root | position, inset, border-radius, background, transition |

### Why The Grab Area Is An Overlay

A handle that reserves its whole grab width in layout pushes the adjacent
regions apart. Between two bordered panels that reads as two borders with a gap
between them rather than as one divider. Keeping the root at line thickness and
overlaying the grab area gives a hairline divider that is still comfortable to
grab.

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `SplitOrientation` | `"horizontal"` | no | resize axis: `"horizontal"` for left/right resize, `"vertical"` for up/down resize |
| `disabled` | `boolean` | `false` | no | suppresses all interaction |
| `ariaLabel` | `string \| null` | `null` | no | accessible label; defaults to `"Resize"` when null |
| `ariaValueNow` | `number \| null` | `null` | no | current position value for accessibility |
| `ariaValueMin` | `number` | `0` | no | minimum value for accessibility |
| `ariaValueMax` | `number` | `100` | no | maximum value for accessibility |

### SplitOrientation Type

```typescript
type SplitOrientation = "horizontal" | "vertical";
```

### Controlled And Uncontrolled

- The handle requests resize updates via callbacks; the host controls actual sizing
- No internal size state is maintained
- The handle does not know about panel dimensions or constraints

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | default | subtle line at 82% opacity of border-default color |
| hover | pointer over hit target | line changes to accent-base color |
| active/dragging | mouse drag in progress | line changes to accent-base color; `data-dragging` attribute present |
| disabled | `disabled=true` | opacity 0.4, default cursor, no interaction, `tabindex=-1` |
| focus-visible | keyboard focus | focus ring outline around the handle |

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Machine-backed via core machinery (g11 extraction sweep): axis position,
drag-delta tracking, and keyboard step resolution (`resizeAxisPosition`,
`resizeDragDelta`, `resizeKeydownStep` — ±8px arrows along the axis,
saturating Home/End) live in `@poodle/headless`. Window move/up listeners
stay adapter-side (drag-gesture effects).

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|----------|--------------|---------|-------|
| `onResizeStart` | `mousedown` on handle (when not disabled) | `number` | position is `clientX` (horizontal) or `clientY` (vertical) |
| `onResizeMove` | `mousemove` on window during drag | `number` | pixel delta since last move event |
| `onResizeEnd` | `mouseup` on window after drag | `number` | final `clientX`/`clientY` position |
| `onResizeStep` | arrow key, Home, or End pressed | `number` | `+/-8` for arrow keys, `+/-9999` for Home/End |

## 6. Accessibility

### Semantics

- Root: `role="separator"` with `aria-orientation` matching the `orientation` prop
- `aria-label` defaults to `"Resize"` when the `ariaLabel` prop is null
- `aria-valuenow`, `aria-valuemin`, `aria-valuemax` passed through from props
- `tabindex="0"` when enabled; `tabindex="-1"` when disabled

### Keyboard

| Key | Behavior |
|-----|----------|
| `ArrowLeft` | decrease size by 8px step (horizontal orientation) |
| `ArrowRight` | increase size by 8px step (horizontal orientation) |
| `ArrowUp` | decrease size by 8px step (vertical orientation) |
| `ArrowDown` | increase size by 8px step (vertical orientation) |
| `Home` | decrease to minimum (delta: -9999) |
| `End` | increase to maximum (delta: +9999) |

### Focus And Announcement

- focus entry: handle is focusable via `tabindex="0"` when not disabled
- focus ring: standard focus-visible outline
- value changes: `aria-valuenow` updates announced by screen reader when host updates the prop

## 7. Layout

### Sizing

- **Horizontal orientation**: `width: var(--poodle-resize-handle-thickness)`, `height: 100%`, `cursor: col-resize`
- **Vertical orientation**: `width: 100%`, `height: var(--poodle-resize-handle-thickness)`, `cursor: row-resize`
- Line: `inset: 0` — fills the root, so the visible line is the handle's whole footprint
- Hit (horizontal): `inset-block: 0`, `inset-inline: calc((hit-size - thickness) / -2)`
- Hit (vertical): `inset-inline: 0`, `inset-block: calc((hit-size - thickness) / -2)`
- `flex-shrink: 0` prevents the handle from collapsing in flex layouts

### Tunable Custom Properties

| Property | Default | Purpose |
|----------|---------|---------|
| `--poodle-resize-handle-thickness` | `0.125rem` | visible line thickness, and the handle's entire layout footprint |
| `--poodle-resize-handle-hit-size` | `0.5rem` | grab extent across the resize axis; overlays the neighbours |

Both are read as `var(..., <default>)` and are never declared on the root: a
declaration there would shadow whatever an ancestor sets, and consumers scope
these on a shell element so that SplitView's divider and the handle inside it
resolve the same value.

A consumer wanting a hairline divider sets `--poodle-resize-handle-thickness:
0.0625rem` without touching the grab size.

### Composition

- parent expectations: between two resizable regions in a flex container
- child expectations: none (self-contained)
- resizing: handle extends full length of the adjacent boundary

## 8. Token Usage -- Exact Values

### Root `.resize-handle`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `flex-shrink` | `0` |

### Root -- horizontal orientation `[data-orientation="horizontal"]`

| Property | Value |
|----------|-------|
| `width` | `var(--poodle-resize-handle-thickness)` (`0.125rem`) |
| `height` | `100%` |
| `cursor` | `col-resize` |

### Root -- vertical orientation `[data-orientation="vertical"]`

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `height` | `var(--poodle-resize-handle-thickness)` (`0.125rem`) |
| `cursor` | `row-resize` |

### Hit `.resize-handle__hit`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `z-index` | `1` |
| `cursor` | `inherit` |

### Hit -- horizontal orientation

| Property | Value |
|----------|-------|
| `inset-block` | `0` |
| `inset-inline` | `calc((var(--poodle-resize-handle-hit-size) - var(--poodle-resize-handle-thickness)) / -2)` (`-0.1875rem`) |

### Hit -- vertical orientation

| Property | Value |
|----------|-------|
| `inset-inline` | `0` |
| `inset-block` | `calc((var(--poodle-resize-handle-hit-size) - var(--poodle-resize-handle-thickness)) / -2)` (`-0.1875rem`) |

### Root -- disabled `[data-disabled]`

| Property | Value |
|----------|-------|
| `cursor` | `default` |
| `opacity` | `0.4` |

### Root -- focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

### Line `.resize-handle__line`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `inset` | `0` |
| `border-radius` | `999rem` |
| `background` | `var(--poodle-color-border-subtle)` |
| `transition` | `background 120ms ease` |

### Line -- hover / dragging

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-accent-base)` |

## 9. Svelte Notes

- `data-orientation` attribute on root reflects the orientation prop
- `data-disabled` attribute present (truthy) when disabled; absent when enabled
- `data-dragging` attribute present (truthy) during active drag; absent otherwise
- Uses `mousedown` on the handle element to start drag
- Attaches `mousemove` and `mouseup` listeners to `window` during drag for
  reliable tracking outside handle bounds (not pointer capture)
- Removes window listeners on drag end and component destroy (`onDestroy`)
- Keyboard step size is 8 pixels; Home/End use +/-9999 as a large sentinel value
- No `data-size` or `data-density` attributes; this component does not participate
  in size/density scaling
- Line visual uses `aria-hidden="true"`

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::resize_handle`
- Spec struct: `ResizeHandleSpec` in primitives crate
- Maps to GPUI drag interaction model
- Cursor changes via GPUI cursor API (`col-resize` / `row-resize`)
- Window-level mouse tracking maps to GPUI's drag capture model
- Keyboard step size must match (8px)

## 10a. Jetstream Notes

- `ResizeHandle::from_spec(spec, theme).on_resize(...)`, driven by a real drag:
  `Start` with `0.0`, `Move` with each frame's axis delta in pixels, `End` with
  `0.0`.
- A delta, not a position: the handle cannot know the panes' sizes, so an
  absolute position would be a guess. The host applies the delta to the ratio it
  already holds.
- Drags do not bubble in this runtime, so the grab overlay and the line each
  carry the handler — the same lesson the sliders taught.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `role="separator"` with `aria-orientation` matches
- [ ] `aria-valuenow`, `aria-valuemin`, `aria-valuemax` passed through
- [ ] `aria-label` defaults to "Resize"
- [ ] `onResizeStart`, `onResizeMove`, `onResizeEnd` callbacks run with correct payloads
- [ ] `onResizeStep` runs with +/-8 for arrows, +/-9999 for Home/End
- [ ] disabled state: `tabindex="-1"`, no interaction, opacity 0.4

### Tier 2: Visual Parity

- [ ] idle line: 82% border-default in color-mix
- [ ] hover/dragging line: accent-base color
- [ ] horizontal: 0.5rem wide, col-resize cursor
- [ ] vertical: 0.5rem tall, row-resize cursor
- [ ] line thickness: 0.125rem with 999rem border-radius
- [ ] background transition: 120ms ease
- [ ] focus ring matches accent focus ring token

### Tier 3: Implementation Freedom

- [ ] pointer/mouse capture and drag internals stay renderer-specific
- [ ] window-level listener strategy is renderer-specific

## 12. Specimen Definitions

### Group: Horizontal split (vertical handle)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Horizontal split | `orientation="horizontal"`, `ariaLabel="Resize horizontal"`, placed between two panes in a row | Vertical resize handle between left and right panes; hover highlights the line in accent color; drag left/right fires resize events |

### Group: Vertical split (horizontal handle)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Vertical split | `orientation="vertical"`, `ariaLabel="Resize vertical"`, placed between two panes in a column | Horizontal resize handle between top and bottom panes; hover highlights; drag up/down fires resize events |

### Group: Disabled (horizontal split)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled horizontal | `orientation="horizontal"`, `disabled`, `ariaLabel="Disabled resize"` | Muted visual (0.4 opacity), default cursor, no interaction; handle between left and right panes |

### Group: Disabled (vertical split)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled vertical | `orientation="vertical"`, `disabled`, `ariaLabel="Disabled resize vertical"` | Muted visual (0.4 opacity), default cursor, no interaction; handle between top and bottom panes |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: split views, dock panels, workspace layouts, IDE-style
  resizable regions
- future follow-up: touch/pointer event support, snap points, double-click
  to reset
