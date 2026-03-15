# g09.001 — Gap Register: Svelte Surface vs GPUI Components

Status: active
Owner: Pug Core
Updated: 2026-03-15

## Summary

- **Svelte primitives**: 72 components
- **Svelte composites**: 41 components
- **Svelte workstation**: 12 components
- **Total Svelte surface**: 125 components
- **GPUI components**: 41 (all primitives)
- **Missing from GPUI**: 84 components (31 primitives + 41 composites + 12 workstation)

## Primitive Gap Register (72 Svelte → 41 GPUI)

### Present in GPUI (41)

| Svelte Component | GPUI Struct | Notes |
|-----------------|-------------|-------|
| Accordion | PugAccordion | ✓ |
| Badge | PugBadge | ✓ |
| Box | PugBox | ✓ |
| Button | PugButton | ✓ |
| Calendar | PugCalendar | ✓ |
| Checkbox | PugCheckbox | ✓ |
| Collapsible | PugCollapsible | ✓ |
| ContextMenu | PugContextMenu | ✓ |
| DatePicker | PugDatePicker | ✓ |
| DateRangePicker | PugDateRangePicker | ✓ |
| DateTimePicker | PugDateTimePicker | ✓ |
| DateTimeRangePicker | PugDateTimeRangePicker | ✓ |
| Dialog | PugDialog | ✓ |
| Drawer | PugDrawer | ✓ |
| Field | PugField | ✓ |
| FormActions | PugFormActions | ✓ |
| Grid | PugGrid | ✓ |
| IconButton | PugIconButton | ✓ |
| Menu | PugMenu | ✓ |
| Menubar | PugMenubar | ✓ |
| NavigationMenu | PugNavigationMenu | ✓ |
| Popover | PugPopover | ✓ |
| Progress | PugProgress | ✓ |
| RadioGroup | PugRadioGroup | ✓ |
| RangeCalendar | PugRangeCalendar | ✓ |
| ScrollShell | PugScrollShell | ✓ |
| SearchField | PugSearchField | ✓ |
| SegmentedControl | PugSegmentedControl | ✓ |
| Select | PugSelect | ✓ |
| Separator | PugSeparator | ✓ |
| Slider | PugSlider | ✓ |
| Stack | PugStack | ✓ |
| StatusIndicator | PugStatusIndicator | ✓ |
| Surface | PugSurface | ✓ |
| Switch | PugSwitch | ✓ |
| Tabs | PugTabs | ✓ |
| TextArea | PugTextArea | ✓ |
| TextInput | PugTextInput | ✓ |
| TimeField | PugTimeField | ✓ |
| Tooltip | PugTooltip | ✓ |

Note: GPUI also has PugTabStrip (from TabStripSpec) which has no separate
Svelte counterpart — Svelte Tabs includes the strip internally.

### Missing — New Component Needed (27)

| Svelte Component | Spec Exists | Batch | Priority |
|-----------------|-------------|-------|----------|
| AlertDialog | DialogSpec (variant) | 002 | High |
| Banner | BannerSpec | 002 | High |
| Callout | CallOutSpec | 002 | High |
| Code | CodeSpec | 002 | High |
| ColorPicker | ColorPickerSpec | 003 | Medium |
| DurationInput | DurationInputSpec | 003 | Medium |
| EditableLabel | EditableLabelSpec | 002 | High |
| Eyebrow | EyebrowSpec | 002 | High |
| FileUpload | FileUploadSpec | 003 | Medium |
| HoverCard | HoverCardSpec | 002 | Medium |
| Meter | MeterSpec | 002 | High |
| NumberEntry | NumberEntrySpec | 003 | High |
| Pill | PillSpec | 002 | High |
| PinInput | PinInputSpec | 003 | High |
| RangeSlider | RangeSliderSpec | 003 | High |
| Rating | RatingSpec | 002 | Medium |
| Skeleton | SkeletonSpec | 002 | High |
| SplitButton | SplitButtonSpec | 003 | Medium |
| TimeAgo | TimeAgoSpec | 003 | Medium |
| TimeZoneSelect | TimeZoneSelectSpec | 003 | Low |
| Toolbar | ToolbarSpec | 003 | High |
| TriStateSwitch | TriStateSwitchSpec | 002 | Medium |
| ZonedDateTimePicker | ZonedDateTimePickerSpec | 003 | Low |
| Inline | — (layout helper) | 002 | Medium |
| Spacer | — (layout helper) | 002 | Medium |
| Toggle | — (button variant) | 002 | Medium |
| ToggleGroup | — (group of toggles) | 002 | Medium |

### Not Applicable to GPUI (4)

