# MetaItem

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `MetaItem`
- Layer: `foundation`
- Summary: a compact labeled metadata item for inline ribbons and header facts
- In scope: optional uppercase label, inline value layout, compact semantics
- Out of scope: copy behavior, status styling, block detail layouts, fetching
  or formatting data

## 2. Public Props

| Prop | Type | Default |
|------|------|---------|
| `label` | `string \| null` | `null` |
| `ariaLabel` | `string \| null` | `null` |
| `typography` | `"body" \| "inherit"` | `"body"` |

## 3. Slots

| Slot | Purpose |
|------|---------|
| default | Value content such as plain text, `Code`, `Pill`, timestamp displays, or links |

## 4. Behavior

- renders an uppercase compact label when `label` is present
- keeps label and value aligned inline and wrapping safely in compact header
  layouts
- allows caller-owned rich value content such as `Code inline` and `Pill`
- `typography="inherit"` uses proportional-inherit scaling so the compact label,
  value text, and inline gap stay proportional inside larger parent copy

## 5. Boundary

- use `MetaItem` inside `MetaBar` or another compact inline metadata context
- use `DetailItem` when the value needs a dedicated row, description, or action
  slot
- caller owns any domain-specific semantics or display logic for the value

## 6. Accessibility

### Semantics

- root may be given `ariaLabel` when label text alone is not sufficient
- if the value content is interactive, the interactive child remains the focus
  target and owns its own semantics

## 7. Token Usage

### `.meta-item`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `flex-wrap` | `wrap` |
| `gap` | `0.375rem` |

### `.meta-item__label`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.6875rem` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `letter-spacing` | `0.08em` |
| `line-height` | `1` |
| `text-transform` | `uppercase` |

### `.meta-item__value`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `0.875rem` |
| `line-height` | `1.4` |

When `typography="inherit"`:

| Property | Value |
|----------|-------|
| `gap` | `0.4286em` |
| label `font-size` | `0.7857em` |
| value `font-size` | `1em` |
| behavior | proportional-inherit rule from `docs/contracts/001-working-rules.md` |

## 8. Runtime Notes

- CSS runtimes should implement `typography="inherit"` literally with
  inherited font metrics and `em`-relative gap and label sizing
- non-CSS runtimes may approximate that proportional behavior with
  ratio-preserving metrics from a 1rem baseline until parent-relative inline
  layout exists

## 9. Keyboard

| Key | Behavior |
|-----|----------|
| none | not interactive unless the slotted value content is interactive |

## 10. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Jetstream label/value typography details | current Jetstream `JsEl` text surface does not yet expose font-family, letter-spacing, or line-height controls | allowed | add richer text styling support, then apply label and value typography literally |
