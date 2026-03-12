# Tabs

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Tabs`
- Layer: `foundation`
- Summary: a tabbed navigation control that coordinates a tablist and one
  active content panel
- In scope: tablist semantics, tab activation, tab-panel relationship,
  orientation, automatic vs manual activation
- Out of scope: reorderable document tabs, close buttons, docking, overflow
  menus

## 2. Anatomy

```text
[Root]
  ├── [TabList]
  │     └── [Tab...]
  └── [TabPanel]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | tabs host | layout spacing |
| TabList | yes | navigation container | border, gap |
| Tab | yes | selectable navigation item | text, background, focus, selected state |
| TabPanel | yes | active content region | surface, spacing, typography |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string` | none | no | controlled active tab |
| `defaultValue` | `string` | none | no | uncontrolled initial active tab |
| `tabs` | `Array<{ value: string; label: string; isDisabled?: boolean }>` | none | yes | tab definitions |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | navigation orientation |
| `activationMode` | `"automatic" \| "manual"` | `"automatic"` | no | whether focus changes selection |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible tablist label exists |
| `onValueChange` | `(value: string) => void` | none | no | active-tab callback |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange`
- uncontrolled: `defaultValue`
- `activationMode` changes whether focus movement commits selection

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | non-selected tab | inactive styling |
| selected | active value | selected styling and active panel |
| focus | roving focus reaches tab | visible focus ring |
| disabled | `isDisabled=true` on tab | muted non-interactive tab |

### Component States

Selected-tab state and roving-focus state are both required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | active tab changes | selected value | may happen on focus or activation depending on mode |

## 6. Accessibility

### Semantics

- Role: tablist, tab, and tabpanel semantics
- Required attributes: selected state, orientation when vertical,
  tab-to-panel relationship, accessible tab names
- Optional attributes: tablist label
- Labeling rules: each tab must have a unique accessible name within the list

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` | moves focus in horizontal tablists |
| `Arrow Up/Down` | moves focus in vertical tablists |
| `Home/End` | moves focus to first/last tab |
| `Enter` or `Space` | activates focused tab in manual mode |
| `Tab` | moves between the tablist and active panel |

### Focus And Announcement

- focus entry: tablist exposes one focusable tab at a time through roving focus
- focus transition: activating a tab preserves focus on the tab unless the host
  workflow intentionally moves into the panel
- focus exit: tab order continues into the active panel content
- live-region behavior: none; active selection must be conveyed through native
  tab semantics
- GPUI-native accessibility mapping notes: GPUI must expose tablist, tab,
  selected state, and tab-to-panel relationships through the native
  accessibility tree rather than only painting segmented buttons

## 7. Layout

### Sizing

- tablist may wrap, scroll, or overflow according to parent layout policy
- tab panels expand to available parent space

### Composition

- parent expectations: inspectors, settings shells, work areas, sidebars
- child expectations: one active panel at a time
- resizing rules: tab selection should not cause layout jump in neighboring tabs

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| TabList | border and spacing roles | navigation grouping |
| Tab | text, background, focus, selected, and disabled roles | interactive chrome |
| TabPanel | surface and spacing roles | content container |
| Motion | motion duration/easing roles | indicator or panel transitions when used |

## 9. Svelte Notes

- should use semantic tab roles with predictable DOM relationships between tabs
  and panels
- automatic activation should not break browser focus visibility

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::tabs`
- GPUI implementation must model roving focus, selected state, and panel
  relationships directly; it may not collapse tabs into unlabeled toolbar
  buttons

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] tablist, tab, and tabpanel semantics match
- [ ] keyboard navigation and activation-mode behavior match
- [ ] selected-state and tab-to-panel accessibility relationships match

### Tier 2: Visual Parity

- [ ] tab emphasis, spacing, and panel hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] panel mounting strategy and indicator animation internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| inactive panels may stay mounted or unmounted | runtime rendering strategy differs | allowed | keep semantics and state continuity strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings navigation, inspectors, sectional work areas
- future follow-up: connect overflow-tab affordances and persistence patterns in
  later composite milestones

## Next Task

Use `Tabs` for navigation tied to active content panels, and reserve
`TabStrip` for reorderable document- or surface-style tab rows.
