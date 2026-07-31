# NavigationMenu

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `NavigationMenu`
- Layer: `foundation`
- Summary: a persistent navigation control that owns top-level nav triggers
  and an associated viewport surface for content disclosure, with pill-style
  trigger buttons and snippet-driven viewport content
- In scope: top-level navigation items, active open state, associated viewport
  surface, keyboard movement across triggers, snippet-driven viewport content
  receiving activeValue and activeItem, disabled items, controlled/uncontrolled
  value
- Out of scope: routing, breadcrumbs, shell-specific sidebars, workstation
  panel systems, menu item semantics (this is navigation, not commands),
  mobile hamburger collapse

## 2. Anatomy

```text
[Root .navigation-menu]  <div>
  ├── [List .navigation-menu__list]  <nav>
  │     └── [Trigger .navigation-menu__trigger]...  <button>
  └── [Viewport .navigation-menu__viewport]  <div>  (when value is set)
        └── {children snippet — receives activeValue, activeItem}
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | grid container for list + viewport | gap, min-width |
| List | yes | horizontal nav trigger strip as `<nav>` element | inline-flex, wrap, gap |
| Trigger | yes | navigation disclosure button with pill-style border | border, radius, background, typography, focus |
| Viewport | conditional | content surface associated with active trigger | border, radius, background, elevation, padding |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `value` | `string \| null` | `null` | no | controlled active item value; null = uncontrolled |
| `defaultValue` | `string \| null` | `null` | no | uncontrolled initial active item |
| `items` | `NavigationMenuItem[]` | — | yes | navigation item definitions |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | explicit density override for item spacing; when null, resolves from inherited presentation |
| `ariaLabel` | `string \| null` | `null` | no | accessible label for the nav element |

### NavigationMenuItem Type

| Field | Type | Default | Required | Notes |
|-------|------|---------|----------|-------|
| `value` | `string` | — | yes | unique identifier |
| `label` | `string` | — | yes | visible trigger text |
| `icon` | `string \| null` | `null` | no | optional icon |
| `disabled` | `boolean` | `false` | no | prevents activation |

### Snippet

- The `children` snippet receives `activeValue` (the currently active item
  value or null) and `activeItem` (the full `NavigationMenuItem` object or
  null)
- Snippet content is rendered inside the viewport when an item is active
- The host is responsible for rendering appropriate content based on the
  active value

### Controlled And Uncontrolled

- controlled: `value` (string) plus `onValueChange` callback
- uncontrolled: `value` is null, uses `defaultValue` as initial state
- `value` represents which navigation item is currently active/open; null
  means none active
- module-level `nextNavigationMenuId` counter for unique IDs across instances
- internal `focusIndex` tracks roving keyboard focus

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| all closed | default | no viewport visible, triggers in default state |
| item active | value matches a trigger | that trigger shows open styling, viewport visible with slot content |
| trigger hover | pointer over trigger (when not disabled) | hover background treatment (accent 12%) |
| trigger focus | keyboard focus on trigger | focus background treatment (accent 12%), outline: none |
| disabled | `disabled` on NavigationMenuItem | trigger muted, non-interactive, reduced opacity |

### Component States

- Active item value (which navigation item is open): controlled or uncontrolled
- `focusIndex`: roving tabindex position across triggers
- Viewport visibility: shown when value is non-null, hidden otherwise

### Behavior Machine

Behavior classification: machine-backed via shared machinery

Trigger roving uses `findNextEnabledIndex`/`firstEnabledIndex`; panel
open/close is single-select-with-deactivation over controllable state;
dismissal (escape + outside) registers on the dismissable-layer stack while
a panel is open.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onValueChange` | active item changes or closes | `string \| null` | null when no item is active |

## 6. Accessibility

### Semantics

- Root: no role (container div)
- List: rendered as a `<nav>` element with `aria-label` when provided
- Triggers: `<button>`, `aria-expanded` reflecting whether their viewport
  content is visible, `aria-controls` pointing to the viewport element id
- Viewport: `aria-labelledby` pointing to the active trigger
- Disabled triggers: `aria-disabled="true"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Arrow Right` | moves focus to next trigger (wraps) |
| `Arrow Left` | moves focus to previous trigger (wraps) |
| `Home` | moves focus to first enabled trigger |
| `End` | moves focus to last enabled trigger |
| `Arrow Down` | opens/activates the focused trigger's viewport content |
| `Enter` or `Space` | opens/activates the focused trigger's viewport content |
| `Escape` | closes the active viewport (sets value to null) |
| Outside click | closes the active viewport |

### Focus And Announcement

- focus entry: first trigger participates in tab order; subsequent triggers
  reached via arrow keys (roving tabindex — one at `tabindex="0"`, others
  at `-1`)
