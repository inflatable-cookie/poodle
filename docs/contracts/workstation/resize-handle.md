# ResizeHandle

Status: seed contract
Updated: 2026-03-17

## 1. Purpose

- Component name: `ResizeHandle`
- Layer: `workstation`
- Summary: a standalone resize interaction primitive for adjusting the size of
  adjacent workstation regions via drag or keyboard
- In scope: horizontal and vertical orientation, drag interaction, keyboard
  resize, visual affordance, hit target sizing
- Out of scope: layout policy (min/max sizes are host-provided), animation,
  persistence of sizes

## 2. Anatomy

```text
[Resize Handle]
  ├── [Hit Target] (invisible, larger than visual)
  └── [Visual Affordance] (thin line or grip indicator)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Hit Target | yes | interactive drag/keyboard target | sizing (min 8px) |
| Visual Affordance | yes | visible indicator of resize position | border, separator roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | resize axis |
| `isDisabled` | `boolean` | `false` | no | suppresses interaction |
| `ariaLabel` | `string \| null` | `null` | no | accessible label |
| `ariaValueNow` | `number \| null` | `null` | no | current ratio for a11y |
| `ariaValueMin` | `number` | `0` | no | minimum value for a11y |
| `ariaValueMax` | `number` | `100` | no | maximum value for a11y |

### Controlled And Uncontrolled

- the handle emits resize deltas; the host controls actual sizing
- no internal size state

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | default | subtle separator line |
| hover | pointer over hit target | highlighted affordance |
| active | drag in progress | strong highlight, cursor change |
| disabled | `isDisabled=true` | no interaction, muted visual |

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `resizeStart` | `{ position: number }` | drag begins |
| `resizeMove` | `{ delta: number }` | drag in progress |
| `resizeEnd` | `{ position: number }` | drag ends |
| `resizeStep` | `{ delta: number }` | keyboard step (arrow key) |

## 6. Accessibility

### Semantics

- Role: `separator` with `aria-orientation`
- Required attributes: `aria-valuenow`, `aria-valuemin`, `aria-valuemax`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Up` | decrease size by step |
| `Arrow Right/Down` | increase size by step |
| `Home` | minimum size |
| `End` | maximum size |

### Focus And Announcement

- focusable when not disabled
- value changes announced via aria-valuenow updates

## 7. Layout

### Sizing

- hit target: minimum 8px perpendicular to orientation
- visual affordance: 1–2px line centered in hit target
- extends full length of the adjacent boundary

### Composition

- parent expectations: between two resizable regions
- child expectations: none

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Visual affordance | separator/border roles | idle line |
| Hover state | accent or interactive roles | hover highlight |
| Active state | accent roles | drag highlight |

## 9. Svelte Notes

- uses pointer events for drag tracking
- `pointer-capture` for reliable drag outside handle bounds
- cursor changes via CSS (`col-resize` / `row-resize`)

## 10. GPUI Notes

- maps to GPUI drag interaction model
- cursor changes via GPUI cursor API

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] drag and keyboard resize semantics match
- [ ] event payloads match
- [ ] ARIA role and value semantics match

### Tier 2: Visual Parity

- [ ] idle/hover/active states use comparable tokens

### Tier 3: Implementation Freedom

- [ ] pointer capture and drag internals stay renderer-specific

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none anticipated | — | — | — |

## 13. Specimen Definitions

Specimen reference: `ResizeHandleSpecimen.svelte`.

### Group: Horizontal split (vertical handle)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Horizontal split | `orientation="horizontal"`, `ariaLabel="Resize horizontal"`, placed between two panes in a row | Vertical resize handle between left and right panes; drag left/right to resize |

### Group: Vertical split (horizontal handle)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Vertical split | `orientation="vertical"`, `ariaLabel="Resize vertical"`, placed between two panes in a column | Horizontal resize handle between top and bottom panes; drag up/down to resize |

### Group: Disabled (horizontal split)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled horizontal | `orientation="horizontal"`, `isDisabled`, `ariaLabel="Disabled resize"` | Muted visual, no interaction; handle between left and right panes |

### Group: Disabled (vertical split)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled vertical | `orientation="vertical"`, `isDisabled`, `ariaLabel="Disabled resize vertical"` | Muted visual, no interaction; handle between top and bottom panes |

## Next Task

Implement ResizeHandle in Svelte during `g11.010`.
