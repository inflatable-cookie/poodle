# SplitButton

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `SplitButton`
- Layer: `foundation`
- Summary: a compound action control that pairs a primary button with a dropdown
  menu toggle for secondary actions
- In scope: variant and size parity with Button, dropdown menu with items and
  separators, keyboard navigation, loading state, disabled state
- Out of scope: nested submenus, icon-only split buttons, toggle/checkbox menu
  items

## 2. Anatomy

```text
[Root .split-button]  <div>
  ├── [Primary .split-button__primary]  <button>
  │   ├── [Spinner] (conditional, when isLoading)
  │   └── [Label (default slot)]
  ├── [Divider .split-button__divider]  <span>
  ├── [Toggle .split-button__toggle]  <button>
  │   └── [Chevron .split-button__chevron]  <svg>
  └── [Menu .split-button__menu]  <div role="menu"> (conditional, when open)
      ├── [Item .split-button__item]  <button role="menuitem"> (repeated)
      └── [Separator .split-button__separator]  <div role="separator"> (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | inline-flex container establishing variant CSS vars | border-radius, position |
| Primary | yes | main action button | background, border, color, typography, height, padding |
| Divider | yes | visual separator between halves | width, background, height |
| Toggle | yes | dropdown trigger button | background, border, color, height, width |
| Chevron | yes | directional indicator in toggle | width, height |
| Menu | no | floating dropdown panel | position, border, radius, background, shadow, z-index |
| Item | no | menu action entry | padding, height, border-radius, color, typography |
| Separator | no | visual divider between menu items | height, margin, background |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `variant` | `"primary" \| "secondary" \| "ghost"` | `"secondary"` | no | appearance family |
| `tone` | `"default" \| "danger"` | `"default"` | no | intent modifier; composes with variant for danger×primary, danger×secondary, danger×ghost |
| `size` | `"sm" \| "md" \| "lg"` | `"md"` | no | control size |
| `items` | `MenuItem[]` | — | yes | dropdown menu entries |
| `isDisabled` | `boolean` | `false` | no | disables entire control |
| `isLoading` | `boolean` | `false` | no | shows spinner in primary half, disables control |
| `ariaLabel` | `string \| null` | `null` | no | accessible name for primary button |
| `menuAriaLabel` | `string` | `"More actions"` | no | accessible name for toggle button |

### MenuItem Type

```typescript
type MenuItem = {
  value: string;
  label: string;
  isDisabled?: boolean;
  kind?: "action";
} | {
  kind: "separator";
};
```

### Slots

| Slot | Purpose |
|------|---------|
| default | primary button label content |

### Controlled And Uncontrolled

- Menu open/close is internally managed
- No controlled open prop

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | resting | variant-specific fill, border, text |
| hover (primary) | pointer enters primary half | `background: var(--pug-split-fill-hover)` |
| hover (toggle) | pointer enters toggle half | `background: var(--pug-split-fill-hover)` |
| active | press on either half | `background` darkened further |
| focus | keyboard focus on either half | standard focus ring |
| menu open | toggle clicked or keyboard opens | menu panel visible below root |
| disabled | `isDisabled=true` or `isLoading=true` | `opacity: state-opacity-disabled`, `cursor: not-allowed` |
| loading | `isLoading=true` | spinner in primary half, control disabled |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `click` | primary button activated | `MouseEvent` | suppressed while disabled or loading |
| `action` | menu item selected | `{value: string}` | fires after menu closes |

## 6. Accessibility

### Semantics

- Root: no semantic role (presentational container)
- Primary button: native `<button>`, `aria-label` from prop
- Toggle button: native `<button>`, `aria-label` from menuAriaLabel, `aria-haspopup="menu"`, `aria-expanded`
- Menu: `role="menu"`
- Items: `role="menuitem"`, `aria-disabled` when item disabled
- Separators: `role="separator"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | activates focused button or menu item |
| `ArrowDown` | opens menu from toggle; moves focus to next item in menu |
| `ArrowUp` | moves focus to previous item in menu |
| `Home` | moves focus to first menu item |
| `End` | moves focus to last menu item |
| `Escape` | closes menu, returns focus to toggle |
| `Tab` | moves focus out of component, closes menu |

### Focus And Announcement

- focus entry: primary button or toggle receives visible focus ring
- menu open: first menu item receives focus
- menu close: focus returns to toggle button

## 7. Layout

### Sizing

- Root: `width: fit-content`, halves stretch vertically
- Primary half: `min-width: 4rem`, `padding: 0 space-control-x`
- Toggle half: `width: 2rem`, `padding: 0`
- Menu: `min-width: 12rem`, positioned absolutely below root

