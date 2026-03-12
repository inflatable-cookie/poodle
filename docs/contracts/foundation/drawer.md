# Drawer

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Drawer`
- Layer: `foundation`
- Summary: an edge-anchored overlay surface for contextual tasks, inspectors,
  and secondary workflows
- In scope: edge anchoring, modal or non-modal posture, backdrop behavior,
  focus management, dismissal, title/content structure
- Out of scope: full resizable dock layouts, persistent sidebars, sheet stacks

## 2. Anatomy

```text
[Root]
  ├── [Backdrop] (conditional)
  └── [Drawer Surface]
        ├── [Header]
        ├── [Body]
        └── [Footer Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | overlay state owner | overlay state |
| Backdrop | no | modal background block | overlay tone, motion |
| Drawer Surface | yes | edge-anchored container | surface, border, radius, elevation |
| Header | no | title/description region | typography, spacing |
| Body | yes | primary content area | spacing, text color |
| Footer Actions | no | action row | spacing, action roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean` | `false` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `edge` | `"left" \| "right" \| "top" \| "bottom"` | `"right"` | no | anchored edge |
| `isModal` | `boolean` | `true` | no | whether background becomes inert |
| `title` | `string \| null` | `null` | no | visible title when present |
| `description` | `string \| null` | `null` | no | visible supporting text |
| `dismissOnEscape` | `boolean` | `true` | no | escape dismissal |
| `dismissOnBackdrop` | `boolean` | `true` | no | backdrop dismissal when modal |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible title exists |
| `onOpenChange` | `(open: boolean) => void` | none | no | open-state callback |
| `onRequestClose` | `() => void` | none | no | close-intent callback |

### Controlled And Uncontrolled

- controlled: `open` plus `onOpenChange`
- uncontrolled: `defaultOpen`
- modality is governed by `isModal`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | drawer hidden |
| opening | open transition begins | edge-entry motion |
| open | open state true | drawer visible |
| closing | dismissal begins | edge-exit motion |

### Component States

A small state machine is appropriate: closed, opening, open, closing, with
modal vs non-modal posture.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | drawer opens or closes | boolean | state ownership callback |
| `onRequestClose` | user attempts dismissal | none | escape, backdrop, or explicit close intent |

## 6. Accessibility

### Semantics

- Role: dialog-like overlay semantics with modal state depending on `isModal`
- Required attributes: accessible title or `ariaLabel`, description when
  present, drawer modality state
- Optional attributes: close button label and helper relations
- Labeling rules: non-modal drawers still need a stable accessible label

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | moves through focusable content; traps only when modal |
| `Shift+Tab` | reverse navigation; traps only when modal |
| `Escape` | requests dismissal when allowed |

### Focus And Announcement

- focus entry: opening may place focus inside the drawer or on the drawer
  surface depending on content
- focus trap: required when `isModal=true`; disallowed when `isModal=false`
- focus restoration: explicit close returns focus to the invoking control or
  reasonable fallback
- live-region behavior: none beyond dialog-like announcement of the drawer's
  title and content context
- GPUI-native accessibility mapping notes: GPUI must preserve the difference
  between modal and non-modal drawers, including background inertness only when
  modal and deterministic focus restoration in both cases

## 7. Layout

### Sizing

- width or height is edge-dependent and constrained by viewport or shell bounds
- large content should compose with `ScrollShell`

### Composition

- parent expectations: inspectors, secondary settings, focused side tasks
- child expectations: structured header/body/footer content
- resizing rules: edge anchoring remains stable while content scrolls within
  the drawer body

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Backdrop | overlay tone and motion roles | modal emphasis |
| Drawer Surface | surface, border, radius, elevation, and overlay roles | shell |
| Header and Body | typography, spacing, and text roles | content layout |
| Footer Actions | spacing and action roles | button row |
| Motion | motion roles | edge-entry and exit transitions |

## 9. Svelte Notes

- should share dialog-grade accessibility infrastructure when modal, while still
  allowing non-modal posture when configured
- edge positioning should not compromise semantic naming or focus restoration

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::drawer`
- GPUI implementation must explicitly handle edge-anchored overlay stacking,
  modal-vs-non-modal focus rules, background inertness, and restoration

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] drawer naming, description, and modality semantics match
- [ ] focus trapping behavior matches based on `isModal`
- [ ] escape, backdrop, and restoration behavior match

### Tier 2: Visual Parity

- [ ] edge anchoring, backdrop emphasis, and shell hierarchy use comparable token roles

### Tier 3: Implementation Freedom

- [ ] slide mechanics and shell-host integration stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact edge motion curve may differ | runtime animation systems differ | allowed | keep modality and restoration semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: inspectors, secondary settings panels, shell task flows
- future follow-up: connect persistent workstation shell sidebars and inspector
  composites in later milestones

## Next Task

Use `Drawer` for edge-anchored task surfaces, and keep always-present shell
panels in workstation composites rather than treating them as overlays.
