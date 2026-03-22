# RelationPicker

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `RelationPicker`
- Layer: `composites`
- Summary: a selection workflow for attaching or choosing related entities,
  assets, presets, or references, with optional multi-level drill-down navigation
- In scope: candidate browse, search, single or multiple selection, selected
  summary, confirm/cancel flow, inline/popover/modal posture, hierarchical
  drill-down with breadcrumbs
- Out of scope: graph semantics, persistence, authorization, domain-specific
  validation rules

## 2. Anatomy

```text
[PickerShell]
  ├── [Toolbar]  (slot)
  │     ├── [DrillBreadcrumbs]  (when drilling or post-drill)
  │     │     ├── [BackButton]
  │     │     └── [BreadcrumbItem...]
  │     ├── [DrillLevelLabel]   (when drilling)
  │     └── [SearchField]
  ├── [SelectionSummary]  (slot: selection, hidden while drilling)
  ├── [DrillList]         (when drilling, replaces candidate list)
  │     └── [DrillListItem...]
  │           ├── [Label]
  │           ├── [Description]  (optional)
  │           ├── [Count]        (optional)
  │           └── [ChevronIcon]
  ├── [CandidateList]     (when not drilling)
  │     └── [CandidateRow...]
  │           ├── [Checkbox]     (multiple mode)
  │           ├── [Label]
  │           ├── [Description]  (optional)
  │           └── [Meta]         (optional)
  ├── [State]             (slot, for custom state content)
  └── [Footer]            (slot)
        ├── [FooterNote]
        └── [FormActions]
              ├── [CancelButton]
              └── [ConfirmButton]
```

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | `"Select items"` | no | picker heading text |
| `description` | `string \| null` | `null` | no | subheading below title |
| `items` | `PickerItem[]` | `[]` | no | flat candidate list |
| `selectedIds` | `string[]` | `[]` | no | controlled selection state |
| `query` | `string` | `""` | no | controlled search query |
| `selectionMode` | `"single" \| "multiple"` | `"multiple"` | no | selection semantics |
| `variant` | `"inline" \| "popover" \| "modal"` | `"inline"` | no | workflow posture |
| `state` | `"ready" \| "empty" \| "loading" \| "error" \| "no-results"` | `"ready"` | no | candidate-set posture |
| `ariaLabel` | `string \| null` | `null` | no | accessible name override |
| `confirmLabel` | `string` | `"Confirm selection"` | no | text for confirm button |
| `cancelLabel` | `string` | `"Cancel"` | no | text for cancel button |
| `drillDown` | `DrillDownConfig \| null` | `null` | no | drill-down navigation config |

### Types

```typescript
type PickerItem = {
  id: string;
  label: string;
  description?: string | null;
  meta?: string | null;
};

type DrillDownItem = PickerItem & {
  count?: number;
  hasChildren?: boolean;
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
```

### Controlled And Uncontrolled

- `selectedIds` and `query` are controlled props; host owns final state
- drill-down state (depth, selections) is managed internally
- collapse states are externally owned when used

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"`, no drill-down | candidate list visible with search |
| drilling | `drillDown` configured, depth < levels.length | drill list shown with level items, breadcrumbs, back button |
| drill-complete | all drill levels selected | final items loaded (via `finalItems` fn or flat `items`), candidate list shown with breadcrumbs |
| empty | `state="empty"` | state area with empty message |
| loading | `state="loading"` | state area with loading message |
| error | `state="error"` | state area with error message |
| no-results | `state="no-results"` | state area with no-results message |

### Component States

Internal drill-down state includes: `drillDepth`, `drillSelections` (map of
level key to selected item), `drillSearchQuery`, `drillItems`, `drillLoading`,
`finalItemsLoaded`, `finalItemsLoading`.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `queryChange` | search query changes | `{ value: string }` | host owns filtering or remote search |
| `selectionChange` | selection toggled | `{ selectedIds: string[] }` | host owns final state |
| `confirm` | confirm button clicked | `{ selectedIds: string[] }` | host commits relation change |
| `cancel` | cancel button clicked | `void` | host decides whether selection resets |
| `drillContext` | drill-down level selected | `{ context: DrillDownContext }` | fires after each drill selection with accumulated context |

## 6. Drill-Down Navigation

When `drillDown` is provided, the picker enters a hierarchical navigation mode:

1. **Level navigation**: each level in `drillDown.levels` presents a searchable
   list of items; selecting an item advances to the next level
2. **Breadcrumbs**: completed levels appear as clickable breadcrumb links above
   the search field, with a back button (chevron-left icon)
3. **Back navigation**: clicking the back button or pressing Escape/Backspace
   (when search is empty) returns to the previous level
4. **Breadcrumb jump**: clicking a breadcrumb navigates directly to that level,
   clearing all subsequent selections
5. **Final items**: when all levels are completed, if `drillDown.finalItems` is
   provided, it is called with the accumulated context and search query to load
   the final candidate list; otherwise, the flat `items` prop is used
