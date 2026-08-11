# Tabs

Status: detailed contract
Updated: 2026-07-29

## 1. Purpose

- Component name: `Tabs`
- Layer: `foundation`
- Summary: a tabbed navigation control that coordinates a tablist and one
  active content panel
- In scope: tablist semantics, tab activation, tab-panel relationship,
  orientation, automatic vs manual activation, visual variants
  (card/pill/block/strip), reorderable tabs, closable tabs, tab counts,
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
| `variant` | `"card" \| "pill" \| "block" \| "strip"` | `"card"` | no | visual variant; `"card"` is the default |
| `activeOutline` | `boolean` | `false` | no | opt-in outline on the active tab — the decoration the former `card` variant had by default (selected item border `accent-base` 32% mixed with `border-subtle`) |
| `activeFill` | `"tint" \| "solid"` | `"tint"` | no | selection treatment on the active tab: `tint` is the accent-tinted fill, `solid` fills the tab fully with `accent-base` and swaps the foreground to `text-inverse` for contrast |
| `bordered` | `boolean` | `true` | no | card variant only: draws the separating border on the list — bottom when horizontal, right when vertical — **and the outer padding that holds the tabs off it**. When false the strip renders flush to its container in both orientations, and the consumer owns any spacing beneath. Use `bordered={false}` for titlebars, toolbars and other confined layouts where the tabs are not above content |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | no | navigation axis |
| `activationMode` | `"automatic" \| "manual"` | `"automatic"` | no | whether focus changes selection |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `reorderable` | `boolean` | `false` | no | enables drag-and-drop and keyboard reorder |
| `collapseWhenOverflow` | `boolean` | `false` | no | when the tablist overflows its container, collapse the tabs into a `Menu` affordance |
| `overflowStrategy` | `"collapse" \| "shed"` | `"collapse"` | no | `collapse` is the single threshold into a `Menu`; `shed` gives up decoration first |
| `shed` | `("icon" \| "count")[]` | `["icon", "count"]` | no | which parts to give up, in order, when `overflowStrategy` is `shed` |
| `fullWidth` | `boolean` | `false` | no | tabs flex to fill the row (sets `data-full-width`) |
| `collapseLabel` | `string \| null` | `null` | no | label for the collapsed-overflow trigger; falls back to the active tab label when null |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for the tablist |
| `showTooltips` | `boolean` | `false` | no | shows tooltips on tab hover |
| `historyKey` | `string \| null` | `null` | no | syncs the active tab to a URL query param with replaceState |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `onValueChange` | `(value: string) => void` | `undefined` | no | callback fired when the active tab changes |
| `onReorder` | `(items: string[]) => void` | `undefined` | no | callback fired when tabs are reordered |
| `onClose` | `(value: string) => void` | `undefined` | no | callback fired when a tab close is requested |
| `onDragPrepare` | `(value: string, event: PointerEvent) => void` | `undefined` | no | primary pointer-down seam for an owning composite that must prepare before native `dragstart` |
| `onDragStart` | `(value: string, event: DragEvent) => void` | `undefined` | no | native drag-start seam after Tabs has established local reorder state |
| `onDragEnd` | `(value: string, event: DragEvent) => void` | `undefined` | no | native drag-end seam before Tabs clears the dragged value |

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

### Graded Overflow

`collapse` has one response to not fitting: the whole strip becomes a menu.
Between "everything fits" and "nothing fits" there is a wide band where a tab
can keep its label by giving up its icon, and then its count — the labels
themselves fit long after the decoration stops being affordable.

`shed` walks that ladder: **full → without icons → without counts → collapsed**,
choosing the richest level that fits. The final collapse happens only if
`collapseWhenOverflow` is also set; otherwise the strip stays fully shed and
overflows as it does today.

- **Icons before counts**, because an icon usually repeats what the label
  already says where a count carries information the label does not. `shed` is
  a list so a consumer who disagrees can reorder it.
- **Labels are never shed**, so no tab becomes an unnamed glyph and no level
  needs forced tooltips.
- **The whole strip sheds together.** One tab keeping its icon while its
  neighbour lost one would read as a bug.
- **The overflow menu does not inherit the shed state.** It has room even when
  the strip does not, and it already carries each count in its item label.
- **Measured, not guessed.** The level is chosen from the real strip, so label
  length and count magnitude move the shed points on their own — no consumer
  has numbers to tune.

#### The Menu Never Inherits The Shed State

When the strip does collapse, `shed` resets. There is no strip left to shed, and
leaving it set hides the icon on the menu's *own trigger* — a collapsed Tabs
showing a bare chevron. The menu already carries each count in its item label,
so both parts return with it.

