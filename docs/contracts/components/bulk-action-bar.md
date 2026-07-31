# BulkActionBar

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `BulkActionBar`
- Layer: `foundation`
- Summary: a contextual action bar that appears when items are selected in a
  list or table, showing selection count and available bulk actions as icon buttons
- In scope: selection summary with count and optional total, action IconButtons
  with default, warning, and danger tones, select-all ghost IconButton,
  clear selection IconButton, disabled/loading gating
- Out of scope: selection management (parent-owned), inline editing, batch
  progress indicators

## 2. Anatomy

```text
[Root .bulk-action-bar]  <div role="region" aria-label="Bulk actions">  (position: fixed, bottom-docked)
  ├── [Summary .bulk-action-bar__summary]  <div>
  │   ├── [Count text]  <strong> "{selectionCount} selected"
  │   ├── [Total text]  <span> "of {totalCount}" (optional)
  │   └── [Select-all IconButton]  ghost variant, icon="check-check" (optional, when showSelectAll && !allSelected)
  ├── [Actions .bulk-action-bar__actions]  <div>
  │   ├── [Action icon-action .bulk-action-bar__icon-action]  <span> (repeated)
  │   │   └── [IconButton]  ghost variant, tone from action
  │   └── [Clear IconButton]  ghost variant, icon="x"
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | fixed bottom-docked region with accent-tinted background | flex layout, fixed positioning, padding, border, radius, background, shadow |
| Summary | yes | selection count, optional total, and optional select-all IconButton | flex, gap, color, typography |
| Actions | yes | icon button row | flex, gap |
| Icon action wrapper | yes | tone wrapper for warning IconButtons | color override via `:global()` |
| Select-all IconButton | no | ghost `IconButton` (`icon="check-check"`) for select-all, inside Summary; label conveyed via ariaLabel/tooltip | ghost variant, `sizeRole="chrome"` |
| Clear IconButton | yes | dismisses the selection | ghost variant, inherits resolved size |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `selectionCount` | `number` | `0` | no | number of selected items |
| `totalCount` | `number \| null` | `null` | no | total item count for "of N" display |
| `size` | `ControlSize \| null` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for container padding |
| `actions` | `BulkAction[]` | `[]` | no | available bulk action definitions |
| `loading` | `boolean` | `false` | no | disables all actions while a batch workflow is running |
| `disabled` | `boolean` | `false` | no | disables all interactions without changing the displayed selection |
| `showSelectAll` | `boolean` | `false` | no | shows a select-all action ahead of bulk actions |
| `allSelected` | `boolean` | `false` | no | lets the parent suppress the select-all affordance once everything is already selected |
| `selectAllLabel` | `string` | `"Select all"` | no | accessible label/tooltip for the select-all IconButton (not a visible text label); suffixed with `(totalCount)` when total is known |
| `onAction` | `((id: string) => void) \| null` | `null` | no | callback fired when a bulk action is triggered |
| `onClear` | `(() => void) \| null` | `null` | no | callback fired when the selection is cleared |
| `onSelectAll` | `(() => void) \| null` | `null` | no | callback fired when the select-all control is triggered |

### BulkAction Type

```typescript
type BulkAction = {
  id: string;
  label: string;
  icon?: IconProp | ComponentType;
  tone?: "default" | "warning" | "danger";
  disabled?: boolean;
};
```

### Controlled And Uncontrolled

- Selection state is externally managed; this component is display-only
  for selection count and action triggers.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | items selected | accent-tinted bar with count and ghost IconButton actions |
| danger action | action has `tone="danger"` | IconButton renders with danger tone |
| warning action | action has `tone="warning"` | wrapper overrides IconButton color to status-warning |
| with total | `totalCount` provided | summary shows "N selected of M" |
| select all | `showSelectAll=true` and `allSelected=false` | summary shows a select-all ghost IconButton (`icon="check-check"`) after the count/total |
| loading | `loading=true` | all action controls disabled |
| disabled | `disabled=true` | all action controls disabled |

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onAction` | action IconButton clicked | `string` | receives the triggered action id |
| `onClear` | clear IconButton clicked | none | parent should clear selection |
| `onSelectAll` | select-all control clicked | none | parent should select all visible items |

## 6. Accessibility

### Semantics

