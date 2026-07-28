# ListCard

> **Surface elevation**: ListCard is a surface consumer (50% strong contrast) — see [surface-elevation.md](./surface-elevation.md).

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `ListCard`
- Layer: `foundation`
- Summary: a compact card for displaying items in list views and square-ish
  tile contexts with
  leading icon/thumbnail, title, badges, subtitle, footer counters, meta,
  explicit actions, selectable state, an exclusive trailing lane, and optional
  link-root navigation
- In scope: interactive and disabled states, leading shape variants, leading fill
  variants (tint/solid), custom accent color theming, snippet-based leading,
  badges, footer, actions, and trailing composition, title truncation, meta display
  with tabular-nums, compact layout, selected state, and reorder-affordance
  presentation
- Out of scope: drag-and-drop workflow ownership, batch-submit reorder flows,
  expandable list cards

## 2. Anatomy

```text
[Root .list-card]  <div> | <a>
  ├── [Sash .list-card__sash]  <span> (optional, diagonal corner ribbon)
  ├── [Handle .list-card__handle]  <span> (optional, reorder affordance only)
  ├── [SelectionIndicator .list-card__selection-indicator]  <span> (optional, when selectionIndicator="checkbox"; overlays leading or renders inline)
  ├── [Leading .list-card__leading]  (optional, via leading snippet)
  ├── [Body .list-card__body]  <div>
  │   ├── [Header .list-card__header]  <div>
  │   │   ├── [Title .list-card__title]  <span> (text prop or titleContent snippet)
  │   │   └── [HeaderAccessories .list-card__header-accessories]  (optional)
  │   │         ├── [Badges .list-card__badges]  (optional, via badges snippet)
  │   │         └── [Corner .list-card__corner]  (optional, via corner snippet)
  │   ├── [Subtitle .list-card__subtitle]  <span> (optional)
  │   └── [Footer .list-card__footer]  (optional, via footer snippet)
  ├── [Meta .list-card__meta]  <span> (optional)
  ├── [Actions .list-card__actions]  (optional, via actions snippet)
  └── [Trailing .list-card__trailing]  (optional, via trailing snippet)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | flex row container; `position: relative; overflow: hidden` when sash present | padding, border, radius, background, gap |
| Sash | no | diagonal ribbon in top-left corner | position, background, color, font, transform |
| Handle | no | compact reorder affordance; visual only | size, color, spacing |
| SelectionIndicator | no | checkbox selection indicator shown when `selectionIndicator="checkbox"` and selectable; overlays the leading area when leading is present, otherwise inline | size |
| Leading | no | avatar, icon, or thumbnail snippet | width, height, border-radius, background, color |
| Body | yes | title/subtitle/footer column | flex, gap |
| Header | yes | title + accessories row | flex, gap, alignment |
| Title | yes | primary text, truncated | font, color, overflow |
| HeaderAccessories | no | shrink-proof cluster holding badges and corner next to the title | flex, gap |
| Badges | no | inline pills/badges next to title | flex, gap |
| Corner | no | supplementary header-corner content (tertiary color) | flex, gap, color |
| Subtitle | no | secondary text, truncated | font-size, color, overflow |
| Footer | no | counter icons or links row | flex, gap |
| Meta | no | right-aligned metadata | font-size, color, font-variant-numeric |
| Actions | no | explicit action trigger area | flex alignment |
| Trailing | no | action button or indicator snippet | flex alignment |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | — | yes | primary display text |
| `subtitle` | `string \| null` | `null` | no | secondary display text |
| `meta` | `string \| null` | `null` | no | right-aligned metadata text |
| `href` | `string \| null` | `null` | no | when present and not disabled/selectable, renders a real link root |
| `leadingShape` | `"circle" \| "rounded-square"` | `"circle"` | no | shape of the leading snippet container |
| `leadingFill` | `"tint" \| "solid"` | `"tint"` | no | fill style — tint uses translucent accent, solid uses opaque accent with white icon |
| `leadingSizeOffset` | `number` | `0` | no | relative leading-size step offset from the resolved card size; rounded to whole steps and clamped to the `xs`→`xl` ladder |
| `accentColor` | `string \| null` | `null` | no | custom CSS color for leading background and icon; overrides theme accent |
| `layout` | `"default" \| "compact" \| "stacked"` | `"default"` | no | `compact` is for dense list and reorder contexts; `stacked` creates a square-ish tile with leading on top and a bottom utility rail |
| `interactive` | `boolean` | `false` | no | enables hover/focus/click behavior |
| `disabled` | `boolean` | `false` | no | disables interaction |
| `selectable` | `boolean` | `false` | no | toggles selected state through the root interaction contract |
| `selected` | `boolean` | `false` | no | selected visual state |
| `active` | `boolean` | `false` | no | the card the user is currently on; quieter than `selected` and orthogonal to it |
| `highlighted` | `boolean` | `false` | no | accent emphasis state; tints border, paints an accent-to-transparent gradient over the fill, and adds an inset accent ring (independent of selection) |
| `selectionIndicator` | `"none" \| "checkbox"` | `"none"` | no | when `"checkbox"` and the card is selectable, renders a checkbox selection indicator; overlays the leading area when a leading snippet is present, otherwise renders inline |
| `showReorderHandle` | `boolean` | `false` | no | visual reorder affordance; does not implement drag/drop behavior |
| `notLive` | `boolean` | `false` | no | dashed border, reduced opacity; still interactive unlike disabled |
| `sash` | `string \| null` | `null` | no | short label for a diagonal corner ribbon (top-left); keep to ~4 chars |
| `sashColor` | `string \| null` | `null` | no | custom CSS color for the sash ribbon background; defaults to positive/green |
| `ariaLabel` | `string \| null` | `null` | no | accessible name |
| `contextMenuItems` | `MenuItem[] \| null` | `null` | no | built-in context menu items; when non-empty, the card opens a menu on right-click (or keyboard `ContextMenu`/`Shift+F10`) without needing an external ContextMenu wrapper |
| `contextMenuAriaLabel` | `string \| null` | `null` | no | accessible name for the context menu overlay and (for `contextMenuTrigger="leading"`) the leading trigger button |
| `contextMenuTrigger` | `"context" \| "leading"` | `"context"` | no | `context` opens via right-click/keyboard; `leading` turns the leading area into a click/Enter/Space menu trigger (ignored while selectable) |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | explicit density override for item spacing; when null, resolves from inherited presentation |
| `onClick` | `((event: MouseEvent) => void) \| null` | `null` | no | called when an interactive card is activated; suppressed while disabled |
| `onSelectedChange` | `((selected: boolean) => void) \| null` | `null` | no | called when a selectable card toggles; suppressed while disabled |
| `onContextAction` | `((value: string) => void) \| null` | `null` | no | called with the activated context-menu item's `value` when a built-in context-menu item is selected |

### Snippets

| Snippet | Purpose |
|---------|---------|
| `titleContent` | custom rich title content when plain string title is not enough |
| `subtitleContent` | custom rich subtitle content when plain subtitle text is not enough |
| `metaContent` | custom rich metadata content when plain meta text is not enough |
| `sashContent` | custom ribbon content when plain sash text is not enough |
| `leading` | avatar, icon, or media thumbnail |
| `badges` | pills or badges displayed inline with the title |
| `corner` | supplementary header-corner content rendered alongside badges in the header accessories cluster (tertiary text color) |
| `footer` | counter icons, links, or supplementary info below subtitle |
| `actions` | explicit action trigger composition |
| `trailing` | exclusive right-edge content; when present, replaces `meta` and `actions` |

### Controlled And Uncontrolled

- Display component; interaction state is externally controlled. Selection is
  host-owned through `selected`, with toggles requested through
  `onSelectedChange`.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | subtle border, surface background |
| hover | pointer enters (when interactive) | elevated background, stronger border |
| focus | keyboard focus (when interactive) | accent focus ring |
| disabled | `disabled=true` | reduced opacity, not-allowed cursor |
| selected | `selected=true` | accent border and focus-style outline |
| active | `active=true` | accent bar down the leading edge, primary-weight title; no border or fill change |
| highlighted | `highlighted=true` | accent-tinted border, accent-to-transparent fill gradient, and inset accent ring; orthogonal to selection |
| compact | `layout="compact"` | denser spacing, smaller leading area, single-line emphasis |
| stacked | `layout="stacked"` | vertical layout with top leading area, body column, and bottom utility rail |
| not-live | `notLive=true` | dashed border (2px), transparent background, greyscale filter, reduced opacity (0.72); still interactive, greyscale and opacity restore on hover |

### Active Versus Selected

Both mark a card out, and they mean different things.

**`selected` marks a card the user has picked for an action** — the multi-select
case, where several cards may be selected and the next click is "delete these".
Its full accent border, 16% fill and ring are proportionate to that.

**`active` marks the card the user is currently on** — one per list, always on
while the list is shown. At `selected`'s weight it would shout permanently, so
it is a bar down the leading edge and nothing else.

They are orthogonal: a card can be both the one you are on and one of several
you have ticked, and it shows both.

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Machine-backed via shared machinery (g11 extraction sweep): the context
menu's escape/outside dismissal registers on the dismissable-layer stack
(overlay and leading trigger area count as inside), and item navigation
already uses the shared menu-list machinery. Selection callbacks are plain
props.

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|----------|--------------|---------|-------|
| `onClick` | card activated (when interactive) | `MouseEvent` | suppressed while disabled |
| `onSelectedChange` | selectable card toggled | `boolean` | receives the next selected state; suppressed while disabled |

## 6. Accessibility

### Semantics

- When `href` is present: renders an anchor root
- When interactive/selectable without `href`: `role="button"`, `tabindex="0"`, `aria-label` from prop or title
- When not interactive: no role (generic container)
- When disabled: `aria-disabled="true"`
- When selectable: `aria-pressed` reflects `selected`
- When `active`: `aria-current="true"` — the current item, which is a different
  claim from `aria-selected` and must not be conflated with it

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | activates link roots |
| `Enter` | activates card (when interactive) |
| `Space` | activates card (when interactive) |
| `Tab` | moves focus to/from card |

### Focus And Announcement

- focus entry: card root receives visible focus ring (when interactive)
- focus exit: focus ring clears immediately
- non-interactive cards are not focusable

## 7. Layout

### Sizing

- Root: flex row by default, stacked column when `layout="stacked"`
- Leading: fixed square — 2rem (circle) or 2.75rem (rounded-square)
- Body: flex 1, min-width 0 for truncation
- Header: flex row, title truncates, badges shrink-proof
- Meta: flex-shrink 0

### Composition

- parent expectations: list views, sidebar navigation, search results
- child expectations: leading icon/avatar/thumbnail, badges (Pill, Badge),
  footer counters (ListCardCounter), explicit actions or trailing status via
  snippets, and optional rich title composition via the `titleContent` snippet
  when the title needs inline formatting rather than plain text
- right-edge composition: `meta` + `actions` may be combined, but `trailing`
  is exclusive and replaces both so the card only has one trailing lane
- leading-size composition: `leadingSizeOffset` shifts the leading block,
  inner icon, and selection indicator together relative to the resolved card
  size without changing title/meta typography
- stacked layout: leading sits on top, subtitle may wrap to two lines, and
  trailing/meta/actions move into a full-width bottom utility rail
- resizing: fills parent width, height auto-fits content
- context menu: pass `contextMenuItems` to use the built-in context menu
  (right-click, or `contextMenuTrigger="leading"` to open from the leading area);
  alternatively wrap ListCard in a standalone ContextMenu for fully external
  ownership
- explicit menu/action composition should use the `actions` snippet rather than
  coupling menu trigger ownership to the leading media area

### Hierarchy Title Guidance

- use `titleContent` when the visible title needs hierarchy rather than plain text
- render parent segments in `var(--poodle-color-text-secondary)` so the final
  leaf remains the primary focal point
- use real chevron icons between segments; do not encode hierarchy with plain
  text delimiters like `>` or `/`
- keep hierarchy titles to one line in default/compact layouts; let the leaf
  truncate first rather than wrapping the whole chain
- prefer one or two ancestor segments; longer chains should collapse earlier
  context into `metaContent` or the subtitle instead of making the title noisy
- if the right edge also needs hierarchy context, use `metaContent` so the
  dimmed/structured styling matches the title treatment

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.75rem` |
| `padding` | `0.625rem 0.75rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 18%, transparent)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-surface) 88%, var(--poodle-color-text-primary))` |
| `transition` | `background, border-color` at `motion-duration-interaction motion-easing-standard` |

