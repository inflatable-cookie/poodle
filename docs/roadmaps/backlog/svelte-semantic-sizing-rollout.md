# Svelte Semantic Sizing and Density — Full Rollout

Status: complete
Updated: 2026-03-26
Depends on: landed presentation model in `packages/svelte/primitives/src/presentation.ts`

## Purpose

Complete the semantic sizing and density rollout across the entire Svelte
component surface. The presentation model (ControlSize, ControlDensity,
SemanticControlSizeRole, UiPresentationProvider, resolveSemanticControlSize)
already exists and works.

## Current state

All 75 interactive Svelte components now have full semantic sizing: exported
`size`/`sizeRole` props, presentation model resolution, `data-size` attribute
emission, AND CSS `[data-size]` variant rules that visually respond to all
five size stops (xs, sm, md, lg, xl). Remaining work is contract/specimen/
component-docs updates to document the new props and showcase size variations.

## What "fully rolled out" means

A component is fully rolled out when:

- it exports `size: ControlSize | null = null`
- it exports `sizeRole: SemanticControlSizeRole = "<appropriate-default>"`
- it exports `density: ControlDensity | null = null` (where density affects
  spacing/inset behavior)
- it reads parent presentation with `getUiPresentation()`
- it resolves inherited size via `resolveSemanticControlSize()`
- it uses `UiPresentationProvider` where nested primitives should inherit the
  local override
- its control chrome (heights, padding, gaps, icon sizes) responds to the
  resolved size/density instead of being hardcoded
- its contract, specimen, and component-docs entry reflect the new props

## Fully landed — props + CSS size variants (75 components)

All interactive components now have size/sizeRole props AND CSS `[data-size]`
rules that visually respond to all five size stops (xs, sm, md, lg, xl).

### Primitives (53)

Button, IconButton, Icon, Spinner, Pill, Toggle, SplitButton, Tabs,
ToggleGroup, TriStateSwitch, SegmentedControl, Toolbar, Checkbox,
RadioGroup, Switch, TextInput, Select, NumberEntry, Slider,
RangeSlider, Rating, PinInput, Pagination, Menu, DatePicker, TextArea,
SearchInput, EditableLabel, FileUpload, DurationInput, ColorPicker,
DateRangePicker, DateTimePicker, DateTimeRangePicker, TimeField,
TimeZoneSelect, ZonedDateTimePicker, Calendar, RangeCalendar, Menubar,
ContextMenu, NavigationMenu, Breadcrumbs, OrderBy, BulkActionBar, Code,
Dialog, AlertDialog, Drawer, Accordion, Collapsible, CollapseToggle, Callout

### Composites (22)

DockRegion, AudioPlayer, ActionDiscoveryPanel, CommandPalette, EditableList,
ReorderableList, MediaBrowsePanel, MediaPicker, RelationPicker, BlockEditor,
LogList, MarkdownEditor, DataTable, CardRadioGroup, FilterToolbar,
VideoPlayer, SelectionSummary, ToastStack, MediaUploadStatusPanel

### Delegates to children — no unique chrome (3)

SearchInput (delegates to TextInput), ConfirmAction (composes
AlertDialog + Button), SplitView (structural layout, delegates
CollapseToggle and resize handles)

## Execution tiers

### Tier 1 — Core form controls

These are the most commonly used interactive primitives. They appear in nearly
every form and panel surface. Completing this tier means the standard form
experience responds coherently to workspace presentation.

| Component | Has `--poodle-size-control-height`? | Default role | Notes |
| --- | --- | --- | --- |
| Checkbox | no | `control` | needs full implementation |
| RadioGroup | no | `control` | needs full implementation |
| Switch | no | `control` | needs full implementation |
| TextInput | yes (CSS only) | `control` | has control-height token but no sizeRole prop |
| TextArea | no | `control` | needs full implementation |
| Select | yes (CSS only) | `control` | has control-height token but no sizeRole prop |
| SearchInput | no | `control` | needs full implementation |

