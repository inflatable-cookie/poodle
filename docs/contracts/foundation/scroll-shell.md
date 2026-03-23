# Scroll Shell

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `ScrollShell`
- Layer: `foundation`
- Summary: a reusable scrolling boundary with explicit viewport ownership,
  direction control, focus behavior, and assistive-technology expectations
- In scope: viewport shell, overflow axis control, keyboard reachability,
  optional region labeling, interior padding, scroll event forwarding
- Out of scope: virtualized list semantics, custom scrollbar styling as a
  contract requirement, inertial physics tuning, scroll-position value model

## 2. Anatomy

```text
[Root .scroll-shell]  <div>
  └── [Viewport .scroll-shell__viewport]  <div>
        └── [Content .scroll-shell__content / .scroll-shell__content--h]
              └── [Children...] (slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | shell boundary; clips overflow | border-radius |
| Viewport | yes | scrollable element; owns overflow | overflow, overscroll-behavior, focus ring, padding |
| Content | yes | content wrapper; enables horizontal min-width | min-width for horizontal scrolling |
| Children | no | arbitrary slotted content | caller-owned |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `direction` | `ScrollDirection: "vertical" \| "horizontal" \| "both"` | `"vertical"` | no | owned scroll axis |
| `padding` | `"none" \| "sm" \| "md"` | `"none"` | no | viewport interior spacing via scaleToSpace utility |
| `asRole` | `"region" \| "group" \| null` | `null` | no | semantic opt-in; defaults to "region" when isFocusable is true |
| `label` | `string \| null` | `null` | no | accessible label; defaults to "Scrollable content" when isFocusable is true and no label provided |
| `isFocusable` | `boolean` | `false` | no | adds viewport to tab order for keyboard scrolling |

### Slots

| Slot | Purpose |
|------|---------|
| default | scrolled content |

### Controlled And Uncontrolled

- no controlled value model
- scroll position is runtime state, not a public value model in this contract

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | neutral scrolling shell, content visible |
| focus | `isFocusable=true` and viewport receives keyboard focus | visible focus ring on viewport |
| overflowed | content exceeds viewport on owned axis | native scrollbar appears, content scrollable |

### Component States

State table is sufficient. Scroll position is runtime state, not a public value
model in this baseline contract.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `scroll` | viewport scroll position changes | native `Event` | forwarded from viewport element |

## 6. Accessibility

### Semantics

- Role: none by default; defaults to `"region"` when `isFocusable=true`
  (unless `asRole` explicitly overrides); `"region"` or `"group"` by explicit
  `asRole` opt-in
- Required attributes:
  - `aria-label` defaults to `"Scrollable content"` when `isFocusable=true`
    and no `label` prop provided
  - `aria-label` from `label` prop when provided
- Optional attributes: `aria-describedby` for scroll instructions
- Labeling rules: when the scroll shell is a named destination or focusable
  region, it must have an accessible label

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | reaches the viewport only when `isFocusable=true` |
| `Arrow Up/Down` | scroll vertically when viewport is focused (direction includes vertical) |
| `Arrow Left/Right` | scroll horizontally when viewport is focused (direction includes horizontal) |
| `Page Up/Page Down` | scroll larger increments when viewport is focused |
| `Home/End` | move to start/end when viewport is focused |

### Focus And Announcement

- focus entry: viewport enters tab order only when `isFocusable=true`
  (tabindex=0)
- focus exit: focus moves to children or out of the shell without trap behavior
- live-region behavior: none by default

### GPUI Accessibility Expectations

- GPUI implementations must expose the scroll container as a native accessible
  region when the contract opts in
- keyboard scrolling must be implemented intentionally where the platform does
  not provide it automatically
- the accessible node should communicate scrollability and region labeling
  without requiring HTML/ARIA mechanics

## 7. Layout

### Sizing

- root and viewport both fill available space (`width: 100%`, `height: 100%`)
- requires explicit or inherited size constraints from parent to create a
  scrolling boundary
- `min-width: 0` and `min-height: 0` on root prevent flex overflow

### Composition

- parent expectations: constrained surface, layout region, or panel with
  explicit height
- child expectations: arbitrary content, including focusable descendants
- resizing rules: viewport expands or contracts with parent constraints while
  preserving declared scroll ownership

## 8. Token Usage — Exact Values

### Root (.scroll-shell)

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `min-height` | `0` |
| `width` | `100%` |
| `height` | `100%` |
| `overflow` | `hidden` |
| `border-radius` | `var(--poodle-radius-surface)` |

### Viewport (.scroll-shell__viewport) — base styles

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `height` | `100%` |
| `overscroll-behavior` | `contain` |
| `border-radius` | `inherit` |

### Viewport overflow — by direction

| Direction | Overflow Styles |
|-----------|----------------|
| `vertical` | `overflow-y: auto; overflow-x: hidden` |
| `horizontal` | `overflow-x: auto; overflow-y: hidden` |
| `both` | `overflow: auto` |

### Viewport focus — .scroll-shell__viewport:focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Content — horizontal mode (.scroll-shell__content--h)

Applied when direction is `"horizontal"` or `"both"`:

| Property | Value |
|----------|-------|
| `min-width` | `max-content` |

### Padding (via scaleToSpace utility, applied as inline style on viewport)

| Scale | Value |
|-------|-------|
| `none` | `0` |
| `sm` | `var(--poodle-space-inline-sm)` |
| `md` | `var(--poodle-space-panel-y)` |

### Viewport Attributes (conditional)

| Condition | Attributes |
|-----------|------------|
| `isFocusable=true` | `tabindex="0"` |
| `isFocusable=true` (default role) | `role="region"` |
| `isFocusable=true` (default label) | `aria-label="Scrollable content"` |
| `asRole` provided | `role="{asRole}"` (overrides default) |
| `label` provided | `aria-label="{label}"` (overrides default) |

## 9. Svelte Notes

- three-layer DOM structure: root (clip boundary), viewport (scroll owner),
  content (sizing wrapper)
- overflow is applied to viewport via inline styles using a direction-to-overflow
  mapping function (`overflowForDirection`)
- when direction is `"horizontal"` or `"both"`, the content wrapper receives
  the `.scroll-shell__content--h` class to set `min-width: max-content`,
  ensuring content does not collapse
- padding is applied as inline style on the viewport via shared `scaleToSpace`
  utility
- `isFocusable=true` adds `tabindex="0"` to viewport, and defaults role to
  `"region"` and aria-label to `"Scrollable content"` (both overridable via
  props)
- `scroll` event is forwarded from the viewport element
- native browser scrolling is used; no JavaScript-driven scroll simulation
- `overscroll-behavior: contain` prevents scroll chaining to parent

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::scroll_shell`
- Spec struct: `ScrollShellSpec` in primitives crate
- GPUI scroll ownership, focusability, and assistive-technology signaling must
  be implemented explicitly
