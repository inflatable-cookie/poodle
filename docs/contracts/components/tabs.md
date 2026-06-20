# Tabs

Status: detailed contract
Updated: 2026-04-01

## 1. Purpose

- Component name: `Tabs`
- Layer: `foundation`
- Summary: a tabbed navigation control that coordinates a tablist and one
  active content panel
- In scope: tablist semantics, tab activation, tab-panel relationship,
  orientation, automatic vs manual activation, visual variants
  (text/card/pill/strip/block), reorderable tabs, closable tabs, tab counts,
  optional visual separators, trailing actions snippet, lightweight URL query sync,
  full-width flex layout, overflow collapse into a menu
- Out of scope: docking

## 2. Anatomy

```text
[Root .poodle-tabs]
  ├── [List .poodle-tabs__list]  role="tablist"
  │     ├── [Item .poodle-tabs__item]...
  │     │     ├── [Tab .poodle-tabs__tab]  role="tab"  <button>
  │     │     │     ├── [Icon] (optional, Icon component using supporting semantic sizing)
  │     │     │     └── [Label .poodle-tabs__label]  <span>
  │     │     ├── [Close .poodle-tabs__close] (optional, when closable)
  │     │     └── [Tooltip] (optional, when `showTooltips`; wraps the tab)
  │     ├── [Collapsed Menu] (optional, when `collapseWhenOverflow` and the list overflows; Menu replacing the tablist)
  │     └── [Actions .poodle-tabs__actions] (optional actions snippet)
  └── [Panel .poodle-tabs__panel]  role="tabpanel" (optional, when `children(activeValue)` snippet exists)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | tabs host, grid container | `space-stack-md` for gap |
| List | yes | navigation container, inline-flex | variant-dependent border/padding |
| Item | yes | wrapper for tab + close button | variant-dependent border/bg |
| Tab | yes | selectable button | text, background, focus ring |
| Label | yes | text content | whitespace, min-width |
| Close | no | close button (when closable) | icon color, hover bg |
| Tooltip | no | hover tooltip over a tab (when `showTooltips`) | Tooltip component tokens |
| Collapsed Menu | no | overflow affordance: collapses the tablist into a `Menu` (when `collapseWhenOverflow` and the list overflows) | Menu component tokens |
| Actions | no | trailing actions snippet | margin-left auto |
| Panel | no | content region (when `children(activeValue)` snippet provided) | border, background, padding |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled active tab |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial active tab |
| `items` | `TabItem[]` | `[]` | yes | tab definitions |
| `variant` | `"text" \| "card" \| "pill" \| "strip" \| "block"` | `"text"` | no | visual variant (`"underline"` is a deprecated alias for `"text"`) |
| `bordered` | `boolean` | `true` | no | when false, hides the bottom border line on the text variant |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | navigation axis |
| `activationMode` | `"automatic" \| "manual"` | `"automatic"` | no | whether focus changes selection |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `reorderable` | `boolean` | `false` | no | enables drag-and-drop and keyboard reorder |
| `collapseWhenOverflow` | `boolean` | `false` | no | when the tablist overflows its container, collapse the tabs into a `Menu` affordance |
| `fullWidth` | `boolean` | `false` | no | tabs flex to fill the row (sets `data-full-width`) |
| `collapseLabel` | `string \| null` | `null` | no | label for the collapsed-overflow trigger; falls back to the active tab label when null |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the tablist |
| `showTooltips` | `boolean` | `false` | no | shows tooltips on tab hover |
| `historyKey` | `string \| null` | `null` | no | syncs the active tab to a URL query param with replaceState |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `onValueChange` | `(value: string) => void` | `undefined` | no | callback fired when the active tab changes |
| `onReorder` | `(items: string[]) => void` | `undefined` | no | callback fired when tabs are reordered |
| `onClose` | `(value: string) => void` | `undefined` | no | callback fired when a tab close is requested |

### TabItem Type

| Field | Type | Default | Required | Notes |
|-------|------|---------|----------|-------|
| `value` | `string` | — | yes | unique identifier |
| `label` | `string` | — | yes | visible text |
| `icon` | `string \| null` | `null` | no | icon registry identifier, renders Icon with supporting semantic sizing |
| `disabled` | `boolean` | `false` | no | prevents activation |
| `closable` | `boolean` | `false` | no | shows close button |
| `count` | `number` | - | no | optional count badge rendered after the label |
| `separator` | `boolean` | `false` | no | draws a visual separator before this tab |

### Controlled And Uncontrolled

- controlled: `value` plus `onValueChange` callback
- uncontrolled: `defaultValue` — internal state tracks selection
- fallback: first non-disabled tab is selected when neither value nor defaultValue is set
- `activationMode` changes whether focus movement commits selection
- `historyKey` mirrors the current tab into `?{historyKey}=...` and restores it on back/forward navigation

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | non-selected tab | `color: text-secondary` |
| selected | active value match | variant-specific bg + `color: text-primary` |
| focus | keyboard focus | `outline: border-width-focus solid accent-focusRing`, `outline-offset: 0.125rem` |
| disabled | `disabled=true` | `opacity: state-opacity-disabled`, `cursor: not-allowed` |
| drag-source | dragging this tab | `opacity: 0.4` |
| drop-target | dragging over this tab | `box-shadow: inset 0 0 0 0.125rem accent-base`, `border-radius: radius-control` |

### Component States

- Selected-tab state: controlled or uncontrolled value tracking
- Roving focus: `focusIndex` tracks which tab has `tabindex="0"`, all others get `tabindex="-1"`
- Drag state: `dragSourceIndex` and `dropTargetIndex` for reorder

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | active tab changes | `string` | called on click, or on focus when `activationMode="automatic"` |
| `onReorder` | tab order changes | `string[]` | new value order array |
| `onClose` | close button clicked or Delete key on closable tab | `string` | tab value being closed |

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
| `Alt+Arrow` | reorders tab (when reorderable) |
| `Delete` | closes tab (when closable) |
| `Tab` | moves between the tablist and active panel |

### Focus And Announcement

- focus entry: roving tabindex — one tab at `tabindex="0"`, all others at `-1`
- focus tracks selectedIndex: when selectedIndex changes, focusIndex updates to match
- focus exit: panel is focusable via `tabindex="0"`

## 7. Layout

### Sizing

- Root: `display: grid`, `gap: space-stack-md`, `min-width: 0`
- Vertical: `grid-template-columns: auto minmax(0, 1fr)`, `align-items: start`
- List: `display: inline-flex`, `flex-wrap: wrap` (text), `flex-wrap: nowrap` (card/pill)
- Card/Pill/Strip overflow: `overflow-x: auto; overflow-y: hidden`
- Item: `display: inline-flex`, `align-items: center`, `min-width: 0`, `position: relative`

### Composition

- parent expectations: settings panels, inspectors, content areas
- child expectations: panel receives arbitrary content via `children(activeValue)` snippet
- resizing: tab selection should not cause layout jump
- hierarchy guidance: tab labels should stay as leaf surface names, not
  breadcrumb chains or section trails
- if ancestor context is needed, put it around the tabset in surrounding
  `PageHeader`/breadcrumbs UI rather than encoding it into each tab label

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-md)` |
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

