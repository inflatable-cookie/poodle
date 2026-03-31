# MetaBar

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `MetaBar`
- Layer: `foundation`
- Summary: an inline wrapping metadata ribbon for page headers and compact
  contextual facts
- In scope: horizontal metadata layout, wrapping, optional visual separators
- Out of scope: value semantics, status treatment, copy affordances, detail-item
  block layout

## 2. Public Props

| Prop | Type | Default |
|------|------|---------|
| `ariaLabel` | `string \| null` | `null` |
| `showSeparators` | `boolean` | `true` |

## 3. Slots

| Slot | Purpose |
|------|---------|
| default | Inline metadata items such as `MetaItem`, `Pill`, `Code`, or caller-owned content |

## 4. Behavior

- lays out children in a wrapping inline row
- keeps metadata visually grouped but distinct from surrounding header copy
- inserts subtle separators between adjacent child items when
  `showSeparators` is true
- does not impose any item-level semantics beyond layout

## 5. Boundary

- use `MetaBar` for compact header or summary metadata
- use `DetailItem` for block-level label/value presentation in detail sections
- callers own item semantics such as copyable IDs, status pills, timestamps,
  and links

## 6. Accessibility

### Semantics

- root may be given `ariaLabel` when the metadata ribbon needs a named group
- child content is caller-owned and should provide its own semantics where
  needed

### Keyboard

| Key | Behavior |
|-----|----------|
| none | layout-only container; interactive behavior comes from child content |
