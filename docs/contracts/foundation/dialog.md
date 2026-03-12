# Dialog

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Dialog`
- Layer: `foundation`
- Summary: a modal blocking overlay for confirmation, form, or focused task
  flows
- In scope: modal semantics, backdrop, title/description, focus trap,
  dismissal, announcement, restoration
- Out of scope: anchored contextual overlays, full-screen multi-step workflows,
  toast notifications

## 2. Anatomy

```text
[Root]
  ├── [Backdrop]
  └── [Dialog Surface]
        ├── [Header]
        ├── [Body]
        └── [Footer Actions] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | modal state owner | overlay state |
| Backdrop | yes | background scrim and interaction block | overlay tone, motion |
| Dialog Surface | yes | modal container | surface, border, radius, elevation |
| Header | no | title/description region | typography, spacing |
| Body | yes | primary content area | spacing, text color |
| Footer Actions | no | action row | spacing, action roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean` | `false` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `title` | `string \| null` | `null` | no | visible title when present |
| `description` | `string \| null` | `null` | no | visible supporting description |
| `kind` | `"dialog" \| "alertdialog"` | `"dialog"` | no | semantic urgency |
| `dismissOnEscape` | `boolean` | `true` | no | escape dismissal |
| `dismissOnBackdrop` | `boolean` | `true` | no | backdrop dismissal |
| `ariaLabel` | `string \| null` | `null` | no | required when no visible title exists |
| `onOpenChange` | `(open: boolean) => void` | none | no | open-state callback |
| `onRequestClose` | `() => void` | none | no | close-intent callback |

### Controlled And Uncontrolled

- controlled: `open` plus `onOpenChange`
- uncontrolled: `defaultOpen`
- close requests may be observed separately through `onRequestClose`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | modal hidden |
| opening | open state transitions in | backdrop and surface enter |
| open | open state true | modal visible and active |
| closing | dismissal starts | exit motion begins |

### Component States

A small state machine is appropriate: closed, opening, open, closing.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | dialog opens or closes | boolean | state ownership callback |
| `onRequestClose` | user attempts dismissal | none | escape or backdrop intent |

## 6. Accessibility

### Semantics

- Role: dialog or alertdialog
- Required attributes: accessible name from title or `ariaLabel`,
  description when present, modal semantics, background inertness
- Optional attributes: close button label and helper relations
- Labeling rules: every dialog needs an accessible title even when visually
  minimal

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | cycles focus within the dialog while open |
| `Shift+Tab` | reverse-cycles focus within the dialog |
| `Escape` | requests dismissal when allowed |
| `Enter` | may activate the focused primary action, not the shell itself |

### Focus And Announcement

- focus entry: open moves focus to the first meaningful focus target or the
  dialog surface when no focusable child exists
- focus trap: focus must remain inside the modal while it is active
- focus restoration: close returns focus to the invoking control or a documented
  fallback
- live-region behavior: dialog open should be announced through modal semantics;
  `alertdialog` urgency must not be faked with color alone
- GPUI-native accessibility mapping notes: GPUI must create a true modal
  accessible subtree, mark background content unavailable to assistive
  technology while blocked, and preserve deterministic restoration on close

## 7. Layout

### Sizing

- dialogs size to content within viewport constraints
- large content should compose with `ScrollShell` inside the surface

### Composition

- parent expectations: confirmation flows, settings sheets, focused tasks
- child expectations: structured header/body/footer content
- resizing rules: action row remains reachable at all supported sizes

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Backdrop | overlay tone and motion roles | background block and emphasis |
| Dialog Surface | surface, border, radius, elevation, and overlay roles | modal shell |
| Header and Body | typography, spacing, and text roles | content layout |
| Footer Actions | spacing and action roles | button row |
| Motion | motion roles | open and close transitions |

## 9. Svelte Notes

- should rely on semantic dialog behavior and explicit focus trapping rather
  than only a centered div with a scrim
- if the browser substrate does not provide background inertness automatically,
  the wrapper must enforce it

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::dialog`
- GPUI implementation must explicitly own modal stacking, focus trapping,
  background blocking, announcement, and restoration behavior

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] dialog role, naming, description, and modal semantics match
- [ ] focus trap and restoration behavior match
- [ ] escape and backdrop dismissal rules match

### Tier 2: Visual Parity

- [ ] backdrop emphasis, shell hierarchy, and action layout use comparable token roles

### Tier 3: Implementation Freedom

- [ ] portal or windowing internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact transition timing may differ slightly | runtime animation systems differ | allowed | keep modality, focus trap, and dismissal semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: settings shells, confirmations, focused task flows
- future follow-up: connect wizard and multi-step composite flows in later
  milestones

## Next Task

Use `Dialog` for centered modal task flows, and use `Drawer` when the same
modal semantics need edge-anchored shell behavior instead.
