# Badge

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `Badge`
- Layer: `foundation`
- Summary: a compact emphasis label for counts, status hints, or lightweight
  categorization
- In scope: short inline content, accent/muted variants
- Out of scope: dismissible chips, selection pills

## 2. Anatomy

```text
[Root]
  └── [Content]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | compact label shell | background, radius, text |
| Content | yes | short text or count | typography, text color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"accent" \| "muted"` | `"accent"` | no | appearance family |
| `ariaLabel` | `string \| null` | `null` | no | optional explicit accessible name |

### Controlled And Uncontrolled

- display primitive only

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| accent | default | emphasized inline badge |
| muted | `variant="muted"` | low-emphasis badge |

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
- Optional attributes: explicit accessible name when visible text is
  abbreviated or symbolic
- Labeling rules: badges stay non-interactive unless wrapped by a higher-order
  interactive contract

### Keyboard

| Key | Behavior |
|-----|----------|
| none | non-interactive by default |

### Focus And Announcement

- focus entry: not focusable by default
- live-region behavior: none
- GPUI-native accessibility mapping notes: GPUI should expose badge content as
  plain text or decorative inline status, not as a button-like element

## 7. Layout

### Sizing

- badge sizes to content
- content should stay short and non-wrapping by default

### Composition

- parent expectations: headers, cards, rows, status text
- child expectations: short text or numeric content

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | background roles | badge shell |
| Root | radius roles | pill-like shape |
| Content | typography and text roles | label styling |

## 9. Svelte Notes

- can remain a styled `span` or equivalent

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::badge`
- keep badge semantics text-like unless a higher-order interactive shell wraps
  it

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] non-interactive inline semantics match

### Tier 2: Visual Parity

- [ ] shell and text roles match

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: counts, labels, compact status hints
- future follow-up: separate notification counts if richer semantics are needed

## Next Task

Use `Badge` for compact static emphasis and `Pill` for richer inline token-like
labels.
