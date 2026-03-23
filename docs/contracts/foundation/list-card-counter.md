# ListCardCounter

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `ListCardCounter`
- Layer: `foundation`
- Summary: a compact inline counter with an icon and numeric value, used inside
  the footer slot of ListCard to display item counts (e.g. "24 documents",
  "8 images")
- In scope: icon + count display, optional tooltip, optional link (href),
  hover color change for linked counters, tabular-nums formatting
- Out of scope: standalone usage outside ListCard, editable counts,
  interactive states beyond link hover

## 2. Anatomy

```text
[Root .list-card-counter]  <span> or <a>
  ├── [Icon]   Icon primitive (size="sm")
  └── [Count]  <span> with numeric value
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | inline-flex container; renders as `<a>` when `href` is provided, `<span>` otherwise; wrapped in Tooltip when `tooltip` is provided | text-secondary color, gap, font-size |
| Icon | yes | Icon primitive at `size="sm"` | inherits color from Root |
| Count | yes | `<span>` displaying the numeric count | inherits color and font-size from Root |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `icon` | `string` | — | yes | icon name passed to the Icon primitive |
| `count` | `number` | — | yes | numeric value to display |
| `tooltip` | `string \| null` | `null` | no | tooltip text; when provided, Root is wrapped in a Tooltip |
| `href` | `string \| null` | `null` | no | when provided, Root renders as an `<a>` element with this href |

### Slots

None.

### Controlled And Uncontrolled

- Fully uncontrolled display component; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no href | icon and count in secondary text color |
| linked | `href` provided | renders as `<a>` element with `text-decoration: none` |
| hover (linked) | mouse over linked counter | text color transitions from secondary to primary |

### Component States

No internal state. All visual states are derived from props.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| (native click) | linked counter clicked | — | `e.stopPropagation()` is called to prevent parent ListCard click from firing |

## 6. Accessibility

### Semantics

- Renders as `<span>` (static) or `<a>` (linked) — semantic element chosen by href prop
- When tooltip is provided, the Tooltip primitive handles `aria-describedby`
- Icon is decorative (inherits meaning from tooltip or surrounding context)

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | focuses linked counters (native `<a>` behavior) |
| `Enter` | activates the link (native `<a>` behavior) |

### Focus And Announcement

- Focus ring: browser default for `<a>` elements
- No custom focus management

## 7. Layout

### Sizing

- `display: inline-flex` with `align-items: center`
- gap: `0.25rem` (4px)
- font-size: `0.75rem` (12px)
- font-variant-numeric: `tabular-nums` for consistent digit widths

### Composition

- Parent expectations: designed to be placed in the `footer` slot of ListCard
- Child expectations: none (self-contained)
- Resizing rules: inline element, wraps with its parent container

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `--poodle-color-text-secondary` | default text and icon color |
| Root (hover, linked) | `--poodle-color-text-primary` | hover text and icon color for linked counters |

## 9. Svelte Notes

- Uses `Icon` primitive with `size="sm"` for the icon
- Uses `Tooltip` primitive wrapper when `tooltip` prop is provided
- Click handler calls `e.stopPropagation()` on linked counters to prevent
  bubbling to the parent ListCard
- Conditional root element: `<a>` when href is set, `<span>` otherwise

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::primitives::list_card_counter`
- Render as inline flex with icon and text child
- Link behavior may need to be handled via click callback rather than href

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] icon renders at sm size across all implementations
- [ ] stopPropagation behavior matches for linked counters
- [ ] tooltip wrapping behavior matches

### Tier 2: Visual Parity

- [ ] gap, font-size, and color tokens match
- [ ] tabular-nums applied
- [ ] hover color transition matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 13. Specimen Definitions

ListCardCounter does not have a standalone specimen. It is demonstrated within
the **ListCardSpecimen** in the "With footer counters" group.

### With Footer Counters (in ListCardSpecimen)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Design system card | three ListCardCounters: `icon="file-text" count=24 tooltip="24 documents"`, `icon="image" count=8 tooltip="8 images"`, `icon="layers" count=3 tooltip="3 sub-folders" href="#sub-folders"` | footer row showing three icon+count pairs; last one is a link |
| Brand guidelines card | two ListCardCounters: `icon="file-text" count=6 tooltip="6 documents"`, `icon="image" count=42 tooltip="42 images"` | footer row with two icon+count pairs |

## 14. Approval And Adoption Notes

- Contract status: `seed contract`
- Approvers: pending
- Downstream adopters: ListCard footer slot
- Future follow-up: consider whether ListCardCounter should accept a
  custom `on:click` handler in addition to href for non-navigation actions
