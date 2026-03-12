# Status Indicator

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `StatusIndicator`
- Layer: `foundation`
- Summary: a compact visual and textual signal for current state such as ready,
  warning, error, or pending
- In scope: status tone, optional icon, optional short label
- Out of scope: full explanatory callouts or banners

## 2. Anatomy

```text
[Root]
  ├── [Icon or Dot]
  └── [Label] (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | compact status host | spacing |
| Icon or Dot | yes | primary status cue | status color |
| Label | no | short text label | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `status` | `"neutral" \| "info" \| "success" \| "warning" \| "danger" \| "pending"` | `"neutral"` | no | semantic state |
| `label` | `string \| null` | `null` | no | optional short visible label |
| `ariaLabel` | `string \| null` | `null` | no | explicit accessible label when visible label is absent or abbreviated |

### Controlled And Uncontrolled

- display primitive only

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral/info/success/warning/danger/pending | status value | matching visual cue |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive by default |

## 6. Accessibility

### Semantics

- Role: inline status text or grouped status content by default
- Required attributes: explicit accessible label when visible content is absent
  or ambiguous
- Optional attributes: none
- Labeling rules: color alone must never be the only status signal

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive by default |

### Focus And Announcement

- focus entry: not focusable by default
- live-region behavior: parent-owned unless status changes must be announced
- GPUI-native accessibility mapping notes: GPUI must expose status meaning via
  text or accessible label, not color alone

## 7. Layout

### Sizing

- indicator remains compact and inline
- label is optional and may truncate based on parent layout

### Composition

- parent expectations: headers, lists, rows, status summaries
- child expectations: icon/dot and optional short text

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Icon or Dot | status semantic roles | primary status cue |
| Label | typography and text roles | short text label |
| Pending | motion/status roles | pending emphasis where animated |

## 9. Svelte Notes

- keep status semantics text-backed rather than color-only

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::status_indicator`
- GPUI implementation must ensure a text or accessible-label path exists so the
  status remains perceivable to assistive technology

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] status meaning is not color-only in either runtime
- [ ] accessible labeling semantics match

### Tier 2: Visual Parity

- [ ] status color/icon roles match

### Tier 3: Implementation Freedom

- [ ] dot/icon rendering details stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| icon shape may differ by platform icon set | icon rendering internals differ | allowed | keep semantic labeling strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: headers, status rows, lightweight summaries
- future follow-up: connect to richer diagnostics surfaces later

## Next Task

Use `StatusIndicator` for compact state summaries and `Banner` or `Callout` for
longer explanatory messaging.