### List — Text variant

| Property | Value |
|----------|-------|
| `padding-bottom` | `0.25rem` |
| `border-bottom` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent)` |

When `bordered` is `false`, the `border-bottom` is removed (set to `0`).

### List — Text vertical

| Property | Value |
|----------|-------|
| `flex-direction` | `column` |
| `padding-bottom` | `0` |
| `padding-right` | `0.5rem` |
| `border-bottom` | `0` |
| `border-right` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent)` |

### List — Card + Pill + Strip + Block

| Property | Value |
|----------|-------|
| `flex-wrap` | `nowrap` |
| `overflow-x` | `auto` |
| `overflow-y` | `hidden` |

### List — Card + Pill + Strip + Block vertical

| Property | Value |
|----------|-------|
| `flex-direction` | `column` |

### List — Pill variant (overrides)

| Property | Value |
|----------|-------|
| `width` | `fit-content` |
| `padding` | `0.1875rem` |
| `border` | `0.125rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 68%, transparent)` |
| `border-radius` | `999px` |
| `gap` | `0.125rem` |

### Item — Card variant

| Property | Value |
|----------|-------|
| `gap` | `0` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 68%, transparent)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 92%, transparent)` |

### Item — Card variant (selected)

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 32%, var(--poodle-color-border-subtle))` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 14%, var(--poodle-color-background-surface))` |

### Tab button (all variants)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-height` | `calc(var(--poodle-size-control-height) - 0.25rem)` |
| `padding` | `0 0.75rem` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1` |
| `white-space` | `nowrap` |

### Tab — Text variant

| Property | Value |
|----------|-------|
| `border-radius` | `var(--poodle-radius-control)` |

### Tab — Text variant (selected)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |

### Tab — Card variant

| Property | Value |
|----------|-------|
| `padding` | `0 0.5rem` |
| `color` | `var(--poodle-color-text-primary)` |

### Tab — Pill variant

| Property | Value |
|----------|-------|
| `min-height` | `calc(var(--poodle-size-control-height) - 0.5rem)` |
| `padding` | `0 0.625rem` |
| `border-radius` | `999px` |

### Tab — Pill variant (selected)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |

### List — Strip variant

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `gap` | `0` |
| `padding` | `0 var(--poodle-space-panel-x, 0.75rem)` |
| `border-bottom` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent)` |

