# AppHeader

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `AppHeader`
- Layer: `workstation`
- Summary: a global workstation shell header for app identity, global actions,
  and window-level utility status
- In scope: app identity, global action slots, optional status/connection
  indicators, drag-region posture, shell utility placement
- Out of scope: project-specific title/details, transport controls, timeline or
  mixer widgets

## 2. Anatomy

```text
[Root Header]
  ├── [Identity Region]
  ├── [Primary Actions] (optional)
  └── [Utility Status] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Header | yes | top shell chrome | background, border, height |
| Identity Region | yes | app name/icon/window identity | typography, icon, spacing |
| Primary Actions | no | global shell actions | gap, action roles |
| Utility Status | no | connection/status indicators | text, status, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | visible app title |
| `isDragRegion` | `boolean` | `false` | no | allows native window drag posture where supported |
| `ariaLabel` | `string \| null` | `null` | no | optional region label |

### Controlled And Uncontrolled

- declarative shell header
- global actions and utility-status content remain host-owned children

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| standard | default | steady shell header |
| drag-region | `isDragRegion=true` | header supports window dragging where supported |
| utility-heavy | utility content present | status/utility space reserved |

### Component States

State table is sufficient.

## 5. Events

No component-owned events beyond child action behavior.

## 6. Accessibility

### Semantics

- Role: banner, toolbar region, or neutral shell header depending on host
  structure
- Required attributes: meaningful label when the header is an addressable shell
  region
- Optional attributes: utility-status descriptions
- Labeling rules: drag-region behavior must not suppress or hide interactive
  controls from assistive technology

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches shell actions and utility controls in logical order |

### Focus And Announcement

- focus entry: the header itself is not focusable by default
- focus exit: utility/status updates should not reorder global actions
- live-region behavior: none by default; child status indicators own any needed
  announcements
- GPUI-native accessibility mapping notes: GPUI must preserve labeled header or
  toolbar structure even when integrated with native title-bar mechanics

## 7. Layout

### Sizing

- fixed shell-header height within theme control-size expectations
- actions and status may compress or overflow according to host policy

### Composition

- parent expectations: top-level workspace shell
- child expectations: action clusters, status indicators, identity text/icon
- resizing rules: identity remains stable while utility actions compress first

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Header | shell background, border, and height roles | shell chrome |
| Identity Region | typography and icon roles | app identity |
| Primary Actions | action spacing roles | command grouping |
| Utility Status | status and subdued text roles | shell metadata |

## 9. Svelte Notes

- expected substrate: `Surface`, `Inline`, and button/status primitives
- wrapper strategy: browser drag-region styling stays implementation detail

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::app_header`
- implementation-only details: native window-chrome integration is allowed, but
  control order, labeling, and utility semantics remain Pug-owned

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] app-header identity and utility-region semantics match
- [ ] interactive control order and naming match

### Tier 2: Visual Parity

- [ ] shell-header proportion and hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] drag-region and native chrome integration stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native drag integration may differ | runtime window systems differ | allowed | keep interaction order and labeling strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: workstation window shells, multi-window desktop apps
- future follow-up: connect status-bar and deeper shell-utility patterns later

## Next Task

Use `AppHeader` for global workstation shell identity and keep project- or
surface-specific context in `ProjectHeader`.