- focus exit: tab moves focus out of the navigation menu
- viewport focus: opening a viewport may move focus into the viewport content
  if it contains focusable elements
- focus restoration: closing a viewport returns focus to its owning trigger
- `aria-expanded` state announced on trigger focus
- live-region behavior: none; expanded state and labeling carry the semantics
- GPUI-native accessibility mapping notes: GPUI must model the navigation menu
  as a nav landmark with expandable trigger-to-viewport relationships

## 7. Layout

### Sizing

- Root: `display: grid`, `gap: var(--poodle-space-stack-md)`, `min-width: 0`
- List: `display: inline-flex`, `flex-wrap: wrap`,
  `gap: var(--poodle-space-inline-sm)`, `align-items: center`
- Trigger: inline-flex with gap for optional icon, min-height from
  `var(--poodle-size-control-height)`, pill-style border
- Viewport: block panel with surface-level padding, sizes to content within
  parent constraints

### Composition

- parent expectations: catalog nav bars, docs-site nav clusters, product
  section menus, settings navigation
- child expectations: slot-driven viewport content (cards, link lists,
  feature grids)
- resizing rules: trigger list wraps; viewport stretches to available width

## 8. Token Usage — Exact Values

### Root `.navigation-menu`

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `gap` | `var(--poodle-space-stack-md)` |
| `min-width` | `0` |

### List `.navigation-menu__list`

| Property | Value |
|----------|-------|
| element | `<nav>` |
| `display` | `inline-flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `align-items` | `center` |

### Trigger `.navigation-menu__trigger`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 88%, transparent)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `600` |
| `line-height` | `1` |

### Trigger — Open (active)

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent)` |
| `border-color` | `color-mix(in srgb, var(--poodle-color-accent-base) 42%, var(--poodle-color-border-default))` |

### Trigger — Hover / Focus

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |
| `outline` | `none` |

### Trigger — Disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Viewport `.navigation-menu__viewport`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent)` |
| `box-shadow` | `var(--poodle-elevation-overlay)` |

### Size adjustments

Size steps `font-size` only. Trigger `min-height` stays
`var(--poodle-size-control-height)` and trigger `padding` stays
`0 var(--poodle-space-control-x)` for every size — those tokens already scale per
size, so no per-size min-height/padding override is applied. `—` means no
override (inherits the base `0.75rem`).

| Size | trigger font-size |
|------|-------------------|
| `xs` | `0.6875rem` |
| `sm` | `—` |
| `md` | `—` |
| `lg` | `0.8125rem` |
| `xl` | `0.875rem` |

### Density adjustments

Density overrides horizontal trigger padding only (size/density orthogonality —
no height change):

| Density | trigger `padding-inline` |
|---------|--------------------------|
| `compact` | `0.5rem` |
| `default` | `var(--poodle-space-control-x)` (inherited) |
| `comfortable` | `0.75rem` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- Module-level `nextNavigationMenuId` counter for unique IDs across instances
- Controlled/uncontrolled value via internal `uncontrolledValue` state
- `focusIndex` tracks roving tabindex across triggers
- Default slot receives `activeValue` and `activeItem` as slot props
- Viewport conditionally rendered when value is non-null
- Triggers use `data-open` attribute for open-state styling
- Roving tabindex is implemented: the trigger at `focusIndex` gets
  `tabindex="0"`, all others get `tabindex="-1"`; ArrowLeft/ArrowRight
  move focus between triggers with wrapping, and `focusIndex` updates
  when a trigger receives focus
- `aria-controls` on triggers references viewport panel id
- The list element is a `<nav>` for landmark semantics
- Triggers have a visible border in their default state (unlike Menubar triggers
  which are borderless); this gives them a pill-like appearance
- The open state uses a distinct accent-base 16% background and a blended
  border-color, differentiating it from the hover state at 12%
- Viewport border opacity is 74% (slightly different from the overlay 72%
  used by Menu/ContextMenu/Menubar)
- This component does not use menu roles (menuitem, etc.) since it is
  navigation disclosure, not command invocation
