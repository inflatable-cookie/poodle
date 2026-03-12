# Popover

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Popover`
- Layer: `foundation`
- Summary: an anchored non-modal overlay for contextual interactive or rich
  informational content
- In scope: trigger/content relationship, anchored placement, outside dismissal,
  optional initial focus, collision handling
- Out of scope: modal flows, menu-specific item semantics, long-lived pinned
  panels

## 2. Anatomy

```text
[Root]
  ├── [Trigger]
  └── [Popover Surface]
        └── [Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | state owner | overlay state |
| Trigger | yes | invokes the popover | button or field roles, focus |
| Popover Surface | conditional | anchored floating shell | surface, border, radius, elevation |
| Content | yes | informational or interactive content | spacing, typography |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `open` | `boolean` | `false` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `placement` | `string` | `"bottom-start"` | no | placement hint |
| `offset` | `number` | `8` | no | trigger gap |
| `dismissOnOutsideInteract` | `boolean` | `true` | no | outside dismissal |
| `initialFocus` | `"first-focusable" \| "content" \| "none"` | `"first-focusable"` | no | initial focus strategy |
| `ariaLabel` | `string \| null` | `null` | no | optional label when no internal heading exists |
| `onOpenChange` | `(open: boolean) => void` | none | no | open-state callback |

### Controlled And Uncontrolled

- controlled: `open` plus `onOpenChange`
- uncontrolled: `defaultOpen`
- popover content state remains external to the primitive

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | surface hidden |
| open | open state true | surface visible and anchored |
| focus-within | content receives focus | visible focus context |

### Component States

Open/closed state and placement state are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | popover opens or closes | boolean | trigger, outside interact, or escape driven |

## 6. Accessibility

### Semantics

- Role: non-modal dialog, group, or descriptive overlay depending on content
- Required attributes: trigger-to-content relationship and accessible naming
  when the content acts as a meaningful region
- Optional attributes: description relation and heading association
- Labeling rules: if content is interactive or long-lived, it must have a
  stable accessible label

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` or `Space` | opens from an interactive trigger when appropriate |
| `Escape` | closes the popover and restores focus to the trigger |
| `Tab` | moves through focusable content without trapping the user |

### Focus And Announcement

- focus entry: opening may move focus into the content according to
  `initialFocus`
- focus exit: non-modal popovers do not trap focus; leaving the content may
  dismiss according to implementation policy
- focus restoration: explicit close returns focus to the trigger
- live-region behavior: none by default; content semantics should carry the
  meaning
- GPUI-native accessibility mapping notes: GPUI must expose popover ownership,
  focus handoff, and restoration without confusing the popover with a modal
  window

## 7. Layout

### Sizing

- content sizes to intrinsic needs within viewport limits
- anchored width may optionally match the trigger when the use case requires it

### Composition

- parent expectations: field helpers, compact settings panels, shell affordances
- child expectations: informational or interactive content blocks
- resizing rules: placement collision handling should preserve reachability

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Popover Surface | surface, border, radius, elevation, and overlay roles | floating shell |
| Content | spacing and typography roles | internal layout |
| Motion | motion roles | open and close transitions when used |

## 9. Svelte Notes

- may compose headless popover primitives, but the contract owns dismissal and
  focus-restoration semantics
- if the content traps focus, the component should likely be `Dialog` or
  `Drawer` instead

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::popover`
- GPUI implementation must intentionally model anchored overlay behavior,
  outside-dismiss rules, and non-modal focus flow through native window or view
  constructs

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] trigger/content relationship and labeling match
- [ ] open, close, outside-dismiss, and escape behavior match
- [ ] focus handoff and restoration match

### Tier 2: Visual Parity

- [ ] surface hierarchy, spacing, and elevation use comparable token roles

### Tier 3: Implementation Freedom

- [ ] exact collision engine and rendering strategy stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact placement fallback order may differ | overlay engine internals vary | allowed | keep trigger relation, dismissal, and focus rules strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: anchored helpers, compact inspector surfaces, shell
  affordances
- future follow-up: connect richer picker and command-surface composites in
  later milestones

## Next Task

Use `Popover` for anchored rich content that stays non-modal; use `Dialog` or
`Drawer` when the workflow needs blocking modality or focus trapping.