- Root: `role="region"`, `aria-label="Bulk actions"`
- Action IconButtons: `ariaLabel` set to action's `label`, tooltip shows label on hover
- Select-all IconButton: ghost `IconButton` (`icon="check-check"`) with `ariaLabel`/`tooltip` from `selectAllLabel` (suffixed `(totalCount)` when total is known)
- Clear IconButton: `ariaLabel="Clear selection"`
- Selection count: live region or announced on change
- Danger/warning tones: visually distinct but no special ARIA role

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | activates focused action |
| `Tab` | moves focus between actions |

### Focus And Announcement

- focus entry: first action receives focus
- selection change: count update announced via live region

## 7. Layout

### Sizing

- Root: a `position: fixed` bottom-docked floating bar with safe-area insets
  (`left`/`right`/`bottom` default to `max(1rem, env(safe-area-inset-*))`), a
  sticky `z-index`, and an optional `max-width`; flex row, wraps, space-between
  alignment
- Summary: flex row, wraps
- Actions: flex row, wraps

### Size vs Density Separation

- **Size** controls: icon button dimensions (via `size`/`sizeRole` prop
  passthrough) and summary font-size. Size does NOT affect container padding or gap.
- **Density** controls: horizontal padding and gap only. Compact tightens,
  comfortable loosens. Vertical padding stays a flat `0.5rem` across densities.

### Composition

- parent expectations: above or below list/table views
- child expectations: none (self-contained)
- resizing: fixed bottom-docked bar spanning between its safe-area insets, wraps on narrow viewports

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `position` | `fixed` |
| `right` | `var(--poodle-bulk-action-bar-right, max(1rem, env(safe-area-inset-right)))` |
| `bottom` | `var(--poodle-bulk-action-bar-bottom, max(1rem, env(safe-area-inset-bottom)))` |
| `left` | `var(--poodle-bulk-action-bar-left, max(1rem, env(safe-area-inset-left)))` |
| `z-index` | `var(--poodle-z-index-sticky, 40)` |
| `box-sizing` | `border-box` |
| `max-width` | `var(--poodle-bulk-action-bar-max-width, none)` |
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `gap` | `var(--poodle-space-inline-md)` |
| `padding` | `0.5rem var(--poodle-space-panel-x)` (vertical pad is intentionally a flat `0.5rem`, not `space-panel-y`) |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `--poodle-recipe-bulk-fill: color-mix(in srgb, var(--poodle-color-background-panel) 93%, var(--poodle-color-text-primary))` |
| `box-shadow` | `0 1rem 2.5rem color-mix(in srgb, black 36%, transparent), 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-default) 28%, transparent)` |

### Summary

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |
| `align-items` | `baseline` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-family` | `var(--poodle-typography-body-family)` |
| `font-size` | `var(--poodle-typography-body-size)` |
| `line-height` | `var(--poodle-typography-body-lineHeight)` |

### Summary total span

| Property | Value |
|----------|-------|
| `color` | `var(--poodle-color-text-secondary)` |

### Actions

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--poodle-space-inline-sm)` |

### Action rendering

Each action in the `actions` array renders as a ghost `IconButton` with:
- `icon` from the action's `icon` prop; the fallback is `"trash-2"` when the
  action tone is `"danger"`, otherwise `"circle"`
- `ariaLabel` and `tooltip` set to the action's `label`
- `tone` mapped from action tone (`"danger"` passes through; `"warning"` uses a wrapper override)
- `size` set to the bar's resolved size

Warning tone is handled by a `.bulk-action-bar__icon-action[data-tone="warning"]` wrapper
that overrides the IconButton's color to `var(--poodle-color-status-warning)` via `:global()`
(hover/focus blend toward `text-primary` at 82%).

### Select-all IconButton (shown when `showSelectAll && !allSelected`)

Rendered as a ghost `IconButton` (`icon="check-check"`, `sizeRole="chrome"`) inside
the Summary. It carries no bespoke chip CSS — its `ariaLabel`/`tooltip` come from
`selectAllLabel` and it is disabled while the bar is unavailable (loading or disabled).

### Size adjustments

Size affects summary font-size only.
Action IconButtons, the select-all IconButton, and the clear IconButton inherit
size via their `size`/`sizeRole` props.

| Size | summary font-size |
|------|-------------------|
| `xs` | `0.75rem` |
| `sm` | `0.8125rem` |
| `md` | `typography-body-size` |
| `lg` | `0.9375rem` |
| `xl` | `1rem` |

