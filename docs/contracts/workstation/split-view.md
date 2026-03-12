# SplitView

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `SplitView`
- Layer: `workstation`
- Summary: a resizable shell layout container that divides space between two or
  more pane regions
- In scope: orientation, divider semantics, size ratios, collapsible panes,
  keyboard-resizable separators
- Out of scope: nested dock orchestration policy, persistence backend,
  app-specific pane content

## 2. Anatomy

```text
[Root Split]
  ├── [Pane A]
  ├── [Divider]
  └── [Pane B]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Split | yes | pane layout container | spacing, background |
| Pane | yes | one side of the split | min sizes, overflow |
| Divider | yes | resize handle and visual separator | border, focus, accent |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `"horizontal" \| "vertical"` | none | yes | split axis |
| `ratio` | `number` | `0.5` | no | controlled primary split ratio |
| `defaultRatio` | `number` | `0.5` | no | uncontrolled initial ratio |
| `minPrimarySize` | `number \| null` | `null` | no | host-defined size constraint |
| `minSecondarySize` | `number \| null` | `null` | no | host-defined size constraint |
| `isPrimaryCollapsed` | `boolean` | `false` | no | optional collapse state |
| `isSecondaryCollapsed` | `boolean` | `false` | no | optional collapse state |
| `onRatioChange` | `(ratio: number) => void` | none | no | resize callback |

### Controlled And Uncontrolled

- controlled: `ratio` plus `onRatioChange`
- uncontrolled: `defaultRatio`
- collapse states are externally owned when used

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| steady | default | panes visible with divider |
| resizing | pointer or keyboard resize active | divider focus/emphasis visible |
| primary-collapsed | `isPrimaryCollapsed=true` | primary pane hidden or reduced |
| secondary-collapsed | `isSecondaryCollapsed=true` | secondary pane hidden or reduced |

### Component States

Ratio state, resize interaction state, and optional pane-collapse state are
required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onRatioChange` | resize commits or streams | numeric ratio | host decides persistence cadence |

## 6. Accessibility

### Semantics

- Role: group with separator semantics for the divider
- Required attributes: divider orientation and current value semantics when the
  divider is keyboard-resizable
- Optional attributes: pane labels or descriptions
- Labeling rules: panes that are meaningful shell regions should remain named by
  their child content or host labels, not by the split container itself

### Keyboard

| Key | Behavior |
|-----|----------|
| arrow keys on divider | adjusts ratio along the split axis |
| `Home/End` | optional jump to min/max documented positions |
| `Enter` or `Space` | optional collapse/restore shortcut if the host supports it |
| `Tab` | reaches the divider and pane content in logical order |

### Focus And Announcement

- focus entry: divider becomes focusable when keyboard resizing is supported
- focus exit: divider focus clears while pane sizing remains updated
- live-region behavior: none; resize and collapse state should be conveyed
  through control semantics
- GPUI-native accessibility mapping notes: GPUI must expose resizable separators
  with orientation and value semantics, not just pointer-only drag handles

## 7. Layout

### Sizing

- split container fills assigned parent space
- panes respect host-defined minimum sizes where possible

### Composition

- parent expectations: `WorkspaceShell`, nested shell regions, panel layouts
- child expectations: dock regions, panel surfaces, or other shell panes
- resizing rules: child focus continuity should survive ratio changes and
  collapse/restore operations

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Split | shell spacing/background roles | layout container |
| Divider | border, focus, and accent roles | resize affordance |
| Pane | size and surface roles | region footprint |

## 9. Svelte Notes

- expected substrate: layout primitives plus explicit divider elements and drag
  handlers
- wrapper strategy: pointer drag and keyboard resize both remain required

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::split_view`
- implementation-only details: GPUI may use native splitter support or custom
  layout code, but keyboard resizing, orientation semantics, and collapse state
  remain required

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] orientation, ratio, and collapse semantics match
- [ ] keyboard-resize behavior matches
- [ ] divider accessibility semantics match

### Tier 2: Visual Parity

- [ ] divider emphasis and pane separation use comparable token roles

### Tier 3: Implementation Freedom

- [ ] drag physics and resize cadence stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact resize feel may differ | runtime event systems differ | allowed | keep keyboard parity and ratio meaning strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: docked workspace shells, floating split inspectors,
  multi-pane utilities
- future follow-up: connect nested split orchestration and persistence later

## Next Task

Use `SplitView` for pane sizing and keep broader dock/persistence orchestration
outside the baseline contract.
