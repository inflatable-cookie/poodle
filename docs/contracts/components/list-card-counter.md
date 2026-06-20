# ListCardCounter

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `ListCardCounter`
- Layer: `foundation`
- Summary: a compact inline counter with an icon and numeric value, used inside
  the `footer` snippet of ListCard to display item counts (e.g. "24 documents",
  "8 images")
- In scope: icon + count display, optional tooltip wrapping, optional link
  rendering via href, hover color transition for linked counters, tabular-nums
  formatting, click stopPropagation for linked counters
- Out of scope: standalone usage outside ListCard, editable counts, interactive
  states beyond link hover

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
| `onClick` | `((event: MouseEvent) => void) \| null` | `null` | no | Optional click handler; invoked after `stopPropagation` runs for linked counters |
| `typography` | `"label" \| "inherit"` | `"label"` | no | label-sized by default; use `"inherit"` to apply proportional-inherit scaling for text, icon, and gap |

### Slots

None.

### Controlled And Uncontrolled

Fully uncontrolled display component. No internal state.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no href | Icon and count tinted via `color-mix(currentColor 36%, transparent)`; icon at `opacity: 0.82` |
| linked | `href` provided | Renders as `<a>` element with `text-decoration: none` |
| hover (linked) | Mouse over linked counter | Text color transitions to the stronger `color-mix(currentColor 58%, transparent)` tint |

No internal component state. All visual states are derived from props.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| (native click) | Counter clicked | `MouseEvent` | For linked counters `e.stopPropagation()` is called to prevent parent ListCard click from firing; the `onClick` prop (if provided) is then invoked |

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
- `font-size: 0.6875rem`
- `typography="inherit"` uses the proportional-inherit rule from
  `docs/contracts/001-working-rules.md` so text, icon, and gap scale together
- `font-variant-numeric: tabular-nums` for consistent digit widths
- Inline element, wraps with its parent container
- Parent expectations: designed to be placed in the `footer` snippet of ListCard

## 8. Token Usage

### `.list-card-counter`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.25rem` |
| `color` | `color-mix(in srgb, currentColor 36%, transparent)` |
| `font-size` | `0.6875rem` |
| `icon size` | `0.75rem` |
| `icon opacity` | `0.82` |
| `font-variant-numeric` | `tabular-nums` |
| `text-decoration` | `none` |

When `typography="inherit"`:

| Property | Value |
|----------|-------|
| `font-size` | `0.8571em` |
| `gap` | `0.3333em` |
| `icon size` | `1em` |

### `a.list-card-counter:hover`

| Property | Value |
|----------|-------|
| `color` | `color-mix(in srgb, currentColor 58%, transparent)` |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Uses `Icon` primitive for the icon (prop name `icon` of type `IconProp`)
- Uses `Tooltip` primitive wrapper when `tooltip` prop is provided
- Click handler calls `e.stopPropagation()` on linked counters to prevent
  bubbling to the parent ListCard, then invokes the optional `onClick` prop
- Conditional root element: `<a>` when href is set, `<span>` otherwise
- `typography="inherit"` applies proportional-inherit scaling to the text,
  icon, and gap metrics as one inline unit
- Four-branch template handles tooltip x href combinations (tooltip+href,
  tooltip+span, href-only, span-only)

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::primitives::list_card_counter`
- Render as inline flex with icon and text child
- Link behavior may need to be handled via click callback rather than href
- for `typography="inherit"`, non-CSS runtimes may approximate parent-owned
  `em` scaling with ratio-preserving metrics from a 1rem baseline until
  parent-relative inline layout exists

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

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Jetstream tooltip trigger wrapping | current Jetstream tooltip helper is panel-only and does not yet compose a standard trigger+overlay path | allowed | add trigger-wrapped tooltip composition, then honor `tooltip` literally |
| Jetstream linked counter semantics | Jetstream has no native anchor widget in this layer, so linked counters only expose linked styling today | allowed | add shell-owned navigation semantics when the runtime surface supports them |
| tabular numerals | current Jetstream text surface does not expose numeric font-feature controls | allowed | add numeric-feature support, then apply `tabular-nums` literally |

## 13. Specimen Definitions

ListCardCounter does not have a standalone specimen. It is demonstrated within
the **ListCardSpecimen** in the "With footer counters" group.

### With Footer Counters (in ListCardSpecimen)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Design system card | Three ListCardCounters: `icon="file-text" count=24 tooltip="24 documents"`, `icon="image" count=8 tooltip="8 images"`, `icon="layers" count=3 tooltip="3 sub-folders" href="#sub-folders"` | Footer row showing three icon+count pairs; last one is a link |
| Brand guidelines card | Two ListCardCounters: `icon="file-text" count=6 tooltip="6 documents"`, `icon="image" count=42 tooltip="42 images"` | Footer row with two icon+count pairs |