#### Why The Measure List Must Stay At Full Fidelity

Every level is measured on the hidden measure list, with the shed attribute set
and removed inside the calculation so no paint sees it. Measuring the *visible*
strip would feed the calculation its own output: shedding icons makes the strip
narrower, which says icons fit, which puts them back.

That is not hypothetical — during implementation the root's shed state also
matched the measure list through a CSS rule that forgot to exclude it, and the
strip flapped to full decoration at 300px while showing "shed both" at 350px and
"collapsed" at 250px. The `:not(.poodle-tabs__list--measure)` in `tabs.css` is
what prevents it.

There is a second feedback path, through the observer rather than the cascade.
The `ResizeObserver` watches the measure list, and deciding a level changes that
list's width twice — once to shed, once to restore — so the observer re-enters
mid-measurement and the transient states reach the screen. Narrowing past the
icon threshold flashed the collapsed menu before settling on shed icons. A
re-entrancy guard, released a frame after the pass, is what stops it; the
observer fires asynchronously, so clearing it synchronously lets the restore
notification straight back in.

#### Out Of Scope

Vertical orientation does not shed. It overflows by height, which these
width measurements say nothing about.

Neither native target sheds. Shedding is a measured-layout behaviour and GPUI
and Jetstream render from spec state without a measurement loop; they carry the
props for parity and ignore them, as they already do for `collapseWhenOverflow`.


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

The drag-source / drop-target visuals are driven by transient host-set drag
state. In Svelte these come from the component's own internal `dragSourceIndex`
/ `dropTargetIndex` during a drag gesture (`[data-drag-source]` /
`[data-drop-target]`). In the spec-driven targets (GPUI, Jetstream) the host
sets `dragValue` / `dropTargetValue` (the tab values being dragged / hovered)
on the spec; both default unset. GPUI box-shadow has no `inset` mode, so its
drop-target ring renders as a `border-width-focus` accent-base border (closest
native equivalent).

### Component States

- Selected-tab state: controlled or uncontrolled value tracking
- Roving focus: `focusIndex` tracks which tab has `tabindex="0"`, all others get `tabindex="-1"`
- Drag state: `dragSourceIndex` and `dropTargetIndex` for reorder

### Behavior Machine

Behavior classification: machine-backed

Moderate-case pilot: selection + roving tabindex, plus three auxiliary
behaviors (drag reorder, tooltip timing, overflow collapse) modeled as
sub-machines so the main chart stays readable. URL-history sync and overflow
measurement are environment effects, not machine states.

#### Context

| Field | Type | Initial | Controllable | Meaning |
|-------|------|---------|--------------|---------|
| `value` | `string \| null` | `defaultValue`, else first enabled item | yes | selected tab value |
| `focusIndex` | `number` | selected index | no | roving-tabindex position; follows selection when selection changes |
| `items` | `TabItem[]` | prop | input | ordered tab descriptors (value, label, disabled, closable, ...) |
| `activationMode` | `"automatic" \| "manual"` | `"automatic"` | input | whether focus movement commits selection |
| `orientation` | `"horizontal" \| "vertical"` | `"horizontal"` | input | maps arrow keys |
| `reorderable` | `boolean` | `false` | input | enables Alt+Arrow and drag reorder |
| `collapsedByOverflow` | `boolean` | `false` | no | presentational: list replaced by menu when it cannot fit |

#### States

Main chart:

| State | Description |
|-------|-------------|
| `idle` | tablist interactive |

Reorder sub-machine: `idle` → `dragging { sourceIndex, dropTargetIndex }` →
`idle` (drop applies reorder; leave/end cancels). Keyboard reorder
(Alt+Arrow) is a single-event action, not a `dragging` entry.

Tooltip sub-machine (active when vertical or `showTooltips`): `hidden` →
`pending { index }` (pointer enter, 300ms timer) → `visible { index }`;
pointer leave from any state → `hidden`, cancelling the timer.

#### Events

| Event | Payload | Source |
|-------|---------|--------|
| `SELECT` | `value` | tab click, collapsed-menu selection, programmatic |
| `FOCUS_MOVE` | `direction: next \| prev \| first \| last`, `fromIndex?` | Arrow keys (orientation-mapped), Home, End |
| `ACTIVATE` | `index?` | Enter/Space on focused tab (manual mode only) |
| `CLOSE` | `value` | close-button click or Delete on closable tab |
| `REORDER_STEP` | `direction: -1 \| 1`, `fromIndex?` | Alt+Arrow when `reorderable` |

