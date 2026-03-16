# Tabs

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Tabs`
- Layer: `foundation`
- Summary: a tabbed navigation control that coordinates a tablist and one
  active content panel
- In scope: tablist semantics, tab activation, tab-panel relationship,
  orientation, automatic vs manual activation, three visual variants
  (underline/card/pill), reorderable tabs, closable tabs, actions slot
- Out of scope: docking, overflow menus, tab persistence

## 2. Anatomy

```text
[Root .pug-tabs]
  ├── [List .pug-tabs__list]  role="tablist"
  │     ├── [Item .pug-tabs__item]...
  │     │     ├── [Tab .pug-tabs__tab]  role="tab"  <button>
  │     │     │     ├── [Icon] (optional, Icon component size="sm")
  │     │     │     └── [Label .pug-tabs__label]  <span>
  │     │     └── [Close .pug-tabs__close] (optional, when isClosable)
  │     └── [Actions .pug-tabs__actions] (optional slot)
  └── [Panel .pug-tabs__panel]  role="tabpanel" (optional, when slot content exists)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | tabs host, grid container | `space-stack-md` for gap |
| List | yes | navigation container, inline-flex | variant-dependent border/padding |
| Item | yes | wrapper for tab + close button | variant-dependent border/bg |
| Tab | yes | selectable button | text, background, focus ring |
| Label | yes | text content | whitespace, min-width |
| Close | no | close button (when isClosable) | icon color, hover bg |
| Actions | no | trailing slot | margin-left auto |
| Panel | no | content region (when slot provided) | border, background, padding |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled active tab |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial active tab |
| `items` | `TabItem[]` | `[]` | yes | tab definitions |
| `variant` | `"underline" \| "card" \| "pill"` | `"underline"` | no | visual variant |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | navigation axis |
| `activationMode` | `"automatic" \| "manual"` | `"automatic"` | no | whether focus changes selection |
| `isReorderable` | `boolean` | `false` | no | enables drag-and-drop and keyboard reorder |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the tablist |

### TabItem Type

| Field | Type | Default | Required | Notes |
|-------|------|---------|----------|-------|
| `value` | `string` | — | yes | unique identifier |
| `label` | `string` | — | yes | visible text |
| `icon` | `string \| null` | `null` | no | icon registry identifier, renders Icon size="sm" |
| `isDisabled` | `boolean` | `false` | no | prevents activation |
| `isClosable` | `boolean` | `false` | no | shows close button |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange` event
- uncontrolled: `defaultValue` — internal state tracks selection
- fallback: first non-disabled tab is selected when neither value nor defaultValue is set
- `activationMode` changes whether focus movement commits selection

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | non-selected tab | `color: text-secondary` |
| selected | active value match | variant-specific bg + `color: text-primary` |
| focus | keyboard focus | `outline: border-width-focus solid accent-focusRing`, `outline-offset: 0.125rem` |
| disabled | `isDisabled=true` | `opacity: state-opacity-disabled`, `cursor: not-allowed` |
| drag-source | dragging this tab | `opacity: 0.4` |
| drop-target | dragging over this tab | `box-shadow: inset 0 0 0 0.125rem accent-base`, `border-radius: radius-control` |

### Component States

- Selected-tab state: controlled or uncontrolled value tracking
- Roving focus: `focusIndex` tracks which tab has `tabindex="0"`, all others get `tabindex="-1"`
- Drag state: `dragSourceIndex` and `dropTargetIndex` for reorder

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `valueChange` | active tab changes | `{ value: string }` | fires on click, or on focus when activationMode="automatic" |
| `reorder` | tab order changes | `{ items: string[] }` | new value order array |
| `close` | close button clicked or Delete key on closable tab | `{ value: string }` | tab value being closed |

## 6. Accessibility

### Semantics

- Root: no role (container div)
- List: `role="tablist"`, `aria-label` from prop, `aria-orientation` from prop
- Tab: `role="tab"`, `aria-selected`, `aria-controls` (links to panel id)
- Panel: `role="tabpanel"`, `tabindex="0"`, `aria-labelledby` (links to tab id)
- Close: `aria-label="Close {tab.label}"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Left/Right` | moves focus in horizontal tablists (wraps via findNextEnabledIndex) |
| `Arrow Up/Down` | moves focus in vertical tablists |
| `Home` | moves focus to first enabled tab |
| `End` | moves focus to last enabled tab |
| `Enter` or `Space` | activates focused tab in manual mode |
| `Alt+Arrow` | reorders tab (when isReorderable) |
| `Delete` | closes tab (when isClosable) |
| `Tab` | moves between the tablist and active panel |

### Focus And Announcement

- focus entry: roving tabindex — one tab at `tabindex="0"`, all others at `-1`
- focus tracks selectedIndex: when selectedIndex changes, focusIndex updates to match
- focus exit: panel is focusable via `tabindex="0"`

## 7. Layout

### Sizing

- Root: `display: grid`, `gap: space-stack-md`, `min-width: 0`
- Vertical: `grid-template-columns: auto minmax(0, 1fr)`, `align-items: start`
- List: `display: inline-flex`, `flex-wrap: wrap` (underline), `flex-wrap: nowrap` (card/pill)
- Card/Pill overflow: `overflow: auto`
- Item: `display: inline-flex`, `align-items: center`, `min-width: 0`

### Composition

- parent expectations: settings panels, inspectors, content areas
- child expectations: panel receives arbitrary content via slot
- resizing: tab selection should not cause layout jump

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--pug-space-stack-md)` |
| `min-width` | `0` |