### Composition

- parent expectations: toolbars, action bars, form actions
- child expectations: label text in primary, menu items via prop
- resizing: width auto-fits content

## 8. Token Usage — Exact Values

### CSS Custom Properties (variant system)

| Var | Secondary (default) | Primary | Ghost |
|-----|---------------------|---------|-------|
| `--pug-split-fill` | `var(--pug-color-background-surface)` | `var(--pug-color-accent-base)` | `color-mix(in srgb, var(--pug-color-background-surface) 42%, transparent)` |
| `--pug-split-fill-hover` | `color-mix(in srgb, var(--pug-split-fill) 84%, var(--pug-color-background-elevated))` | same formula | same formula |
| `--pug-split-border` | `var(--pug-color-border-default)` | `color-mix(in srgb, var(--pug-color-accent-base) 84%, black)` | `color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent)` |
| `--pug-split-text` | `var(--pug-color-text-primary)` | `var(--pug-color-text-inverse)` | `var(--pug-color-text-primary)` |

### Tone: danger

| Selector | `--pug-split-fill` | `--pug-split-border` | `--pug-split-text` | `--pug-split-shadow` |
|----------|--------------------|----------------------|--------------------|----------------------|
| `[data-tone="danger"]` (secondary base) | `color-mix(in srgb, var(--pug-color-status-danger) 16%, var(--pug-color-background-surface))` | `color-mix(in srgb, var(--pug-color-status-danger) 46%, var(--pug-color-border-default))` | `var(--pug-color-text-primary)` | default |
| `[data-variant="primary"][data-tone="danger"]` | `var(--pug-color-status-danger)` | `color-mix(in srgb, var(--pug-color-status-danger) 84%, black)` | `var(--pug-color-text-inverse)` | `inset 0 0.0625rem 0 color-mix(white 14%, transparent), 0 0.375rem 1.125rem color-mix(black 18%, transparent)` |
| `[data-variant="ghost"][data-tone="danger"]` | `transparent` | `transparent` | `var(--pug-color-status-danger)` | `none` |

### Root

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `display` | `inline-flex` |
| `align-items` | `stretch` |
| `width` | `fit-content` |
| `border-radius` | `var(--pug-treatment-interactive-solid-radius, var(--pug-radius-control))` |

### Primary and Toggle (shared)

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `height` | `var(--pug-size-control-height)` |
| `border` | `0.0625rem solid var(--pug-split-border)` |
| `background` | `var(--pug-split-fill)` |
| `color` | `var(--pug-split-text)` |
| `cursor` | `pointer` |
| `font-family` | `var(--pug-typography-label-family)` |
| `font-size` | `var(--pug-typography-label-size)` |
| `font-weight` | `var(--pug-typography-label-weight)` |
| `letter-spacing` | `0.01em` |
| `line-height` | `1` |
| `transition` | `background, border-color` at `motion-duration-interaction motion-easing-standard` |

### Size adjustments

| Size | height | font-size |
|------|--------|-----------|
| `sm` | `calc(var(--pug-size-control-height) - 0.375rem)` | `0.75rem` |
| `md` | `var(--pug-size-control-height)` | `var(--pug-typography-label-size)` |
| `lg` | `calc(var(--pug-size-control-height) + 0.375rem)` | `0.875rem` |

### Primary half

| Property | Value |
|----------|-------|
| `gap` | `var(--pug-space-inline-sm)` |
| `min-width` | `4rem` |
| `padding` | `0 var(--pug-space-control-x)` |
| `border-right` | `0` |
| `border-radius` | `var(--pug-treatment-interactive-solid-radius, var(--pug-radius-control)) 0 0 var(--pug-treatment-interactive-solid-radius, var(--pug-radius-control))` |

### Toggle half

| Property | Value |
|----------|-------|
| `width` | `2rem` |
| `padding` | `0` |
| `border-left` | `0` |
| `border-radius` | `0 var(--pug-treatment-interactive-solid-radius, var(--pug-radius-control)) var(--pug-treatment-interactive-solid-radius, var(--pug-radius-control)) 0` |

### Divider

| Property | Value |
|----------|-------|
| `width` | `0.0625rem` |
| `background` | `color-mix(in srgb, var(--pug-split-text) 22%, transparent)` |
| `align-self` | `center` |
| `height` | `60%` |

### Chevron

| Property | Value |
|----------|-------|
| `width` | `0.75rem` |
| `height` | `0.75rem` |

### Spinner

