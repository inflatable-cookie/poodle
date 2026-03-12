# Pill

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Pill`
- Layer: `foundation`
- Summary: a rounded inline label for compact categorization or metadata
- In scope: tone, appearance, compact sizing, optional monospace styling
- Out of scope: removable chips, multi-select tag inputs

## 2. Anatomy

```text
[Root]
  └── [Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | rounded metadata shell | border, background, radius |
| Content | yes | short label content | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `"neutral" \| "success" \| "danger"` | `"neutral"` | no | semantic tone |
| `appearance` | `"solid" \| "subtle"` | `"solid"` | no | fill style |
| `size` | `"xxs" \| "xs" \| "sm"` | `"xs"` | no | compact scale |
| `font` | `"normal" \| "mono"` | `"normal"` | no | content font variant |
| `isMuted` | `boolean` | `false` | no | visual de-emphasis |
| `ariaLabel` | `string \| null` | `null` | no | optional explicit label |

### Controlled And Uncontrolled

- display primitive only

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| neutral | default | neutral pill shell |
| success | `tone="success"` | positive styling |
| danger | `tone="danger"` | warning/danger styling |
| muted | `isMuted=true` | lower-emphasis pill |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | non-interactive by default |

## 6. Accessibility

### Semantics

- Role: inline text by default
- Required attributes: none
- Optional attributes: accessible label when visible text is abbreviated or
  symbolic
- Labeling rules: pills stay non-interactive unless a higher-order contract
  wraps them

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive by default |

### Focus And Announcement

- focus entry: not focusable by default
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI should expose pill content as
  text-like metadata, not as a control

## 7. Layout

### Sizing

- pill sizes to content with compact padding
- content may truncate according to parent layout rules

### Composition

- parent expectations: metadata rows, headers, cards, filter summaries
- child expectations: short text content

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | border/background/radius roles | shell |
| Content | text/typography roles | label styling |
| Tone | status or neutral semantic roles | tone treatment |

## 9. Svelte Notes

- can remain a styled inline element

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::pill`
- keep semantics non-interactive unless wrapped by a control-specific contract

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] non-interactive metadata semantics match

### Tier 2: Visual Parity

- [ ] tone, appearance, and typography roles match

### Tier 3: Implementation Freedom

- [ ] truncation and rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: metadata displays, labels, status tags
- future follow-up: add dismissible-chip semantics separately if needed

## Next Task

Keep `Pill` static and metadata-oriented until a true chip/tag-input contract is
needed.