### Root interactive hover

| Property | Value |
|----------|-------|
| `cursor` | `pointer` |
| `background` | `color-mix(in srgb, var(--poodle-surface) 82%, var(--poodle-color-text-primary))` |
| `border-color` | `color-mix(in srgb, var(--poodle-color-border-default) 52%, transparent)` |

### Root focus

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `-0.0625rem` |

### Root disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Root not-live

| Property | Value |
|----------|-------|
| `border` | `0.1875rem dashed color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent)` (border-color becomes `var(--poodle-color-border-default)` on hover) |
| `background` | `color-mix(in srgb, var(--poodle-surface) 32%, transparent)` |
| `filter` | `grayscale(1)` (restores to `grayscale(0)` on hover) |
| `opacity` | `0.72` (restores to `1` on hover) |

### Root highlighted

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 34%, transparent)` |
| `background` | `linear-gradient(90deg, color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 10%, transparent), transparent 24%)` over the base fill |
| `box-shadow` | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 12%, transparent)` |

### Sash

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `0.34375rem` |
| `left` | `-2.25rem` |
| `width` | `6rem` |
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `padding` | `0.125rem 0` |
| `background` | `var(--list-card-sash, var(--poodle-color-positive-base, #22c55e))` |
| `color` | `#fff` |
| `font-size` | `0.5625rem` |
| `font-weight` | `700` |
| `text-transform` | `uppercase` |
| `transform` | `rotate(-45deg)` |
| `pointer-events` | `none` |
| `z-index` | `1` |

