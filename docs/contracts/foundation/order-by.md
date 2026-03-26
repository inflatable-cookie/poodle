# OrderBy

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `OrderBy`
- Layer: `foundation`
- Summary: a toolbar of sort-field toggle buttons for controlling list or table
  sort order with ascending, descending, and clear cycling
- In scope: sort field buttons, asc/desc/clear toggle cycle, directional arrow
  indicator, reset button, disabled state
- Out of scope: column header sort controls (see Table), multi-field sort,
  drag-and-drop field reordering

## 2. Anatomy

```text
[Root .order-by]  <div role="toolbar">
  ├── [Label .order-by__label]  <span>
  ├── [Fields .order-by__fields]  <div>
  │   ├── [Field .order-by__field]  <button> (repeated)
  │   │   ├── [Field label text]
  │   │   └── [Arrow .order-by__arrow]  <svg> (conditional, when active)
  │   └── ...
  └── [Reset .order-by__reset]  <button> (conditional, when activeSort)
      └── [Reset icon]  <svg>
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
| `activeSort` | `ActiveSort \| null` | `null` | no | current sort state |
| `ariaLabel` | `string` | `"Sort by"` | no | accessible name for toolbar |
| `disabled` | `boolean` | `false` | no | disables all field buttons |

### SortField Type

```typescript
type SortField = {
  value: string;
  label: string;
  disabled?: boolean;
};
```

### ActiveSort Type

```typescript
type ActiveSort = {
  field: string;
  direction: "asc" | "desc";
};
```

### Controlled And Uncontrolled

- Sort state is controlled via `activeSort` prop
- Changes dispatched via `change` event; parent updates prop

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | no active sort | all field buttons in default appearance |
| field active (asc) | field matches activeSort with direction="asc" | accent border and fill, arrow pointing up |
| field active (desc) | field matches activeSort with direction="desc" | accent border and fill, arrow rotated 180deg |
| field hover | pointer enters field button | elevated background |
| field disabled | field `disabled=true` | reduced opacity, not-allowed cursor |
| all disabled | `disabled=true` | all buttons disabled, reduced opacity |
| reset visible | activeSort is non-null | reset button appears |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `change` | field toggled or reset clicked | `{sort: ActiveSort \| null}` | null when sort cleared |

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

## 9. Svelte Notes

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
