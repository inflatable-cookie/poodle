# CardRadioGroup

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `CardRadioGroup`
- Layer: `composites`
- Summary: a radio group rendered as a grid of Card primitives, each with a radio indicator, title, and optional description -- suitable for plan selection, size pickers, and similar exclusive-choice UIs
- In scope: single selection from a set of card-based options, radio indicator with checked/unchecked states, configurable grid columns, per-item and group-level disabled states, keyboard navigation (arrow keys with roving tabindex), custom card content via snippet, size and density variants
- Out of scope: multi-select (use CheckboxGroup), inline radio buttons (use RadioGroup), card content editing, drag-and-drop reordering

## 2. Anatomy

```text
[Root .poodle-card-radio-group]  <div role="radiogroup">
  └── [Option .poodle-card-radio-group__option]  <div role="radio"> (repeated)
        └── [Card]  Card primitive (interactive, selected)
              ├── [Header slot]
              │     └── [HeaderRow .poodle-card-radio-group__header]
              │           ├── [Indicator .poodle-card-radio-group__indicator]
              │           │     └── [Dot .poodle-card-radio-group__dot]  (when checked)
              │           └── [Title .poodle-card-radio-group__title]
              ├── [Description .poodle-card-radio-group__description]  <p> (optional)
              └── [CardContent]  (optional, via `card(...)` snippet)
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Root | `<div>` | yes | CSS Grid container with `role="radiogroup"`, optional `aria-label` |
| Option | `<div>` | yes (repeated) | `role="radio"`, `aria-checked`, `aria-disabled`, roving `tabindex` |
| Card | `Card` primitive | yes | `interactive` and `selected` bindings |
| HeaderRow | `<div>` | yes | Flex row containing indicator and title |
| Indicator | `<span>` | yes | Circular radio indicator; border-only when unchecked, filled accent when checked |
| Dot | `<span>` | no | Inner dot visible only when checked |
| Title | `<span>` | yes | Item label text |
| Description | `<p>` | no | Optional item description text |
| CardContent | snippet | no | Custom content via `card(...)` snippet |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `CardRadioItem[]` | `[]` | no | Array of options |
| `value` | `string \| null \| undefined` | `undefined` | no | Currently selected value; bind for two-way. `undefined` = uncontrolled (component owns state); any defined value (including `null`) = controlled |
| `columns` | `1 \| 2 \| 3 \| 4` | `2` | no | Number of grid columns |
| `ariaLabel` | `string \| null` | `null` | no | Accessible label for the radiogroup |
| `disabled` | `boolean` | `false` | no | Disables the entire group |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for spacing |
| `onValueChange` | `((value: string) => void) \| undefined` | `undefined` | no | Fires after value update; does not fire for disabled items |

### Types

```ts
type CardRadioItem = {
  value: string;
  label: string;
  description?: string | null;
  disabled?: boolean;
};
```

### Snippets

| Snippet | Scope | Purpose |
|---------|-------|---------|
| `card` | `{ item: CardRadioItem, checked: boolean, disabled: boolean }` | Custom content rendered inside each Card, below the description |

### Controlled And Uncontrolled

- `value` defaults to `undefined`. While `undefined`, the component is
  uncontrolled and owns its own selection state internally
- Supplying any defined `value` (including `null` for an explicit empty
  selection) makes it controlled; the host owns updates via `bind:value` or
  `onValueChange`
- Selection state is also surfaced via `onValueChange`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| unchecked | item not selected | Indicator shows border only, Card is not selected |
| checked | item is selected | Indicator fills with accent color and shows inner dot, Card shows selected state |
| disabled (item) | `item.disabled` is true | Option has `cursor: not-allowed`, reduced opacity, cannot be selected |
| disabled (group) | `disabled` is true | All options disabled regardless of per-item `disabled` |
| focus-visible | keyboard focus on option | Card shows focus ring outline |

### Component States

| State | Description |
|-------|-------------|
| `value` | Tracks which item is currently selected |
| `isChecked` (derived) | `value === item.value` per item |
| `isItemDisabled` (derived) | `disabled || item.disabled === true` per item |

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | User selects an option | `string` | Fires after value update; does not fire for disabled items |

## 6. Accessibility

### Semantics

- Root: `role="radiogroup"` with optional `aria-label`
- Each option: `role="radio"` with `aria-checked="true"` or `"false"`, `aria-disabled="true"` when disabled
- Indicator: `aria-hidden="true"` (visual-only, semantics on the option wrapper)
- Card: receives `ariaLabel={item.label}` for screen reader announcement

### Keyboard

| Key | Behavior |
|-----|----------|
| `ArrowRight` / `ArrowDown` | Move to next enabled option (wraps around) |
| `ArrowLeft` / `ArrowUp` | Move to previous enabled option (wraps around) |
| `Tab` | Enters/exits the radiogroup; roving tabindex ensures correct entry point |

### Focus And Announcement

- Roving tabindex: `tabindex="0"` on the checked item (or first item if none checked); `tabindex="-1"` on all others
- Disabled items have `tabindex="-1"` and are skipped by arrow navigation
- Focus is programmatically moved via `el.focus()` after arrow key selection

## 7. Layout

### Sizing

- Root: CSS Grid with `grid-template-columns: repeat(var(--columns, 2), 1fr)`
- Grid gap: `0.75rem` (density-adjusted)
- Header row gap: `0.5rem`
- Indicator size: `1.125rem` width and height, `flex-shrink: 0` (size-adjusted)
- Indicator border: `0.125rem` solid
- Inner dot: `0.375rem` width and height (size-adjusted)

### Composition

- Composes: `Card` primitive from `@poodle/svelte`
- Parent expectations: form sections, settings pages, plan selection flows
- Resizing rules: grid columns distribute evenly; cards stretch to fill column width

## 8. Token Usage -- Exact Values

#### `.poodle-card-radio-group` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(var(--columns, 2), 1fr)` |
| `gap` | `0.75rem` |