### Leading snippet

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `flex-shrink` | `0` |
| `width` | `2rem` (circle) or `2.75rem` (rounded-square) |
| `height` | `2rem` (circle) or `2.75rem` (rounded-square) |
| `overflow` | `hidden` |
| `border-radius` | `999px` (circle) or `var(--poodle-radius-control)` (rounded-square) |
| `background` | tint: `color-mix(in srgb, var(--list-card-accent, var(--poodle-color-accent-base)) 12%, transparent)` — solid: `var(--list-card-accent, var(--poodle-color-accent-base))` |
| `color` | tint: `var(--list-card-accent, var(--poodle-color-accent-base))` — solid: `#fff` |
| `font-size` | `0.875rem` |
| `font-weight` | `600` |

### Body

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `min-width` | `0` |
| `display` | `flex` |
| `flex-direction` | `column` |
| `gap` | `0.0625rem` |

### Header

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `baseline` |
| `gap` | `0.375rem` |

### Title

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `min-width` | `0` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `font-weight` | `500` |
| `color` | `var(--poodle-color-text-primary)` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |
| `white-space` | `nowrap` |

### Header Accessories

| Property | Value |
|----------|-------|
| `flex-shrink` | `0` |
| `display` | `flex` |
| `align-items` | `center` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |

### Badges and Corner

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-xs)` |

Corner additionally sets `color: var(--poodle-color-text-tertiary)`.

### Selection Indicator

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `flex-shrink` | `0` |
| `width` / `height` | `var(--poodle-list-card-selection-indicator-size)` (size-scaled) |

The overlay variant (when a leading snippet is present) is `position: absolute; inset: 0` over the leading area.

### Subtitle

| Property | Value |
|----------|-------|
| `font-size` | `0.75rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |
| `white-space` | `nowrap` |

