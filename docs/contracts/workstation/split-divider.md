# SplitDivider

Status: seed contract
Updated: 2026-03-17

## 1. Purpose

- Component name: `SplitDivider`
- Layer: `workstation`
- Summary: a visual and interactive divider between split regions, combining
  a resize handle with optional collapse affordances
- In scope: divider visual treatment, integrated resize handle, optional
  collapse buttons for adjacent regions, orientation variants
- Out of scope: split layout policy, min/max region sizes, persistence

## 2. Anatomy

```text
[Split Divider]
  ├── [Collapse Button: Before] (optional)
  ├── [Resize Handle]
  └── [Collapse Button: After] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Split Divider | yes | divider container | border, spacing |
| Resize Handle | yes | embedded resize interaction | separator, interactive roles |
| Collapse Button: Before | no | collapses region before divider | icon, interactive roles |
| Collapse Button: After | no | collapses region after divider | icon, interactive roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | divider axis |
| `showCollapseBefore` | `boolean` | `false` | no | show collapse button for before region |
| `showCollapseAfter` | `boolean` | `false` | no | show collapse button for after region |
| `isBeforeCollapsed` | `boolean` | `false` | no | before region collapsed state |
| `isAfterCollapsed` | `boolean` | `false` | no | after region collapsed state |
| `isDisabled` | `boolean` | `false` | no | suppresses all interaction |
| `ariaLabel` | `string \| null` | `null` | no | divider label |

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | default | subtle divider line with optional collapse buttons |
| hover | pointer over divider | highlighted treatment |
| dragging | resize in progress | active highlight |
| before-collapsed | before region collapsed | collapse button shows expand icon |
| after-collapsed | after region collapsed | collapse button shows expand icon |

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `resizeStart` | `{ position: number }` | drag begins |
| `resizeMove` | `{ delta: number }` | drag in progress |
| `resizeEnd` | `{ position: number }` | drag ends |
| `collapseBefore` | `{ isCollapsed: boolean }` | before collapse toggle |
| `collapseAfter` | `{ isCollapsed: boolean }` | after collapse toggle |

## 6. Accessibility

### Semantics

- Role: `separator` with orientation
- Collapse buttons: standard button role with descriptive labels

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow` keys | resize via embedded handle |
| `Enter` on collapse button | toggle collapse |

## 7. Layout

### Sizing

- divider occupies a thin strip between two regions
- collapse buttons centered on the divider
- hit target extends beyond visual for ease of interaction

### Composition

- parent expectations: between two split regions
- child expectations: none (collapse buttons are internal)

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Divider line | separator/border roles | visual boundary |
| Collapse buttons | icon button roles | collapse affordance |
| Hover/active | accent/interactive roles | interaction feedback |

## 9. Svelte Notes

- composes ResizeHandle internally
- collapse buttons use IconButton or similar primitive

## 10. GPUI Notes

- composes GPUI resize interaction internally
- collapse buttons use GPUI icon button equivalent

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] resize and collapse semantics match
- [ ] event payloads match

### Tier 2: Visual Parity

- [ ] divider and button treatment matches

### Tier 3: Implementation Freedom

- [ ] internal composition details stay renderer-specific

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none anticipated | — | — | — |

## 13. Specimen Definitions

Specimen reference: `SplitDividerSpecimen.svelte`.

### Group: Horizontal with collapse affordances

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Horizontal with collapse | `orientation="horizontal"`, `showCollapseBefore`, `showCollapseAfter`, placed between left and right panes | Vertical divider with collapse buttons for both adjacent panes; resize by drag |

### Group: Vertical with collapse affordances

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Vertical with collapse | `orientation="vertical"`, `showCollapseBefore`, `showCollapseAfter`, placed between top and bottom panes | Horizontal divider with collapse buttons for both adjacent panes; resize by drag |

### Group: Resize only (no collapse)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Resize only | `orientation="horizontal"`, no collapse props, placed between left and right panes | Plain horizontal divider with resize handle only; no collapse buttons |

## Next Task

Implement SplitDivider in Svelte during `g11.010`.