6. **Level loading**: if a level's `items` is a function, it is called with the
   current search query and accumulated drill context; the picker shows a
   loading state while the function resolves
7. **Search isolation**: each drill level has its own search query, independent
   of the main picker search query

## 7. Accessibility

### Semantics

- candidate list uses `<ul>` with `aria-label`
- candidate buttons use `aria-pressed` for selection state
- candidate descriptions use `aria-describedby` linking
- multiple mode shows `Checkbox` primitive alongside each candidate
- single mode uses button-press pattern (no radio group)
- status live region (via PickerShell) announces result/selection counts

### Keyboard

| Key | Behavior |
|-----|----------|
| `ArrowDown` / `ArrowRight` | focus next candidate |
| `ArrowUp` / `ArrowLeft` | focus previous candidate |
| `Home` | focus first candidate |
| `End` | focus last candidate |
| `Escape` | during drill-down: go back one level (if depth > 0) |
| `Backspace` | during drill-down with empty search: go back one level |
| `Enter` / `Space` | on candidate: toggle selection; on drill item: select and advance |

### Focus And Announcement

- focus entry: search field receives initial focus
- candidate focus wraps (last -> first, first -> last)
- live-region behavior: status text updated on filter/selection changes
- GPUI-native accessibility mapping notes: GPUI must expose candidate list,
  selection state, selected-summary items, and confirm/cancel actions as a
  coherent workflow rather than unrelated controls

## 8. Composition Guidance

- use `single` when choosing one relation target
- use `multiple` when attaching several related entities or assets
- use `inline` when the picker is part of a larger form or detail flow
- use `popover` for compact transient selection
- use `modal` when the selection task needs focused workspace and explicit
  commit/cancel rhythm
- use `drillDown` for hierarchical data (e.g. category > subcategory > items)

## 9. Layout

### Sizing

- inherits layout from PickerShell
- candidate items use grid layout with checkbox + content + meta columns
- drill items use flex layout with content + meta/chevron
- breadcrumb items have max-width 8rem with text ellipsis

### Composition

- composes: `PickerShell`, `SelectionSummary`, `SearchField`, `Checkbox`,
  `Button`, `FormActions`, `Icon`
- parent expectations: inline containers, popovers, modal dialogs
- child expectations: candidate items are internally rendered from `items` prop

## 10. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Candidate item | `--pug-color-border-subtle` | item border |
| Candidate item | `--pug-radius-surface` | item radius |
| Candidate item | `--pug-color-background-surface` | item background (86% alpha mix) |
| Candidate (selected) | `--pug-color-accent-base` | selected border (60% mix) and background (10% mix) |
| Candidate description | `--pug-color-text-secondary` | subdued text |
| Candidate focus | `--pug-color-accent-focusRing` | focus outline |
| Drill breadcrumb | `--pug-color-accent-base` | breadcrumb link color |
| Drill level label | `--pug-color-text-secondary` | uppercase level heading |
| Drill item hover | `--pug-color-background-surface` | hover background |
| Drill item focus | `--pug-color-accent-focusRing` | focus outline |
| Footer note | `--pug-color-text-secondary` | subdued footer text |

## 11. Svelte Notes

- uses `createEventDispatcher` for all events
- internal `statusId` used for `aria-describedby` on search field
- `candidateButtons` array for programmatic focus management
- drill-down state is reactive via Svelte `$:` declarations
- drill search query is separate from main `query` prop

## 12. GPUI Notes

- expected crate/module surface: `pug_gpui::composites::relation_picker`
- drill-down navigation must preserve breadcrumb and back semantics

## 13. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] event names and payloads match
- [ ] selection mode behavior matches (single vs multiple)
- [ ] drill-down navigation semantics match
- [ ] keyboard navigation matches

### Tier 2: Visual Parity

- [ ] candidate item styling matches
- [ ] drill breadcrumb styling matches
- [ ] selected state visual treatment matches

### Tier 3: Implementation Freedom

- [ ] internal state management approach may differ
- [ ] async loading patterns may differ

## 14. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none yet | n/a | pending | review during first implementation |

## 15. Specimen Definitions

### Multiple Selection

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Multiple selection | `title="Select components"`, `description="Choose related components."`, `selectionMode="multiple"`, six candidate items with label/description/meta, two pre-selected (`selectedIds=["btn","card"]`) | Picker with search, candidate list with checkboxes, two items checked, selection summary visible below |

### Single Selection

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Single selection | `title="Choose a parent"`, same six candidate items, `selectionMode="single"` | Picker with search and candidate list using radio-style single selection, no items pre-selected |

## 16. Approval And Adoption Notes

- contract status: `seed contract`
- approvers: pending
- downstream adopters: attach-or-choose workflows, entity relation editors,
  hierarchical category pickers
- future follow-up: use `RelationPicker` for attach-or-choose workflows and
  keep domain-specific relation validation above the composite layer