### Footer

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.5rem` |
| `margin-top` | `0.125rem` |

### Meta

| Property | Value |
|----------|-------|
| `flex-shrink` | `0` |
| `font-size` | `0.75rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-variant-numeric` | `tabular-nums` |

### Trailing snippet

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `flex-shrink` | `0` |

## 9. Helper: ListCardCounter

A small companion component for rendering icon + count pairs in the footer snippet.

### Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `icon` | `string` | — | yes | icon name |
| `count` | `number` | — | yes | display count |
| `tooltip` | `string \| null` | `null` | no | tooltip text |
| `href` | `string \| null` | `null` | no | when set, renders as `<a>` and stops click propagation |

### Token Usage

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.25rem` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.75rem` |
| `font-variant-numeric` | `tabular-nums` |

- Anchor variant: `color: var(--poodle-color-text-primary)` on hover

## 10. Svelte Notes

- `data-disabled`, `data-not-live`, `data-leading-shape`, `data-leading-fill` data attributes on root
- `--list-card-accent` custom property set via inline style when `accentColor` is provided
- `--list-card-sash` custom property set via inline style when `sashColor` is provided
- Root gets `position: relative; overflow: hidden` via `list-card--has-sash` class when sash is present
- Interactive mode adds click handling and keydown handling for Enter/Space
- Title text always truncated with ellipsis
- Leading snippet provides default container styling (circle or rounded-square)
- Trailing snippet is unstyled pass-through
- Badges and corner snippets render inside the `header-accessories` cluster inline with the title
- Footer snippet renders below subtitle for counter icons
- Built-in context menu: when `contextMenuItems` is non-empty the card owns a context-menu overlay (right-click, or leading-trigger via `contextMenuTrigger="leading"`), with `ContextMenu`/`Shift+F10` keyboard support and `onContextAction(value)`; a standalone ContextMenu wrapper remains an alternative for fully external ownership
- `data-highlighted` reflects the `highlighted` prop on root
- Both roots (`<a>` when `href` is set and not `selectable`, `<div>` otherwise) emit `data-size` from the same resolved size — the explicit `size` prop, else `sizeRole` against inherited presentation. The root variant must never change the resolved size.

## 11. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::list_card`
- Spec struct: `ListCardSpec` in primitives crate
- Component struct: `PoodleListCard` in components crate
- Flex layout with fixed-width leading column
- Text truncation uses GPUI's text ellipsis support
- `tabular-nums` may require GPUI font feature flag