| Property | Value |
|----------|-------|
| `width` | `0.75rem` |
| `height` | `0.75rem` |
| `border` | `0.125rem solid color-mix(in srgb, currentColor 24%, transparent)` |
| `border-top-color` | `currentColor` |
| `border-radius` | `999px` |
| `animation` | `rotate(360deg) 0.8s linear infinite` |

### Hover (not disabled)

| Property | Value |
|----------|-------|
| `background` | `var(--pug-split-fill-hover)` |

### Focus

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

### Disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--pug-state-opacity-disabled)` |

### Menu

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `calc(100% + 0.375rem)` |
| `left` | `0` |
| `z-index` | `var(--pug-z-index-overlay-menu)` |
| `min-width` | `12rem` |
| `padding` | `0.25rem` |
| `border` | `0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent)` |
| `border-radius` | `var(--pug-radius-surface)` |
| `background` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| `box-shadow` | `var(--pug-elevation-overlay)` |

### Item

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `width` | `100%` |
| `min-height` | `2rem` |
| `padding` | `0.375rem 0.5rem` |
| `border` | `0` |
| `border-radius` | `calc(var(--pug-radius-control) - 0.125rem)` |
| `background` | `transparent` |
| `color` | `var(--pug-color-text-primary)` |
| `cursor` | `pointer` |
| `font` | `inherit` |
| `font-size` | `var(--pug-typography-label-size)` |
| `text-align` | `left` |

### Item hover / focus-visible

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--pug-color-accent-base) 16%, transparent)` |
| `outline` | `none` |

### Separator (menu)

| Property | Value |
|----------|-------|
| `width` | `100%` |
| `height` | `0.0625rem` |
| `margin` | `0.25rem 0` |
| `background` | `color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent)` |

## 9. Svelte Notes

- Uses CSS custom properties (`--pug-split-fill`, etc.) for the variant system
- `data-variant`, `data-tone`, `data-size`, `data-open` data attributes on root
- Menu state managed via internal `$state(false)` boolean
- Click-outside handler closes menu
- `MenuItem[]` prop drives menu rendering; separators rendered as dividers
- Chevron rotates 180deg when menu is open via CSS transform

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::split_button`
- Spec struct: `SplitButtonSpec` in primitives crate
- Component struct: `PugSplitButton` in components crate
- Menu positioning: GPUI must calculate absolute position relative to root
- Variant CSS var system maps to Rust enum with computed color values
- Divider height percentage (60%) must be calculated from actual control height

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] variant and size props produce matching visual output
- [ ] primary click fires click event, item selection fires action event
- [ ] keyboard navigation matches (ArrowDown/Up, Home/End, Enter/Space, Escape)
- [ ] disabled and loading states suppress interaction
- [ ] menu ARIA roles match (menu, menuitem, separator)

### Tier 2: Visual Parity

- [ ] all three variant color schemes plus danger tone combinations match exactly
- [ ] all three sizes match (height, font-size)
- [ ] divider appearance matches (22% text opacity, 60% height)
- [ ] menu panel appearance matches (border, radius, background, shadow)
- [ ] item hover state matches (16% accent-base)
- [ ] focus ring matches
- [ ] spinner appearance matches

### Tier 3: Implementation Freedom

- [ ] menu open/close animation is platform-owned
- [ ] click-outside detection method is platform-owned
- [ ] transition timing is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Menu animation | GPUI may not support CSS transitions for menu reveal | allowed | match where possible |
| Click-outside detection | platform-specific event handling | allowed | same behavioral result |

## 13. Specimen Definitions

### Group: Primary variant

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Primary variant | `variant="primary"`, `items=[Save as draft, Save as template, (separator), Discard changes]`, label "Save" | Primary-filled split button with chevron toggle; clicking toggle reveals menu with three items and one separator |

### Group: Secondary variant

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Secondary variant | `variant="secondary"`, `items=[Export as CSV, Export as JSON, Export as PDF]`, label "Export" | Secondary-styled split button; menu shows three export options without separators |

### Group: Danger tone

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Danger tone | `tone="danger"`, `items=[Delete selected, Delete all]`, label "Delete" | Danger-tinted split button (danger fill/border/text); menu shows two destructive actions |

### Group: Loading state

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Loading state | `variant="primary"`, `isLoading=true`, label "Saving..." | Spinner visible in primary half; entire control disabled and non-interactive |

### Group: Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `variant="secondary"`, `isDisabled=true`, label "Save" | Reduced opacity; cursor not-allowed; no interaction possible |

### Group: Last action

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Last action | (reactive display) | Text showing the last triggered action from click or menu item selection; confirms event wiring |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: toolbars, action bars, form submission groups
- future follow-up: icon support in menu items, nested submenus