Status: ✅ complete — props + CSS variants landed

### Tier 2 — Numeric and specialized inputs

These are less universal than core form controls but still appear frequently in
data-entry, settings, and configuration surfaces.

| Component | Has `--poodle-size-control-height`? | Default role | Notes |
| --- | --- | --- | --- |
| NumberEntry | yes (CSS only) | `control` | has control-height token but no sizeRole prop |
| Slider | no | `control` | needs full implementation |
| RangeSlider | no | `control` | needs full implementation |
| Rating | no | `control` | needs full implementation |
| PinInput | yes (CSS only) | `control` | has control-height token but no sizeRole prop |
| EditableLabel | no | `control` | needs full implementation |
| FileUpload | no | `control` | needs full implementation |
| DurationInput | no | `control` | needs full implementation |
| ColorPicker | yes (CSS only) | `control` | has control-height token but no sizeRole prop |

Status: ✅ complete — props + CSS variants landed

### Tier 3 — Date and time family

These are compound pickers that typically wrap or compose other primitives.
Some may mostly delegate to upgraded children once Tier 1 is done, but each
still needs its own sizeRole prop and presentation wiring.

| Component | Has `--poodle-size-control-height`? | Default role | Notes |
| --- | --- | --- | --- |
| DatePicker | yes (CSS only) | `control` | has control-height token but no sizeRole prop |
| DateRangePicker | no | `control` | needs full implementation |
| DateTimePicker | no | `control` | needs full implementation |
| DateTimeRangePicker | no | `control` | needs full implementation |
| TimeField | no | `control` | needs full implementation |
| TimeZoneSelect | no | `control` | needs full implementation |
| ZonedDateTimePicker | no | `control` | needs full implementation |
| Calendar | no | `control` | date grid used inside pickers |
| RangeCalendar | no | `control` | range date grid |

Status: ✅ complete — props + CSS variants landed

### Tier 4 — Navigation, menus, and action bars

These are interactive chrome surfaces that typically live in toolbars, panels,
or shell regions. Many should default to `chrome` since they are supporting
navigation/action UI rather than primary controls.

| Component | Has `--poodle-size-control-height`? | Default role | Notes |
| --- | --- | --- | --- |
| Menu | yes (CSS only) | `chrome` | menu items |
| Menubar | no | `chrome` | menu bar |
| ContextMenu | no | `chrome` | context menu overlay |
| NavigationMenu | no | `chrome` | navigation menu |
| Breadcrumbs | no | `chrome` | nav breadcrumbs |
| Pagination | yes (CSS only) | `control` | page nav buttons |
| OrderBy | no | `control` | sort direction buttons |
| BulkActionBar | yes (CSS only) | `control` | batch action bar |
| Code | no | `chrome` | code block with copy button |

Status: ✅ complete — props + CSS variants landed

### Tier 5 — Overlays and disclosure

These are container-level components (dialogs, drawers, accordions) that host
other controls. Their own chrome (close buttons, trigger buttons, section
headers) should track presentation, and they should propagate context to
children.

| Component | Has `--poodle-size-control-height`? | Default role | Notes |
| --- | --- | --- | --- |
| Dialog | no | `control` | close button, footer actions |
| AlertDialog | no | `control` | wraps Dialog + action buttons |
| Drawer | no | `control` | panel with close/actions |
| Accordion | no | `control` | collapsible section triggers |
| Collapsible | no | `control` | toggle trigger |
| CollapseToggle | no | `chrome` | standalone collapse button |
| Callout | no | `control` | optional dismiss button |

Status: ✅ complete — props + CSS variants landed

### Tier 6 — Remaining composites

These are composite components with interactive elements that should participate
in the presentation model.

