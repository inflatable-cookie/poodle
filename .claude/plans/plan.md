# ListCard Expansion Plan

## Summary

Expand ListCard to support richer item patterns needed by Underlay projects: media thumbnails, context menu actions, inline badges/pills, and child-count footers.

## Design Decisions (from user feedback)

- **Leading shape**: `leadingShape` prop on ListCard (`"circle"` default | `"rounded-square"`) — component controls the shape
- **Item actions**: Context menu on the whole card (not an icon-button overlay)
- **Badges area**: New `badges` slot on the **title row** (right-aligned alongside title)
- **Meta**: Kept as-is (vertically centered on the right)
- **Footer counters**: Named `footer` slot + a small `ListCardCounter` helper component; counters can be links

## New Anatomy

```
[Root .list-card]
  ├── [Leading .list-card__leading]  (optional, circle or rounded-square)
  │   └── slot "leading" — icon, avatar, or <img> thumbnail
  ├── [Body .list-card__body]  (replaces __content)
  │   ├── [Header row .list-card__header]  (title + badges, single line)
  │   │   ├── [Title .list-card__title]
  │   │   └── [Badges .list-card__badges]  (optional slot, flex-shrink-0)
  │   ├── [Subtitle .list-card__subtitle]  (optional)
  │   └── [Footer .list-card__footer]  (optional slot — counters row)
  ├── [Meta .list-card__meta]  (optional, vertically centered)
  └── [Trailing .list-card__trailing]  (optional slot)
```

## Changes

### 1. ListCard.svelte — New prop + slots + structural changes

- Add `leadingShape: "circle" | "rounded-square"` prop (default `"circle"`)
- Add `badges` slot in header row right of title
- Add `footer` slot below subtitle
- Rename `__content` to `__body`, wrap title in `__header` flex row

### 2. ListCard.svelte — CSS

- Leading: `data-leading-shape="rounded-square"` sets `border-radius: var(--pug-radius-control)`
- Header row: `display: flex; align-items: baseline; gap: 0.375rem`
- Badges: `flex-shrink: 0; display: flex; gap: 0.25rem`
- Footer: `display: flex; align-items: center; gap: 0.5rem`

### 3. ListCardCounter.svelte — New helper component

Props: `icon`, `count`, `tooltip`, `href` (optional, renders as `<a>`)
Renders: inline-flex icon + count at 0.75rem, secondary text color
Click stops propagation when href is set so it doesn't trigger card click

### 4. Context Menu — Composition pattern (no ListCard changes needed)

ContextMenu wraps ListCard as its trigger. Specimen demonstrates the pattern.

### 5. Specimen updates — New demo groups

- With thumbnail (rounded-square leading)
- With badges (pills in title row)
- With footer counters (ListCardCounter, some as links)
- With context menu (ContextMenu wrapping ListCard)

### 6. Contract update — list-card.md

- Updated anatomy, new props, new slots, new token tables
- Document ListCardCounter helper
- Context menu composition in Svelte Notes

### 7. Exports — index.ts

- Export ListCardCounter from primitives

## Files

1. `packages/svelte/primitives/src/ListCard.svelte` — modify
2. `packages/svelte/primitives/src/ListCardCounter.svelte` — new file
3. `packages/svelte/primitives/src/index.ts` — add export
4. `packages/svelte/preview/src/specimens/ListCardSpecimen.svelte` — modify
5. `docs/contracts/foundation/list-card.md` — modify