The `--columns` CSS variable is set inline from the `columns` prop (1, 2, 3, or 4).

#### `.poodle-card-radio-group__option`

| Property | Value |
|----------|-------|
| `cursor` | `pointer` |
| `outline` | `none` |

#### `.poodle-card-radio-group__option:focus-visible :global(.poodle-card)`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

#### `.poodle-card-radio-group__option[aria-disabled="true"]`

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

#### `.poodle-card-radio-group__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.5rem` |

#### `.poodle-card-radio-group__indicator` (Unchecked)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.125rem` |
| `height` | `1.125rem` |
| `flex-shrink` | `0` |
| `border` | `0.125rem solid var(--poodle-color-border-default)` |
| `border-radius` | `999px` |
| `background` | `transparent` |
| `transition` | `border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard), background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.poodle-card-radio-group__indicator[data-checked="true"]` (Checked)

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-base)` |
| `background` | `var(--poodle-color-accent-base)` |

#### `.poodle-card-radio-group__dot`

| Property | Value |
|----------|-------|
| `width` | `0.375rem` |
| `height` | `0.375rem` |
| `border-radius` | `999px` |
| `background` | `var(--poodle-color-text-inverse)` |

#### `.poodle-card-radio-group__title`

| Property | Value |
|----------|-------|
| `font-size` | `0.9375rem` |
| `font-weight` | `600` |
| `color` | `var(--poodle-color-text-primary)` |

#### `.poodle-card-radio-group__description`

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `font-size` | `0.8125rem` |
| `line-height` | `1.5` |
| `color` | `var(--poodle-color-text-secondary)` |

### Size Adjustments

| Size | Indicator | Dot | Title font-size | Description font-size |
|------|-----------|-----|----------------|----------------------|
| `xs` | `0.875rem` | `0.25rem` | `0.75rem` | `0.6875rem` |
| `sm` | `1rem` | (default) | `0.8125rem` | `0.75rem` |
| `md` | (default `1.125rem`) | (default `0.375rem`) | (default `0.9375rem`) | (default `0.8125rem`) |
| `lg` | `1.25rem` | `0.4375rem` | `1.0625rem` | `0.875rem` |
| `xl` | `1.375rem` | `0.5rem` | `1.125rem` | `0.9375rem` |

### Density Adjustments

| Density | Root gap | Card padding |
|---------|----------|-------------|
| `compact` | `0.5rem` | `padding-inline: 0.5rem` (via `:global(.poodle-card)`) |
| `default` | `0.75rem` | (Card default) |
| `comfortable` | `0.875rem` | (Card default) |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-checked` | `.poodle-card-radio-group__indicator` | Targets checked indicator styling |
| `data-disabled` | `.poodle-card-radio-group__indicator`, `.poodle-card-radio-group__title`, `.poodle-card-radio-group__description` | Marks disabled items (present in markup) |
| `data-card-radio-index` | `.poodle-card-radio-group__option` | Used for DOM focus management via `querySelector` |
| `aria-disabled` | `.poodle-card-radio-group__option` | Targets disabled option styling |
| `data-size` | `.poodle-card-radio-group` root | Drives size variant CSS |
| `data-density` | `.poodle-card-radio-group` root | Drives density variant CSS |

## 9. Svelte Notes

- Selection flows through the `onValueChange` callback and `bind:value`; no
  `createEventDispatcher` / `change` event
- Composes `Card` primitive from `@poodle/svelte` with `interactive` and `selected` props
- Arrow key navigation uses `data-card-radio-index` attributes and `document.querySelector` for DOM focus management
- Disabled items are filtered out of the enabled navigation list
- Resolves size via `resolveSemanticControlSize` from `getUiPresentation()`

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::card_radio_group`
- Render as a flex grid with Card children
- Arrow key navigation needs custom key event handling

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] `onValueChange` callback name and payload matches
- [ ] roving tabindex behavior matches
- [ ] arrow key navigation skips disabled items and wraps
- [ ] CardRadioItem type is identical

### Tier 2: Visual Parity

- [ ] indicator size, border, and dot styling match per size variant
- [ ] checked/unchecked transitions match
- [ ] focus ring styling matches
- [ ] disabled opacity matches
- [ ] density spacing matches

### Tier 3: Implementation Freedom

- [ ] rendering internals stay internal

## 12. Specimen Definitions

### Plan Selection (2 Columns)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Plan selection (2 columns) | four items (Free, Pro, Team, Enterprise); Enterprise is `disabled`; `value="pro"`, `columns={2}`, `ariaLabel="Select a plan"` | 2x2 grid with Pro checked, Enterprise dimmed and non-interactive |

### Instance Size (3 Columns)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Instance size (3 columns) | three items (Small, Medium, Large) with descriptions; `value` initially null, `columns={3}`, `ariaLabel="Select an instance size"` | 3-column grid, no initial selection |

### Disabled Group

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled group | same three size items; `value="md"`, `columns={3}`, `disabled`, `ariaLabel="Disabled selection"` | 3-column grid with Medium checked, entire group dimmed and non-interactive |
