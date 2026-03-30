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

## 3. Slots

| Slot | Purpose |
|------|---------|
| default | Value content such as plain text, `Code`, `Pill`, timestamp displays, or links |

## 4. Behavior

- renders an uppercase compact label when `label` is present
- keeps label and value aligned inline and wrapping safely in compact header
  layouts
- allows caller-owned rich value content such as `Code inline` and `Pill`

## 5. Boundary

- use `MetaItem` inside `MetaBar` or another compact inline metadata context
- use `DetailRow` when the value needs a dedicated row, description, or action
  slot
- caller owns any domain-specific semantics or display logic for the value

## 6. Accessibility

### Semantics

- root may be given `ariaLabel` when label text alone is not sufficient
- if the value content is interactive, the interactive child remains the focus
  target and owns its own semantics

### Keyboard

| Key | Behavior |
|-----|----------|
| none | not interactive unless the slotted value content is interactive |
