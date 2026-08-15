import type { ComponentType } from "react";

import { SceneSpecimen } from "./SceneSpecimen";
import { AccordionSpecimen } from "./specimens/AccordionSpecimen";
import { ActionDiscoveryPanelSpecimen } from "./specimens/ActionDiscoveryPanelSpecimen";
import { AgentQuestionRecordSpecimen } from "./specimens/AgentQuestionRecordSpecimen";
import { AgentQuestionSpecimen } from "./specimens/AgentQuestionSpecimen";
import { AgentSubagentSpecimen } from "./specimens/AgentSubagentSpecimen";
import { AlertDialogSpecimen } from "./specimens/AlertDialogSpecimen";
import { AppHeaderSpecimen } from "./specimens/AppHeaderSpecimen";
import { MessageCenterSpecimen } from "./specimens/MessageCenterSpecimen";
import { HistoryCenterSpecimen } from "./specimens/HistoryCenterSpecimen";
import { AudioPlayerSpecimen } from "./specimens/AudioPlayerSpecimen";
import { AudioMeterSpecimen } from "./specimens/AudioMeterSpecimen";
import { MeterSurfaceSpecimen } from "./specimens/MeterSurfaceSpecimen";
import { AudioSwitchSpecimen } from "./specimens/AudioSwitchSpecimen";

import { BlockEditorSpecimen } from "./specimens/BlockEditorSpecimen";
import { BoxSpecimen } from "./specimens/BoxSpecimen";
import { BreadcrumbsSpecimen } from "./specimens/BreadcrumbsSpecimen";
import { BulkActionBarSpecimen } from "./specimens/BulkActionBarSpecimen";
import { ButtonSpecimen } from "./specimens/ButtonSpecimen";
import { CalendarSpecimen } from "./specimens/CalendarSpecimen";

import { RemediationBannerSpecimen } from "./specimens/RemediationBannerSpecimen";
import { CardRadioGroupSpecimen } from "./specimens/CardRadioGroupSpecimen";
import { CardSpecimen } from "./specimens/CardSpecimen";
import { CardToggleGroupSpecimen } from "./specimens/CardToggleGroupSpecimen";
import { CheckboxSpecimen } from "./specimens/CheckboxSpecimen";
import { CodeInputSpecimen } from "./specimens/CodeInputSpecimen";
import { CodeSpecimen } from "./specimens/CodeSpecimen";
import { CollapseToggleSpecimen } from "./specimens/CollapseToggleSpecimen";
import { CollapsibleSpecimen } from "./specimens/CollapsibleSpecimen";
import { ColorPickerSpecimen } from "./specimens/ColorPickerSpecimen";
import { CommandPaletteSpecimen } from "./specimens/CommandPaletteSpecimen";
import { ConfirmActionSpecimen } from "./specimens/ConfirmActionSpecimen";
import { ContextMenuSpecimen } from "./specimens/ContextMenuSpecimen";
import { DataTableSpecimen } from "./specimens/DataTableSpecimen";
import { DatePickerSpecimen } from "./specimens/DatePickerSpecimen";
import { DateRangePickerSpecimen } from "./specimens/DateRangePickerSpecimen";
import { DateTimePickerSpecimen } from "./specimens/DateTimePickerSpecimen";
import { DateTimeRangePickerSpecimen } from "./specimens/DateTimeRangePickerSpecimen";
import { DateTimeZonePickerSpecimen } from "./specimens/DateTimeZonePickerSpecimen";
import { DebugDialogSpecimen } from "./specimens/DebugDialogSpecimen";
import { DetailItemSpecimen } from "./specimens/DetailItemSpecimen";
import { DetailSectionGroupSpecimen } from "./specimens/DetailSectionGroupSpecimen";
import { DetailSectionSpecimen } from "./specimens/DetailSectionSpecimen";
import { DetailShellSpecimen } from "./specimens/DetailShellSpecimen";
import { DialogSpecimen } from "./specimens/DialogSpecimen";
import { DockRegionSpecimen } from "./specimens/DockRegionSpecimen";
import { DrawerSpecimen } from "./specimens/DrawerSpecimen";
import { DragNumberFieldSpecimen } from "./specimens/DragNumberFieldSpecimen";
import { DurationInputSpecimen } from "./specimens/DurationInputSpecimen";
import { EditableLabelSpecimen } from "./specimens/EditableLabelSpecimen";
import { EditableListSpecimen } from "./specimens/EditableListSpecimen";
import { EmbedInputSpecimen } from "./specimens/EmbedInputSpecimen";
import { EmbedPreviewSpecimen } from "./specimens/EmbedPreviewSpecimen";

