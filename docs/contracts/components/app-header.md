# AppHeader

Status: contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `AppHeader`
- Layer: `composites`
- Summary: a global shell header for app identity, global actions, and
  window-level utility status with size- and density-aware shell scaling
- In scope: app identity with title and subtitle, global action snippets, optional
  utility indicators, drag-region posture, responsive collapse, size- and
  density-aware shell spacing, size/density inheritance for nested controls
- Out of scope: project-specific title/details, transport controls, timeline or
  mixer widgets

## 2. Anatomy

```text
[Root Header]  <header>
  ├── [Identity Region]
  │     └── [Title Group]  (when no identity snippet)
  │           ├── <strong> title
  │           └── <span> subtitle  (optional)
  ├── [Center Region]  (optional snippet; presence is the layout signal)
  └── [Trailing Column]
        ├── [Actions Region]  (optional snippet)
        └── [Utility Region]  (optional snippet)
```

Without a `center` snippet there is no Center Region and no Trailing Column:
the Actions and Utility Regions are flat siblings of the Identity Region,
exactly as before the centre region existed. With `center` present, Actions
and Utility are grouped into a single Trailing Column so they can share the
third grid track.

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root Header | yes | top shell chrome | background, border, height |
| Identity Region | yes | app name/icon or custom identity snippet | typography, icon, spacing |
| Title Group | no | default identity when no identity snippet is provided | title + subtitle layout |
| Center Region | no | optional centred region (destinations, tabs); its presence switches the grid | gap, control grouping |
| Trailing Column | no | groups Actions + Utility when a Center Region is present | gap, action roles |
| Actions Region | no | global shell actions | gap, action roles |
| Utility Region | no | connection/status indicators | text, status, spacing |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string \| null` | `null` | no | visible app title; ignored when `identity()` is provided |
| `subtitle` | `string \| null` | `null` | no | secondary text shown alongside title in baseline alignment |
| `dragRegion` | `boolean` | `false` | no | enables native window drag posture via `data-drag-region` attribute |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the header; falls back to `title` |
| `size` | `ControlSize \| null` | `null` | no | explicit semantic size override for header height, title text, subtitle text, and nested controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for shell spacing and nested controls |
| `element` | `HTMLElement \| null` | `null` | no | **Svelte only.** bindable escape hatch: the rendered `<header>` DOM element, for hosts that need to attach behaviour to the root. Excluded from `AppHeaderSpec` — see Element Access below |

### Element Access

Both web runtimes expose the **raw `<header>` DOM element** — never a handle
object — so a host can attach behaviour (for example window dragging) to the
header root.

- **Svelte**: `bind:element` — the `element` prop is `$bindable`, defaulting to
  `null`. Use `bind:element={myHeader}` and read it inside a `$effect`.
- **React**: `ref` — forwarded to the `<header>` via `forwardRef`
  (`AppHeaderProps` itself is unchanged; `ref` is React's own mechanism, not a
  prop in the type). `useRef<HTMLElement>(null)` and pass it as `<AppHeader
  ref={myRef} />`.

Intended use: the host attaches behaviour to the element. Poodle implements no
drag gesture and imports no `@tauri-apps/*`; `data-drag-region` keeps its
current meaning as a marker a host (or its CSS) can act on.

Non-goals (deliberate, per g13-b014 rulings):

- No rest-props spread (`{...rest}`): it adds no capability over element
  access, and every `{...rest}` is an unbounded surface the IR cannot model
  (`BTN-15` is carried as a `NEG-02` escape hatch).
- No `action` prop: Svelte actions have no React equivalent, so an action prop
  would be a Svelte-only API, which the Runtime Parity Authority rule forbids
  (`docs/contracts/001-working-rules.md` §Runtime Parity Authority).
- No additional named props (`id`, `class`, `style`, or any other): element
  access only.

#### Parity Notes

- `element` (Svelte) and `ref` (React) are **web-only** and deliberately
  absent from `AppHeaderSpec` (sanctioned `WEB_ONLY_PROPS` entry in
  `packages/svelte/preview/scripts/contract-spec-drift.ts`).
- GPUI and Jetstream: **`AC` (adapter capability)**. Native window dragging is
  a platform capability the shell owns, and a native renderer has no element
  to hand out — the host moves the window through its own titlebar/chrome
  integration. No native escape hatch is invented; element access is a
  web-runtime concern and must not reach the portable spec.

## 4. Snippets

| Snippet | Purpose | Fallback |
|---------|---------|----------|
| `identity()` | custom identity content (logo, branded element) | title/subtitle text |
| `center()` | centred region (destination tabs, status cluster) | none |
| `actions()` | primary global actions (buttons, menubar) | none |
| `utility()` | trailing utility controls (icon buttons, status) | none |

### Presence Is The Signal

`center` is a presence-switched snippet, not a layout prop. Supplying it
switches the grid to the symmetric side-column layout and groups `actions()`
and `utility()` into the trailing column; omitting it keeps the default
`minmax(0, 1fr) auto auto` grid and the flat three-region DOM — a header
without `center` renders byte-identical DOM and computed grid to a pre-centre
header. There is deliberately no `layout` or `columns` prop: a
`layout: "centered"` with an empty middle would be a state with no meaning,
and a free-form `columns` prop would leak CSS through the API.

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| standard | default | steady shell header with three-column grid |
| centred | `center` snippet supplied | symmetric side-column grid with the centre truly centred and actions/utility sharing the trailing column |
| drag-region | `dragRegion=true` | header supports window dragging where supported |
| compact density | `density="compact"` | tighter padding and inter-region spacing |
| comfortable density | `density="comfortable"` | looser padding and inter-region spacing |
| size ladder | `size="xs"..."xl"` | header height and title/subtitle typography scale with nested controls |
| collapsed | viewport <= 45rem | default header collapses to a single column; a centred header reflows to `auto minmax(0, 1fr) auto` (one row, centre absorbing the free space) |

## 6. Events

No component-owned events. Child action behavior is host-owned.

## 7. Accessibility

### Semantics

- Element: `<header>` with `aria-label` (falls back to `title`)
- Drag-region behavior must not suppress or hide interactive controls from
  assistive technology

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches shell actions and utility controls in logical order |

### Focus And Announcement

- The header itself is not focusable by default
- Utility/status updates should not reorder global actions
- GPUI-native accessibility mapping notes: GPUI must preserve labeled header or
  toolbar structure even when integrated with native title-bar mechanics

## 8. Layout

### Default (>45rem, no `center`)

- Three-column grid: `minmax(0, 1fr) auto auto`
- Gap: `--poodle-app-header-gap`
- Min-height: `--poodle-app-header-min-height`
- Padding: `--poodle-app-header-padding-block --poodle-app-header-padding-inline`
- Border-bottom: `0.0625rem solid --poodle-color-border-subtle`
- Identity occupies the leading track; actions then utility pack right

### Centred (>45rem, `center` supplied)

- Symmetric three-column grid:

  ```css
  grid-template-columns:
    minmax(var(--poodle-app-header-side-min, 0), 1fr)
    auto
    minmax(var(--poodle-app-header-side-min, 0), 1fr);
  ```

- The centre region sits in the middle `auto` track; actions and utility
  share the trailing `1fr` track as one trailing column, justified to the
  end exactly as they were as separate columns. Both side tracks split the
  free space equally, which is what keeps the centre truly centred.
- The trailing column is only emitted when `center` is present — the default
  DOM is untouched.

### `--poodle-app-header-side-min`

A collapse guard for the symmetric side columns, declared on
`.poodle-app-header` and defaulting to `0`. A consumer raises it (soundcheck
uses `9rem`) so a wide side column cannot drag the centre off. Overriding it
hits the same specificity trap as `--poodle-app-header-min-height` below:
a plain `.poodle-app-header { … }` override is `0,1,0` and silently loses to
the `[data-center]` gate's `0,2,0`, so use a matching-or-higher selector
(e.g. `.app-shell .poodle-app-header[data-center]`, `0,3,0`).

### Responsive (<=45rem)

- Default header collapses to a single-column grid: `1fr`
- A centred header **reflows rather than stacks**: `auto minmax(0, 1fr) auto`
  — one row, the centre absorbing the free space and no longer strictly
  centred. Stacking four regions would make the bar tall, and a titlebar
  cannot grow; soundcheck does exactly this at its own breakpoint.
- Utility region switches to `justify-content: flex-start`

### Overriding Header Height

The header's minimum height comes from the custom property
`--poodle-app-header-min-height`, defaulting to `--poodle-size-panel-header`
(declared on `.poodle-app-header`). The size ladder overrides that property per
size at `.poodle-app-header[data-size="xs"]` … `[data-size="xl"]` — specificity
`0,2,0` (one class + one attribute).

**The specificity trap:** a plain `.poodle-app-header { … }` override from an
app is `0,1,0` and **silently loses** to the ladder's `0,2,0`, so per-app
heights appear to have no effect. The sanctioned route is overriding the custom
property with a selector that matches or exceeds `0,2,0`.

Worked example — a 60px header for one app, independent of `dragRegion`:

```css
/* Wins over every ladder step: ancestor scope + attribute = 0,3,0.
   `[data-size]` is always present on the header, so this tracks the ladder
   at every size. */
