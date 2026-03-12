# Banner

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Banner`
- Layer: `foundation`
- Summary: a prominent inline message surface for important status,
  remediation, or warning content
- In scope: tonal messaging, optional dismissal/action slot, assistive
  announcement posture
- Out of scope: toast lifecycle, modal blocking behavior

## 2. Anatomy

```text
[Root]
  ├── [Icon] (optional)
  ├── [Content]
  └── [Actions or Dismiss] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | prominent message shell | background, border, radius |
| Icon | no | severity cue | icon color |
| Content | yes | message content | typography, text color |
| Actions or Dismiss | no | remediation or dismissal controls | action/icon roles |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `"info" \| "success" \| "warning" \| "danger"` | `"info"` | no | semantic urgency |
| `isDismissible` | `boolean` | `false` | no | whether a dismiss action is present |
| `ariaLabel` | `string \| null` | `null` | no | optional explicit label |
| `announceMode` | `"none" \| "polite" \| "assertive"` | `"polite"` | no | assistive announcement urgency |
| `onDismiss` | `() => void` | none | no | dismissal callback |

### Controlled And Uncontrolled

- display primitive with optional dismissal callback

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| info | default | informational banner |
| success | tone changed | positive banner |
| warning | tone changed | warning banner |
| danger | tone changed | high-severity banner |
| dismissible | `isDismissible=true` | dismiss affordance visible |

### Component States

No internal state beyond optional dismissal presence.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onDismiss` | dismiss affordance triggered | none | optional |

## 6. Accessibility

### Semantics

- Role: status or alert-like messaging depending on urgency
- Required attributes: accessible message content; dismissal control must be
  named when present
- Optional attributes: explicit label and announce mode overrides
- Labeling rules: announcement mode must match actual urgency and not overstate
  informational content

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches dismiss/action controls when present |
| `Enter` or `Space` | activates dismiss/action controls |

### Focus And Announcement

- focus entry: banner itself is not focusable by default unless required by a
  remediation flow
- live-region behavior: follows `announceMode`; important updates should be
  announced consistently in both runtimes
- GPUI-native accessibility mapping notes: GPUI must map banner urgency into
  native announcement mechanisms or equivalent accessible event signaling, not
  only visible styling

## 7. Layout

### Sizing

- width follows parent container
- content height grows with message length and actions

### Composition

- parent expectations: shell headers, forms, status regions, inspectors
- child expectations: concise message plus optional action

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | surface/background/border roles | shell |
| Icon | icon roles | severity cue |
| Content | typography and text roles | message text |
| Tone | status semantic roles | urgency treatment |
| Actions | action/icon roles | remediation controls |

## 9. Svelte Notes

- may use live regions or ARIA status/alert patterns based on `announceMode`

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::banner`
- GPUI implementation must intentionally map announcement urgency, not just
  render a colored box with text

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] urgency and announcement semantics match
- [ ] dismiss action accessibility matches when present

### Tier 2: Visual Parity

- [ ] tone and prominence use comparable token roles

### Tier 3: Implementation Freedom

- [ ] announcement mechanism internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact announcement timing may vary slightly by platform API | assistive API internals differ | allowed | keep urgency meaning and presence strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: shell warnings, remediation banners, status regions
- future follow-up: connect to notification/toast work in later milestones

## Next Task

Use `Banner` when inline messaging needs stronger urgency or announcement than a
plain `Callout`.
