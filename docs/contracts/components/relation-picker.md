# RelationPicker

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `RelationPicker`
- Layer: `composites`
- Summary: a selection workflow for attaching or choosing related entities, assets, presets, or references, with optional multi-level drill-down navigation
- In scope: candidate browse, search, single or multiple selection, selected summary, confirm/cancel flow, inline/popover/modal posture, hierarchical drill-down with breadcrumbs
- Out of scope: graph semantics, persistence, authorization, domain-specific validation rules

## 2. Anatomy

```text
[Root]
  └── [PickerShell]
        ├── [Toolbar]  (snippet)
        │     ├── [DrillBreadcrumbs]  (when drilling or post-drill with breadcrumbs)
        │     │     ├── [BackButton]  Icon: chevron-left
        │     │     ├── [Separator]   "/"
        │     │     └── [BreadcrumbItem...]
        │     ├── [DrillLevelLabel]   (when drilling)
        │     ├── [TextInput type="search"]       (drill search or main search)
        │     ├── [FilterControls]    (one labeled Select per `filters` entry, when provided)
        ├── [SelectionSummary]  (selection snippet, visible whenever there is a selection)
        ├── [DrillList]         (when drilling, replaces candidate list)
        │     └── [DrillListItem...]
        │           └── [DrillButton]
        │                 ├── [Copy]
        │                 │     ├── [Label]       <strong>
        │                 │     └── [Description]  <small> (optional)
        │                 └── [Meta]
        │                       ├── [Count]        (optional)
        │                       └── [ChevronIcon]  Icon: chevron-right
        ├── [DrillEmpty]        (when drilling, no items, not loading)
        ├── [CandidateList]     (when not drilling)
        │     └── [CandidateRow...]
        │           ├── [Checkbox]     (multiple mode only)
        │           ├── [CandidateButton]
        │           │     └── [CandidateCopy]
        │           │           ├── [Label]       <strong>
        │           │           └── [Detail]      <small> (description + meta, optional)
        ├── [State]             (stateContent snippet, via PickerShell)
        └── [Footer]            (footer snippet)
              └── [FormActions]
                    ├── [FooterNote]
                    └── [FooterActions]
                          ├── [CancelButton]  ghost variant
                          └── [ConfirmButton]  primary variant
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | Wrapper with class `relation-picker`, data attributes |
| picker-shell | `PickerShell` | Structural shell with title, description, variant, state, status |
| drill-breadcrumbs | `<div>` | Flex row of breadcrumb items with back button |
| drill-back | `<button>` | Back navigation icon button, `aria-label="Go back"` |
| drill-separator | `<span>` | "/" separator between breadcrumb items |
| drill-breadcrumb-item | `<button>` | Clickable breadcrumb, navigates to that level |
| drill-level-label | `<div>` | Uppercase label for current drill level |
| search-field | `TextInput type="search"` | Main or drill search, with clear and keydown handling |
| selection-summary | `SelectionSummary` | Removable selection pills; remains visible during drill navigation whenever there is at least one selection |
| drill-list | `<ul>` | Grid list of drill-down items |
| drill-button | `<button>` | Full-width button for each drill item |
| drill-copy | `<span>` | Grid of label and description |
| drill-meta | `<span>` | Flex row of count and chevron icon |
| drill-empty | `<li>` | "No items found" centered message |
| candidate-list | `<ul>` | Grid list of candidate items, `aria-label="Available candidates"` |
| candidate-item | `<li>` | Grid row with checkbox, button, data-selected attribute |
| candidate-button | `<button>` | Selection button with `aria-pressed`, `aria-describedby` |
| candidate-copy | `<span>` | Grid of label and detail text |
| footer-note | `<p>` | Descriptive text about selection mode |
| footer-actions | `<div>` | Flex row of cancel and confirm buttons |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | `"Select items"` | no | Picker heading text |
| `description` | `string \| null` | `null` | no | Subheading below title |
| `items` | `PickerItem[]` | `[]` | no | Flat candidate list |
| `selectedItems` | `PickerItem[]` | `[]` | no | Pre-known full item records for the current selection; used to keep the selection summary labels stable even when the selected ids are not in the current `items` view |
| `selectedIds` | `string[] \| undefined` | `undefined` | no | Controlled selection state when supplied; uncontrolled local state otherwise |
| `query` | `string \| undefined` | `undefined` | no | Controlled search query when supplied; uncontrolled local state otherwise |
| `selectionMode` | `"single" \| "multiple"` | `"multiple"` | no | Selection semantics |
| `variant` | `"inline" \| "popover" \| "modal"` | `"inline"` | no | Workflow posture |
| `state` | `"ready" \| "empty" \| "loading" \| "error" \| "no-results"` | `"ready"` | no | Candidate-set posture |
| `ariaLabel` | `string \| null` | `null` | no | Accessible name override |
| `searchPlaceholder` | `string` | `"Search picker results"` | no | Placeholder text for the search field |
| `filters` | `PickerFilterConfig[]` | `[]` | no | Filter controls rendered in the toolbar; each is a labeled `Select` over its options |
| `filterValues` | `Record<string, string \| undefined>` | `{}` | no | Current value for each filter, keyed by filter `key` |
| `stateTitle` | `string \| null` | `null` | no | Passed through to PickerShell's `stateTitle` for the non-ready state area |
| `stateMessage` | `string \| null` | `null` | no | Passed through to PickerShell's `stateMessage` for the non-ready state area |
| `confirmLabel` | `string` | `"Confirm selection"` | no | Text for confirm button |
| `cancelLabel` | `string` | `"Cancel"` | no | Text for cancel button |
| `footerNote` | `string \| null` | `null` | no | Overrides the default selection-mode footer note text |
| `showFooter` | `boolean` | `true` | no | When `false`, the confirm/cancel footer is not rendered |
| `showSelectionSummary` | `boolean` | `true` | no | When `false`, the selection summary region is not rendered |
| `drillDown` | `DrillDownConfig \| null` | `null` | no | Drill-down navigation config |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for picker actions and drill-down controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for breadcrumbs and drill-list spacing |

### Types

```ts
type PickerItem = {
  id: string;
  label: string;
  description?: string | null;
  meta?: string | null;
  disabled?: boolean;
};

