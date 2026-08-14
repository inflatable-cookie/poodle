// --- Primitives ---
export { default as Accordion } from "./Accordion.svelte";
export { default as AgentChatInput } from "./AgentChatInput.svelte";
export { default as AudioPlayer } from "./AudioPlayer.svelte";
export { default as AlertDialog } from "./AlertDialog.svelte";
export { default as Avatar } from "./Avatar.svelte";
export { default as Box } from "./Box.svelte";
export { default as Breadcrumbs } from "./Breadcrumbs.svelte";
export { default as BulkActionBar } from "./BulkActionBar.svelte";
export { default as Button } from "./Button.svelte";
export { default as Callout } from "./Callout.svelte";
export { default as RemediationBanner } from "./RemediationBanner.svelte";
export { default as Card } from "./Card.svelte";
export { default as Code } from "./Code.svelte";
export { default as ColorPicker } from "./ColorPicker.svelte";
export { default as Checkbox } from "./Checkbox.svelte";
export { default as Calendar } from "./Calendar.svelte";
export { default as ContextMenu } from "./ContextMenu.svelte";
export { default as CollapseToggle } from "./CollapseToggle.svelte";
export { default as Collapsible } from "./Collapsible.svelte";
export { default as DetailItem } from "./DetailItem.svelte";
export { default as DatePicker } from "./DatePicker.svelte";
export { default as DateRangePicker } from "./DateRangePicker.svelte";
export { default as DateTimePicker } from "./DateTimePicker.svelte";
export { default as DateTimeRangePicker } from "./DateTimeRangePicker.svelte";
export { default as Dialog } from "./Dialog.svelte";
export { default as Drawer } from "./Drawer.svelte";
export { default as DurationInput } from "./DurationInput.svelte";
export { default as EditableLabel } from "./EditableLabel.svelte";
export { default as Eyebrow } from "./Eyebrow.svelte";
export { default as Field } from "./Field.svelte";
export { default as FieldSet } from "./FieldSet.svelte";
export { default as FileUpload } from "./FileUpload.svelte";
export {
  DEFAULT_COMPRESSION,
  compressImage,
  formatFileSize,
  type FileUploadValidationError,
  type ImageCompressionOptions,
} from "./file-upload";
export { default as FilterBuilder } from "./FilterBuilder.svelte";
export { default as FormActions } from "./FormActions.svelte";
export { default as Grid } from "./Grid.svelte";
export { default as HoverCard } from "./HoverCard.svelte";
export { default as Icon } from "./Icon.svelte";
export { default as IconButton } from "./IconButton.svelte";
export { default as IconProvider } from "./IconProvider.svelte";
export { default as Meter } from "./Meter.svelte";
export { default as ListCard } from "./ListCard.svelte";
export { default as ListCardCounter } from "./ListCardCounter.svelte";
export { default as ListGrid } from "./ListGrid.svelte";
export { default as Menu } from "./Menu.svelte";
export { default as MetaBar } from "./MetaBar.svelte";
export { default as MetaItem } from "./MetaItem.svelte";
export { default as NumberInput } from "./NumberInput.svelte";
export { default as OrderBy } from "./OrderBy.svelte";
export { default as NavCard } from "./NavCard.svelte";
export { default as NavigationMenu } from "./NavigationMenu.svelte";
export { default as Pill } from "./Pill.svelte";
export { default as CodeInput } from "./CodeInput.svelte";
export { default as Popover } from "./Popover.svelte";
export { default as Pagination } from "./Pagination.svelte";
export { default as PaginationSummary } from "./PaginationSummary.svelte";
export { default as PasswordRequirements } from "./PasswordRequirements.svelte";
export { default as Progress } from "./Progress.svelte";
export { default as Radio } from "./Radio.svelte";
export { default as RefSelect } from "./RefSelect.svelte";
export { default as RadioGroup } from "./RadioGroup.svelte";
export { default as Rating } from "./Rating.svelte";
export { default as Region } from "./Region.svelte";
export { default as ResizeHandle } from "./ResizeHandle.svelte";
export { default as RangeSlider } from "./RangeSlider.svelte";
export { default as SegmentedControl } from "./SegmentedControl.svelte";
export { default as Select } from "./Select.svelte";
export { default as ScrollShell } from "./ScrollShell.svelte";
export { default as Separator } from "./Separator.svelte";
export { default as SplitButton } from "./SplitButton.svelte";
export { default as Skeleton } from "./Skeleton.svelte";
export { default as Slider } from "./Slider.svelte";
export { default as Spinner } from "./Spinner.svelte";
export { default as Spacer } from "./Spacer.svelte";
export { default as Stack } from "./Stack.svelte";
export { default as Stepper } from "./Stepper.svelte";
export { default as AgentMessage } from "./AgentMessage.svelte";
export { default as AgentPlan } from "./AgentPlan.svelte";
export { default as AgentPlanRecord } from "./AgentPlanRecord.svelte";
export { default as AgentQuestion } from "./AgentQuestion.svelte";
export { default as AgentQuestionRecord } from "./AgentQuestionRecord.svelte";
export { default as AgentSubagent } from "./AgentSubagent.svelte";
export { default as AgentTranscript } from "./AgentTranscript.svelte";
export { default as ChangedFiles } from "./ChangedFiles.svelte";
export { default as ToolCall } from "./ToolCall.svelte";
export { default as ToolCallGroup } from "./ToolCallGroup.svelte";
export { default as StatusBar } from "./StatusBar.svelte";
export { default as StatusIndicator } from "./StatusIndicator.svelte";
export { default as Surface } from "./Surface.svelte";
export { default as Switch } from "./Switch.svelte";
export { default as Text } from "./Text.svelte";
export { default as TextLink } from "./TextLink.svelte";
export { default as Tabs } from "./Tabs.svelte";
export { default as Table } from "./Table.svelte";
export { default as TimeAgo } from "./TimeAgo.svelte";
export { default as TextInput } from "./TextInput.svelte";
export { default as TokenInput } from "./TokenInput.svelte";
export { default as TimeInput } from "./TimeInput.svelte";
export { default as TimeZoneSelect } from "./TimeZoneSelect.svelte";
export { default as ThemeSelect } from "./ThemeSelect.svelte";
export {
  createThemeController,
  getThemeController,
  currentTheme,
  type ThemeController,
  type ThemeControllerConfig,
} from "./theme-controller";
export { default as ToggleGroup } from "./ToggleGroup.svelte";
export { default as Toolbar } from "./Toolbar.svelte";
export { default as Tooltip } from "./Tooltip.svelte";
export { default as TriStateSwitch } from "./TriStateSwitch.svelte";
export { default as Menubar } from "./Menubar.svelte";
export { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
export { default as VideoPlayer } from "./VideoPlayer.svelte";
export { default as DateTimeZonePicker } from "./DateTimeZonePicker.svelte";
export {
  formatDisplayDate,
  formatDisplayDateTime,
} from "./date";
export {
  controlHeightRem,
  controlSpaceXRem,
  getUiPresentation,
  panelSpaceXRem,
  panelSpaceYRem,
  resolveSemanticControlSize,
  resolveSupportingVisualSize,
} from "./presentation";

// Overlay infrastructure: hosts building their own anchored surface get the
// same portalling and placement the library uses (002-anchored-overlays.md).
export { anchored, type AnchoredOptions } from "./anchored";
export { portal } from "./portal";
export type {
  OverlaySurfaceGeometry,
  OverlaySurfaceGeometryChange,
  OverlaySurfaceGeometryChangeHandler,
  OverlayViewportRect,
} from "@inflatable-cookie/poodle-core";

// --- Composites ---
export { default as ActionDiscoveryPanel } from "./ActionDiscoveryPanel.svelte";
export { default as AppHeader } from "./AppHeader.svelte";
export { default as EditableList } from "./EditableList.svelte";
export { default as ErrorBoundary } from "./ErrorBoundary.svelte";
export { default as BlockEditor } from "./BlockEditor.svelte";
export { default as CardRadioGroup } from "./CardRadioGroup.svelte";
export { default as CardToggleGroup } from "./CardToggleGroup.svelte";
export { default as CommandPalette } from "./CommandPalette.svelte";
export { default as ConfirmAction } from "./ConfirmAction.svelte";
export { default as DataTable } from "./DataTable.svelte";
export { default as DetailSectionGroup } from "./DetailSectionGroup.svelte";
export { default as DetailSection } from "./DetailSection.svelte";
export { default as DockRegion } from "./DockRegion.svelte";
export { default as DetailShell } from "./DetailShell.svelte";
export { default as EmbedInput } from "./EmbedInput.svelte";
export { default as EmbedPreview } from "./EmbedPreview.svelte";
export {
  detectParsedEmbed,
  getProviderAccent,
  getThumbnailUrl,
  lookupMeta,
  parseEmbed,
  renderEmbed,
  resolveEmbedParseState,
} from "./embed-input";
export { default as EmptyState } from "./EmptyState.svelte";
export { default as FilterToolbar } from "./FilterToolbar.svelte";
export { default as FormDialog } from "./FormDialog.svelte";
export { default as FormLayout } from "./FormLayout.svelte";
export { default as InlineListSection } from "./InlineListSection.svelte";
export { default as DebugDialog } from "./DebugDialog.svelte";
export { default as LicenceActivation } from "./LicenceActivation.svelte";
export { default as LicenceSeats } from "./LicenceSeats.svelte";
export { default as LicenceStatus } from "./LicenceStatus.svelte";
export { default as LogList } from "./LogList.svelte";
export { default as ListContainer } from "./ListContainer.svelte";
export { default as MarkdownEditor } from "./MarkdownEditor.svelte";
export { default as PageLoading } from "./PageLoading.svelte";
export { default as MediaPicker } from "./MediaPicker.svelte";
export { default as MediaBrowsePanel } from "./MediaBrowsePanel.svelte";
export {
  computeFileHash,
  createResetMediaBrowseState,
  loadMediaBrowsePage,
  mergeMediaBrowseItems,
  runMediaUploadWorkflow,
  uploadMediaWithKnownHash,
} from "./media-workflow";
export { default as MediaPreview } from "./MediaPreview.svelte";
export { default as MediaThumbnail } from "./MediaThumbnail.svelte";
export { default as PageHeader } from "./PageHeader.svelte";
export { default as PickerShell } from "./PickerShell.svelte";
export { default as RelationPicker } from "./RelationPicker.svelte";
export { default as SelectionSummary } from "./SelectionSummary.svelte";
export { default as SettingsShell } from "./SettingsShell.svelte";
export { default as SidebarNav } from "./SidebarNav.svelte";
export { default as Tree } from "./Tree.svelte";
export { default as SplitView } from "./SplitView.svelte";
export { default as MetricTile } from "./MetricTile.svelte";
export { default as StateTile } from "./StateTile.svelte";
export { default as ValidationSummary } from "./ValidationSummary.svelte";
export { default as ModelPicker } from "./ModelPicker.svelte";
export { default as ModelConnectionPicker } from "./ModelConnectionPicker.svelte";
export { default as ModelConnectionSetup } from "./ModelConnectionSetup.svelte";
export { default as ModelConnectionCard } from "./ModelConnectionCard.svelte";
export { default as ModelCatalogueEditor } from "./ModelCatalogueEditor.svelte";
export { default as MessageCenter } from "./MessageCenter.svelte";
export { default as HistoryCenter } from "./HistoryCenter.svelte";
export { default as UpdateStatus } from "./UpdateStatus.svelte";
export { default as UpdateCenter } from "./UpdateCenter.svelte";
export { default as ToastStack } from "./ToastStack.svelte";
export { default as ToastHost } from "./ToastHost.svelte";
export {
  parseWorkspaceLayoutSnapshot,
  serializeWorkspaceLayoutSnapshot,
} from "./persistence";

// --- Types ---
export type {
  IconProp,
  AccordionItem,
  AlertDialogTone,
  AnnouncementMode,
  CalloutAnnounceMode,
  ColorInputMode,
  ButtonTone,
  ButtonVariant,
  RemediationAction,
  CalendarWeekStart,
  CollapseDirection,
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
  DateTimeValue,
  DateTimeRangeValue,
  DateRangeValue,
  DialogKind,
  DrawerEdge,
  FileUploadItem,
  EditableLabelActivationMode,
  LayoutAlign,
  LayoutJustify,
  MenuItem,
  MenubarItem,
  NavigationMenuItem,
  TimeZoneOption,
  FormActionAlign,
  FormActionDangerItem,
  Orientation,
  OverlayPlacement,
  OverflowMode,
  PillAppearance,
  PillFont,
  PillSize,
  PillTone,
  PopoverInitialFocus,
  RadioGroupOption,
  PasswordRequirementsPolicy,
  ScrollDirection,
  SegmentedControlOption,
  StepperStep,
  ChangedFile,
  ChangedFileStatus,
  ToolCallStatus,
  TranscriptActivity,
  TranscriptBlock,
  TranscriptChangedFiles,
  TranscriptItem,
  TranscriptMessage,
  TranscriptRole,
  TranscriptToolCall,
  TranscriptToolRun,
  TranscriptDecidedPlan,
  TranscriptSubagentGroup,
  AgentPlanStatus,
  AgentPlanSettledStatus,
  AgentPlanDecision,
  AgentQuestionOption,
  AgentQuestionItem,
  AgentQuestionOutcome,
  AgentQuestionAnswer,
  AgentSubagentStatus,
  AgentSubagentItem,
  AnsweredQuestion,
  SelectItems,
  SelectOption,
  SelectOptionGroup,
  SeparatorTone,
  SkeletonPreset,
  SkeletonShape,
  SpinnerSize,
  SpinnerTone,
  SpinnerVariant,
  SpaceScale,
  StatusTone,
  ValidationState,
  ValidationSummaryEntry,
  SurfaceBorder,
  SurfaceTone,
  TabActivationMode,
  TabDefinition,
  TabItem,
  TabStripItem,
  TabVariant,
  ToggleGroupOption,
  TriStateValue,
  InputValidationStatus,
  InputValidator,
  ValidationResult,
  ZonedDateTimeValue,
  // Shared types
  ActiveSort,
  BreadcrumbItem,
  BulkAction,
  CardVariant,
  FilterClause,
  FilterCombinator,
  FilterExpression,
  FilterFieldDefinition,
  FilterFieldKind,
  FilterOperand,
  FilterOperandKind,
  FilterOperatorDefinition,
  FilterOption,
  AgentChatAttachment,
  AgentChatStatus,
  ModelAxisKind,
  ModelAxisOption,
  ModelAxisValue,
  ModelCapabilityAxis,
  ModelOption,
  ModelSelection,
  RefKind,
  RefOption,
  OrderByField,
  OrderByFieldDefinition,
  OrderByTriggerVariant,
  OrderByValue,
  SortField,
  SortDirection,
  SplitOrientation,
  SplitToggleVisibility,
  ThemeOption,
  ThemeSwatch,
  // Composite types
  AspectRatio,
  BlockType,
  BlockEditorMode,
  BlockTypeDefinition,
  BlockTypeGroup,
  BlockTypeItems,
  BrowseState,
  CardRadioItem,
  CardToggleItem,
  CenterRegionSnapshot,
  CommandActionItem,
  DiscoveryState,
  DockCollapsedPosture,
  DockEdge,
  DockEmphasis,
  DockRegionSnapshot,
  DockSizing,
  EmbedMeta,
  EmbedParseResult,
  DrillDownConfig,
  DrillDownContext,
  DrillDownItem,
  DrillDownItemsFn,
  DrillDownLevel,
  DrillDownSearchFn,
  EditorBlock,
  EmptyStateVariant,
  AuditLogEntry,
  LogActionType,
  LogActor,
  LogEntry,
  LogFilter,
  LogLevel,
  MediaKind,
  MediaPickerItem,
  MediaBrowseState,
  MediaDuplicateCheckResult,
  MediaUploadCompleteResult,
  MediaUploadDisplayStep,
  MediaUploadDuplicateResult,
  MediaWorkflowPageResponse,
  MediaWorkflowPaginationParams,
  MediaUploadInitResult,
  MediaUploadPlan,
  MediaUploadProgress,
  MediaUploadWorkflowResult,
  MediaUploadWorkflowStep,
  MediaState,
  MessageCenterItem,
  HistoryEntry,
  HistoryEntryPosition,
  HistoryBranch,
  HistoryStatus,
  DockExternalDragCancelContext,
  DockExternalDragCancelReason,
  DockExternalDragEndContext,
  DockExternalDragPreparation,
  DockExternalDragPrepareContext,
  DockExternalDragSource,
  DockExternalDragStartContext,
  DockExternalDropContext,
  DockExternalDropEligibilityContext,
  DockExternalDropTarget,
  PanelDragData,
  PanelTabItem,
  PanelVariant,
  ParsedEmbed,
  PickerFilterConfig,
  PickerFilterOption,
  PickerItem,
  PickerVariant,
  EditableListItem,
  SelectionMode,
  SidebarNavGroup,
  SidebarNavItem,
  TreeNode,
  StripRegionSnapshot,
  TableColumn,
  TableCellValue,
  TableRow,
  TableRowAction,
  TableFilters,
  TableFilterType,
  TablePagination,
  TableSortDirection,
  ToastItem,
  ToastHostPlacement,
  ToastHostStore,
  ToastHostStoreItem,
  ToastTone,
  WorkspaceLayoutSnapshot,
} from "./types";
export type { IconNodeElement, IconNodes, IconSet } from "./icon-registry";
export { default as AudioMeter } from "./AudioMeter.svelte";
export { default as AudioSwitch } from "./AudioSwitch.svelte";
export { default as DragNumberField } from "./DragNumberField.svelte";
export { default as EnvelopeEditor } from "./EnvelopeEditor.svelte";
export { default as Fader } from "./Fader.svelte";
export { default as GainReductionMeter } from "./GainReductionMeter.svelte";
export { default as Keyboard } from "./Keyboard.svelte";
export { default as Knob } from "./Knob.svelte";
export { default as ModMatrixGrid } from "./ModMatrixGrid.svelte";
export { default as ValueReadout } from "./ValueReadout.svelte";
export { default as WaveformDisplay } from "./WaveformDisplay.svelte";
export { default as XYPad } from "./XYPad.svelte";
