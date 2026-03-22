# BulkActionBar

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `BulkActionBar`
- Layer: `foundation`
- Summary: a contextual action bar that appears when items are selected in a
  list or table, showing selection count and available bulk actions
- In scope: selection summary with count and optional total, action buttons
  with default and danger tones, clear selection
- Out of scope: selection management (parent-owned), inline editing, batch
  progress indicators

## 2. Anatomy

```text
[Root .bulk-action-bar]  <div role="region" aria-label="Bulk actions">
  ├── [Summary .bulk-action-bar__summary]  <div>
  │   ├── [Count text]  "{selectionCount} selected"
  │   └── [Total text]  <span> "of {totalCount}" (optional)
  ├── [Actions .bulk-action-bar__actions]  <div>
  │   ├── [Button .bulk-action-bar__button]  <button> (repeated)
  │   └── ...
  └── [Clear button]  (via clear event)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | region container with accent-tinted background | flex layout, padding, border, radius, background |
| Summary | yes | selection count and optional total display | flex, gap, color, typography |
| Actions | yes | action button row | flex, gap |
| Button | yes | action trigger with optional danger tone | height, padding, border, radius, background, color |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `selectionCount` | `number` | `0` | no | number of selected items |
| `totalCount` | `number \| null` | `null` | no | total item count for "of N" display |
| `actions` | `BulkAction[]` | — | yes | available bulk action definitions |

### BulkAction Type

```typescript
type BulkAction = {
  id: string;
  label: string;
  icon?: string;
  tone?: "default" | "danger";
};
```

### Controlled And Uncontrolled

- Selection state is externally managed; this component is display-only
  for selection count and action triggers.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | items selected | accent-tinted bar with count and action buttons |
| danger action | action has `tone="danger"` | button with danger border and text color |
| with total | `totalCount` provided | summary shows "N selected of M" |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `action` | action button clicked | `{id: string}` | identifies which action was triggered |
| `clear` | clear/deselect triggered | `void` | parent should clear selection |

## 6. Accessibility

### Semantics

- Root: `role="region"`, `aria-label="Bulk actions"`
- Action buttons: native `<button>` elements with action labels
- Selection count: live region or announced on change
- Danger actions: visually distinct but no special ARIA role

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` / `Space` | activates focused action button |
| `Tab` | moves focus between action buttons |

### Focus And Announcement

- focus entry: first action button receives focus
- selection change: count update announced via live region

## 7. Layout

### Sizing

- Root: flex row, wraps, space-between alignment
- Summary: flex row, wraps
- Actions: flex row, wraps

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
| `gap` | `var(--pug-space-inline-md)` |
| `padding` | `var(--pug-space-panel-y) var(--pug-space-panel-x)` |
| `border` | `0.0625rem solid var(--pug-color-border-subtle)` |
| `border-radius` | `var(--pug-radius-surface)` |
| `background` | `--pug-recipe-bulk-fill: color-mix(in srgb, var(--pug-color-background-panel) 93%, var(--pug-color-text-primary))` |

### Summary

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--pug-space-inline-sm)` |
| `align-items` | `baseline` |
| `color` | `var(--pug-color-text-primary)` |
| `font-family` | `var(--pug-typography-body-family)` |
| `font-size` | `var(--pug-typography-body-size)` |
| `line-height` | `var(--pug-typography-body-lineHeight)` |

### Summary total span

| Property | Value |
|----------|-------|
| `color` | `var(--pug-color-text-secondary)` |

### Actions

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `flex-wrap` | `wrap` |
| `gap` | `var(--pug-space-inline-sm)` |

### Button (default tone)

| Property | Value |
|----------|-------|
| `min-height` | `var(--pug-size-control-height)` |
| `padding` | `0 var(--pug-space-control-x)` |
| `border` | `0.0625rem solid var(--pug-color-border-default)` |
| `border-radius` | `var(--pug-radius-control)` |
| `background` | `var(--pug-color-background-surface)` |
| `color` | `var(--pug-color-text-primary)` |
| `cursor` | `pointer` |

### Button (danger tone)

| Property | Value |
|----------|-------|
| `border-color` | `color-mix(in srgb, var(--pug-color-status-danger) 65%, transparent)` |
| `color` | `var(--pug-color-status-danger)` |

### Button focus

| Property | Value |
|----------|-------|
| `outline` | `var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing)` |
| `outline-offset` | `0.125rem` |

## 9. Svelte Notes

- Bar typically conditionally rendered when `selectionCount > 0`
- `data-tone` attribute on danger action buttons
- Summary text uses template: `"{selectionCount} selected"` with optional
  `"of {totalCount}"` span
- Action buttons rendered from `actions` array prop; actions with `icon` render as IconButton
- Clear button is an IconButton with `icon="x"`
- Summary count is wrapped in a `<strong>` tag

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::bulk_action_bar`
- Spec struct: `BulkActionBarSpec` in primitives crate
- Component struct: `PugBulkActionBar` in components crate
- Action callbacks identified by `id` string
- Accent-tinted background uses color-mix equivalent in Rust
- Danger tone maps to status-danger color tokens

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] selectionCount and totalCount display correctly
- [ ] action event fires with correct id
- [ ] clear event fires correctly
- [ ] danger tone produces distinct button styling

### Tier 2: Visual Parity

- [ ] accent-tinted background matches (10% accent-base)
- [ ] padding and gap match
- [ ] border and border-radius match
- [ ] summary typography matches
- [ ] button dimensions and styling match
- [ ] danger button border and text color match
- [ ] focus ring matches

### Tier 3: Implementation Freedom

- [ ] conditional rendering logic is platform-owned
- [ ] live region announcement method is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Live region announcement | GPUI may use different accessibility announcement method | allowed | same functional result |

## 13. Specimen Definitions

### Group: With selection count

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| With selection count | `selectionCount=5`, `totalCount=42`, `actions=[Export (icon: download), Archive (icon: inbox), Delete (icon: trash-2, tone: danger)]` | Bar showing "5 selected of 42" with three action buttons; Delete button has danger styling; clicking any action displays the action id below |

### Group: Single item selected

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Single item selected | `selectionCount=1`, `actions=[Export (icon: download), Archive (icon: inbox)]` (subset, no danger) | Bar showing "1 selected" (no total); only two action buttons, no danger-toned button |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: list views, table views, file managers, batch operations
- future follow-up: progress indicator for long-running bulk actions, undo support
