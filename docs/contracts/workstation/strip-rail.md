# StripRail

Status: seed contract
Updated: 2026-03-17

## 1. Purpose

- Component name: `StripRail`
- Layer: `workstation`
- Summary: a compact edge rail for workstation shells, providing icon-first or
  mixed-content navigation along any of the four edges
- In scope: four-edge orientation, icon-first and mixed-content modes,
  active/idle/compact/collapsed variants, item selection, item ordering
- Out of scope: item content semantics (which icons, what they activate),
  persistence of active item, app-specific strip policies

## 2. Anatomy

```text
[Strip Rail]
  ├── [Strip Items]
  │     ├── [Strip Item] (1..n)
  │     │     ├── [Icon]
  │     │     ├── [Label] (optional, hidden in compact/collapsed)
  │     │     └── [Badge] (optional)
  │     └── [Separator] (optional, between item groups)
  └── [Strip Footer] (optional)
        └── [Utility Items]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Strip Rail | yes | rail container along one edge | background, border, spacing |
| Strip Item | yes (1+) | individual navigation target | icon, text, active/idle state |
| Icon | yes | primary visual identifier | icon size, color roles |
| Label | no | text label, hidden in compact modes | typography roles |
| Badge | no | notification or status indicator | badge roles |
| Separator | no | visual divider between item groups | separator roles |
| Strip Footer | no | utility items pinned to end of rail | spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `orientation` | `"horizontal" \| "vertical"` | `"vertical"` | no | edge direction |
| `edge` | `"top" \| "bottom" \| "left" \| "right"` | `"left"` | no | which edge this rail occupies |
| `items` | `StripItem[]` | `[]` | no | ordered list of strip items |
| `value` | `string \| null` | `null` | no | active item id (controlled) |
| `defaultValue` | `string \| null` | `null` | no | initial active item (uncontrolled) |
| `mode` | `"icon" \| "mixed"` | `"icon"` | no | display mode |
| `isCollapsed` | `boolean` | `false` | no | collapsed posture |
| `ariaLabel` | `string \| null` | `null` | no | rail label |

### Types

```typescript
type StripItem = {
  id: string;
  label: string;
  icon: string;
  badge?: string | null;
  group?: string | null;
  isDisabled?: boolean;
};
```

### Controlled And Uncontrolled

- supports both controlled (`value`) and uncontrolled (`defaultValue`) patterns
- host owns item list and active state in controlled mode

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | no item active | all items at idle weight |
| active | item selected | active item highlighted, others idle |
| compact | viewport or host constraint | labels hidden, icons only |
| collapsed | `isCollapsed=true` | rail minimized to narrow icon strip or hidden |
| disabled-item | item `isDisabled=true` | individual item visually suppressed |

### Component States

| State | Description |
|-------|-------------|
| expanded | full rail with icons and optional labels |
| compact | icons only, labels hidden |
| collapsed | minimal or hidden posture |

## 5. Events

| Event | Payload | When |
|-------|---------|------|
| `valueChange` | `{ value: string }` | user selects a strip item |
| `collapsedChange` | `{ isCollapsed: boolean }` | rail collapse/expand |

## 6. Accessibility

### Semantics

- Role: `tablist` or `toolbar` depending on whether items activate panels
- Items: `tab` or `button` role
- Required attributes: `aria-label` on rail, `aria-selected` on active item

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Up/Down` (vertical) | move between items |
| `Arrow Left/Right` (horizontal) | move between items |
| `Home` | first item |
| `End` | last item |
| `Enter` / `Space` | activate focused item |

### Focus And Announcement

- focus enters the active item or first item
- item activation announces the item label
- collapsed state announces via live region

## 7. Layout

### Sizing

- vertical rails: fixed width, full height of parent region
- horizontal rails: full width, fixed height
- item sizing scales with control size token
- compact mode reduces to icon-only width/height

### Composition

- parent expectations: WorkspaceLayout strip region slot
- child expectations: StripItem children, optional footer slot
- resizing rules: rail width/height is fixed; items scroll if overflow

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Rail background | surface background, shell chrome | rail container |
| Rail border | separator role | edge boundary |
| Active item | accent or selection color roles | active indication |
| Idle item | muted text and icon roles | idle state |
| Badge | badge color roles | notification indicator |
| Spacing | spacing scale | item gaps, rail padding |

## 9. Svelte Notes

- expected substrate: `div` with `role="tablist"` or `role="toolbar"`
- orientation prop maps to ARIA `aria-orientation`
- items rendered as buttons or tab elements
- badge uses existing Badge primitive

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::workstation::strip_rail`
- orientation maps to GPUI flex direction
- active item state via `Model<T>` observation

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] item selection semantics match
- [ ] keyboard navigation matches
- [ ] collapsed/compact state transitions match
- [ ] event payloads match

### Tier 2: Visual Parity

- [ ] active/idle item treatment uses comparable tokens
- [ ] rail sizing matches across orientations

### Tier 3: Implementation Freedom

- [ ] animation and transition details stay internal
- [ ] scroll overflow handling can differ

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none anticipated | — | — | — |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: Loophole Aura (activity bar, status strip), any
  workstation application needing edge navigation
- future follow-up: drag-reorder strip items, strip item context menus

## Next Task

Implement StripRail in Svelte during `g11.010`.