### List — Block variant

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `width` | `fit-content` |
| `max-width` | `100%` |
| `gap` | `0` |
| `padding` | `0` |
| `border-bottom` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 90%, transparent)` |

### Item — Strip variant

| Property | Value |
|----------|-------|
| `border-bottom` | `0.125rem solid transparent` |
| `margin-bottom` | `-0.0625rem` |

### Item — Block variant

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex` | `0 0 auto` |
| `min-width` | `0` |
| `separator` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent)` between sibling items |

### Item — Strip variant (selected)

| Property | Value |
|----------|-------|
| `border-bottom-color` | `var(--poodle-color-accent-base)` |

### Item — Strip variant (hover)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-surface-hover) 50%, transparent)` |

### Tab — Strip variant

| Property | Value |
|----------|-------|
| `min-height` | `2.25rem` |
| `padding` | `0 0.625rem` |
| `border-radius` | `0` |

### Tab — Block variant

| Property | Value |
|----------|-------|
| `justify-content` | `center` |
| `width` | `auto` |
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border-radius` | `0` |

### Item — Block variant (selected)

Note: In the block variant, the selected background is applied on the **item wrapper**, not the tab button itself.

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 14%, var(--poodle-color-background-surface))` |
| `color` | `var(--poodle-color-text-primary)` |

### Item — Block variant (hover)

Note: In the block variant, the hover background is applied on the **item wrapper**, not the tab button itself.

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-surface-hover) 40%, transparent)` |

### Tab — Strip variant (selected)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

### List — Strip vertical

| Property | Value |
|----------|-------|
| `padding` | `0` |
| `overflow` | `visible` |
| `border-bottom` | `0` |
| `border-right` | `0.0625rem solid var(--poodle-color-border-subtle)` |

### Item — Strip vertical

| Property | Value |
|----------|-------|
| `border-bottom` | `0` |
| `border-right` | `0.125rem solid transparent` |
| `margin-bottom` | `0` |
| `margin-right` | `-0.125rem` |

### Item — Strip vertical (selected)

| Property | Value |
|----------|-------|
| `border-right-color` | `var(--poodle-color-accent-base)` |

### Item — Strip vertical (first-child)

| Property | Value |
|----------|-------|
| `padding-top` | `0.75rem` |

### Item — Strip vertical (last-child)

| Property | Value |
|----------|-------|
| `padding-bottom` | `0.75rem` |

### Tab — Strip vertical

| Property | Value |
|----------|-------|
| `justify-content` | `center` |
| `min-height` | `0` |
| `min-width` | `2.25rem` |
| `padding` | `0.5rem` |

### Tab — Block vertical (selected)

| Property | Value |
|----------|-------|
| `background` | same selected fill treatment without an accent edge |

### Vertical orientation — label + close

| Property | Value |
|----------|-------|
| `.poodle-tabs__label` `display` | `none` |
| `.poodle-tabs__close` `display` | `none` |

### Tab — Focus

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Tab — Disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

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
| `min-height` | `0` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `calc(var(--poodle-radius-control) - 0.125rem)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |
| `margin-right` | `0.25rem` |

### Close button — Hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 84%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |

### Close button — Focus

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Full-width (`data-full-width="true"`, non-vertical)

Applies when `fullWidth` is set and orientation is horizontal.

