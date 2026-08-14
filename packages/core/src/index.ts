export type { TransitionResult, PartAttrs, AttrValue } from "./machine";

export { findNextEnabledIndex, firstEnabledIndex } from "./nav";

export {
  actionIcon,
  actionState,
  canSubmit,
  contextPercentage,
  resolveSubmitIntent,
  type AgentChatAttachment,
  type ComposerKeyEvent,
  type SubmitGate,
  type SubmitIntent,
  type SubmitIntentOptions,
} from "./agent-chat-input";

export {
  clauseLabel,
  cloneOperand,
  defaultOperatorsForKind,
  emptyOperand,
  findOperator,
  isClauseComplete,
  isOperandValid,
  resolveDefaultOperator,
  resolveOperators,
  type FilterClause,
  type FilterCombinator,
  type FilterExpression,
  type FilterFieldDefinition,
  type FilterFieldKind,
  type FilterOperand,
  type FilterOperandKind,
  type FilterOperatorDefinition,
  type FilterOption,
} from "./filter-builder";

export type {
  DrillDownConfig,
  DrillDownContext,
  DrillDownItem,
  DrillDownItemsFn,
  DrillDownLevel,
  DrillDownSearchFn,
  PickerFilterConfig,
  PickerFilterOption,
  PickerItem,
} from "./relation-picker";

export {
  SEGMENTED_AXIS_MAX_OPTIONS,
  applicableAxes,
  axesForModel,
  axisAccepts,
  axisControlKind,
  axisDefaultValue,
  axisSummaryFragment,
  axisValue,
  groupHeadingFor,
  initialSelection,
  modelLabel,
  resolveSelection,
  summaryText,
  type ModelAxisBinding,
  type ModelAxisKind,
  type ModelAxisOption,
  type ModelAxisRef,
  type ModelAxisValue,
  type ModelCapabilityAxis,
  type ModelImage,
  type ModelOption,
  type ModelSelection,
} from "./model-picker";

export {
  MODEL_CATALOGUE_FIXTURES,
  MODEL_CONNECTION_CARD_FIXTURES,
  MODEL_CONNECTION_PICKER_FIXTURES,
  filterModelConnectionOptions,
  groupModelConnectionOptions,
  hiddenModelCatalogueItems,
  modelCatalogueFocusAfterHide,
  modelCatalogueReorderAnnouncement,
  modelCatalogueStateCopy,
  modelCatalogueVisibilityAnnouncement,
  modelConnectionAvailabilityLabel,
  modelConnectionAvailabilityTone,
  modelConnectionOptionSelectable,
  modelConnectionPickerResultAnnouncement,
  modelConnectionPickerStateCopy,
  modelConnectionReadinessTone,
  modelConnectionSelectedSummary,
  modelConnectionSetupCanContinue,
  modelConnectionSetupCanSubmit,
  modelConnectionSetupSelectedOption,
  modelConnectionSetupTransition,
  requestModelCatalogueOrder,
  requestModelCatalogueVisibility,
  resolveModelConnectionPickerShellState,
  shownModelCatalogueItems,
  type ModelCatalogueItem,
  type ModelCatalogueState,
  type ModelCatalogueVisibilityChange,
  type ModelConnectionAvailability,
  type ModelConnectionBadge,
  type ModelConnectionBadgeTone,
  type ModelConnectionOption,
  type ModelConnectionOptionGroup,
  type ModelConnectionPickerShellState,
  type ModelConnectionPickerState,
  type ModelConnectionReadiness,
  type ModelConnectionSetupContext,
  type ModelConnectionSetupEffect,
  type ModelConnectionSetupEvent,
  type ModelConnectionSetupResult,
  type ModelConnectionSetupStage,
  type ModelConnectionStatusTone,
  type ModelConnectionSummary,
} from "./model-connection";

export { getFocusableElements, trapFocusKeydown } from "./dom/focus";

export {
  disclosureTransition,
  type DisclosureContext,
  type DisclosureEvent,
  type DisclosureEffect,
  type DisclosureResult,
} from "./disclosure";

export {
  buildVisiblePages,
  canRequestPage,
  type VisiblePage,
} from "./pagination";

export {
  validationStatusToState,
  parseNumberish,
  parseStep,
  clampNullable,
  slugify,
  isValidSlugFormat,
  type InputValidationStatus,
  type ValidationState,
} from "./input";

