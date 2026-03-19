# PanelHeader

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `PanelHeader`
- Layer: `workstation`
- Summary: a reusable header row for workstation panels with title, panel tabs,
  utility actions, and collapse/menu affordances
- In scope: title/icon, panel-tab hosting, utility-action slots, collapse/menu
  affordances, active/inactive panel header treatment
- Out of scope: panel body content, dock layout ownership, app-specific panel
  actions

## 2. Anatomy

```text
[Root Header]
  ├── [Title Block] (optional)
  ├── [Panel Tabs] (optional)
  └── [Utility Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Header | yes | panel chrome row | background, border, height |
| Title Block | no | icon and panel title | typography, icon, spacing |
| Panel Tabs | no | region-local panel tab surface | tab spacing, active state |
| Utility Actions | no | panel-scoped actions and menus | action spacing, icon roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | visible panel title |
| `isActive` | `boolean` | `false` | no | active panel emphasis |
| `isCollapsible` | `boolean` | `false` | no | collapse affordance may appear |
| `ariaLabel` | `string \| null` | `null` | no | optional label override |

### Controlled And Uncontrolled

- declarative header composite
- panel-tab, action, and collapse state remain host-owned

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| inactive | default | neutral header chrome |
| active | `isActive=true` | emphasized title/tab state |
| tabbed | panel tabs present | tab row integrated into header |
| collapsible | `isCollapsible=true` | collapse affordance visible |

### Component States

State table is sufficient.

## 5. Events

No component-owned events beyond child action or tab behavior.

## 6. Accessibility

### Semantics

- Role: group, toolbar region, or header section within a panel surface
- Required attributes: accessible name when the header is the primary panel
  label source
- Optional attributes: utility action descriptions
- Labeling rules: utility actions must not replace the panel title as the
  accessible identity of the panel

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches panel tabs and utility actions in logical order |

### Focus And Announcement

- focus entry: the header itself is not focusable by default
- focus exit: active panel semantics should remain perceivable when focus moves
  into the panel body
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI must preserve the distinction
  between panel identity, panel tabs, and utility actions rather than flattening
  them into unlabeled icons

## 7. Layout

### Sizing

- fixed header height aligned with `PanelSurface`
- title, tabs, and actions may compress according to host policy

### Composition

- parent expectations: `PanelSurface`, dock regions, floating panel windows
- child expectations: `PanelTabs`, icon buttons, menus, labels
- resizing rules: tab strip or title may take priority based on host layout, but
  utility actions remain reachable

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Header | panel shell background, border, and spacing roles | chrome |
| Title Block | label and icon roles | panel identity |
| Panel Tabs | tab strip and selected/focus roles | navigation |
| Utility Actions | action spacing and icon roles | controls |

## 9. Svelte Notes

- expected substrate: `PanelSurface`, `TabStrip`, `Inline`, and action
  primitives
- wrapper strategy: header layout remains Pug-owned even if tabs/actions are
  slotted

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::panel_header`
- implementation-only details: GPUI may realize the header as native panel
  chrome, but identity/tab/action semantics still need explicit accessible
  structure

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] panel-identity and utility-region semantics match
- [ ] active/inactive header meaning matches
- [ ] tab/action order matches

### Tier 2: Visual Parity

- [ ] header height and emphasis use comparable token roles

### Tier 3: Implementation Freedom

- [ ] title truncation and packing mechanics stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact packing priority may differ | runtime layout differs | allowed | keep reachability and identity strict |

## 13. Specimen Definitions

Specimen file: `PanelHeaderSpecimen.svelte` (not yet created).

The specimen should demonstrate the following groups based on the contract's states and props:

### Group: Basic panel header

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With title | `title="Explorer"` | Neutral header chrome with panel title |
| Active header | `title="Explorer"`, `isActive` | Emphasized title treatment indicating active panel |

### Group: With panel tabs

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Tabbed header | `title` omitted, panel tabs slot with multiple tabs | Tab row integrated into header chrome |
| Active tabbed | panel tabs slot, `isActive` | Tabs with active panel emphasis |

### Group: Utility actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With actions | `title="Explorer"`, utility actions slot with icon buttons | Title plus right-aligned utility action icons |
| Collapsible | `title="Explorer"`, `isCollapsible`, utility actions slot | Collapse affordance visible alongside actions |

## 14. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: docked panels, floating utility panels
- future follow-up: pair with deeper maximize/pin behavior if real adopters need
  it

## Next Task

Use `PanelHeader` to host title, tabs, and utility actions while keeping panel
body and dock orchestration in `PanelSurface` and `DockRegion`.