| Selector | Property | Value |
|----------|----------|-------|
| List | `display` | `flex` |
| List | `width` | `100%` |
| Item | `flex` | `1 1 0` |
| Tab | `width` | `100%` |
| Tab | `justify-content` | `center` |

### Actions snippet

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `margin-left` | `auto` |

### Panel

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent)` |

### Drag-and-drop states

| State | Property | Value |
|-------|----------|-------|
| draggable item | `cursor` | `grab` |
| drag source | `opacity` | `0.4` |
| drop target | `box-shadow` | `inset 0 0 0 0.125rem var(--poodle-color-accent-base)` |
| drop target | `border-radius` | `var(--poodle-radius-control)` |

## 9. Svelte Notes

- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- Uses `data-variant`, `data-orientation`, `data-selected` data attributes for styling
- Roving focus via manual tabindex management and `bind:this` on tab buttons
- Uncontrolled mode: internal `uncontrolledValue` state, controlled mode: `value` prop passthrough
- Items list can be reordered via drag events or keyboard Alt+Arrow
- Module-level `nextTabsId` counter for unique IDs across instances
- Close button stops click propagation so the parent tab does not also activate
- `children(activeValue)` receives `activeValue` as snippet argument
- `data-full-width` — set when `fullWidth` is true; drives the full-width flex layout (non-vertical only)
- `showTooltips` wraps each tab in a `Tooltip`; for vertical/icon-only tabs the tooltip surfaces the hidden label
- `collapseWhenOverflow` measures the tablist against its container and, on overflow, replaces the tabs with a `Menu` trigger labeled by `collapseLabel` (falling back to the active tab label)
- Variant resolution: `variant="underline"` is normalized to `"text"` (`resolvedVariant`); the rendered `data-variant` is `"text"`. `"text"` is the canonical Svelte name and the default

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::tabs`
- Spec struct: `TabsSpec` in primitives crate holds tab definitions + variant
- Component struct: `PoodleTabs` in components crate renders via `IntoElement`
- Opacity multipliers centralized in spec: `pill_border_opacity() -> 0.68`, `pill_active_bg_opacity() -> 0.18`
- Note: `"underline"` is accepted as a deprecated alias for `"text"` in the variant prop. The Rust `TabVariant` enum names this canonical member `Underline` (enum: `Underline | Card | Pill | Block`); it is the same variant Svelte renders as `data-variant="text"`. The naming difference is implementation-side only — both target Svelte's text/underline variant.
- GPUI must model `color-mix` as `token.opacity(token.a * multiplier)` since GPUI has no CSS color-mix
- Text variant border opacity: 82% → `0.82` multiplier on border-subtle
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

- [ ] all five variants render with exact token/dimension match
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

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Text variant (default, with panel)

Text tabs with associated panel content:

| Tab label | Panel content | State |
|-----------|--------------|-------|
| Overview | "Overview content goes here." | active (default) |
| Features | "Features content goes here." | inactive |
| Pricing | "Pricing content goes here." | inactive |
| FAQ | — | disabled |

### Card variant (closable, reorderable)

Card tabs simulating file tabs:

| Tab label | Props |
|-----------|-------|
| index.ts | active (default) |
| App.svelte | closable |
| utils.ts | closable |
| types.ts | closable |

### Pill variant (with icons)

Pill tabs with leading icons:

| Tab label | Icon | State |
|-----------|------|-------|
| Home | house | active (default) |
| Settings | settings | inactive |
| Users | users | inactive |

### Text (with icons, no panel)

Text tabs with icons and no panel below:

| Tab label | Icon | State |
|-----------|------|-------|
| Home | house | active (default) |
| Settings | settings | inactive |
| Users | users | inactive |

### Strip variant (full-width bar with icons, closable, reorderable)

Full-width strip tabs:

| Tab label | Icon | State |
|-----------|------|-------|
| Editor | code | active (default) |
| Preview | eye | inactive, closable |
| Terminal | terminal | inactive, closable |
| Output | file-text | inactive, closable |

### Strip variant — vertical (icon-only, collapsed panel)

Vertical strip with icon-only tabs:

| Icon | aria-label | State |
|------|------------|-------|
| folder | Explorer | active (default) |
| search | Search | inactive |
| layers | Source Control | inactive |
| terminal | Debug | inactive |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: settings navigation, inspectors, sectional work areas
- future follow-up: overflow-tab affordances, persistence patterns
