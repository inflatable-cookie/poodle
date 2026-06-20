# TabStrip

> **Implementation note**: no standalone Svelte component. In Svelte, the tablist is built directly into `Tabs` rather than extracted as a separate primitive. `TabStrip` exists as a standalone component in Jetstream (`tab_strip.rs`) where separating the tablist from panel management is useful.

Status: detailed contract
Updated: 2026-04-11

## 1. Purpose

- Component name: `TabStrip`
- Layer: `foundation`
- Summary: the tablist-only primitive underneath `Tabs`. Renders a horizontal
  or vertical row of tab items without any associated panels or activation
  coordination. Useful when the caller owns the rendered content region and
  only needs the tablist affordance (closable and reorderable tabs, selection
  state, keyboard navigation).
- In scope: ordered item list with value/label/disabled/closable state,
  controlled and uncontrolled selection via `value` / `defaultValue`,
  horizontal and vertical orientation, reorderable flag, aria-label on the
  tablist region
- Out of scope: tab panels (use `Tabs` for the full panel-coupled surface),
  variant styling (text/card/pill/strip — owned by `Tabs`), URL query sync,
  overflow menus

## 2. Anatomy

```text
[Root .tab-strip]  <div role="tablist">
  └── [TabItem]  <button role="tab"> (one per TabStripItem)
        ├── [Label]  <span>
        └── [CloseButton]  <button> (when is_closable)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | tablist container | layout, gap |
| TabItem | yes | single tab button | padding, typography, focus ring |
| CloseButton | no | per-item close affordance | icon button tokens |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `TabStripItem[]` | `[]` | yes | ordered tab definitions |
| `value` | `string \| null` | `null` | no | controlled selected item value |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial selected item |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | layout axis |
| `isReorderable` | `boolean` | `false` | no | when true, tabs can be drag-reordered |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the tablist region |

### Types

```ts
type TabStripItem = {
  value: string;
  label: string;
  isDisabled: boolean;
  isClosable: boolean;
};
```

### Derived Helpers

- `current_value()` — resolves effective selection: `value` → `defaultValue`
  → first non-disabled item
- `current_item()` — `TabStripItem` for the current value
- `closable_item_count()` — count of items with `is_closable` true

### Controlled And Uncontrolled

- controlled: `value` plus `valueChange` event
- uncontrolled: `defaultValue` (internal state tracks selection)
- Close and reorder events are host-owned; TabStrip emits change notifications
  but does not mutate the item list

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| horizontal | `orientation="horizontal"` (default) | row layout with left/right arrow navigation |
| vertical | `orientation="vertical"` | column layout with up/down arrow navigation; labels and close buttons stay visible (TabStrip does not adopt the `Tabs` `strip`-variant icon-only collapse) |
| selected | `value` matches an item | active tab's indicator visible |
| disabled item | `item.isDisabled=true` | item dimmed, not selectable, skipped by arrow keys |
| closable item | `item.isClosable=true` | item renders a close button |
| reorderable | `isReorderable=true` | items support drag reordering |

## 5. Accessibility

- Root: `role="tablist"`, `aria-orientation` matching `orientation`
- Each item: `role="tab"`, `aria-selected`, `aria-controls` (when coupled to a panel)
- Keyboard:
  - `ArrowLeft` / `ArrowRight` (horizontal) or `ArrowUp` / `ArrowDown` (vertical) — move selection
  - `Home` / `End` — first / last enabled tab
  - `Enter` / `Space` — activate (manual mode) or confirm focus
  - `Delete` — close tab when `isClosable`

## 6. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Item gap | `space.inline.sm` | gap between tabs (from `item_gap_token()`); TabStrip follows the text/card list spacing, **not** the `Tabs` `strip` variant's `gap: 0` butted treatment |
| Focus ring | `color.accent.focusRing` | keyboard focus indicator (from `focus_ring_color_token()`) |
| Disabled | `state.opacity.disabled` | disabled item dimming (from `disabled_opacity_token()`) |

## 7. Rust Spec

- Rust type: `poodle_specs::TabStripSpec`
- File: `packages/contracts/components/src/tab_strip.rs`

## 8. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Overlaps with `Tabs` foundation | TabStrip is the strip-only primitive; Tabs adds panel coupling, variants, and activation modes | allowed | document selection guide — use Tabs by default, reach for TabStrip only when panels are host-owned |
| No variant styling | Tabs carries the text/card/pill/strip/block variant system; TabStrip ships one default treatment | allowed | variants can migrate into TabStrip if needed |

## Next Task

Add a selection guide note to `tabs.md` pointing at TabStrip for the
host-owned-panels scenario so the two are clearly differentiated.
