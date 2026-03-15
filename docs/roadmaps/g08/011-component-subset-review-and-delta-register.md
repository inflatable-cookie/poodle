# g08.011 — Component Subset Review and Delta Register

Status: Completed (updated for full parity)
Updated: 2026-03-14

## Objective

Document which Pug components the Jetstream adapter supports. Following the
decision to target full parity across all three runtimes, the Jetstream adapter
now supports all 117 renderable components (63 primitives + 41 composites +
13 workstation).

## Component Coverage Summary

### Full support: 117 components

**63 primitives** across 7 categories:
- Structural (8): Box, Stack, Grid, Surface, Separator, ScrollShell, Banner, CallOut
- Action (4): Button, IconButton, FormActions, Toolbar
- Input (8): TextInput, TextArea, SearchField, Field, NumberEntry, PinInput, EditableLabel, TimeField
- Selection (8): Checkbox, RadioGroup, Switch, Select, Slider, RangeSlider, SegmentedControl, TriStateSwitch
- Feedback/display (6): Progress, Badge, StatusIndicator, Skeleton, Meter, Rating
- Overlay (13): Dialog, Drawer, Popover, Menu, Tooltip, Tabs, Accordion, Collapsible, HoverCard, ContextMenu, TabStrip, NavigationMenu, Menubar
- Informational/temporal (16): Code, Eyebrow, Pill, TimeAgo, SplitButton, ColorPicker, FileUpload, DurationInput, TimeZoneSelect, ZonedDateTimePicker, Calendar, RangeCalendar, DatePicker, DateRangePicker, DateTimePicker, DateTimeRangePicker

Note: AccordionItemSpec is a sub-spec of AccordionSpec and is not independently
rendered. The 64th export from pug-primitives is this sub-spec, bringing the
renderable primitive count to 63 across all three runtimes.

**41 composites** across 5 categories:
- Form/validation (5): FormShell, ValidationSummary, RemediationBanner, InlineRemediation, ConfirmAction
- Data/browse (13): DataTable, ListShell, GridShell, DetailShell, DetailSection, FilterToolbar, PickerShell, RelationPicker, SelectionSummary, PaginationSummary, MediaThumbnail, MediaPreview, EmptyState
- Editing/media (8): AudioPlayer, VideoPlayer, MediaPicker, MarkdownEditor, BlockEditor, EmbedInput, EmbedPreview, EmbedShell
- Navigation/list (7): AutonomousList, ReorderableList, Breadcrumbs, CardRadioGroup, InlineEditableField, OrderBy, SlugField
- Operational (8): PageHeader, PageLoading, StateTile, ToastStack, LogList, NavCard, NavCardGrid, ListCard

**13 workstation** specs:
- ActionDiscoveryPanel, AppHeader, CommandPalette, CommandPaletteShell, DockRegion, PanelHeader, PanelSurface, PanelTabs, ProjectHeader, ShellStatusBar, SplitView, SurfaceTabs, WorkspaceShell

### Intentionally unsupported: 0 components

All previously unsupported components have been implemented. The engine will be
embedded in various environments (including web-oriented contexts) that can make
use of all component types.

### Coverage ratio

- Supported: 117 / 117 renderable components (100%)
- Unsupported: 0
- `UNSUPPORTED_COMPONENTS` constant removed from lib.rs

## Verification

```
cargo test — 142 tests passing
AdapterManifest::unsupported_components() returns empty slice
```
