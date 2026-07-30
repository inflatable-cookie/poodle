export type { TransitionResult, PartAttrs, AttrValue } from "./machine";

export { findNextEnabledIndex, firstEnabledIndex } from "./nav";

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
  sliderTransition,
  rangeSliderTransition,
  normalizeSliderValue,
  normalizeRangeValue,
  safeSliderMax,
  clampValue,
  snapToStep,
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