### Density adjustments

Density controls horizontal padding and gap only. It does NOT affect vertical
padding (a flat `0.5rem`), icon button sizes, or summary font-size.

| Density | padding-inline | root gap | actions gap |
|---------|----------------|----------|-------------|
| `compact` | `0.75rem` | `0.375rem` | `0.125rem` |
| `default` | `var(--poodle-space-panel-x)` | `var(--poodle-space-inline-md)` | `var(--poodle-space-inline-sm)` |
| `comfortable` | `1.25rem` | `1rem` | `0.5rem` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- `data-density` attribute on root reflects the resolved density (`compact`, `default`, or `comfortable`)
- Bar typically conditionally rendered when `selectionCount > 0`
- Action buttons are ghost `IconButton` components, not custom `<button>` elements
- Warning tone uses a wrapper span (`.bulk-action-bar__icon-action[data-tone]`)
  with `:global()` color override since IconButton only supports `"default"` and `"danger"` tones natively
- Clear button is a ghost `IconButton` with `icon="x"` and `size={resolvedSize}`
- Select-all control is a ghost `IconButton` (`icon="check-check"`, `sizeRole="chrome"`) rendered inside the Summary after the count/total when `showSelectAll && !allSelected`
- Action icon fallback is `"trash-2"` for danger tone, `"circle"` otherwise
- Root is a `position: fixed` bottom-docked floating bar (safe-area insets, sticky z-index, drop shadow)
- Summary count is wrapped in a `<strong>` tag

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::bulk_action_bar`
- Spec struct: `BulkActionBarSpec` in primitives crate
- Component struct: `PoodleBulkActionBar` in components crate
- Action callbacks identified by `id` string
- Accent-tinted background uses color-mix equivalent in Rust
- Danger tone maps to status-danger color tokens
- Actions rendered as icon buttons, not labeled text buttons

## 10a. Jetstream Notes

- `BulkActionBar::from_spec(spec, theme).on_action(...).on_select_all(...).on_clear(...)`.
  `on_action` carries the pressed action's id; disabled actions never fire.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] selectionCount and totalCount display correctly
- [ ] `onAction` fires with correct id
- [ ] `onClear` fires correctly
- [ ] danger tone produces distinct IconButton styling
- [ ] warning tone overrides IconButton color to status-warning

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] density variants affect only horizontal padding and gap (vertical pad flat 0.5rem)
- [ ] accent-tinted background matches (93% panel, 7% text-primary)
- [ ] fixed bottom-docked positioning + drop shadow match
- [ ] gap between summary and actions matches (space-inline-md)
- [ ] gap between action IconButtons matches (space-inline-sm)
- [ ] border and border-radius match
- [ ] summary typography matches per size
- [ ] select-all is a ghost IconButton (check-check) inside the summary
- [ ] IconButton actions are ghost variant with tooltip

### Tier 3: Implementation Freedom

- [ ] conditional rendering logic is platform-owned
- [ ] live region announcement method is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Live region announcement | GPUI may use different accessibility announcement method | allowed | same functional result |
| Warning tone wrapper | Svelte uses `:global()` override; GPUI can apply color directly | allowed | same visual result |

## 13. Specimen Definitions

### Group: With selection count

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With selection count | `selectionCount=5`, `totalCount=42`, `showSelectAll`, `actions=[Export (icon: download), Archive (icon: inbox), Delete (icon: trash-2, tone: danger), Review (icon: triangle-alert, tone: warning)]` | Bar showing "5 selected of 42" with a select-all check-check IconButton in the summary, four ghost IconButton actions and a clear X; Delete has danger styling, Review has warning color; clicking any action displays the action id below |

### Group: Single item selected

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Single item selected | `selectionCount=1`, `actions=[Export (icon: download), Archive (icon: inbox)]` (subset, no danger) | Bar showing "1 selected" (no total); only two IconButton actions, no danger-toned button |

### Group: Loading and disabled actions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Loading and disabled | `selectionCount=12`, `totalCount=12`, `loading`, `showSelectAll`, `allSelected`, `actions=[Publish (icon: rocket), Delete (icon: trash-2, tone: danger, disabled)]` | All actions disabled; delete additionally per-action disabled |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: list views, table views, file managers, batch operations
- future follow-up: progress indicator for long-running bulk actions, undo support