Keyboard events carry the index of the tab that received the key
(`fromIndex`/`index`); the machine prefers it over tracked `focusIndex` when
the two diverge, matching the pre-machine behavior where handlers used their
own tab index.
| `DRAG_START` / `DRAG_OVER` / `DRAG_LEAVE` / `DROP` / `DRAG_END` | indices | pointer drag when `reorderable` |
| `URL_POP` | `value \| null` | environment: `popstate` when `historyKey` set |
| `OVERFLOW_CHANGE` | `collapsed: boolean` | environment: measurement effect |

#### Transitions

| State | Event | Guard | Target | Actions / Effects |
|-------|-------|-------|--------|-------------------|
| `idle` | `SELECT` | item exists | `idle` | set `value`, sync `focusIndex` to selection, `onValueChange(value)`; effect `syncHistory` |
| `idle` | `FOCUS_MOVE` | — | `idle` | move `focusIndex` to next enabled item, wrapping and skipping disabled; effect `focusTab`; when `activationMode="automatic"`, also commit selection as `SELECT` |
| `idle` | `ACTIVATE` | `activationMode="manual"` | `idle` | commit selection as `SELECT` |
| `idle` | `CLOSE` | item `closable` | `idle` | `onClose(value)` only — parent owns item removal |
| `idle` | `REORDER_STEP` | `reorderable`, target in bounds | `idle` | reorder items, keep focus on moved tab (effect `focusTab`), `onReorder(order)` |
| `idle` | `DRAG_START` | `reorderable`, item enabled | `dragging` | record `sourceIndex` |
| `dragging` | `DRAG_OVER` / `DRAG_LEAVE` | — | `dragging` | update/clear `dropTargetIndex` |
| `dragging` | `DROP` | valid target | `idle` | apply reorder as in `REORDER_STEP` |
| `dragging` | `DRAG_END` | — | `idle` | clear drag context, no reorder |
| `idle` | `URL_POP` | `historyKey` set | `idle` | set `value` from URL, falling back to first enabled item |
| `idle` | `OVERFLOW_CHANGE` | `collapseWhenOverflow`, horizontal | `idle` | set `collapsedByOverflow`; collapsed rendering delegates selection to a Menu, which re-enters via `SELECT` |

Disabled items: never selectable, never focus targets (`FOCUS_MOVE` skips
them, wrapping modulo item count), not draggable.

#### Effects

| Effect | What It Does | Cleanup |
|--------|--------------|---------|
| `focusTab` | focuses the tab element at `focusIndex` after render | none |
| `syncHistory` | when `historyKey` set: mirror `value` into `?{historyKey}=` via `history.replaceState` (deleting the param when at the default tab); subscribe to `popstate` and emit `URL_POP` | unsubscribe `popstate` on unmount |
| `measureOverflow` | when `collapseWhenOverflow`: compare natural list width (hidden measurement copy) against available width via ResizeObserver + window resize; emit `OVERFLOW_CHANGE` | disconnect observer, remove listener on unmount |
| `tooltipTimer` | 300ms delay between `pending` and `visible` in the tooltip sub-machine | clear timer on leave/unmount |

#### Part Attribute Output

| Part | Attribute | Value |
|------|-----------|-------|
| root | `data-scope` / `data-part` | `tabs` / `root` |
| root | `data-orientation` / `data-variant` / `data-bordered` / `data-active-outline` / `data-active-fill` / `data-collapsed` / `data-full-width` | resolved inputs and overflow state |
| list | `data-part` / `role` | `list` / `"tablist"` |
| list | `aria-label` / `aria-orientation` | `ariaLabel` / `orientation` |
| tab | `data-part` / `role` / `id` | `trigger` / `"tab"` / `poodle-tab-{instance}-{value}` |
| tab | `aria-selected` | `"true"` on the selected tab, else `"false"` |
| tab | `aria-controls` | panel id, only when a panel snippet exists |
| tab | `tabindex` | `0` at `focusIndex`, else `-1` (roving) |
| tab | `disabled` | item `disabled` |
| tab | `data-state` | `active` \| `inactive` |
| item wrapper | `role` / `data-selected` / `data-drag-source` / `data-drop-target` | `"presentation"` / selection and drag flags |
| panel | `data-part` / `role` / `id` | `panel` / `"tabpanel"` / `poodle-tabpanel-{instance}-{value}` |
| panel | `aria-labelledby` / `tabindex` | selected tab id / `0` |
| close | `aria-label` | `Close {label}` |

