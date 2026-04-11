# BulkActionBar

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `BulkActionBar`
- Layer: `foundation`
- Summary: a contextual action bar that appears when items are selected in a
  list or table, showing selection count and available bulk actions as icon buttons
- In scope: selection summary with count and optional total, action IconButtons
  with default, warning, and danger tones, select-all / deselect-all text button,
  clear selection IconButton, disabled/loading gating
- Out of scope: selection management (parent-owned), inline editing, batch
  progress indicators

## 2. Anatomy

```text
[Root .bulk-action-bar]  <div role="region" aria-label="Bulk actions">
  ├── [Summary .bulk-action-bar__summary]  <div>
  │   ├── [Count text]  <strong> "{selectionCount} selected"
  │   └── [Total text]  <span> "of {totalCount} visible rows" (optional)
  ├── [Actions .bulk-action-bar__actions]  <div>
  │   ├── [Select-all button .bulk-action-bar__button]  <button> (optional)
  │   ├── [Action icon-action .bulk-action-bar__icon-action]  <span> (repeated)
  │   │   └── [IconButton]  ghost variant, tone from action
  │   └── [Clear IconButton]  ghost variant, icon="x"
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | region container with accent-tinted background | flex layout, padding, border, radius, background |
| Summary | yes | selection count and optional total display | flex, gap, color, typography |
| Actions | yes | icon button row | flex, gap |
| Icon action wrapper | yes | tone wrapper for warning IconButtons | color override via `:global()` |
| Select-all button | no | text button for select-all / deselect-all | height, padding, border, radius, background, color |
| Clear IconButton | yes | dismisses the selection | ghost variant, inherits resolved size |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `selectionCount` | `number` | `0` | no | number of selected items |
| `totalCount` | `number \| null` | `null` | no | total item count for "of N visible rows" display |
| `size` | `ControlSize \| null` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `"chrome" \| "control" \| "prominent"` | `"control"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for container padding |
| `actions` | `BulkAction[]` | `[]` | no | available bulk action definitions |
| `loading` | `boolean` | `false` | no | disables all actions while a batch workflow is running |
| `disabled` | `boolean` | `false` | no | disables all interactions without changing the displayed selection |
| `showSelectAll` | `boolean` | `false` | no | shows a select-all / deselect-all action ahead of bulk actions |
| `allSelected` | `boolean` | `false` | no | controls whether the select-all action presents as deselect-all |
| `selectAllLabel` | `string` | `"Select all"` | no | label used when `allSelected` is false |
| `deselectAllLabel` | `string` | `"Deselect all"` | no | label used when `allSelected` is true |

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
| with total | `totalCount` provided | summary shows "N selected of M visible rows" |
| select all | `showSelectAll=true` | action row shows select-all or deselect-all text button |
| loading | `loading=true` | all action controls disabled |
| disabled | `disabled=true` | all action controls disabled |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `action` | action IconButton clicked | `{id: string}` | identifies which action was triggered |
| `clear` | clear IconButton clicked | `void` | parent should clear selection |
| `selectAll` | select-all / deselect-all clicked | `void` | parent decides whether to select all or clear all |

## 6. Accessibility

### Semantics

- Root: `role="region"`, `aria-label="Bulk actions"`
- Action IconButtons: `ariaLabel` set to action's `label`, tooltip shows label on hover
- Select-all button: native `<button>` with text label
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

- Root: flex row, wraps, space-between alignment
- Summary: flex row, wraps
- Actions: flex row, wraps

### Size vs Density Separation

- **Size** controls: icon button dimensions (via `size` prop passthrough),
  summary font-size, select-all button min-height and font-size.
  Size does NOT affect container padding or gap.
- **Density** controls: container padding only. Compact tightens padding,
  comfortable loosens it. Default inherits from panel tokens.

### Composition

- parent expectations: above or below list/table views
- child expectations: none (self-contained)
- resizing: fills parent width, wraps on narrow viewports

## 8. Token Usage — Exact Values