export {
  hoverTransition,
  type HoverState,
  type HoverContext,
  type HoverEvent,
  type HoverEffect,
  type HoverResult,
} from "./hover";

export {
  menuTransition,
  menuNavigableItems,
  menuListNavigate,
  menuListCanActivate,
  menuItemHasSubmenu,
  type MenuListItem,
  type MenuListMove,
  type MenuState,
  type MenuContext,
  type MenuEvent,
  type MenuEffect,
  type MenuResult,
} from "./menu";

export {
  modalTransition,
  type ModalState,
  type ModalContext,
  type ModalEvent,
  type ModalEffect,
  type ModalResult,
} from "./modal";
export {
  layerContains,
  registerDismissLayer,
  resolveDismiss,
  type DismissLayer,
} from "./dom/dismiss";
export { createInstanceId } from "./dom/id";
export {
  copyOverlayViewportRect,
  equalOverlaySurfaceGeometry,
  observeOverlaySurfaceGeometry,
  type OverlaySurfaceGeometry,
  type OverlaySurfaceGeometryChange,
  type OverlaySurfaceGeometryChangeHandler,
  type OverlaySurfaceGeometryObserver,
  type OverlaySurfaceGeometryObserverOptions,
  type OverlayViewportRect,
} from "./dom/overlay-geometry";
export {
  anchorElement,
  clipsOverflow,
  collectClipAncestors,
  collectScrollParents,
  intersectClip,
  isAnchorClipped,
  isPointAnchorClipped,
  observeAnchorMovement,
  pointAnchor,
  resolveClipRect,
  resolveLayerZIndex,
  resolvePortalTarget,
  viewportClipRect,
  type AnchorObservation,
  type AnchorTarget,
  type AnchorViewport,
  type ClipRect,
  type VirtualAnchor,
} from "./dom/anchor";

export {
  checkboxTransition,
  checkboxState,
  checkboxParts,
  type CheckboxContext,
  type CheckboxEvent,
  type CheckboxEffect,
  type CheckboxResult,
  type CheckboxPartProps,
  type CheckboxParts,
} from "./checkbox";

export {
  popoverTransition,
  popoverParts,
  type PopoverState,
  type PopoverInitialFocus,
  type PopoverContext,
  type PopoverEvent,
  type PopoverEffect,
  type PopoverResult,
  type PopoverPartProps,
  type PopoverParts,
} from "./popover";

export {
  switchTransition,
  switchState,
  type SwitchContext,
  type SwitchEvent,
  type SwitchEffect,
  type SwitchResult,
} from "./switch";

export {
  singleSelectTransition,
  type SelectOption,
  type SingleSelectContext,
  type SingleSelectEvent,
  type SingleSelectEffect,
  type SingleSelectResult,
} from "./single-select";

export {
  editLabelTransition,
  listReorderKeyIntent,
  type EditLabelState,
  type EditLabelContext,
  type EditLabelEvent,
  type EditLabelEffect,
  type EditLabelResult,
  type ListReorderIntent,
} from "./edit";

export {
  sanitizeCodeValue,
  clampCodePosition,
  codeSelectionRange,
  codeSlotSelection,
  codeInsertReplacement,
  codeGroupEndIndices,
} from "./code-input";

export {
  mergeTokens,
  splitTokenInput,
  tokenBackspaceRemoves,
  type TokenSplit,
} from "./token";

export {
  resolveRatingStep,
  roundRatingToStep,
  clampRatingDisplayValue,
  normalizeRatingValue,
  trimRatingFraction,
  ratingFillRatio,
  ratingPointerValue,
  ratingSelectValue,
  ratingKeyboardStep,
} from "./rating";

export {
  resizeAxisPosition,
  resizeDragDelta,
  resizeKeydownStep,
  type ResizeOrientation,
} from "./resize";

export {
  canDecidePlan,
  decidePlan,
  planStatusLabel,
  planRecordSummary,
  type AgentPlanStatus,
  type AgentPlanSettledStatus,
  type AgentPlanDecision,
} from "./agent-plan";

export {
  isTerminalSubagentStatus,
  subagentStatusLabel,
  subagentStatusSpins,
  type AgentSubagentStatus,
  type AgentSubagentItem,
} from "./agent-subagent";