import { EnvelopeEditorSpecimen } from "./specimens/EnvelopeEditorSpecimen";
import { ErrorBoundarySpecimen } from "./specimens/ErrorBoundarySpecimen";
import { EyebrowSpecimen } from "./specimens/EyebrowSpecimen";
import { FieldSetSpecimen } from "./specimens/FieldSetSpecimen";
import { FieldSpecimen } from "./specimens/FieldSpecimen";
import { FileUploadSpecimen } from "./specimens/FileUploadSpecimen";
import { FaderSpecimen } from "./specimens/FaderSpecimen";
import { AgentChatInputSpecimen } from "./specimens/AgentChatInputSpecimen";
import { AgentTranscriptSpecimen } from "./specimens/AgentTranscriptSpecimen";
import { FilterBuilderSpecimen } from "./specimens/FilterBuilderSpecimen";
import { FilterToolbarSpecimen } from "./specimens/FilterToolbarSpecimen";
import { FormActionsSpecimen } from "./specimens/FormActionsSpecimen";
import { FormDialogSpecimen } from "./specimens/FormDialogSpecimen";
import { FormLayoutSpecimen } from "./specimens/FormLayoutSpecimen";
import { GridSpecimen } from "./specimens/GridSpecimen";
import { GainReductionMeterSpecimen } from "./specimens/GainReductionMeterSpecimen";
import { HoverCardSpecimen } from "./specimens/HoverCardSpecimen";
import { IconButtonSpecimen } from "./specimens/IconButtonSpecimen";
import { IconProviderSpecimen } from "./specimens/IconProviderSpecimen";
import { IconSpecimen } from "./specimens/IconSpecimen";
import { InlineListSectionSpecimen } from "./specimens/InlineListSectionSpecimen";
import { KnobSpecimen } from "./specimens/KnobSpecimen";
import { ListCardSpecimen } from "./specimens/ListCardSpecimen";
import { ListContainerSpecimen } from "./specimens/ListContainerSpecimen";
import { ListGridSpecimen } from "./specimens/ListGridSpecimen";
import { LicenceActivationSpecimen } from "./specimens/LicenceActivationSpecimen";
import { LicenceSeatsSpecimen } from "./specimens/LicenceSeatsSpecimen";
import { LicenceStatusSpecimen } from "./specimens/LicenceStatusSpecimen";
import { ModelCatalogueEditorSpecimen } from "./specimens/ModelCatalogueEditorSpecimen";
import { ModelConnectionCardSpecimen } from "./specimens/ModelConnectionCardSpecimen";
import { ModelConnectionPickerSpecimen } from "./specimens/ModelConnectionPickerSpecimen";
import { ModelConnectionSetupSpecimen } from "./specimens/ModelConnectionSetupSpecimen";
import { LogListSpecimen } from "./specimens/LogListSpecimen";
import { MarkdownEditorSpecimen } from "./specimens/MarkdownEditorSpecimen";
import { MediaBrowsePanelSpecimen } from "./specimens/MediaBrowsePanelSpecimen";
import { MediaPickerSpecimen } from "./specimens/MediaPickerSpecimen";
import { MediaPreviewSpecimen } from "./specimens/MediaPreviewSpecimen";
import { MediaThumbnailSpecimen } from "./specimens/MediaThumbnailSpecimen";
import { MenubarSpecimen } from "./specimens/MenubarSpecimen";
import { MenuSpecimen } from "./specimens/MenuSpecimen";
import { MetaBarSpecimen } from "./specimens/MetaBarSpecimen";
import { MeterSpecimen } from "./specimens/MeterSpecimen";
import { ModelPickerSpecimen } from "./specimens/ModelPickerSpecimen";
import { RefSelectSpecimen } from "./specimens/RefSelectSpecimen";
import { MetricTileSpecimen } from "./specimens/MetricTileSpecimen";
import { StateTileSpecimen } from "./specimens/StateTileSpecimen";
import { ValidationSummarySpecimen } from "./specimens/ValidationSummarySpecimen";
import { NavCardSpecimen } from "./specimens/NavCardSpecimen";
import { NavigationMenuSpecimen } from "./specimens/NavigationMenuSpecimen";
import { NumberInputSpecimen } from "./specimens/NumberInputSpecimen";
import { OrderBySpecimen } from "./specimens/OrderBySpecimen";
import { PageHeaderSpecimen } from "./specimens/PageHeaderSpecimen";
import { PageLoadingSpecimen } from "./specimens/PageLoadingSpecimen";
import { PaginationSpecimen } from "./specimens/PaginationSpecimen";
import { PaginationSummarySpecimen } from "./specimens/PaginationSummarySpecimen";
import { PasswordRequirementsSpecimen } from "./specimens/PasswordRequirementsSpecimen";
import { PickerShellSpecimen } from "./specimens/PickerShellSpecimen";

