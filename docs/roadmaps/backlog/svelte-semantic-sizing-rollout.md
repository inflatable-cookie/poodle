# Svelte Semantic Sizing and Density — Full Rollout

Status: active
Updated: 2026-03-26
Depends on: landed presentation model in `packages/svelte/primitives/src/presentation.ts`

## Purpose

Complete the semantic sizing and density rollout across the entire Svelte
component surface. The presentation model (ControlSize, ControlDensity,
SemanticControlSizeRole, UiPresentationProvider, resolveSemanticControlSize)
already exists and works.

## Current state

All interactive Svelte components now export `size` and `sizeRole` props and
resolve through the shared presentation model. The prop wiring and
`data-size` attribute are landed across 54 primitives and 21 composites (75
total). Remaining work is CSS size-variant styling and contract/specimen
updates — the components accept and resolve semantic sizing but most do not
yet visually respond to all five size stops in their CSS.

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

## Fully landed — props + CSS size variants (24 components)

These components have full size/sizeRole/density props AND CSS that responds
to all five size stops:

### Primitives (12)

- Button (`control`)
- IconButton (`control`)
- Icon (`chrome`)
- Spinner (`control`)
- Pill (`chrome`)
- Toggle (`control`)
- SplitButton (`control`)
- Tabs (`chrome`)
- ToggleGroup (`control`)
- TriStateSwitch (`control`)
- SegmentedControl (`control`)
- Toolbar (`chrome`)

### Composites (12)

- DockRegion (`chrome`)
- AudioPlayer (`control`)
- ActionDiscoveryPanel (`control`)
- CommandPalette (`control`)
- EditableList (`control`)
- ReorderableList (`control`)
- MediaBrowsePanel (`control`)
- MediaPicker (`control`)
- RelationPicker (`control`)
- BlockEditor (`control`)
- LogList (`control`)
- MarkdownEditor (`control`)

## Props wired — CSS size variants pending (51 components)

These components now export `size`/`sizeRole` props, resolve through
`getUiPresentation()` + `resolveSemanticControlSize()`, and emit
`data-size` on their root element. They inherit the correct resolved size
from the presentation context. What remains is adding CSS `[data-size]`
variant rules where the component's own chrome (heights, padding, icon
sizes) should visually respond to different size stops.

### Primitives (42)

Checkbox, RadioGroup, Switch, TextInput, TextArea, Select, Combobox,
SearchField, NumberEntry, Slider, RangeSlider, Rating, PinInput,
EditableLabel, FileUpload, DurationInput, ColorPicker, DatePicker,
DateRangePicker, DateTimePicker, DateTimeRangePicker, TimeField,
TimeZoneSelect, ZonedDateTimePicker, Calendar, RangeCalendar, Menu,
Menubar, ContextMenu, NavigationMenu, Breadcrumbs, Pagination, OrderBy,
BulkActionBar, Code, Dialog, AlertDialog, Drawer, Accordion, Collapsible,
CollapseToggle, Callout

### Composites (9)

DataTable, CardRadioGroup, FilterToolbar, VideoPlayer, ConfirmAction,
SelectionSummary, ToastStack, MediaUploadStatusPanel, SplitView

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
| Combobox | yes (CSS only) | `control` | has control-height token but no sizeRole prop |
| SearchField | no | `control` | needs full implementation |

Status: props wired, CSS variants pending

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

Status: props wired, CSS variants pending

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

Status: props wired, CSS variants pending

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

Status: props wired, CSS variants pending

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

Status: props wired, CSS variants pending

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

Status: props wired, CSS variants pending

## Intentionally excluded — token-inherited and sufficient

These components are display surfaces, layout shells, or structural containers
where fixed values are content typography or layout constants rather than
control chrome. They do not need `sizeRole` props.

### Primitives

Box, Spacer, Stack, Grid, Region, Surface, Card, ListCard, ListCardCounter,
NavCard, NavCardGrid, Eyebrow, Separator, ScrollShell, Skeleton,
StatusIndicator, StatusBar, Progress, Meter, TimeAgo, DetailRow, FormActions,
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