Note: `data-scope`/`data-part`/`data-state` are added during the core swap
(additive); the remaining attributes match the current implementation.

#### Machinery Dependencies

Roving tabindex (wrapping, disabled-skipping index navigation), id wiring
(tab/panel pairs). Collapsed mode composes the Menu component rather than core
machinery. History sync and overflow measurement are Tabs-specific effects,
not shared services.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | active tab changes | `string` | called on click, or on focus when `activationMode="automatic"` |
| `onReorder` | tab order changes | `string[]` | new value order array |
| `onClose` | close button clicked or Delete key on closable tab | `string` | tab value being closed |
| `onDragPrepare` | primary pointer down on an enabled reorderable tab | `string`, `PointerEvent` | lets an owning composite start asynchronous work before native `dragstart`; does not replace Tabs reorder |
| `onDragStart` | native drag starts | `string`, `DragEvent` | runs after Tabs writes its private same-region reorder payload |
| `onDragEnd` | native drag ends | `string`, `DragEvent` | runs once for the value captured at drag start |

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
- List: `display: inline-flex`, `flex-wrap: wrap` (card), `flex-wrap: nowrap` (pill/strip/block)
- Pill/Strip/Block overflow: `overflow-x: auto; overflow-y: hidden`
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

### List — Card variant

| Property | Value |
|----------|-------|
| `padding-bottom` | `0.25rem` |
| `border-bottom` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent)` |

When `bordered` is `false`, the `border-bottom` is removed (set to `0`).

### List — Card vertical

| Property | Value |
|----------|-------|
| `flex-direction` | `column` |
| `padding-bottom` | `0` |
| `padding-right` | `0.5rem` |
| `border-bottom` | `0` |
| `border-right` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent)` |

### List — Pill + Strip + Block

| Property | Value |
|----------|-------|
| `flex-wrap` | `nowrap` |
| `overflow-x` | `auto` |
| `overflow-y` | `hidden` |

### List — Pill + Strip + Block vertical

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

### Switches on strip: the underline is replaced, not combined

`activeOutline` and `activeFill` apply to all four variants. On `strip` they
interact with the variant's own selection mechanism, and the switch wins.

Strip marks selection with a border on the item — `border-bottom` when
horizontal, `border-right` when vertical — pulled over the list edge by a
negative margin. Both switches decorate that same element, so the indicator and
the decoration cannot coexist. When either switch is on, strip **drops its
underline** and the switch provides the selection affordance instead. The
negative margin is cleared with it, so the item no longer overlaps the list
edge.

`block` keeps its `border-left` separators under `activeOutline`; the outline
covers the remaining sides. Solid fill overrides block's and strip's own
item-hover backgrounds, so the fill does not revert on hover.

### Item (activeOutline)

Applies when `activeOutline` is set, on every variant. A transparent border on
every item keeps the layout stable when the selected item's border becomes
visible — the opt-in outline never nudges the tab bar.

The border sits on the **item**, the chip that wraps both the tab button and
the close button. The tab is a `<button>` and cannot contain another button, so
bordering the tab would leave the close affordance outside the outline. The
item also carries the chip radius, so the outline is correctly rounded.

| Property | Value |
|----------|-------|
| `border` | `0.0625rem solid transparent` |

### Item (activeOutline, selected)

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 32%, var(--poodle-color-border-subtle))` |

This is the former `card` variant's selected-border value, so opting in restores
exactly the outline the old variant drew by default.

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

### Selection surface: which element carries what

One rule, applied by every variant and both opt-in switches:

- **Item** carries the *chip* — `border-radius`, `background`, and the
  `activeOutline` border. The item wraps the tab button and the close button,
  so a fill placed here encloses the close affordance. Placing it on the tab
  would leave close outside the selection, and the tab is a `<button>` that
  cannot contain another button.
- **Tab and Close** carry the *foreground* — `color`. `.poodle-tabs__tab` sets
  an explicit `color`, so it does **not** inherit from the item; a colour set
  on the chip never reaches the label. Selected-text rules therefore target
  `.poodle-tabs__tab` and `.poodle-tabs__close` directly, which also keeps the
  close glyph legible on a solid fill.

### Item — Card variant (chip)

| Property | Value |
|----------|-------|
| `border-radius` | `var(--poodle-radius-control)` |

### Item — Card variant (selected)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent)` |

### Tab and Close — Card variant (selected)

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

### Item — Card variant (activeFill="solid", selected)

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-accent-base)` |

### Tab and Close — activeFill="solid", selected

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-inverse)` |