### Root

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `align-items` | `center` |
| `justify-content` | `space-between` |
| `gap` | `var(--poodle-space-inline-md)` |
| `padding` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-subtle)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `--poodle-recipe-bulk-fill: color-mix(in srgb, var(--poodle-color-background-panel) 93%, var(--poodle-color-text-primary))` |

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
- `icon` from the action's `icon` prop (falls back to `"circle"`)
- `ariaLabel` and `tooltip` set to the action's `label`
- `tone` mapped from action tone (`"danger"` passes through; `"warning"` uses a wrapper override)
- `size` set to the bar's resolved size

Warning tone is handled by a `.bulk-action-bar__icon-action[data-tone="warning"]` wrapper
that overrides the IconButton's color to `var(--poodle-color-status-warning)` via `:global()`.

### Select-all button (shown when `showSelectAll` is true)

| Property | Value |
|----------|-------|
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `0 var(--poodle-space-control-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `var(--poodle-color-background-surface)` |
| `color` | `var(--poodle-color-text-primary)` |
| `font-size` | `var(--poodle-typography-body-size)` |

### Size adjustments

Size affects summary font-size and select-all button dimensions.
Action IconButtons and clear IconButton inherit size via their `size` prop.

| Size | summary font-size | select-all button min-height | select-all button font-size |
|------|-------------------|-----------------------------|-----------------------------|
| `xs` | `0.75rem` | `calc(control-height - 0.5rem)` | `0.75rem` |
| `sm` | `0.8125rem` | `calc(control-height - 0.375rem)` | `0.8125rem` |
| `md` | `typography-body-size` | `control-height` | `typography-body-size` |
| `lg` | `0.9375rem` | `calc(control-height + 0.375rem)` | `0.9375rem` |
| `xl` | `1rem` | `calc(control-height + 0.5rem)` | `1rem` |

### Density adjustments

Density controls container padding only. It does NOT affect icon button sizes,
summary font-size, or action gap.

| Density | padding |
|---------|---------|
| `compact` | `0.25rem 0.5rem` |
| `default` | `var(--poodle-space-panel-y) var(--poodle-space-panel-x)` |
| `comfortable` | `0.625rem 1rem` |

## 9. Svelte Notes

- `data-size` attribute on root reflects the resolved size
- `data-density` attribute on root reflects the resolved density (`compact`, `default`, or `comfortable`)
- Bar typically conditionally rendered when `selectionCount > 0`
- Action buttons are ghost `IconButton` components, not custom `<button>` elements
- Warning tone uses a wrapper span (`.bulk-action-bar__icon-action[data-tone]`)
  with `:global()` color override since IconButton only supports `"default"` and `"danger"` tones natively
- Clear button is a ghost `IconButton` with `icon="x"` and `size={resolvedSize}`
- Select-all control is a text button (`.bulk-action-bar__button`) rendered before action IconButtons when enabled
- Summary count is wrapped in a `<strong>` tag

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::bulk_action_bar`
- Spec struct: `BulkActionBarSpec` in primitives crate
- Component struct: `PoodleBulkActionBar` in components crate
- Action callbacks identified by `id` string
- Accent-tinted background uses color-mix equivalent in Rust
- Danger tone maps to status-danger color tokens
- Actions rendered as icon buttons, not labeled text buttons

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] selectionCount and totalCount display correctly
- [ ] action event fires with correct id
- [ ] clear event fires correctly
- [ ] danger tone produces distinct IconButton styling
- [ ] warning tone overrides IconButton color to status-warning

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table
- [ ] density variants affect only container padding
- [ ] accent-tinted background matches (93% panel, 7% text-primary)
- [ ] gap between summary and actions matches (space-inline-md)
- [ ] gap between action IconButtons matches (space-inline-sm)
- [ ] border and border-radius match
- [ ] summary typography matches per size
- [ ] select-all button dimensions and styling match
- [ ] IconButton actions are ghost variant with tooltip
- [ ] focus ring matches on select-all button

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
| With selection count | `selectionCount=5`, `totalCount=42`, `showSelectAll`, `actions=[Export (icon: download), Archive (icon: inbox), Delete (icon: trash-2, tone: danger), Review (icon: triangle-alert, tone: warning)]` | Bar showing "5 selected of 42 visible rows" with four ghost IconButton actions and a clear X; Delete has danger styling, Review has warning color; clicking any action displays the action id below |

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
