# OrderBy

Status: detailed contract
Updated: 2026-03-27

## 1. Purpose

- Component name: `OrderBy`
- Layer: `foundation`
- Summary: a popover-backed ordered sort builder for lists and tables with
  multi-field sorting, direction toggles, ordered summaries, and clear/reset
  behavior
- In scope: ordered sort arrays, add/remove fields, per-field direction,
  compact trigger summary, clear/reset, disabled state
- Out of scope: column header sort controls (see Table), drag-and-drop field
  reordering

## 2. Anatomy

```text
[Root .order-by] <div>
  ├── [Trigger .order-by__trigger] <button>
  │   ├── [Label .order-by__label]
  │   ├── [Summary .order-by__summary]
  │   └── [Chevron .order-by__chevron]
  ├── [Reset .order-by__reset] <button> (conditional)
  └── [Panel .order-by__panel] <div> (inside popover)
      ├── [List .order-by__list]
      │   └── [Item .order-by__item] <div> (repeated)
      ├── [Add .order-by__add]
      └── [Footer .order-by__footer] (conditional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | toolbar container | flex layout, gap |
| Label | yes | "Sort by" label text | font, color, letter-spacing |
| Fields | yes | field button group | flex layout, gap |
| Field | yes | sort toggle button | height, padding, border, radius, background, color, font |
| Arrow | no | directional indicator on active field | width, height, transform |
| Reset | no | clear sort button | width, height, border-radius, background, color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `fields` | `SortField[]` | — | yes | available sort fields |
| `value` | `OrderByValue` | `[]` | no | ordered multi-field sort state |
| `activeSort` | `ActiveSort \| null` | `null` | no | legacy first-sort compatibility signal |
| `maxFields` | `number \| null` | `null` | no | optional cap on active sort fields |
| `compact` | `boolean` | `false` | no | shortens long trigger summaries |
| `ariaLabel` | `string` | `"Sort by"` | no | accessible name for toolbar |
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for spacing |
| `disabled` | `boolean` | `false` | no | disables all field buttons |

### SortField Type

```typescript
type SortField = {
  key?: string;
  value?: string;
  label: string;
  disabled?: boolean;
  defaultDirection?: "asc" | "desc";
};
```

### OrderByValue Type

```typescript
type OrderByValue = Array<{
  key: string;
  direction: "asc" | "desc";
}>;
```

### ActiveSort Type

```typescript
type ActiveSort = {
  field: string;
  direction: "asc" | "desc";
};
```

### Controlled And Uncontrolled

- Multi-field sort state is controlled via `value`
- `activeSort` remains available as a compatibility signal for single-sort flows
- Changes dispatched via `change` event; parent updates `value`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no active sort | trigger shows placeholder summary |
| populated | one or more sort fields active | trigger shows ordered summary |
| compact populated | `compact=true` and more than 2 fields | summary truncates to first two plus count |
| disabled | `disabled=true` | trigger and controls unavailable |
| reset visible | one or more fields active | reset button appears |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `change` | any sort mutation | `{value: OrderByValue, sort: ActiveSort \| null}` | `sort` mirrors the first item for compatibility |

## 6. Accessibility

### Semantics

- Root: `role="toolbar"`, `aria-label` from prop
- Field buttons: `aria-pressed="true"` when active, `aria-pressed="false"` otherwise
- Reset button: `aria-label="Clear sort"`
- Disabled fields: `aria-disabled="true"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | toggles field sort (asc→desc→null cycle) or activates reset |
| `Tab` | moves focus between toolbar and surrounding elements |
| `ArrowRight` | moves focus to next field button within toolbar |
| `ArrowLeft` | moves focus to previous field button within toolbar |

### Focus And Announcement

- focus entry: first or active field button receives focus
- focus ring: standard accent outline on focused button
- sort change: screen reader announces new sort state

## 7. Layout

### Sizing

- Root: flex row, wraps
- Field buttons: inline-flex, height 1.75rem
- Reset button: 1.5rem square

### Composition

- parent expectations: list headers, table toolbars, filter panels
- child expectations: none (self-contained)
- resizing: wraps when parent width insufficient

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.5rem` |
| `flex-wrap` | `wrap` |

### Root disabled

| Property | Value |
|----------|-------|
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Label

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `text-transform` | `uppercase` |
| `letter-spacing` | `0.05em` |

### Fields

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.25rem` |