import { PopoverSpecimen } from "./specimens/PopoverSpecimen";
import { ProgressSpecimen } from "./specimens/ProgressSpecimen";
import { RadioGroupSpecimen } from "./specimens/RadioGroupSpecimen";
import { RadioSpecimen } from "./specimens/RadioSpecimen";
import { RangeSliderSpecimen } from "./specimens/RangeSliderSpecimen";
import { RatingSpecimen } from "./specimens/RatingSpecimen";
import { RegionSpecimen } from "./specimens/RegionSpecimen";
import { RelationPickerSpecimen } from "./specimens/RelationPickerSpecimen";
import { ResizeHandleSpecimen } from "./specimens/ResizeHandleSpecimen";
import { ScrollShellSpecimen } from "./specimens/ScrollShellSpecimen";
import { SegmentedControlSpecimen } from "./specimens/SegmentedControlSpecimen";
import { SelectionSummarySpecimen } from "./specimens/SelectionSummarySpecimen";
import { SelectSpecimen } from "./specimens/SelectSpecimen";
import { SeparatorSpecimen } from "./specimens/SeparatorSpecimen";
import { SettingsShellSpecimen } from "./specimens/SettingsShellSpecimen";
import { SidebarNavSpecimen } from "./specimens/SidebarNavSpecimen";
import { SkeletonSpecimen } from "./specimens/SkeletonSpecimen";
import { SliderSpecimen } from "./specimens/SliderSpecimen";
import { SpacerSpecimen } from "./specimens/SpacerSpecimen";