### Root (vertical orientation)

| Property | Value |
|----------|-------|
| `grid-template-columns` | `auto minmax(0, 1fr)` |
| `align-items` | `start` |

### List (all variants)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `stretch` |
| `gap` | `0.25rem` |

### List — Underline variant

| Property | Value |
|----------|-------|
| `padding-bottom` | `0.25rem` |
| `border-bottom` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 82%, transparent)` |

### List — Underline vertical

| Property | Value |
|----------|-------|
| `flex-direction` | `column` |
| `padding-bottom` | `0` |
| `padding-right` | `0.5rem` |
| `border-bottom` | `0` |
| `border-right` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 82%, transparent)` |

### List — Card + Pill

| Property | Value |
|----------|-------|
| `flex-wrap` | `nowrap` |
| `overflow` | `auto` |

### List — Card + Pill vertical

| Property | Value |
|----------|-------|
| `flex-direction` | `column` |

### List — Pill variant (overrides)

| Property | Value |
|----------|-------|
| `width` | `fit-content` |
| `padding` | `0.1875rem` |
| `border` | `0.125rem solid color-mix(in srgb, var(--pug-color-border-subtle) 68%, transparent)` |
| `border-radius` | `999px` |
| `gap` | `0.125rem` |

### Item — Card variant

| Property | Value |
|----------|-------|
| `gap` | `0` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 68%, transparent)` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `color-mix(in srgb, var(--pug-color-background-surface) 92%, transparent)` |

### Item — Card variant (selected)

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--pug-color-accent-base) 32%, var(--pug-color-border-subtle))` |
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 14%, var(--pug-color-background-surface))` |

### Tab button (all variants)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--pug-space-inline-sm)` |
| `min-height` | `calc(var(--pug-size-control-height) - 0.25rem)` |
| `padding` | `0 0.75rem` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-secondary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1` |
| `white-space` | `nowrap` |

### Tab — Underline variant

| Property | Value |
|----------|-------|
| `border-radius` | `var(--pug-radius-control)` |

### Tab — Underline variant (selected)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent)` |
| `color` | `var(--pug-color-text-primary)` |

### Tab — Card variant

| Property | Value |
|----------|-------|
| `min-height` | `calc(var(--pug-size-control-height) - 0.75rem)` |
| `padding` | `0 0.625rem` |
| `color` | `var(--pug-color-text-primary)` |

### Tab — Pill variant

| Property | Value |
|----------|-------|
| `min-height` | `calc(var(--pug-size-control-height) - 0.5rem)` |
| `padding` | `0 0.625rem` |
| `border-radius` | `999px` |