type PickerFilterOption = {
  id: string;
  label: string;
};

type PickerFilterConfig = {
  key: string;
  label: string;
  options: PickerFilterOption[];
  includeAll?: boolean;
  allLabel?: string;
};

type DrillDownItem = PickerItem & {
  count?: number;
  expandable?: boolean;
};

type DrillDownContext = Record<string, string>;

type DrillDownSearchFn = (
  query: string,
  context: DrillDownContext,
) => DrillDownItem[] | Promise<DrillDownItem[]>;

type DrillDownLevel = {
  key: string;
  label: string;
  items: DrillDownItem[] | DrillDownSearchFn;
  searchPlaceholder?: string;
};

type DrillDownItemsFn = (
  query: string,
  context: DrillDownContext,
) => PickerItem[] | Promise<PickerItem[]>;

type DrillDownConfig = {
  levels: DrillDownLevel[];
  finalItems?: DrillDownItemsFn;
};

type BrowseState = "ready" | "empty" | "loading" | "error" | "no-results";
type PickerVariant = "inline" | "popover" | "modal";
type SelectionMode = "single" | "multiple";
```

### Snippets

| Snippet | Signature | Notes |
|---------|-----------|-------|
| `renderItem` | `(item: PickerItem, selected: boolean)` | Custom candidate rendering; replaces the default label/detail copy for each candidate row |
| `stateContent` | `()` | Custom state content, passed through to PickerShell's `stateContent` snippet |

### Controlled / Uncontrolled

- `selectedIds` and `query` can run controlled or uncontrolled
- Drill-down state (depth, selections, drill search query) is managed internally
- In uncontrolled mode, local state is still mirrored through
  `onSelectionChange` and `onQueryChange`

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"`, no drill-down | Candidate list visible with search |
| drilling | `drillDown` configured, depth < levels.length | Drill list shown with level items, breadcrumbs, back button, drill search |
| drill-complete | All drill levels selected | Final items loaded (via `finalItems` fn or flat `items`), candidate list shown with breadcrumbs |
| drill-loading | Level items is async function, awaiting | PickerShell in loading state |
| selection-visible-while-drilling | `selectedIds.length > 0` during drill navigation | SelectionSummary remains visible while changing branches or levels |
| empty | `state="empty"` | State area with empty message |
| loading | `state="loading"` | State area with spinner and loading message |
| error | `state="error"` | State area with error message |
| no-results | `state="no-results"` | State area with no-results message |
| final-items-loading | Drill complete, `finalItems` async resolving | Loading state during final items fetch |

### Component States

Internal drill-down state includes: `drillDepth`, `drillSelections` (map of level key to selected item), `drillSearchQuery`, `drillItems`, `drillLoading`, `finalItemsLoaded`, `finalItemsLoading`.

