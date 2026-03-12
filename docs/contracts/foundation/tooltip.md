# Tooltip

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Tooltip`
- Layer: `foundation`
- Summary: a brief non-interactive descriptive overlay tied to a trigger or
  subject element
- In scope: delayed open on hover/focus, descriptive content, placement,
  dismissal, accessible description relationship
- Out of scope: interactive content, menus, teaching bubbles, persistent help
  panels

## 2. Anatomy

```text
[Root]
  ├── [Trigger or Subject]
  └── [Tooltip Bubble]
        └── [Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | state owner | overlay state |
| Trigger or Subject | yes | element that owns the tooltip | focus and hover context |
| Tooltip Bubble | conditional | floating descriptive shell | surface, border, elevation |
| Content | yes | short descriptive text | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `content` | `string` | none | yes | tooltip text |
| `open` | `boolean` | `false` | no | controlled open state |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial state |
| `delayMs` | `number` | `300` | no | open delay |
| `placement` | `string` | `"top"` | no | placement hint |
| `onOpenChange` | `(open: boolean) => void` | none | no | open-state callback |

### Controlled And Uncontrolled

- controlled: `open` plus `onOpenChange`
- uncontrolled: `defaultOpen`
- the trigger or subject owns the tooltip relationship and focus context

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | bubble hidden |
| pending | hover or focus delay active | no bubble yet |
| open | delay elapsed or forced open | bubble visible |

### Component States

Closed, pending-open, and open states are required.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | tooltip opens or closes | boolean | hover, focus, escape, or leave driven |

## 6. Accessibility

### Semantics

- Role: tooltip or equivalent descriptive relationship
- Required attributes: trigger-to-tooltip description relationship when the
  tooltip is the accessibility source
- Optional attributes: none in the baseline contract
- Labeling rules: tooltip content supplements a trigger; it does not replace a
  missing accessible name

### Keyboard

| Key | Behavior |
|-----|----------|
| focus navigation | may reveal the tooltip when the trigger receives focus |
| `Escape` | dismisses the visible tooltip |

### Focus And Announcement

- focus entry: tooltip itself is not focusable
- focus exit: tooltip dismisses when the trigger loses the relevant focus or
  hover state
- live-region behavior: none; tooltip content should be exposed through a
  descriptive relationship rather than an unsolicited announcement
- GPUI-native accessibility mapping notes: GPUI must expose tooltip text as
  native help or descriptive metadata where available; it may not create a
  focusable floating window for simple help text

## 7. Layout

### Sizing

- bubble fits concise text and wraps when needed
- placement adjusts to viewport constraints

### Composition

- parent expectations: icon buttons, dense controls, shell affordances
- child expectations: short non-interactive content only
- resizing rules: tooltip should avoid obscuring the trigger when reasonable

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Tooltip Bubble | surface, border, elevation, and overlay roles | bubble shell |
| Content | compact typography and text roles | help text |
| Motion | motion roles | fade or scale transitions when used |

## 9. Svelte Notes

- should use tooltip semantics or description wiring rather than inventing a
  menu-like overlay
- interactive content should escalate to `Popover`

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::tooltip`
- GPUI implementation must choose native help-text exposure and dismissal
  behavior that preserves non-focusable descriptive semantics

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] descriptive relationship and non-interactive semantics match
- [ ] open and dismiss behavior for hover, focus, and escape match
- [ ] tooltip content does not replace the trigger's accessible name

### Tier 2: Visual Parity

- [ ] bubble hierarchy and compact help styling use comparable token roles

### Tier 3: Implementation Freedom

- [ ] exact delay timers and animation internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| precise hover timing may vary slightly | platform event timing differs | allowed | keep descriptive semantics and dismissal strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: icon buttons, dense shell actions, compact indicators
- future follow-up: define richer hovercard/help-panel patterns separately if
  product need appears

## Next Task

Use `Tooltip` only for short non-interactive descriptions; escalate to
`Popover` when the overlay must contain interactive or persistent content.