### Tab — Pill variant (selected)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent)` |
| `color` | `var(--pug-color-text-primary)` |

### Tab — Focus

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Tab — Disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Label

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `white-space` | `nowrap` |

### Close button

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.25rem` |
| `height` | `1.25rem` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `calc(var(--pug-radius-control) - 0.125rem)` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-secondary)` |
| `cursor` | `pointer` |

### Close button — Hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-background-surface) 84%, transparent)` |
| `color` | `var(--pug-color-text-primary)` |

### Close button — Focus

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Actions slot

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `margin-left` | `auto` |

### Panel

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `padding` | `var(--pug-space-panel-y) var(--pug-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 74%, transparent)` |
| `border-radius` | `var(--pug-radius-surface)` |
| `background` | `color-mix(in srgb, var(--pug-color-background-panel) 96%, transparent)` |

### Drag-and-drop states

| State | Property | Value |
|-------|----------|-------|
| draggable item | `cursor` | `grab` |
| drag source | `opacity` | `0.4` |
| drop target | `box-shadow` | `inset 0 0 0 0.125rem var(--pug-color-accent-base)` |
| drop target | `border-radius` | `var(--pug-radius-control)` |

## 9. Svelte Notes

- Uses `data-variant`, `data-orientation`, `data-selected` data attributes for styling
- Roving focus via manual tabindex management and `bind:this` on tab buttons
- Uncontrolled mode: internal `uncontrolledValue` state, controlled mode: `value` prop passthrough
- Items list can be reordered via drag events or keyboard Alt+Arrow
- Module-level `nextTabsId` counter for unique IDs across instances
- Close button uses `on:click|stopPropagation`
- Panel slot receives `activeValue` as slot prop

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::tabs`
- Spec struct: `TabsSpec` in primitives crate holds tab definitions + variant
- Component struct: `PugTabs` in components crate renders via `IntoElement`
- Opacity multipliers centralized in spec: `pill_border_opacity() -> 0.68`, `pill_active_bg_opacity() -> 0.18`
- GPUI must model `color-mix` as `token.opacity(token.a * multiplier)` since GPUI has no CSS color-mix
- Underline border opacity: 82% → `0.82` multiplier on border-subtle
- Card item border opacity: 68% → `0.68` multiplier on border-subtle
- Card item bg opacity: 92% → `0.92` on background-surface
- Card selected: accent 32% mix + border-subtle (not simple opacity), accent 14% mix + background-surface
- Panel border: 74% → `0.74` on border-subtle; panel bg: 96% → `0.96` on background-panel

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] tablist, tab, and tabpanel semantics match
- [ ] keyboard navigation and activation-mode behavior match
- [ ] roving focus (tabindex 0/-1) matches
- [ ] reorder keyboard (Alt+Arrow) and drag-and-drop behavior matches
- [ ] close button and Delete key behavior matches
- [ ] controlled/uncontrolled value resolution matches
- [ ] fallback to first enabled tab matches

### Tier 2: Visual Parity

- [ ] all three variants render with exact token/dimension match
- [ ] color-mix percentages match (82%, 68%, 18%, 14%, 32%, 74%, 92%, 96%)
- [ ] font-size 0.75rem, font-weight 600, line-height 1 match
- [ ] min-height calc expressions match per variant
- [ ] padding values match per variant
- [ ] focus ring style matches
- [ ] disabled opacity matches
- [ ] drag-and-drop visual states match

### Tier 3: Implementation Freedom

- [ ] panel mounting strategy (keep-alive vs unmount) is implementation-owned
- [ ] ID generation scheme is implementation-owned
- [ ] indicator animation internals are implementation-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Inactive panels may stay mounted or unmounted | runtime rendering strategy differs | allowed | keep semantics and state continuity strict |
| GPUI uses opacity multiplication instead of CSS color-mix | platform capability | allowed | visual result must match |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings navigation, inspectors, sectional work areas
- future follow-up: overflow-tab affordances, persistence patterns
