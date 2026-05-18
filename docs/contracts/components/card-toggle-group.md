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
[Root .card-toggle-group]  <div role="group">
  └── [Option .card-toggle-group__option]  <div role="button"> (repeated)
        └── [Card]  Card primitive (interactive, selected)
              ├── [Header slot]
              │     └── [HeaderRow .card-toggle-group__header]
              │           ├── [Indicator .card-toggle-group__indicator]
              │           ├── [Title .card-toggle-group__title]
              │           └── [Count .card-toggle-group__count] (optional)
              ├── [Description .card-toggle-group__description]  <p> (optional)
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

## 6. Use With CardRadioGroup

Use `CardRadioGroup` when the user must make one radio-style choice.

Use `CardToggleGroup` when the selection is a toggleable mode or named view,
especially when an active card may be cleared or resolved back to a default by
the parent.