- **Svelte gap**: the trigger currently renders only the `__label` span and
  never reads `item.icon`, so the contract's per-item `icon` is not yet
  displayed by the reference. The trigger gap token (`--poodle-space-inline-sm`)
  exists for the icon; Svelte should render the icon ahead of the label. The
  `icon` prop stays in the contract — this is a Svelte under-implementation, not
  a contract removal.

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::navigation_menu`
- Spec struct: `NavigationMenuSpec` in primitives crate holds item definitions
- Component struct: `PoodleNavigationMenu` in components crate renders via `IntoElement`
- GPUI must model this as a nav landmark with expandable disclosure triggers
- The viewport is not an overlay in the Menu sense; it is an inline disclosed
  surface below the trigger strip
- GPUI must model `color-mix` as `token.opacity(token.a * multiplier)` since GPUI has no CSS color-mix
- Trigger border opacity: 72% on border-subtle
- Trigger bg opacity: 88% on background-surface
- Open trigger: 16% accent-base, border 42% accent-base mixed with border-default
- Hover trigger: 12% accent-base
- Viewport border: 74% on border-subtle
- Viewport bg: 96% on background-panel
- Trigger typography (label-family, 0.75rem, weight 600) matches Menubar triggers
- The slot pattern translates to a content callback or child-builder that
  receives the active item
- Leading `icon` (contract §3): `NavigationMenuEntry` carries an optional
  `icon: Option<String>`. When set, the trigger renders the icon ahead of the
  label, separated by the trigger gap (`space.inline.sm`), sized from the
  effective control size and tinted to the trigger foreground (`text-primary`).
  Both Rust targets (GPUI + Jetstream) render it; the Svelte reference still
  under-implements (see §9 Svelte gap).

## 10a. Jetstream Notes

- `NavigationMenu::from_spec(spec, theme).on_change(...)`, carrying the chosen
  entry's value. Disabled entries never fire.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] nav element with aria-label matches
- [ ] trigger aria-expanded and aria-controls match
- [ ] viewport aria-labelledby matches
- [ ] arrow left/right roving across triggers matches (with wrapping)
- [ ] home/end moves to first/last enabled trigger
- [ ] arrow down/enter/space opens viewport
- [ ] escape closes viewport and restores focus to trigger
- [ ] roving tabindex (0/-1) on triggers matches
- [ ] `onValueChange` callback semantics match
- [ ] disabled trigger behavior matches (aria-disabled, opacity, cursor)
- [ ] slot receives activeValue and activeItem

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] root uses grid with 0.5rem gap
- [ ] list uses inline-flex wrap with 0.25rem gap
- [ ] trigger uses label-family, 0.75rem, weight 600
- [ ] trigger default background uses background-surface 88%
- [ ] trigger default border uses border-subtle 72%
- [ ] trigger min-height uses control-height minus 0.125rem
- [ ] trigger padding 0 0.875rem matches
- [ ] trigger gap 0.375rem for icon matches
- [ ] trigger open uses accent-base 16% background, blended border-color (42% accent with border-default)
- [ ] trigger hover/focus uses accent-base 12% background
- [ ] viewport padding uses panel-y and panel-x space tokens
- [ ] viewport border uses border-subtle 74%
- [ ] viewport background uses panel 96%
- [ ] viewport uses elevation-overlay and radius-surface
- [ ] disabled uses state-opacity-disabled

### Tier 3: Implementation Freedom

- [ ] exact viewport animation stays internal
- [ ] trigger wrap behavior at narrow widths stays internal
- [ ] ID generation scheme is implementation-owned
- [ ] viewport mounting strategy is implementation-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact viewport animation may differ | overlay and motion internals differ | allowed | keep nav meaning and focus rules strict |
| GPUI viewport may use different layout primitive | desktop layout differs from CSS grid | allowed | spacing, padding, and visual density must match |
| GPUI uses opacity multiplication instead of CSS color-mix | platform capability | allowed | visual result must match |
| Viewport content rendering strategy may differ | platform rendering model | allowed | slot props must be equivalent |
| Rust `NavigationMenuEntry` adds a per-item `description: Option<String>` field | the Rust targets have no `children` snippet, so the active item's `description` is the viewport content source (slot-prop equivalent) | allowed | keep slot props equivalent; `icon` is now present on the Rust entry per §3 |

## 13. Specimen Definitions

### Horizontal Navigation

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Horizontal navigation | `ariaLabel="Main navigation"`, `value="components"` (initially active), five items: Home, Components, Tokens, Guides, Changelog (disabled) | Horizontal row of pill-style trigger buttons; Components trigger shows active/open styling with accent background and blended border; Changelog trigger shows disabled state at reduced opacity; viewport below shows active section name |

#### Navigation Items

| Item | Props / Config | Expected Visual |
|------|---------------|-----------------|
| Home | `value="home"` | Default pill-style trigger |
| Components | `value="components"`, initially selected | Active trigger with accent background |
| Tokens | `value="tokens"` | Default pill-style trigger |
| Guides | `value="guides"` | Default pill-style trigger |
| Changelog | `value="changelog"`, `disabled=true` | Disabled trigger at reduced opacity, not-allowed cursor |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: catalog nav bars, docs-site nav clusters, product
  section menus, feature discovery surfaces, settings navigation, documentation
  browsers
- future follow-up: connect with routing integration at composite layer if
  needed; keep route ownership outside the primitive; nested navigation,
  mobile collapse
