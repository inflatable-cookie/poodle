# CardToggleGroup

Status: detailed contract
Updated: 2026-05-18

## 1. Purpose

- Component name: `CardToggleGroup`
- Layer: `composites`
- Summary: a toggle group rendered as a grid of Card primitives, suitable for
  optional rich-card selection such as named list views, content states, or
  mode cards
- In scope: single card selection, optional deactivation, count display,
  disabled items, disabled group, keyboard navigation, custom card content via
  snippet, size and density variants
- Out of scope: required radio selection (use `CardRadioGroup`), multi-select
  card groups, editing card content, drag-and-drop reordering

## 2. Anatomy

```text
[Root .poodle-card-toggle-group]  <div role="group">
  └── [Option .poodle-card-toggle-group__option]  <div role="button"> (repeated)
        └── [Card]  Card primitive (interactive, selected)
              ├── [Header slot]
              │     └── [HeaderRow .poodle-card-toggle-group__header]
              │           ├── [Title .poodle-card-toggle-group__title]
              │           └── [Count .poodle-card-toggle-group__count] (optional)
              ├── [Description .poodle-card-toggle-group__description]  <p> (optional)
              └── [CardContent]  (optional, via `card(...)` snippet)
```

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `CardToggleItem[]` | `[]` | no | Array of options |
| `value` | `string \| null \| undefined` | `undefined` | no | Controlled selected value; leave undefined for uncontrolled mode |
| `defaultValue` | `string \| null` | `null` | no | Initial uncontrolled value |
| `allowDeactivation` | `boolean` | `false` | no | Selecting the active card clears the value |
| `columns` | `1 \| 2 \| 3 \| 4` | `2` | no | Number of grid columns |
| `ariaLabel` | `string \| null` | `null` | no | Accessible label for the group |
| `disabled` | `boolean` | `false` | no | Disables the entire group |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for spacing |
| `onValueChange` | `(value: string \| null) => void` | `undefined` | no | Called after user selection changes |

### Types

```ts
type CardToggleItem = {
  value: string;
  label: string;
  description?: string | null;
  count?: string | number | null;
  disabled?: boolean;
};
```

### Snippets

| Snippet | Scope | Purpose |
|---------|-------|---------|
| `card` | `{ item: CardToggleItem, selected: boolean, disabled: boolean }` | Custom content rendered inside each Card, below the description |

## 4. Behavior

- Selection is single-value.
- When `allowDeactivation=false`, selecting the active card keeps it selected.
- When `allowDeactivation=true`, selecting the active card clears the value to
  `null`.
- Disabled items cannot be selected or deactivated.
- Group-level `disabled` disables every item.

## 5. Accessibility

- Root: `role="group"` with optional `aria-label`
- Each option: `role="button"` with `aria-pressed="true"` or `"false"`
- Disabled options expose `aria-disabled="true"`
- `ArrowRight` / `ArrowDown` move to the next enabled option
- `ArrowLeft` / `ArrowUp` move to the previous enabled option
- `Space` / `Enter` toggles the focused option

## 6. Layout

### Sizing

- Root is a responsive auto-fit CSS grid; the `columns` prop is an upper bound,
  not a fixed count. Cards reflow to fit available width
- Grid template: `repeat(auto-fit, minmax(min(100%, max(min-width, calc((100% -
  (columns - 1) * gap) / columns))), 1fr))`
- `--poodle-card-toggle-group-gap`: `0.75rem` (density-adjusted)
- `--poodle-card-toggle-group-min-width`: `12rem` base (size-adjusted), the
  floor cards may shrink to before wrapping
- Each option has `height: 100%` and the inner Card stretches to fill, so cards
  in a row share equal height

### Composition

- Composes: `Card` primitive from `@poodle/svelte` (interactive, selected)
- Parent expectations: settings panels, list-view pickers, mode selectors
- Resizing rules: cards distribute evenly until they hit `min-width`, then wrap

## 7. Token Usage — Exact Values

#### `.poodle-card-toggle-group` (Root)

| Property | Value |
|----------|-------|
| `display` | `grid` |
| `grid-template-columns` | `repeat(auto-fit, minmax(…min-width / column-fraction…, 1fr))` (see §6) |
| `gap` | `var(--poodle-card-toggle-group-gap)` (`0.75rem`) |