### Behavior Machine

Behavior classification: adapter-owned interaction (g11.004 sweep)

Composite over PickerShell; owns query/selection plumbing. Behavior belongs to composed primitives; classify with them.

## 5. Callbacks

| Callback | When It Fires | Signature |
|----------|---------------|-----------|
| `onQueryChange` | Search query changes (typing or clear) | `(value: string) => void` |
| `onSelectionChange` | Selection toggled (single or multiple) | `(selectedIds: string[]) => void` |
| `onFilterChange` | A toolbar filter value changes | `(key: string, value: string \| undefined) => void` |
| `onConfirm` | Confirm button clicked | `(selectedIds: string[]) => void` |
| `onCancel` | Cancel button clicked | `() => void` |
| `onDrillContext` | Drill-down level selected | `(context: DrillDownContext) => void` |

## 6. Accessibility

### Semantics

- Candidate list uses `<ul>` with `aria-label="Available candidates"`
- Candidate buttons use `aria-pressed` for selection state
- Disabled candidates (`item.disabled`) carry `aria-disabled` + the native `disabled` attribute and are not selectable
- Candidate descriptions use `aria-describedby` linking (id: `relation-picker-item-{id}`)
- Multiple mode shows `Checkbox` primitive alongside each candidate
- Single mode uses button-press pattern (no radio group)
- Status live region (via PickerShell) announces result/selection counts
- Search field has `aria-describedby` linking to status element
- Drill list uses `<ul>` with `aria-label` from current level label
- Back button has `aria-label="Go back"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `ArrowDown` / `ArrowRight` | Focus next candidate |
| `ArrowUp` / `ArrowLeft` | Focus previous candidate |
| `Home` | Focus first candidate |
| `End` | Focus last candidate |
| `Escape` | During drill-down with depth > 0: go back one level |
| `Backspace` | During drill-down with empty search: go back one level |
| `Enter` / `Space` | On candidate: toggle selection; on drill item: select and advance |

### Focus

- Focus entry: search field receives initial focus
- Candidate focus wraps (last to first, first to last)
- Live-region behavior: status text updated on filter/selection changes
- `candidateButtons` array provides programmatic focus management
- In single-select mode the full candidate row is the active button hit area

## 7. Layout

### Sizing

- Inherits layout from PickerShell
- Candidate items use grid layout: `auto minmax(0, 1fr) auto` columns
- Drill items use flex layout with content and meta/chevron
- Breadcrumb items have `max-width: 8rem` with text ellipsis

### Composition

- Composes: `PickerShell`, `SelectionSummary`, `TextInput type="search"`, `Checkbox`, `Button`, `FormActions`, `Icon`
- Parent expectations: inline containers, popovers, modal dialogs
- Wraps children in `UiPresentationProvider` with resolved size and density
- SelectionSummary item labels are cached from visited candidate and final-item
  sets so drill navigation does not blank previously selected labels

## 8. Token Usage — Exact Values

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-selected` | candidate item `<li>` | `"true"`, `"false"` |
| `data-size` | root `<div>` | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | root `<div>` | `"compact"`, `"default"`, `"comfortable"` |

### Drill Breadcrumbs `.drill-breadcrumbs`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.25rem` |
| padding-bottom | `var(--poodle-space-stack-sm)` |

### Drill Breadcrumbs Back Button `.drill-breadcrumbs__back`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `center` |
| width | `1.5rem` (default, varies by size) |
| height | `1.5rem` (default, varies by size) |
| padding | `0` |
| border | `none` |
| border-radius | `var(--poodle-radius-sm, 0.25rem)` |
| background | `transparent` |
| color | `var(--poodle-color-text-secondary)` |
| cursor | `pointer` |
| `:hover` background | `var(--poodle-color-surface-hover, rgba(148, 163, 184, 0.12))` |
| `:hover` color | `var(--poodle-color-text-primary)` |

### Drill Breadcrumbs Separator `.drill-breadcrumbs__sep`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.6875rem` |
| opacity | `0.6` |

### Drill Breadcrumbs Item `.drill-breadcrumbs__item`

| Property | Value |
|----------|-------|
| min-height | `calc(breadcrumb-control - 0.25rem)` |
| padding | `0.125rem` vertical, horizontal varies by size |
| border | `none` |
| border-radius | `var(--poodle-radius-sm, 0.25rem)` |
| background | `transparent` |
| color | `var(--poodle-color-accent-base)` |
| font-size | `var(--poodle-typography-label-size)` |
| font-weight | `500` |
| cursor | `pointer` |
| white-space | `nowrap` |
| overflow | `hidden` |
| text-overflow | `ellipsis` |
| max-width | `8rem` |
| `:hover` background | `var(--poodle-color-surface-hover, rgba(148, 163, 184, 0.12))` |