## 12. Parity Checklist

### Tier 1: Strict Parity

- [ ] title, subtitle, meta display correctly
- [ ] interactive mode enables click and keyboard activation
- [ ] disabled state suppresses interaction
- [ ] ARIA role matches (button when interactive)
- [ ] leadingShape variants render correctly
- [ ] leadingFill tint/solid variants render correctly
- [ ] accentColor custom theming applies to leading

### Tier 2: Visual Parity

- [ ] padding and gap match
- [ ] border and border-radius match
- [ ] hover background and border match
- [ ] focus ring matches
- [ ] leading snippet default styling matches (circle and rounded-square)
- [ ] title truncation matches
- [ ] subtitle and meta typography match
- [ ] disabled opacity matches
- [ ] badges render inline with title
- [ ] footer renders below subtitle

### Tier 3: Implementation Freedom

- [ ] transition timing is platform-owned
- [ ] snippet mechanism is platform-owned

## 13. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| tabular-nums font variant | may require GPUI font feature flag | allowed | match where possible |
| GPUI active bar is a child rectangle with rounded leading corners, not an inset shadow | GPUI's `BoxShadow` has no inset flag, so the bar cannot be clipped by the card's radius as it is on the web and Jetstream | allowed | revisit if gpui gains inset shadows |
| ListCardCounter helper | Svelte-specific helper, GPUI may inline | allowed | match API if feasible |

## 14. Specimen Definitions