import { SplitButtonSpecimen } from "./specimens/SplitButtonSpecimen";
import { SplitViewSpecimen } from "./specimens/SplitViewSpecimen";
import { StackSpecimen } from "./specimens/StackSpecimen";
import { StatusBarSpecimen } from "./specimens/StatusBarSpecimen";
import { StatusIndicatorSpecimen } from "./specimens/StatusIndicatorSpecimen";
import { StepperSpecimen } from "./specimens/StepperSpecimen";
import { SurfaceSpecimen } from "./specimens/SurfaceSpecimen";
import { SwitchSpecimen } from "./specimens/SwitchSpecimen";
import { TableSpecimen } from "./specimens/TableSpecimen";
import { TabsSpecimen } from "./specimens/TabsSpecimen";
import { TextInputSpecimen } from "./specimens/TextInputSpecimen";
import { TextLinkSpecimen } from "./specimens/TextLinkSpecimen";
import { TextSpecimen } from "./specimens/TextSpecimen";
import { TimeAgoSpecimen } from "./specimens/TimeAgoSpecimen";
import { TimeInputSpecimen } from "./specimens/TimeInputSpecimen";
import { ThemeSelectSpecimen } from "./specimens/ThemeSelectSpecimen";
import { TimeZoneSelectSpecimen } from "./specimens/TimeZoneSelectSpecimen";
import { ToastHostSpecimen } from "./specimens/ToastHostSpecimen";
import { ToastStackSpecimen } from "./specimens/ToastStackSpecimen";
import { ToggleGroupSpecimen } from "./specimens/ToggleGroupSpecimen";
import { TokenInputSpecimen } from "./specimens/TokenInputSpecimen";
import { ToolbarSpecimen } from "./specimens/ToolbarSpecimen";
import { TooltipSpecimen } from "./specimens/TooltipSpecimen";
import { TreeSpecimen } from "./specimens/TreeSpecimen";
import { ValueReadoutSpecimen } from "./specimens/ValueReadoutSpecimen";
import { TriStateSwitchSpecimen } from "./specimens/TriStateSwitchSpecimen";
import { UiPresentationProviderSpecimen } from "./specimens/UiPresentationProviderSpecimen";
import { VideoPlayerSpecimen } from "./specimens/VideoPlayerSpecimen";
import { UpdateCenterSpecimen } from "./specimens/UpdateCenterSpecimen";
import { UpdateStatusSpecimen } from "./specimens/UpdateStatusSpecimen";
import { XYPadSpecimen } from "./specimens/XYPadSpecimen";
import { KeyboardSpecimen } from "./specimens/KeyboardSpecimen";
import { ModMatrixGridSpecimen } from "./specimens/ModMatrixGridSpecimen";
import { WaveformDisplaySpecimen } from "./specimens/WaveformDisplaySpecimen";
import { allComponents } from "./registry";

/**
 * Slug -> specimen component, mirroring the Svelte preview's specimens/registry.ts.
 * Slugs match the component-registry so the two galleries route identically.
 */