### Drill Level Label `.drill-level-label`

| Property | Value |
|----------|-------|
| font-size | `var(--poodle-typography-label-size)` |
| font-weight | `600` |
| text-transform | `uppercase` |
| letter-spacing | `0.08em` |
| color | `var(--poodle-color-text-secondary)` |
| padding-bottom | `0.25rem` |

### Drill List `.drill-list`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `0.125rem` (default, varies by density) |
| margin | `0` |
| padding | `0` |
| list-style | `none` |

### Drill List Button `.drill-list__button`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `space-between` |
| gap | `var(--poodle-space-inline-md)` |
| width | `100%` |
| padding | `0.5rem 0.625rem` (default, varies by size/density) |
| border | `none` |
| border-radius | `var(--poodle-radius-control)` |
| background | `transparent` |
| color | `var(--poodle-color-text-primary)` |
| cursor | `pointer` |
| text-align | `left` |
| font | `inherit` |
| font-size | `var(--poodle-typography-body-size)` |
| `:hover` background | `color-mix(in srgb, var(--poodle-color-background-surface) 60%, transparent)` |
| `:focus-visible` outline | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `:focus-visible` outline-offset | `-0.0625rem` |

### Drill List Copy `.drill-list__copy`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `0.125rem` |
| min-width | `0` |
| `strong` font-weight | `500` |
| `strong` overflow | `hidden`, text-overflow `ellipsis`, white-space `nowrap` |
| `small` color | `var(--poodle-color-text-secondary)` |
| `small` font-size | `var(--poodle-typography-label-size)` |
| `small` overflow | `hidden`, text-overflow `ellipsis`, white-space `nowrap` |

### Drill List Meta `.drill-list__meta`

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.25rem` |
| flex-shrink | `0` |
| color | `var(--poodle-color-text-secondary)` |

### Drill List Count `.drill-list__count`

| Property | Value |
|----------|-------|
| font-size | `var(--poodle-typography-label-size)` |
| opacity | `0.7` |

### Drill List Empty `.drill-list__empty`

| Property | Value |
|----------|-------|
| padding | `calc(list-y * 2.5)` |
| text-align | `center` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.8125rem` |

### Candidate List `.relation-picker__list`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-relation-picker-list-gap)` |
| margin | `0` |
| padding | `0` |
| list-style | `none` |

### Candidate Item `.relation-picker__item`

| Property | Value |
|----------|-------|
| display | `grid` |
| grid-template-columns | `auto minmax(0, 1fr) auto` (`multiple`) / `minmax(0, 1fr)` (`single`) |
| align-items | `center` |
| gap | `var(--poodle-relation-picker-item-gap)` |
| padding | `var(--poodle-relation-picker-item-y) var(--poodle-relation-picker-item-x)` |
| border | `0.0625rem solid var(--poodle-color-border-subtle)` |
| border-radius | `var(--poodle-radius-control)` |
| background | `color-mix(in srgb, var(--poodle-color-background-surface) 86%, transparent)` |
| color | `var(--poodle-color-text-primary)` |

#### Candidate Item Selected `[data-selected="true"]`

| Property | Value |
|----------|-------|
| border-color | `color-mix(in srgb, var(--poodle-color-accent-base) 60%, transparent)` |
| background | `color-mix(in srgb, var(--poodle-color-accent-base) 10%, transparent)` |

#### Candidate Item Disabled (`item.disabled`)

When a `PickerItem` has `disabled: true`, its row sets `data-disabled`, the
candidate button gets `aria-disabled` and the native `disabled` attribute, the
checkbox is disabled, the row renders at `opacity: 0.55`, and toggling is
skipped (the item cannot be selected or deselected).

### Candidate Item Button `.relation-picker__item-button`

| Property | Value |
|----------|-------|
| display | `grid` |
| grid-template-columns | `minmax(0, 1fr)` |
| gap | `0.25rem` |
| min-width | `0` |
| padding | `0` |
| border | `0` |
| background | `transparent` |
| color | `inherit` |
| cursor | `pointer` |
| text-align | `left` |
| font | `inherit` |
| `:focus-visible` outline | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `:focus-visible` outline-offset | `0.125rem` |
| `:focus-visible` border-radius | `var(--poodle-radius-control)` |

