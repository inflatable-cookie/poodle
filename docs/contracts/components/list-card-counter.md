# ListCardCounter

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `ListCardCounter`
- Layer: `foundation`
- Summary: a compact inline counter with an icon and numeric value, used inside
  the footer slot of ListCard to display item counts (e.g. "24 documents",
  "8 images")
- In scope: icon + count display, optional tooltip wrapping, optional link
  rendering via href, hover color transition for linked counters, tabular-nums
  formatting, click stopPropagation for linked counters
- Out of scope: standalone usage outside ListCard, editable counts, interactive
  states beyond link hover, custom click handlers

## 2. Anatomy

```text
[Tooltip]  (optional wrapper, when tooltip prop is provided)
  └── [Root .list-card-counter]  <span> or <a>
        ├── [Icon]   Icon primitive
        └── [Count]  <span> with numeric value
```

| Part | Element | Notes |
|------|---------|-------|
| Tooltip | `Tooltip` | Wraps Root only when `tooltip` prop is provided |
| Root | `<span>` or `<a>` | `<a>` when `href` is provided, `<span>` otherwise; class `list-card-counter` |
| Icon | `Icon` | Renders the icon via the `icon` prop |
| Count | `<span>` | Displays the numeric `count` value |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `icon` | `IconProp` | -- | yes | Icon name or nodes passed to the Icon primitive |
| `count` | `number` | -- | yes | Numeric value to display |
| `tooltip` | `string \| null` | `null` | no | Tooltip text; when provided, Root is wrapped in a Tooltip |
| `href` | `string \| null` | `null` | no | When provided, Root renders as an `<a>` element with this href |

### Slots

None.

### Controlled And Uncontrolled

Fully uncontrolled display component. No internal state.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no href | Icon and count in secondary text color |
| linked | `href` provided | Renders as `<a>` element with `text-decoration: none` |
| hover (linked) | Mouse over linked counter | Text color transitions from secondary to primary |

No internal component state. All visual states are derived from props.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| (native click) | Linked counter clicked | -- | `e.stopPropagation()` is called to prevent parent ListCard click from firing |

## 6. Accessibility

- Root renders as `<span>` (static) or `<a>` (linked) -- semantic element chosen by href prop
- When tooltip is provided, the Tooltip primitive handles `aria-describedby`
- Icon is decorative (inherits meaning from tooltip or surrounding context)
- `Tab` focuses linked counters (native `<a>` behavior)
- `Enter` activates the link (native `<a>` behavior)
- Focus ring: browser default for `<a>` elements

## 7. Layout

- `display: inline-flex` with `align-items: center`
- `gap: 0.25rem`
- `font-size: 0.75rem`
- `font-variant-numeric: tabular-nums` for consistent digit widths
- Inline element, wraps with its parent container
- Parent expectations: designed to be placed in the `footer` slot of ListCard

## 8. Token Usage

### `.list-card-counter`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.25rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.75rem` |
| `font-variant-numeric` | `tabular-nums` |
| `text-decoration` | `none` |

### `a.list-card-counter:hover`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Uses `Icon` primitive for the icon (prop name `icon` of type `IconProp`)
- Uses `Tooltip` primitive wrapper when `tooltip` prop is provided
- Click handler calls `e.stopPropagation()` on linked counters to prevent
  bubbling to the parent ListCard
- Conditional root element: `<a>` when href is set, `<span>` otherwise
- Four-branch template handles tooltip x href combinations (tooltip+href,
  tooltip+span, href-only, span-only)

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::primitives::list_card_counter`
- Render as inline flex with icon and text child
- Link behavior may need to be handled via click callback rather than href

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] icon renders correctly across all implementations
- [ ] stopPropagation behavior matches for linked counters
- [ ] tooltip wrapping behavior matches

### Tier 2: Visual Parity

- [ ] gap, font-size, and color tokens match
- [ ] tabular-nums applied
- [ ] hover color transition matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Specimen Definitions

ListCardCounter does not have a standalone specimen. It is demonstrated within
the **ListCardSpecimen** in the "With footer counters" group.

### With Footer Counters (in ListCardSpecimen)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Design system card | Three ListCardCounters: `icon="file-text" count=24 tooltip="24 documents"`, `icon="image" count=8 tooltip="8 images"`, `icon="layers" count=3 tooltip="3 sub-folders" href="#sub-folders"` | Footer row showing three icon+count pairs; last one is a link |
| Brand guidelines card | Two ListCardCounters: `icon="file-text" count=6 tooltip="6 documents"`, `icon="image" count=42 tooltip="42 images"` | Footer row with two icon+count pairs |