export {
  submitsOnSelect,
  toggleQuestionSelection,
  resolveQuestionAnswer,
  declineQuestion,
  canSubmitQuestion,
  questionProgress,
  showsQuestionProgress,
  nextQuestionIndex,
  questionBatchComplete,
  answeredQuestionSummary,
  isChosenOption,
  type AgentQuestionOption,
  type AgentQuestionItem,
  type AgentQuestionOutcome,
  type AgentQuestionAnswer,
  type QuestionProgressState,
  type QuestionProgress,
  type AnsweredQuestion,
} from "./agent-question";

export {
  groupTranscriptItems,
  toolRunLeadCall,
  toolRunHiddenCount,
  toolRunStatus,
  changedFilesTotals,
  buildChangedFileTree,
  changedFileScopes,
  transcriptWindow,
  isPinnedToBottom,
  type TranscriptRole,
  type ToolCallStatus,
  type TranscriptMessage,
  type TranscriptToolCall,
  type ChangedFile,
  type ChangedFileStatus,
  type TranscriptChangedFiles,
  type TranscriptActivity,
  type TranscriptAnsweredQuestion,
  type TranscriptDecidedPlan,
  type TranscriptSubagentGroup,
  type TranscriptItem,
  type TranscriptToolRun,
  type TranscriptBlock,
  type ChangedFilesTotals,
  type ChangedFileNode,
  type TranscriptWindow,
} from "./agent-transcript";

export {
  blocksFromMarked,
  inlineFromMarked,
  markdownPlainText,
  type MdBlock,
  type MdInline,
  type MarkedToken,
} from "./markdown-blocks";

export {
  flattenVisibleTreeRows,
  findTreeNode,
  isTreeBranch,
  treeCheckableUnder,
  treeCheckState,
  treeToggleCheck,
  treeRangeSelection,
  treeSiblingReorderTarget,
  treeKeydownIntent,
  treeVirtualWindow,
  type TreeNodeLike,
  type TreeRow,
  type TreeCheckState,
  type TreeKeyIntent,
  type TreeVirtualWindow,
} from "./tree";

export {
  toggleGroupTransition,
  toggleGroupIsSelected,
  type ToggleGroupValue,
  type ToggleGroupContext,
  type ToggleGroupEvent,
  type ToggleGroupEffect,
  type ToggleGroupResult,
} from "./toggle-group";

export {
  historyCenterTransition,
  historyCenterKeydownEvent,
  historyCenterDefaultContext,
  historyCenterVisibleRows,
  historyCenterJoinPages,
  historyCenterForkCount,
  historyCenterRejectionMessage,
  type HistoryEntry,
  type HistoryEntryPosition,
  type HistoryPathPage,
  type HistoryContinuation,
  type HistoryCenterState,
  type HistoryCenterContext,
  type HistoryCenterEvent,
  type HistoryCenterEffect,
  type HistoryCenterResult,
  type HistoryCenterRow,
  type HistoryCenterRowId,
  type HistoryCenterOpenFork,
  type HistoryCenterRejectionCode,
} from "./history-center";

export {
  installManagerLabel,
  updateDownloadLabel,
  updateErrorMessage,
  updateRejectionMessage,
  updateRejectionRetry,
  updateStatusView,
  type Channel,
  type DeferralCause,
  type InstallManager,
  type OfferReason,
  type UpdateAheadOfChannel,
  type UpdateAvailabilityProjection,
  type UpdateControllerStatus,
  type UpdateDeferral,
  type UpdatePresence,
  type UpdateProgressProjection,
  type UpdateRejectionCode,
  type UpdateStatusAction,
  type UpdateStatusInput,
  type UpdateStatusNotice,
  type UpdateStatusTone,
  type UpdateStatusView,
  type UpdateStatusViewState,
} from "./update";