.app-shell .poodle-app-header[data-size] {
  --poodle-app-header-min-height: 3.75rem; /* 60px */
}
```

A matching-`0,2,0` alternative (`.app-shell .poodle-app-header`) works only
when the app stylesheet is loaded after Poodle's — equal specificity is a
source-order fight, so prefer the `0,3,0` form above. The attribute selector
does **not** require `dragRegion` to be set: `data-size` is always rendered.

### Composition

- Parent expectations: top-level workspace shell
- Child expectations: action clusters, status indicators, identity text/icon
- Resizing rules: identity remains stable while utility actions compress first

## 9. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root Header | `background-panel`, `border-subtle` | shell chrome |
| Identity title | `--poodle-app-header-title-size`, line-height `1.2` | app identity |
| Subtitle | `text-secondary`, `--poodle-app-header-subtitle-size` | secondary text |
| Actions/Utility | `--poodle-app-header-region-gap` | control grouping |

### Token Usage — Exact CSS Values

#### `.app-header` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `minmax(0, 1fr) auto auto` |
| `gap` | `var(--poodle-app-header-gap)` |
| `align-items` | `center` |
| `min-height` | `var(--poodle-app-header-min-height)` |
| `padding` | `var(--poodle-app-header-padding-block) var(--poodle-app-header-padding-inline)` |
| `border-bottom` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent)` |
| `overflow` | `visible` |
| `--poodle-app-header-side-min` | `0` (collapse guard for the centred side columns) |