The solid fill applies to the selected tab on any variant; the foreground
switches to `text-inverse`, the same token the primary Button uses on
`accent-base`, so the filled tab keeps legible contrast against every accent.

### Tab — Pill variant

| Property | Value |
|----------|-------|
| `min-height` | `calc(var(--poodle-size-control-height) - 0.5rem)` |
| `padding` | `0 0.625rem` |
| `border-radius` | `999px` |

### Item — Pill variant (selected)

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
- Variant resolution: the rendered `data-variant` is the resolved `variant` prop; `"card"` is the canonical Svelte name and the default. `data-active-outline` and `data-active-fill` carry `activeOutline` / `activeFill` on the root

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::tabs`
- Spec struct: `TabsSpec` in primitives crate holds tab definitions + variant
- Component struct: `PoodleTabs` in components crate renders via `IntoElement`
- Opacity multipliers centralized in spec: `pill_border_opacity() -> 0.68`, `pill_active_bg_opacity() -> 0.18`
- The Rust `TabVariant` enum is `Card | Pill | Block`, matching the web union minus `strip` — the strip variant renders through the separate `TabStripSpec`/`TabStrip` component on the native targets. The `TabVariant` `Strip` member is a known gap, deliberately deferred; see the g13-013 batch log.
- The renamed `Card` variant renders icon, count, and close-button accessories on every tab, with the close button wired to `on_close` (inert when unwired, so an unwired X does not bubble to the tab and select what it was closing).
- `activeOutline` maps to a 1px border on the selected tab element: `mix_srgb(accent, border-subtle, 0.32)` — the former card selected-border value. All tabs get a transparent 1px border so selection does not shift layout.
- `activeFill="solid"` maps to a full `accent-base` background on the selected tab with `color.text.inverse` foreground.
- GPUI must model `color-mix` as `token.opacity(token.a * multiplier)` since GPUI has no CSS color-mix
- Card variant border opacity: 82% → `0.82` multiplier on border-subtle
- Panel border: 74% → `0.74` on border-subtle; panel bg: 96% → `0.96` on background-panel

## 10a. Jetstream Notes

- `Tabs::from_spec(spec, theme).on_change(...).on_close(...)`, carrying the
  tab's value in both cases.
- The close button takes a handler of its own — an inert one when no `on_close`
  is wired. Clicks bubble to the nearest clickable ancestor, so without it the X
  would select the tab it was closing, which is the worst available outcome for
  that gesture.
- A disabled tab is not a selection route, and neither is the close button on
  one.

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

- [ ] all four variants render with exact token/dimension match
- [ ] color-mix percentages match (82%, 18%, 74%, 92%, 96%; activeOutline 32%)
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
| Jetstream has no reorder or drag events | `onReorder` and the `onDrag*` trio are keyboard- and drag-driven on the web; neither route exists on this target yet | accepted, tracked | g12.017 |
| Inactive panels may stay mounted or unmounted | runtime rendering strategy differs | allowed | keep semantics and state continuity strict |
| GPUI uses opacity multiplication instead of CSS color-mix | platform capability | allowed | visual result must match |

## 13. Specimen Definitions

All preview apps must render the following specimens identically.

### Card variant (default, with panel)

Card tabs with associated panel content:

| Tab label | Panel content | State |
|-----------|--------------|-------|
| Overview | "Overview content goes here." | active (default) |
| Features | "Features content goes here." | inactive |
| Pricing | "Pricing content goes here." | inactive |
| FAQ | — | disabled |

### Card variant (closable, reorderable)

Card tabs simulating file tabs, with close buttons wired to `onClose`:

| Tab label | Props |
|-----------|-------|
| index.ts | active (default) |
| App.svelte | closable |
| utils.ts | closable |
| types.ts | closable |

### Card variant (active outline)

Card tabs with `activeOutline` set. The selected tab carries the former
`card` variant's outline (accent 32% border); everything else is flat:

| Tab label | State |
|-----------|-------|
| Overview | active (default), outlined |
| Features | inactive |
| Pricing | inactive |
| FAQ | disabled |

### Card variant (solid fill)

Card tabs with `activeFill="solid"` set. The selected tab is fully
accent-filled with inverse foreground:

| Tab label | State |
|-----------|-------|
| Overview | active (default), solid fill |
| Features | inactive |
| Pricing | inactive |
| FAQ | disabled |

### Pill variant (with icons)

Pill tabs with leading icons:

| Tab label | Icon | State |
|-----------|------|-------|
| Home | house | active (default) |
| Settings | settings | inactive |
| Users | users | inactive |

### Card (with icons, no panel)

Card tabs with icons and no panel below:

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