- if GPUI lacks automatic parity for keyboard scrolling or spoken scroll-region
  semantics, Poodle must add that behavior rather than documenting it away
- visual scrollbar appearance may differ between platforms (known delta), but
  scrollability and focus behavior must not
- `overscroll-behavior: contain` equivalent must prevent scroll chaining where
  possible
- direction-to-overflow mapping must produce equivalent axis locking

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] scroll-axis ownership matches (vertical, horizontal, both)
- [ ] focusability rules match (isFocusable adds to tab order)
- [ ] keyboard scrolling behavior matches when viewport is focused
- [ ] named-region semantics match (default role/label when focusable)
- [ ] scroll event forwarding matches

### Tier 2: Visual Parity

- [ ] focus ring matches (border-width-focus, accent-focusRing, 0.125rem offset)
- [ ] border-radius matches (radius-surface)
- [ ] padding scale values match
- [ ] overflow clipping and scrollbar presence communicate the same intent
- [ ] horizontal content min-width: max-content behavior matches

### Tier 3: Implementation Freedom

- [ ] native browser scrollbars vs GPUI-native scroll rendering stays internal
- [ ] CSS overflow vs GPUI scroll API stays internal
- [ ] overscroll-behavior implementation may differ

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| scrollbar visuals may differ | platform-native rendering is acceptable | allowed | keep behavior parity strict |
| overscroll-behavior | GPUI may not support contain semantics natively | allowed | prevent scroll chaining where possible |

## 13. Specimen Definitions

### Group: Vertical scroll

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Vertical scroll | `direction="vertical"`, `label="Scrollable content"`, 12 Surface children in a 10rem-tall container | Vertical scrollbar visible; 12 bordered surface items stack vertically; scrolling reveals items beyond the viewport |

### Group: Horizontal scroll

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Horizontal scroll | `direction="horizontal"`, `label="Horizontal items"`, 10 Surface children in a flex row | Horizontal scrollbar visible; 10 bordered surface items arranged in a row; each item has nowrap text "Column N" |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: lists, shell panels, menus, inspectors, browsers,
  detail views, data tables
- future follow-up: add virtualization guidance when list/grid contracts arrive;
  consider scroll-position value model for programmatic scroll control