export {
  LICENCE_ACCOUNT_FAILED_MESSAGE,
  LICENCE_FILE_REQUIRED_MESSAGE,
  LICENCE_FILE_UNREADABLE_MESSAGE,
  LICENCE_KEY_TOO_SHORT_MESSAGE,
  LICENCE_KEY_TYPO_MESSAGE,
  LICENCE_KEY_UNREADABLE_MESSAGE,
  LICENCE_MIRROR_FIELDS,
  LICENCE_MIRROR_VARIANT_FIELDS,
  LICENCE_RELEASE_CONFIRM_TITLE,
  LICENCE_THIS_MACHINE,
  LICENCE_UNNAMED_MACHINE,
  licenceFileContentsBase64,
  licenceKeyProblemMessage,
  licenceMachineLabel,
  licenceOtherSeats,
  licenceSeatRows,
  licenceStatusView,
  licenceTimestampMilliseconds,
  resolveLicenceSubmit,
  type LicenceAccountTokenProvider,
  type LicenceActivationMode,
  type LicenceActivationRoute,
  type LicenceAttention,
  type LicenceCoverageRow,
  type LicenceCredential,
  type LicenceKeyCodeInputOptions,
  type LicenceKeyFormat,
  type LicenceKeyProblem,
  type LicenceKeyResult,
  type LicenceSeat,
  type LicenceSeatRow,
  type LicenceStatusIndicator,
  type LicenceStatusInput,
  type LicenceStatusState,
  type LicenceStatusTone,
  type LicenceStatusView,
  type LicenceSubmitDraft,
  type LicenceSubmitResolution,
  type LicenceTimedText,
  type LicenceTrustBasis,
  type LicenceTrustRow,
  type LicenceUsability,
} from "./licence";

export {
  sliderTransition,
  rangeSliderTransition,
  normalizeSliderValue,
  normalizeRangeValue,
  safeSliderMax,
  clampValue,
  snapToStep,
  createSliderControlContext,
  sliderControlTransition,
  sliderVisualState,
  createRangeSliderControlContext,
  rangeSliderControlTransition,
  rangeSliderVisualState,
  type SliderVariant,
  type SliderPolarity,
  type SliderControlContext,
  type SliderControlEvent,
  type SliderVisualState,
  type RangeSliderControlContext,
  type RangeSliderControlEvent,
  type RangeSliderVisualState,
  type SliderContext,
  type SliderEvent,
  type SliderEffect,
  type SliderResult,
  type RangeSliderContext,
  type RangeSliderEvent,
  type RangeSliderEffect,
  type RangeSliderResult,
} from "./slider";

export {
  resolveOverlayPosition,
  type OverlaySide,
  type OverlayPlacement,
  type RectLike,
  type ViewportSize,
  type OverlayPosition,
} from "./position";

export * from "./date";

export * from "./color";

export * from "./embed-input";

export * from "./file-upload";

export {
  createReorderState,
  handleDragStart,
  handleDragOver,
  handleDrop,
  type ReorderState,
} from "./tabs-reorder";

export {
  durationTotalSeconds,
  adjustDurationSegment,
  setDurationSegment,
  padDurationSegment,
  type DurationValue,
  type DurationSegment,
} from "./duration";

export {
  isSelectOptionDisabled,
  flattenSelectOptions,
  filterSelectOptions,
  filterSelectGroups,
  selectOpenHighlightIndex,
  type SelectOptionLike,
  type SelectGroupLike,
} from "./select";

export {
  resolveToastTone,
  normalizeToast,
  isToastSticky,
  reconcileToastTimers,
  type ToastTone,
  type ToastHostInput,
  type NormalizedToast,
  type ToastTimerPlan,
} from "./toast";

export {
  tabsTransition,
  tabsKeydownEvent,
  tabsTooltipTransition,
  resolveTabsValue,
  applyReorder,
  tabsListParts,
  tabsTabParts,
  tabsPanelParts,
  tabsTabId,
  tabsPanelId,
  type TabsItem,
  type TabsState,
  type TabsContext,
  type TabsEvent,
  type TabsEffect,
  type TabsResult,
  type TabsTooltipState,
  type TabsTooltipEvent,
  type TabsTooltipEffect,
  type TabsPartProps,
} from "./tabs";

export {
  createDockExternalDragController,
  dockPanelDragSession,
  type DockEdgeLike,
  type DockPanelLike,
  type DockPanelDragSessionEntry,
  type DockExternalDragAccessors,
  type DockExternalDragCancelContext,
  type DockExternalDragCancelReason,
  type DockExternalDragController,
  type DockExternalDragEndContext,
  type DockExternalDragPreparation,
  type DockExternalDragPrepareContext,
  type DockExternalDragSource,
  type DockExternalDragStartContext,
  type DockExternalDropContext,
  type DockExternalDropEligibilityContext,
  type DockExternalDropTarget,
} from "./dock-external-drag";
export * from "./audio/index";