The `--columns` CSS variable is set inline from the `columns` prop (1–4).

#### `.poodle-card-toggle-group__option`

| Property | Value |
|----------|-------|
| `height` | `100%` |
| `cursor` | `pointer` |
| `outline` | `none` |

#### `.poodle-card-toggle-group__option :global(.poodle-card)`

| Property | Value |
|----------|-------|
| `--poodle-card-gap` | `0.625rem` (size-adjusted) |
| `--poodle-card-padding-block` | `0.75rem` (size-adjusted) |
| `--poodle-card-padding-inline` | `0.75rem` (size-adjusted) |
| `height` | `100%` |

#### `.poodle-card-toggle-group__option:focus-visible :global(.poodle-card)`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

#### `.poodle-card-toggle-group__option[aria-disabled="true"]`

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

#### `.poodle-card-toggle-group__header`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.75rem` |

#### `.poodle-card-toggle-group__title`

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `font-size` | `0.9375rem` (size-adjusted) |
| `font-weight` | `600` |
| `color` | `var(--poodle-color-text-primary)` |

#### `.poodle-card-toggle-group__count` (optional)

| Property | Value |
|----------|-------|
| `margin-left` | `auto` |
| `flex-shrink` | `0` |
| `padding` | `0.0625rem 0.5rem` (size-adjusted) |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `999px` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font-size` | `0.75rem` (size-adjusted) |
| `font-weight` | `700` |
| `line-height` | `1.25` |

#### `.poodle-card-toggle-group__description` (optional)

| Property | Value |
|----------|-------|
| `margin` | `0` |
| `font-size` | `0.8125rem` (size-adjusted) |
| `line-height` | `1.5` |
| `color` | `var(--poodle-color-text-secondary)` |

### Size Adjustments

| Size | min-width | Card gap | Card padding | Title font-size | Description font-size | Count padding | Count font-size |
|------|-----------|----------|--------------|-----------------|-----------------------|---------------|-----------------|
| `xs` | `9.5rem` | `0.4375rem` | `0.5rem` | `0.6875rem` | `0.625rem` | `0.03125rem 0.3125rem` | `0.625rem` |
| `sm` | `10.5rem` | `0.5rem` | `0.625rem` | `0.75rem` | `0.6875rem` | `0.03125rem 0.375rem` | `0.6875rem` |
| `md` | `11.25rem` | `0.5625rem` | `0.6875rem` | `0.875rem` | `0.75rem` | `0.03125rem 0.4375rem` | `0.71875rem` |
| `lg` | `11.75rem` | `0.75rem` | `0.875rem` | `1rem` | `0.875rem` | `0.09375rem 0.5625rem` | `0.8125rem` |
| `xl` | `12rem` | `0.875rem` | `1rem` | `1.125rem` | `0.9375rem` | `0.125rem 0.625rem` | `0.875rem` |

The base (unscoped) values — Card gap `0.625rem`, Card padding `0.75rem`, title
`0.9375rem`, description `0.8125rem`, count padding `0.0625rem 0.5rem`, count
font `0.75rem`, min-width `12rem` — apply when no `data-size` rule matches.

### Density Adjustments

| Density | Root gap (`--poodle-card-toggle-group-gap`) |
|---------|---------------------------------------------|
| `compact` | `0.5rem` |
| `default` | `0.75rem` |
| `comfortable` | `1rem` |

Density only adjusts the inter-card grid gap; it does not change card height or
padding.

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-disabled` | `__title`, `__count`, `__description` | Marks disabled items (present in markup) |
| `data-card-toggle-index` | `__option` | DOM focus management via `querySelector` |
| `aria-pressed` | `__option` | Selection state |
| `aria-disabled` | `__option` | Targets disabled option styling |
| `data-size` | root | Drives size variant CSS |
| `data-density` | root | Drives density variant CSS |

## 8. Use With CardRadioGroup

Use `CardRadioGroup` when the user must make one radio-style choice.

Use `CardToggleGroup` when the selection is a toggleable mode or named view,
especially when an active card may be cleared or resolved back to a default by
the parent.