| Svelte Component | Reason |
|-----------------|--------|
| Icon | Utility — GPUI uses gpui::Svg directly |
| IconProvider | Utility — Svelte-specific context provider |
| Combobox | Covered by PugSelect with search/filter |
| Pagination | Covered by PaginationSummary composite |
| Table | Raw HTML table — use DataTable composite |

## Composite Gap Register (41 Svelte → 0 GPUI)

All 41 composites are missing from GPUI. All have specs in `pug-composites`.

### Batch 004 — Form and Data (15)

| Svelte Component | Spec |
|-----------------|------|
| BulkActionBar | — (compose from primitives) |
| ConfirmAction | ConfirmActionSpec |
| DataTable | DataTableSpec |
| DetailRow | — (compose from primitives) |
| DetailSection | DetailSectionSpec |
| DetailShell | DetailShellSpec |
| FilterToolbar | FilterToolbarSpec |
| FormDialog | FormShellSpec |
| GridShell | GridShellSpec |
| ListShell | ListShellSpec |
| OrderBy | OrderBySpec |
| PaginationSummary | PaginationSummarySpec |
| PickerShell | PickerShellSpec |
| RelationPicker | RelationPickerSpec |
| SelectionSummary | SelectionSummarySpec |

### Batch 005 — Editing, Media, and Operational (17)

| Svelte Component | Spec |
|-----------------|------|
| AudioPlayer | AudioPlayerSpec |
| AutonomousList | AutonomousListSpec |
| BlockEditor | BlockEditorSpec |
| EmbedInput | EmbedInputSpec |
| EmbedPreview | EmbedPreviewSpec |
| EmbedShell | EmbedShellSpec |
| EmptyState | EmptyStateSpec |
| InlineEditableField | InlineEditableFieldSpec |
| LogList | LogListSpec |
| MarkdownEditor | MarkdownEditorSpec |
| MediaPicker | MediaPickerSpec |
| MediaPreview | MediaPreviewSpec |
| MediaThumbnail | MediaThumbnailSpec |
| PageLoading | PageLoadingSpec |
| SlugField | SlugFieldSpec |
| StateTile | StateTileSpec |
| ToastStack | ToastStackSpec |
| VideoPlayer | VideoPlayerSpec |

### Remaining Composites (9)

| Svelte Component | Spec |
|-----------------|------|
| Breadcrumbs | BreadcrumbsSpec |
| Card | — (compose from Surface) |
| CardRadioGroup | CardRadioGroupSpec |
| ListCard | ListCardSpec |
| NavCard | NavCardSpec |
| NavCardGrid | NavCardGridSpec |
| PageHeader | PageHeaderSpec |
| ReorderableList | ReorderableListSpec |

## Workstation Gap Register (12 Svelte → 0 GPUI)

All 12 workstation surfaces are missing from GPUI. All have specs in
`pug-workstation`.

### Batch 006

| Svelte Component | Spec |
|-----------------|------|
| ActionDiscoveryPanel | ActionDiscoveryPanelSpec |
| AppHeader | AppHeaderSpec |
| CommandPalette | CommandPaletteSpec |
| CommandPaletteShell | CommandPaletteShellSpec |
| DockRegion | DockRegionSpec |
| PanelHeader | PanelHeaderSpec |
| PanelSurface | PanelSurfaceSpec |
| PanelTabs | PanelTabsSpec |
| ProjectHeader | ProjectHeaderSpec |
| ShellStatusBar | ShellStatusBarSpec |
| SplitView | SplitViewSpec |
| SurfaceTabs | SurfaceTabsSpec |
| WorkspaceShell | WorkspaceShellSpec |

## Batch Assignments Summary

| Batch | Milestone | Components | Type |
|-------|-----------|-----------|------|
| 002 | Missing primitives: structural + informational | 16 | Primitives |
| 003 | Missing primitives: input + temporal | 11 | Primitives |
| 004 | Missing composites: form + data | 15 | Composites |
| 005 | Missing composites: editing + media + operational | 17+9=26 | Composites |
| 006 | Missing workstation | 13 | Workstation |
| **Total** | | **81** | |

## Specimen Status

Components with hand-built mockup specimens that need upgrading to real
components (tracked in g09.007–012):

- bx, stack, grid, surface, separator, scroll_shell (structural — partial real)
- banner, callout (structural — uses PugSurface/PugBadge, not PugBanner)
- split_button, number_entry, pin_input, toolbar, time_field (action — mixed)
- meter, rating, skeleton, pill, temporal, code, color_picker, file_upload (feedback — mockups)
- range_slider (selection — mockup)
- All composites (grouped into catchall files)
- All workstation (grouped into catchall files)