export const specimenMap: Record<string, ComponentType<{ slug?: string }>> = {
  "accordion": AccordionSpecimen,
  "action-discovery-panel": ActionDiscoveryPanelSpecimen,
  "agent-question": AgentQuestionSpecimen,
  "agent-question-record": AgentQuestionRecordSpecimen,
  "agent-subagent": AgentSubagentSpecimen,
  "alert-dialog": AlertDialogSpecimen,
  "app-header": AppHeaderSpecimen,
  "message-center": MessageCenterSpecimen,
  "history-center": HistoryCenterSpecimen,
  "audio-player": AudioPlayerSpecimen,
  "audio-meter": AudioMeterSpecimen,
  "meter-surface": MeterSurfaceSpecimen,
  "audio-switch": AudioSwitchSpecimen,
  "avatar": SceneSpecimen,
  "block-editor": BlockEditorSpecimen,
  "box": BoxSpecimen,
  "breadcrumbs": BreadcrumbsSpecimen,
  "bulk-action-bar": BulkActionBarSpecimen,
  "button": ButtonSpecimen,
  "calendar": CalendarSpecimen,
  "callout": SceneSpecimen,
  "remediation-banner": RemediationBannerSpecimen,
  "card": CardSpecimen,
  "card-radio-group": CardRadioGroupSpecimen,
  "card-toggle-group": CardToggleGroupSpecimen,
  "checkbox": CheckboxSpecimen,
  "code": CodeSpecimen,
  "code-input": CodeInputSpecimen,
  "collapse-toggle": CollapseToggleSpecimen,
  "collapsible": CollapsibleSpecimen,
  "color-picker": ColorPickerSpecimen,
  "command-palette": CommandPaletteSpecimen,
  "confirm-action": ConfirmActionSpecimen,
  "context-menu": ContextMenuSpecimen,
  "data-table": DataTableSpecimen,
  "date-picker": DatePickerSpecimen,
  "date-range-picker": DateRangePickerSpecimen,
  "date-time-picker": DateTimePickerSpecimen,
  "date-time-range-picker": DateTimeRangePickerSpecimen,
  "date-time-zone-picker": DateTimeZonePickerSpecimen,
  "debug-dialog": DebugDialogSpecimen,
  "detail-item": DetailItemSpecimen,
  "detail-section": DetailSectionSpecimen,
  "detail-section-group": DetailSectionGroupSpecimen,
  "detail-shell": DetailShellSpecimen,
  "dialog": DialogSpecimen,
  "dock-region": DockRegionSpecimen,
  "drawer": DrawerSpecimen,
  "drag-number-field": DragNumberFieldSpecimen,
  "duration-input": DurationInputSpecimen,
  "editable-label": EditableLabelSpecimen,
  "editable-list": EditableListSpecimen,
  "embed-input": EmbedInputSpecimen,
  "embed-preview": EmbedPreviewSpecimen,
  "empty-state": SceneSpecimen,
  "envelope-editor": EnvelopeEditorSpecimen,
  "error-boundary": ErrorBoundarySpecimen,
  "eyebrow": EyebrowSpecimen,
  "field": FieldSpecimen,
  "field-set": FieldSetSpecimen,
  "file-upload": FileUploadSpecimen,
  "fader": FaderSpecimen,
  "agent-chat-input": AgentChatInputSpecimen,
  "agent-transcript": AgentTranscriptSpecimen,
  "filter-builder": FilterBuilderSpecimen,
  "filter-toolbar": FilterToolbarSpecimen,
  "form-actions": FormActionsSpecimen,
  "form-dialog": FormDialogSpecimen,
  "form-layout": FormLayoutSpecimen,
  "grid": GridSpecimen,
  "gain-reduction-meter": GainReductionMeterSpecimen,
  "keyboard": KeyboardSpecimen,
  "hover-card": HoverCardSpecimen,
  "icon": IconSpecimen,
  "icon-button": IconButtonSpecimen,
  "icon-provider": IconProviderSpecimen,
  "inline-list-section": InlineListSectionSpecimen,
  "knob": KnobSpecimen,
  "mod-matrix-grid": ModMatrixGridSpecimen,
  "list-card": ListCardSpecimen,
  "list-card-counter": ListCardSpecimen,
  "list-container": ListContainerSpecimen,
  "list-grid": ListGridSpecimen,
  "licence-activation": LicenceActivationSpecimen,
  "licence-seats": LicenceSeatsSpecimen,
  "licence-status": LicenceStatusSpecimen,
  "model-catalogue-editor": ModelCatalogueEditorSpecimen,
  "model-connection-card": ModelConnectionCardSpecimen,
  "model-connection-picker": ModelConnectionPickerSpecimen,
  "model-connection-setup": ModelConnectionSetupSpecimen,
  "log-list": LogListSpecimen,
  "markdown-editor": MarkdownEditorSpecimen,
  "media-browse-panel": MediaBrowsePanelSpecimen,
  "media-picker": MediaPickerSpecimen,
  "media-preview": MediaPreviewSpecimen,
  "media-thumbnail": MediaThumbnailSpecimen,
  "menu": MenuSpecimen,
  "menubar": MenubarSpecimen,
  "meta-bar": MetaBarSpecimen,
  "meta-item": MetaBarSpecimen,
  "meter": MeterSpecimen,
  "model-picker": ModelPickerSpecimen,
  "ref-select": RefSelectSpecimen,
  "metric-tile": MetricTileSpecimen,
  "state-tile": StateTileSpecimen,
  "validation-summary": ValidationSummarySpecimen,
  "nav-card": NavCardSpecimen,
  "navigation-menu": NavigationMenuSpecimen,
  "number-input": NumberInputSpecimen,
  "order-by": OrderBySpecimen,
  "page-header": PageHeaderSpecimen,
  "page-loading": PageLoadingSpecimen,
  "pagination": PaginationSpecimen,
  "pagination-summary": PaginationSummarySpecimen,
  "password-requirements": PasswordRequirementsSpecimen,
  "picker-shell": PickerShellSpecimen,
  "pill": SceneSpecimen,
  "popover": PopoverSpecimen,
  "progress": ProgressSpecimen,
  "radio": RadioSpecimen,
  "radio-group": RadioGroupSpecimen,
  "range-slider": RangeSliderSpecimen,
  "rating": RatingSpecimen,
  "region": RegionSpecimen,
  "relation-picker": RelationPickerSpecimen,
  "resize-handle": ResizeHandleSpecimen,
  "scroll-shell": ScrollShellSpecimen,
  "segmented-control": SegmentedControlSpecimen,
  "select": SelectSpecimen,
  "selection-summary": SelectionSummarySpecimen,
  "separator": SeparatorSpecimen,
  "settings-shell": SettingsShellSpecimen,
  "sidebar-nav": SidebarNavSpecimen,
  "skeleton": SkeletonSpecimen,
  "slider": SliderSpecimen,
  "spacer": SpacerSpecimen,
  "spinner": SceneSpecimen,
  "split-button": SplitButtonSpecimen,
  "split-view": SplitViewSpecimen,
  "stack": StackSpecimen,
  "status-bar": StatusBarSpecimen,
  "status-indicator": StatusIndicatorSpecimen,
  "stepper": StepperSpecimen,
  "surface": SurfaceSpecimen,
  "switch": SwitchSpecimen,
  "table": TableSpecimen,
  "tabs": TabsSpecimen,
  "text": TextSpecimen,
  "text-input": TextInputSpecimen,
  "text-link": TextLinkSpecimen,
  "time-ago": TimeAgoSpecimen,
  "time-input": TimeInputSpecimen,
  "theme-select": ThemeSelectSpecimen,
  "time-zone-select": TimeZoneSelectSpecimen,
  "toast-host": ToastHostSpecimen,
  "toast-stack": ToastStackSpecimen,
  "toggle-group": ToggleGroupSpecimen,
  "token-input": TokenInputSpecimen,
  "toolbar": ToolbarSpecimen,
  "tooltip": TooltipSpecimen,
  "tree": TreeSpecimen,
  "tri-state-switch": TriStateSwitchSpecimen,
  "ui-presentation-provider": UiPresentationProviderSpecimen,
  "value-readout": ValueReadoutSpecimen,
  "video-player": VideoPlayerSpecimen,
  "waveform-display": WaveformDisplaySpecimen,
  "update-center": UpdateCenterSpecimen,
  "update-status": UpdateStatusSpecimen,
  "xy-pad": XYPadSpecimen,
};

const missingSpecimens = allComponents
  .filter((component) => component.hasSpecimen && specimenMap[component.slug] === undefined)
  .map((component) => component.slug);
const unknownSpecimens = Object.keys(specimenMap).filter(
  (slug) => !allComponents.some((component) => component.slug === slug),
);

if (missingSpecimens.length > 0 || unknownSpecimens.length > 0) {
  throw new Error(
    [
      missingSpecimens.length > 0 ? `Missing React specimens: ${missingSpecimens.join(", ")}` : "",
      unknownSpecimens.length > 0 ? `Unknown React specimens: ${unknownSpecimens.join(", ")}` : "",
    ]
      .filter(Boolean)
      .join("\n"),
  );
}
