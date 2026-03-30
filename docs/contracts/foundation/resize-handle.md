# ResizeHandle

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `ResizeHandle`
- Layer: `foundation`
- Summary: a standalone resize interaction primitive for adjusting the size of
  adjacent regions via mouse drag or keyboard input, rendering as a thin visual
  line within an invisible hit target
- In scope: horizontal and vertical orientation, mouse drag interaction with
  window-level move/up listeners, keyboard resize with arrow keys and Home/End,
  visual affordance line, hover/active/disabled states, ARIA separator role with
  value semantics
- Out of scope: layout policy (min/max sizes are host-provided), animation,
  persistence of sizes, touch/pointer events (uses mouse events), snap points

## 2. Anatomy

```text
[Root .resize-handle]  <div role="separator" aria-orientation="..." tabindex="...">
  └── [Line .resize-handle__line]  <span aria-hidden="true">
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<div>` with separator role; serves as the hit target and focus receiver | position, display, flex-shrink, cursor, width/height per orientation |
| Line | yes | `<span>` visual affordance line centered in the hit target | position, width/height, border-radius, background, transition |

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

- The handle emits resize deltas via events; the host controls actual sizing
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

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `resizeStart` | `mousedown` on handle (when not disabled) | `{ position: number }` | `position` is `clientX` (horizontal) or `clientY` (vertical) |
| `resizeMove` | `mousemove` on window during drag | `{ delta: number }` | pixel delta since last move event |
| `resizeEnd` | `mouseup` on window after drag | `{ position: number }` | final `clientX`/`clientY` position |
| `resizeStep` | arrow key, Home, or End pressed | `{ delta: number }` | `+/-8` for arrow keys, `+/-9999` for Home/End |

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

- **Horizontal orientation**: `width: 0.5rem`, `height: 100%`, `cursor: col-resize`
- **Vertical orientation**: `width: 100%`, `height: 0.5rem`, `cursor: row-resize`
- Line (horizontal): `width: 0.125rem`, `height: 100%`
- Line (vertical): `width: 100%`, `height: 0.125rem`
- `flex-shrink: 0` prevents the handle from collapsing in flex layouts

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
| `width` | `0.5rem` |
| `height` | `100%` |
| `cursor` | `col-resize` |

### Root -- vertical orientation `[data-orientation="vertical"]`

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `height` | `0.5rem` |
| `cursor` | `row-resize` |

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
| `border-radius` | `999rem` |
| `background` | `color-mix(in srgb, var(--poodle-color-border-default) 82%, transparent)` |
| `transition` | `background 120ms ease` |

### Line -- horizontal orientation

| Property | Value |
|----------|-------|
| `width` | `0.125rem` |
| `height` | `100%` |

### Line -- vertical orientation

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `height` | `0.125rem` |

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

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `role="separator"` with `aria-orientation` matches
- [ ] `aria-valuenow`, `aria-valuemin`, `aria-valuemax` passed through
- [ ] `aria-label` defaults to "Resize"
- [ ] `resizeStart`, `resizeMove`, `resizeEnd` events fire with correct payloads
- [ ] `resizeStep` fires with +/-8 for arrows, +/-9999 for Home/End
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