### Field button

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `gap` | `0.25rem` |
| `height` | `1.75rem` |
| `padding` | `0 0.5rem` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `font-family` | `var(--poodle-typography-label-family)` |
| `font-size` | `0.75rem` |
| `font-weight` | `var(--poodle-typography-label-weight)` |
| `transition` | `background, border-color` at `motion-duration-interaction motion-easing-standard` |

### Field button hover

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-background-surface) 84%, var(--poodle-color-background-elevated))` |

### Field button active (sorted)

| Property | Value |
|----------|-------|
| `border-color` | `var(--poodle-color-accent-base)` |
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, var(--poodle-color-background-surface))` |
| `color` | `var(--poodle-color-accent-base)` |

### Field button disabled

| Property | Value |
|----------|-------|
| `cursor` | `not-allowed` |
| `opacity` | `var(--poodle-state-opacity-disabled)` |

### Field button focus

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

### Arrow

| Property | Value |
|----------|-------|
| `width` | `0.75rem` |
| `height` | `0.75rem` |
| `transition` | `transform` at `motion-duration-interaction motion-easing-standard` |

### Arrow (descending)

| Property | Value |
|----------|-------|
| `transform` | `rotate(180deg)` |

### Reset button

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.5rem` |
| `height` | `1.5rem` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `cursor` | `pointer` |

### Reset button hover

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-primary)` |

### Reset icon

| Property | Value |
|----------|-------|
| `width` | `0.75rem` |
| `height` | `0.75rem` |

### Size adjustments

| Size | field height | field padding | field font-size |
|------|-------------|--------------|----------------|
| `xs` | `1.25rem` | `0 0.375rem` | `0.625rem` |
| `sm` | `1.5rem` | `0 0.4375rem` | `0.6875rem` |
| `md` | `1.75rem` | `0 0.5rem` | `0.75rem` |
| `lg` | `2rem` | `0 0.5625rem` | `0.8125rem` |
| `xl` | `2.25rem` | `0 0.625rem` | `0.875rem` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- `data-density` — resolved density value (`compact`, `default`, or `comfortable`)
- Toggle cycle: clicking inactive field sets asc; clicking active asc field
  sets desc; clicking active desc field clears sort (null)
- `.order-by__field--active` CSS class on active field button (not `data-active` attribute), `data-direction` attribute for direction
- Reset button conditionally rendered when activeSort is non-null
- `aria-pressed` on each field button reflects active state
- Toolbar keyboard navigation uses `roving tabindex` pattern

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::order_by`
- Spec struct: `OrderBySpec` in primitives crate
- Component struct: `PoodleOrderBy` in components crate
- Toggle cycle logic is pure function: `(current, clicked) → new ActiveSort|null`
- Arrow rotation may use GPUI transform or pre-rotated SVG paths
- Roving tabindex pattern maps to GPUI focus management

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] toggle cycle matches (asc→desc→null)
- [ ] change event payload matches
- [ ] aria-pressed on field buttons matches
- [ ] reset clears sort to null
- [ ] disabled state suppresses interaction
- [ ] keyboard navigation within toolbar matches

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] field button dimensions match (1.75rem height, 0.5rem padding)
- [ ] active field accent styling matches
- [ ] arrow rotation matches (180deg for desc)
- [ ] reset button styling matches
- [ ] label typography matches (0.75rem, uppercase, 0.05em spacing)
- [ ] hover background matches
- [ ] focus ring matches
- [ ] disabled opacity matches

### Tier 3: Implementation Freedom

- [ ] transition timing is platform-owned
- [ ] roving tabindex implementation is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Arrow rotation method | GPUI may use different rotation approach | allowed | same visual result |

## 13. Specimen Definitions

### Sort Controls

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Sort controls | `fields`: Name, Date, Size, Type (disabled); `activeSort` bound | Toolbar with "Sort by" label, four field buttons (Type visually disabled), status text showing current sort |

### Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled | `fields`: Name, Date; `disabled` | Toolbar at reduced opacity with all field buttons disabled |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: list views, table toolbars, data browsing interfaces
- future follow-up: multi-field sort, custom sort indicators