| Component | Has `--poodle-size-control-height`? | Default role | Notes |
| --- | --- | --- | --- |
| DataTable | yes (CSS only) | `control` | sort buttons, checkboxes, row actions |
| CardRadioGroup | yes (CSS only) | `control` | radio card selection |
| FilterToolbar | no | `chrome` | filter toggle buttons |
| VideoPlayer | no | `control` | play/pause, seek, volume (similar to AudioPlayer) |
| ConfirmAction | no | `control` | confirm/cancel buttons |
| SelectionSummary | yes (CSS only) | `control` | clear button, chip remove buttons |
| ToastStack | no | `chrome` | dismiss/action buttons |
| MediaUploadStatusPanel | no | `control` | cancel/retry buttons |
| SplitView | no | `chrome` | resize handles, collapse toggles |

Status: ✅ complete — props + CSS variants landed

## Intentionally excluded — token-inherited and sufficient

These components are display surfaces, layout shells, or structural containers
where fixed values are content typography or layout constants rather than
control chrome. They do not need `sizeRole` props.

### Primitives

Box, Spacer, Stack, Grid, Region, Surface, Card, ListCard, ListCardCounter,
NavCard, NavCardGrid, Eyebrow, Separator, ScrollShell, Skeleton,
StatusIndicator, StatusBar, Progress, Meter, TimeAgo, DetailItem, FormActions,
Field, FieldSet, Tooltip, Popover, HoverCard, Table, PaginationSummary,
UiPresentationProvider, IconProvider

### Composites

MetricTile, MediaPreview, EmbedPreview, PickerShell, EmptyState, FormLayout,
FormDialog, AppHeader, DetailSection, DetailShell, PageHeader, MediaThumbnail,
ListContainer, PageLoading, EmbedInput

## Execution pattern

For each component in each tier:

1. read the component contract in `docs/contracts/`
2. add `size: ControlSize | null = null` prop
3. add `sizeRole: SemanticControlSizeRole = "<default>"` prop
4. add `density: ControlDensity | null = null` prop where density affects
   spacing/inset
5. read parent presentation with `getUiPresentation()`
6. resolve size via `resolveSemanticControlSize()`
7. replace hardcoded control chrome with size/density-responsive values
8. use `UiPresentationProvider` where nested primitives should inherit
9. update contract to document the new props
10. update specimen to show size/density variations
11. update component-docs entry

## Validation

After each tier:

- `effigy health`
- `effigy docs:check`
- `git diff --check`

## Completion criteria

The rollout is complete when every component in Tiers 1–6 has been upgraded or
explicitly reclassified with justification. The "intentionally excluded" list
should only grow if a component is genuinely non-interactive display/layout.

## Remaining work

✅ **All work complete.** The semantic sizing and density rollout is fully landed.

Summary of completed phases:

1. ~~**Size props + CSS variants**~~ — ✅ All 75 interactive components have
   `size`/`sizeRole` props, presentation resolution, `data-size` emission,
   and CSS `[data-size]` rules for all five size stops (xs–xl).
2. ~~**Size contract updates**~~ — ✅ 53 contracts document `size`/`sizeRole`
   props, size adjustment token tables, `data-size` Svelte Notes, and parity
   checklist items.
3. ~~**Size specimen updates**~~ — ✅ 49 specimens show "Sizes" sections
   demonstrating all five size stops.
4. ~~**Size component-docs entries**~~ — ✅ All 73 interactive component-docs
   entries list `size`/`sizeRole` props.
5. ~~**Density props + CSS variants**~~ — ✅ All 75 interactive components
   export `density: ControlDensity | null = null`, resolve via presentation
   context, emit `data-density`, and have CSS `[data-density]` rules for
   compact/comfortable spacing adjustments.
6. ~~**Density contract updates**~~ — ✅ ~57 contracts document the `density`
   prop and `data-density` attribute.
7. ~~**Density specimen updates**~~ — ✅ ~49 specimens show "Densities"
   sections demonstrating compact, default, and comfortable.
8. ~~**Density component-docs entries**~~ — ✅ All 73 interactive
   component-docs entries list the `density` prop.
