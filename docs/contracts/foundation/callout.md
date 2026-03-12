# Callout

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Callout`
- Layer: `foundation`
- Summary: a low-to-medium emphasis informational block for inline contextual
  messaging
- In scope: neutral and danger/informational tones, non-modal contextual
  content
- Out of scope: dismissible banners, toast notifications, alert dialogs

## 2. Anatomy

```text
[Root]
  ├── [Icon] (optional)
  └── [Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | contextual message shell | background, border, radius |
| Icon | no | message affordance | icon color |
| Content | yes | callout text and inline content | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `"neutral" \| "info" \| "success" \| "warning" \| "danger"` | `"neutral"` | no | semantic tone |
| `ariaLabel` | `string \| null` | `null` | no | optional accessible label when surrounding context is insufficient |

### Controlled And Uncontrolled

- display primitive only

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral | default | low-emphasis message shell |
| tonal | tone changed | matching border/background/text emphasis |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive by default |

## 6. Accessibility

### Semantics

- Role: usually static group/region content, not alert by default
- Required attributes: none by default
- Optional attributes: accessible label when the message needs an explicit
  programmatic summary
- Labeling rules: message text itself is normally sufficient if present

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive by default |

### Focus And Announcement

- focus entry: not focusable by default
- live-region behavior: none by default; use `Banner` or higher-order alerting
  surfaces when announcement behavior is required
- GPUI-native accessibility mapping notes: GPUI should expose callouts as
  grouped informational content, not as automatically announced alerts unless a
  higher-level contract demands it

## 7. Layout

### Sizing

- width follows parent
- height grows with content

### Composition

- parent expectations: forms, inspectors, cards, settings sections
- child expectations: informative text and optional inline content

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | surface/background/border roles | shell |
| Icon | icon roles | message affordance |
| Content | typography and text roles | message text |
| Tone | status semantic roles | tonal emphasis |

## 9. Svelte Notes

- can remain a styled container with no default live-region behavior

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::callout`
- keep callout informational and non-announcing unless wrapped by a more
  urgent messaging contract

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] informational/non-alert semantics match

### Tier 2: Visual Parity

- [ ] tonal shell treatment uses comparable token roles

### Tier 3: Implementation Freedom

- [ ] icon presence and layout internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: contextual messaging in forms and shells
- future follow-up: route urgent messaging to `Banner`

## Next Task

Use `Callout` for contextual information and reserve announced or dismissible
messaging for `Banner`.