### Interactive List Cards

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| design-system-v2.figma | `title`, `subtitle`, `meta="14.2 MB"`, `interactive`, leading icon (folder) | Interactive card with circle leading icon, title, subtitle, and right-aligned meta |
| component-specs.pdf | `title`, `subtitle`, `meta="2.8 MB"`, `interactive`, leading icon (file-text) | Interactive card with circle leading icon |
| brand-assets.zip | `title`, `subtitle="Archived"`, `meta="48 MB"`, `disabled`, leading icon (layers) | Disabled card at reduced opacity with not-allowed cursor |

### Hierarchy Titles

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Cash flow forecasts | `interactive`, `titleContent` snippet with dimmed ancestors and chevron icons, `metaContent` snippet with muted right-edge metadata | Hierarchy-style title with real icon separators instead of plain text delimiters |
| Week 1: Cash Flow | `layout="compact"`, `showReorderHandle`, `titleContent` snippet with dimmed parent and chevron icon, info badge | Compact hierarchy row suitable for nested reorder/list contexts |

### Rounded-Square Leading (Thumbnails)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| hero-banner.png | `title`, `subtitle`, `meta="3.1 MB"`, `leadingShape="rounded-square"`, `interactive` | Card with rounded-square leading container instead of circle |
| onboarding-flow.mp4 | `title`, `subtitle`, `meta="128 MB"`, `leadingShape="rounded-square"`, `interactive` | Card with rounded-square leading container |

### With Badges

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| API integration guide | `title`, `subtitle`, `meta="Draft"`, `interactive`, badges snippet with accent Pill "New" | Card with pill badge inline next to title |
| Q4 planning deck | `title`, `subtitle`, `interactive`, badges snippet with muted Badge "3" + caution Pill "Review" | Card with multiple badges inline next to title |

### With Footer Counters

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Design system | `title`, `subtitle`, `leadingShape="rounded-square"`, `interactive`, badges snippet with positive Pill "Active", footer with 3 ListCardCounters | Card with badge, and footer row showing icon+count pairs (24 docs, 8 images, 3 sub-folders) |
| Brand guidelines | `title`, `subtitle`, `leadingShape="rounded-square"`, `interactive`, footer with 2 ListCardCounters | Card with footer row showing icon+count pairs (6 docs, 42 images) |

### Solid Fill With Accent Colors

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Design tokens | `leadingShape="rounded-square"`, `leadingFill="solid"`, `accentColor="#6366f1"`, `interactive` | Card with opaque indigo leading background and white icon |
| Components | `leadingShape="rounded-square"`, `leadingFill="solid"`, `accentColor="#ec4899"`, `interactive` | Card with opaque pink leading background and white icon |
| Documentation | `leadingShape="rounded-square"`, `leadingFill="solid"`, `accentColor="#14b8a6"`, `interactive` | Card with opaque teal leading background and white icon |
| Default accent | `leadingShape="rounded-square"`, `leadingFill="solid"`, `interactive` (no accentColor) | Card with theme accent solid leading background |

### With Context Menu

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Right-click for actions | `interactive`, wrapped in ContextMenu with Open, Rename, Duplicate, separator, Delete items | Card that shows context menu on right-click |

### Not Live (Dashed Border)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Unpublished draft | `interactive`, `notLive`, `meta="Draft"` | Card with dashed border, reduced opacity, greyscale filter; restores on hover |
| Staging environment | `interactive`, `notLive`, `leadingShape="rounded-square"`, badges snippet with caution Pill "Pending" | Not-live card with badge |

### Corner Sash Badges

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Free tier plan | `sash="Free"`, `interactive` | Card with diagonal green ribbon in top-left corner |
| Premium integration | `sash="New"`, `sashColor="#6366f1"`, `leadingFill="solid"`, `accentColor="#6366f1"`, `interactive` | Card with diagonal indigo ribbon |
| Legacy connector | `sash="EOL"`, `sashColor="#ef4444"`, `interactive` | Card with diagonal red ribbon |

### Static List Card

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Read-only item | `title="Read-only item"`, `subtitle="No click handler"`, not interactive | Non-interactive card with no hover/focus behavior |

## 15. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: list views, sidebar navigation, search results, file browsers
- future follow-up: multi-select support, swipe actions