#### `.app-header[data-center]` (Centred, Additional)

| Property | Value |
|----------|-------|
| `grid-template-columns` | `minmax(var(--poodle-app-header-side-min, 0), 1fr) auto minmax(var(--poodle-app-header-side-min, 0), 1fr)` |

#### `.app-header__identity`, `.app-header__center`, `.app-header__actions`, `.app-header__utility` (Shared)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-app-header-region-gap)` |
| `min-width` | `0` |

#### `.app-header__trailing` (Only When `center` Is Present)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `flex-end` |
| `gap` | `var(--poodle-app-header-gap)` |
| `min-width` | `0` |

#### `.app-header__title-group`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `baseline` |
| `gap` | `var(--poodle-app-header-region-gap)` |
| `min-width` | `0` |

#### `.app-header__identity strong` (Title)

| Property | Value |
|----------|-------|
| `font-size` | `var(--poodle-app-header-title-size)` |
| `line-height` | `1.2` |
| `white-space` | `nowrap` |

#### `.app-header__subtitle`

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `var(--poodle-app-header-subtitle-size)` |
| `line-height` | `1.2` |
| `white-space` | `nowrap` |
| `overflow` | `hidden` |
| `text-overflow` | `ellipsis` |

#### `.app-header__utility` (Additional)

| Property | Value |
|----------|-------|
| `justify-content` | `flex-end` |

### Responsive Breakpoint: `max-width: 45rem`

#### `.app-header`

| Property | Value |
|----------|-------|
| `grid-template-columns` | `1fr` |

#### `.app-header[data-center]` (Centred Reflow)

| Property | Value |
|----------|-------|
| `grid-template-columns` | `auto minmax(0, 1fr) auto` |

#### `.app-header__utility`

| Property | Value |
|----------|-------|
| `justify-content` | `flex-start` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-drag-region` | `<header>` root | enables native window drag posture |
| `data-center` | `<header>` root | present when a `center` snippet is supplied; gates the symmetric grid and the narrow reflow |
| `data-size` | `<header>` root | size ladder for shell height and typography |
| `data-density` | `<header>` root | density ladder for shell spacing |

## 10. Specimen Definitions

### Full App Window Header (Title + Menubar + Utility)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Full app window header | `title="Poodle Studio"`, actions slot with inline Menubar (File, Edit, View, Help menus with shortcuts), utility slot with 3 ghost IconButtons (search, bell, settings) | Full-width header with app title, integrated menubar, and trailing utility icons; simulated app body area below |

### With Title, Actions, And Utility

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With title, actions, and utility | `title="My Application"`, actions slot with 2 ghost Buttons ("New", "Open"), utility slot with settings ghost IconButton | Header with title, action buttons in primary region, settings icon trailing |

### Title Only

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Title only | `title="Poodle Workstation"` | Minimal header displaying only the app title, no actions or utility controls |

### Custom Identity Slot

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Custom identity slot | identity slot with custom logo badge ("P") and bold "Poodle Studio" text, utility slot with bell and user ghost IconButtons | Header with custom branded identity region replacing default title, trailing utility icons |
| Centred header | `title="My Application"`, `center` slot with a strip Tabs group ("Editor", "Preview", "Terminal"), actions slot with New/Open ghost Buttons, utility slot with settings ghost IconButton | Header with symmetric side columns, destination tabs truly centred, actions + utility sharing the trailing column |
| Centred header at narrow width | same centred config inside a `≤45rem` frame | One-row reflow: `auto minmax(0, 1fr) auto`, centre absorbing the free space and no longer strictly centred |
| Density ladder | `density="compact" \| "default" \| "comfortable"` with actions and utility controls | Header spacing tightens or loosens while nested controls follow the same density |
| Size ladder | `size="xs" \| "sm" \| "md" \| "lg" \| "xl"` with actions and utility controls | Header height and title/subtitle scale together while nested controls follow the same size |