### Candidate Copy `.relation-picker__item-copy`

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `0.25rem` |
| `strong`, `small`, footer-note margin | `0` |
| `strong` font-weight | `500` |
| `small` color | `var(--poodle-color-text-secondary)` |
| `small` font-size | `var(--poodle-relation-picker-desc-size)` |
| `small` line-height | `1.4` |

### Footer Note `.relation-picker__footer-note`

| Property | Value |
|----------|-------|
| `flex` | `1 1 18rem` |
| `min-width` | `0` |
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `var(--poodle-relation-picker-desc-size)` |
| line-height | `1.5` |

### Footer Actions `.relation-picker__footer-actions`

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-wrap | `wrap` |
| gap | `var(--poodle-space-inline-sm)` |
| `margin-left` | `auto` |
| justify-content | `flex-end` |

### Size Adjustments

| Size | Breadcrumb control | Breadcrumb X padding | List X padding |
|------|--------------------|---------------------|----------------|
| `xs` | `1.25rem` | `0.25rem` | `0.5rem` |
| `sm` | `1.5rem` | `0.375rem` | `0.625rem` |
| `md` | `1.75rem` | `0.375rem` | `0.625rem` |
| `lg` | `2rem` | `0.5rem` | `0.75rem` |
| `xl` | `2.25rem` | `0.625rem` | `0.875rem` |

### Density Adjustments

| Density | List Y padding | List gap |
|---------|---------------|----------|
| `compact` | `0.5rem` | `0.1875rem` |
| `default` | `0.5rem` | `0.25rem` |
| `comfortable` | `0.5rem` | `0.3125rem` |

## 9. Svelte Notes

- Internal `statusId` used for `aria-describedby` on search field
- `candidateButtons` array for programmatic focus management
- Drill-down state is reactive via runes-based derived and effect logic
- Drill search query (`drillSearchQuery`) is separate from main `query` prop
- `loadDrillItems()` handles both sync (array) and async (function) level items, with client-side filtering for sync items
- `loadFinalItems()` calls `drillDown.finalItems` with accumulated context and query
- `drillSelect()` advances drill depth, records selection, and calls `onDrillContext`
- `drillBack()` decrements depth, removes level selection, clears final items
- `drillNavigateTo()` jumps to any breadcrumb depth, clearing subsequent selections
- `handleDrillSearchKeydown()` intercepts Escape and Backspace (when search empty) to navigate back
- `filteredItems` performs client-side filtering on label, description, and meta fields
- `activeItems` resolves to `finalItemsLoaded` when drill-down provides items, otherwise falls back to `items` prop
- `pickerStatusText` provides contextual status announcements
- Footer note text changes based on `selectionMode`
- Wraps content in `UiPresentationProvider` with resolved size and density
- `selectedIds` and `query` each support uncontrolled fallback state when the
  corresponding prop is omitted
- Selection labels are cached across visited drill branches so the summary
  stays stable while navigating away from the current branch

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::relation_picker`
- Drill-down navigation must preserve breadcrumb and back semantics

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] All props have the same meaning and defaults
- [ ] Event names and payloads match
- [ ] Selection mode behavior matches (single vs multiple)
- [ ] Drill-down navigation semantics match (advance, back, breadcrumb jump)
- [ ] Keyboard navigation matches (Arrow, Home, End, Escape, Backspace)

### Tier 2: Visual Parity

- [ ] Candidate item styling matches (grid layout, selected state)
- [ ] Drill breadcrumb styling matches
- [ ] Drill list button styling matches
- [ ] Size and density adjustments match

### Tier 3: Implementation Freedom

- [ ] Internal state management approach may differ
- [ ] Async loading patterns may differ
- [ ] PickerShell implementation may differ

## 12. Specimen Definitions

### Multiple Selection

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Multiple selection | `title="Select components"`, `description="Choose related components."`, `selectionMode="multiple"`, six candidate items with label/description/meta, two pre-selected (`selectedIds=["btn","card"]`) | Picker with search, candidate list with checkboxes, two items checked, selection summary visible below |

### Single Selection

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Single selection | `title="Choose a parent"`, same six candidate items, `selectionMode="single"` | Picker with search and candidate list using button-press single selection, no items pre-selected |

### Drill-Down Navigation

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Drill-down | `drillDown` with two levels (e.g. Category > Subcategory), `finalItems` returning filtered items | Drill list with items, breadcrumbs appearing as levels are selected, final candidate list after all levels complete |
