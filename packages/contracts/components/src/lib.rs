mod accordion;
mod alert_dialog;
mod avatar;
mod badge;
mod banner;
mod history_center;
mod r#box;
mod breadcrumbs;
mod bulk_action_bar;
mod button;
mod calendar;
mod call_out;
mod card;
mod card_toggle_group;
mod checkbox;
mod code;
mod code_input;
mod collapse_toggle;
mod collapsible;
mod color_picker;
mod context_menu;
mod date_picker;
mod date_range_picker;
mod date_time_picker;
mod date_time_range_picker;
mod date_time_zone_picker;
mod detail_item;
mod dialog;
mod drawer;
mod duration_input;
mod editable_label;
mod eyebrow;
mod field;
mod field_set;
mod file_upload;
mod filter_builder;
mod form_actions;
mod form_dialog;
mod form_layout;
mod grid;
mod hover_card;
mod icon;
mod icon_button;
mod icon_provider;
mod list_card;
mod list_card_counter;
mod list_grid;
mod menu;
mod menubar;
mod meta_bar;
mod meta_item;
mod meter;
mod nav_card;
mod navigation_menu;
mod number_input;
mod order_by;
mod pagination;
mod password_requirements;
mod pill;
mod popover;
mod progress;
mod radio_group;
mod radio;
mod range_slider;
mod rating;
mod region;
mod resize_handle;
mod scroll_shell;
mod segmented_control;
mod select;
mod separator;
mod skeleton;
mod slider;
mod spacer;
mod spinner;
mod split_button;
mod stack;
mod status_indicator;
mod stepper;
mod surface;
mod switch;
mod tab_strip;
mod table;
mod tabs;
mod text;
mod text_input;
mod text_link;
mod theme_select;
mod time_ago;
mod time_field;
mod time_zone_select;
mod toggle_group;
mod token_input;
mod toolbar;
mod tooltip;
mod tri_state_switch;
mod types;
mod ui_presentation_provider;

// Composite modules
mod action_discovery_panel;
mod agent_chat_input;
mod agent_message;
mod agent_plan;
mod agent_plan_record;
mod agent_question;
mod agent_question_record;
mod agent_subagent;
mod agent_transcript;
mod app_header;
mod audio;
mod audio_player;
mod block_editor;
mod card_radio_group;
mod changed_files;
mod command_palette;
mod composite_types;
mod confirm_action;
mod data_table;
mod debug_dialog;
mod detail_section;
mod detail_section_group;
mod detail_shell;
mod dock_region;
mod editable_list;
mod embed_input;
mod embed_preview;
mod empty_state;
mod error_boundary;
mod filter_toolbar;
mod form_shell;
mod inline_list_section;
mod inline_remediation;
mod licence_activation;
mod licence_seats;
mod licence_status;
mod list_container;
mod log_list;
mod markdown_editor;
mod media_browse_panel;
mod media_picker;
mod media_preview;
mod media_thumbnail;
mod message_center;
mod metric_tile;
mod model_catalogue_editor;
mod model_connection_card;
mod model_connection_picker;
mod model_connection_setup;
mod model_picker;
mod page_header;
mod page_loading;
mod pagination_summary;
mod picker_shell;
mod ref_select;
mod relation_picker;
mod remediation_banner;
mod selection_summary;
mod settings_shell;
mod shell_status_bar;
mod sidebar_nav;
mod split_view;
mod state_tile;
mod toast_host;
mod toast_stack;
mod tool_call;
mod tool_call_group;
mod tree;
mod update_center;
mod update_status;
mod validation_summary;
mod video_player;

pub use accordion::{AccordionSelectionMode, AccordionSpec};
pub use agent_chat_input::{
    AgentChatAttachment, AgentChatInputSpec, AgentChatStatus, ComposerKey, ComposerKeyModifiers,
    SubmitIntent,
};
pub use agent_message::AgentMessageSpec;
pub use agent_plan::AgentPlanSpec;
pub use agent_plan_record::AgentPlanRecordSpec;
pub use agent_question::AgentQuestionSpec;
pub use agent_question_record::AgentQuestionRecordSpec;
pub use agent_subagent::AgentSubagentSpec;
pub use agent_transcript::AgentTranscriptSpec;
pub use alert_dialog::{AlertDialogSpec, AlertDialogTone};
pub use audio::{
    AudioMeterSpec, AudioMeterStyle, AudioSwitchSpec, DragNumberFieldSpec, EnvelopeEditorSpec,
    FaderSpec, GainReductionMeterSpec, KeyboardSpec, KnobSpec, ModMatrixGridSpec, ValueReadoutSpec,
    WaveformDisplaySpec, XYPadSpec,
};
pub use avatar::{AvatarShape, AvatarSize, AvatarSpec, AvatarTone};
pub use badge::BadgeSpec;
pub use banner::BannerSpec;
pub use breadcrumbs::{
    BreadcrumbItem, BreadcrumbsSpec, ELLIPSIS_VALUE as BREADCRUMBS_ELLIPSIS_VALUE,
};
pub use bulk_action_bar::{BulkAction, BulkActionBarSpec, BulkActionTone};
pub use button::{ButtonFit, ButtonSpec};
pub use calendar::{CalendarMode, CalendarSpec};
pub use call_out::{CallOutSpec, CalloutAnnounceMode};
pub use card::{CardLayout, CardSpec, CardVariant};
pub use card_toggle_group::{CardToggleGroupSpec, CardToggleOption};
pub use changed_files::ChangedFilesSpec;
pub use checkbox::CheckboxSpec;
pub use code::{CodeInlineVariant, CodeSpec, CodeTypography};
pub use code_input::{CodeInputCompletion, CodeInputSpec};
pub use collapse_toggle::{CollapseDirection, CollapseToggleSpec};
pub use collapsible::CollapsibleSpec;
pub use color_picker::{ColorInputMode, ColorPickerSpec};
pub use context_menu::ContextMenuSpec;
pub use date_picker::DatePickerSpec;
pub use date_range_picker::DateRangePickerSpec;
pub use date_time_picker::DateTimePickerSpec;
pub use date_time_range_picker::DateTimeRangePickerSpec;
pub use detail_item::{DetailItemLayout, DetailItemPresentation, DetailItemSpan, DetailItemSpec};
pub use dialog::DialogSpec;
pub use drawer::DrawerSpec;
pub use duration_input::DurationInputSpec;
pub use editable_label::{EditableLabelActivation, EditableLabelSpec, EditableLabelVariant};
pub use eyebrow::{EyebrowElement, EyebrowSize, EyebrowSpacing, EyebrowSpec};
pub use field::{FieldRelationships, FieldSpec};
pub use field_set::{FieldSetSpec, SpaceScale};
pub use file_upload::{format_file_size, FileUploadItem, FileUploadSpec, FileUploadStatus};
pub use form_actions::FormActionsSpec;
pub use form_dialog::FormDialogSpec;
pub use form_layout::FormLayoutSpec;
pub use grid::{GridColumns, GridSpec, GridTrack};
pub use hover_card::HoverCardSpec;
pub use icon::{IconSize, IconSpec};
pub use icon_button::IconButtonSpec;
pub use icon_provider::IconProviderSpec;
pub use list_card::{
    LeadingFill, LeadingShape, ListCardContextMenuTrigger, ListCardLayout, ListCardSpec,
    SelectionIndicator,
};
pub use list_card_counter::ListCardCounterSpec;
pub use list_grid::{ListGridSpec, ListGridVariant};
pub use licence_activation::{LicenceActivationSpec, LicenceKeyCodeInputOptions};
pub use licence_seats::LicenceSeatsSpec;
pub use licence_status::LicenceStatusSpec;
pub use menu::MenuSpec;
pub use menubar::MenubarSpec;
pub use meta_bar::MetaBarSpec;
pub use meta_item::MetaItemSpec;
pub use meter::{MeterLevel, MeterShape, MeterSpec, MeterTone};
pub use model_catalogue_editor::{
    ModelCatalogueEditorSpec, MODEL_CATALOGUE_HIDDEN_SECTION_ID,
};
pub use model_connection_card::ModelConnectionCardSpec;
pub use model_connection_picker::ModelConnectionPickerSpec;
pub use model_connection_setup::ModelConnectionSetupSpec;
pub use model_picker::{
    ModelAxisBinding, ModelAxisControl, ModelAxisControlKind, ModelAxisKind, ModelAxisOption,
    ModelAxisRef, ModelAxisValue, ModelCapabilityAxis, ModelImage, ModelOption,
    ModelPickerEmphasis, ModelPickerSpec, ModelPickerVariant, ModelSelection,
    SEGMENTED_AXIS_MAX_OPTIONS,
};
pub use nav_card::NavCardSpec;
pub use navigation_menu::NavigationMenuSpec;
pub use number_input::NumberInputSpec;
pub use order_by::{
    ActiveSort, OrderByField, OrderBySpec, OrderByTriggerVariant, SortDirection, SortField,
};
pub use pagination::{PageItem, PaginationSpec, PaginationVariant};
pub use password_requirements::{PasswordRequirementsPolicy, PasswordRequirementsSpec};
pub use pill::{PillAppearance, PillFont, PillSize, PillSpec, PillTone};
pub use popover::PopoverSpec;
pub use progress::ProgressSpec;
pub use r#box::BoxSpec;
pub use radio_group::RadioGroupSpec;
pub use radio::RadioSpec;
pub use ref_select::{RefKind, RefOption, RefSelectEmphasis, RefSelectSpec, RefSelectVariant};
pub use tool_call::ToolCallSpec;
pub use tool_call_group::ToolCallGroupSpec;
/// Deprecated: use `CalendarSpec` with `CalendarMode::Range` instead.
pub type RangeCalendarSpec = CalendarSpec;
pub use range_slider::RangeSliderSpec;
pub use rating::RatingSpec;
pub use region::RegionSpec;
pub use resize_handle::ResizeHandleSpec;
pub use scroll_shell::ScrollShellSpec;
/// Deprecated: use `TextInputSpec` with `input_type("search")` instead.
pub type SearchInputSpec = TextInputSpec;
/// Deprecated: use `TextInputSpec` with `input_type("search")` instead.
pub type SearchFieldSpec = TextInputSpec;
pub use segmented_control::SegmentedControlSpec;
pub use select::{SelectMode, SelectSpec, SelectVariant};
pub use separator::SeparatorSpec;
pub use skeleton::{SkeletonPreset, SkeletonSpec};
pub use slider::{SliderPolarity, SliderSpec, SliderVariant};
pub use spacer::SpacerSpec;
pub use spinner::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};
pub use split_button::{SplitButtonSpec, SplitMenuItem};
pub use stack::{LayoutJustify, StackDirection, StackSpec};
pub use status_indicator::StatusIndicatorSpec;
pub use stepper::{StepStatus, StepperSpec, StepperStep};
pub use surface::SurfaceSpec;
pub use switch::{SwitchSpec, SwitchTone};
pub use tab_strip::TabStripSpec;
pub use table::{ColumnAlign, TableColumn, TableRow, TableSpec};
pub use tabs::{ActiveEdge, ActiveFill, TabsOverflowStrategy, TabsShedPart, TabsSpec};
pub use text::{TextElement, TextLeading, TextSize, TextSpacing, TextSpec, TextTone, TextWeight};
/// Deprecated: use `TextInputSpec` with `rows > 1` instead.
pub type TextAreaSpec = TextInputSpec;
pub use history_center::{HistoryCenterRejection, HistoryCenterSpec, HistoryCenterStatus};
pub use text_input::TextInputSpec;
pub use text_link::{TextLinkSpec, TextLinkTone};
pub use time_ago::{TimeAgoFutureFormat, TimeAgoSpec, TimeAgoTooltipFormat};
pub use time_field::TimeFieldSpec;
pub use time_zone_select::{
    default_time_zone_options, TimeZoneSelectSpec, TIME_ZONE_EMPTY_MESSAGE, TIME_ZONE_PLACEHOLDER,
};
pub use token_input::TokenInputSpec;
pub use toolbar::ToolbarSpec;
/// Deprecated: Toggle has been superseded by Button with `pressed` prop.
pub type ToggleSpec = ButtonSpec;
pub use date_time_zone_picker::DateTimeZonePickerSpec;
pub use toggle_group::{ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};
pub use tooltip::TooltipSpec;
pub use tri_state_switch::TriStateSwitchSpec;
pub use ui_presentation_provider::UiPresentationProviderSpec;
/// Deprecated: Use `DateTimeZonePickerSpec` instead.
pub type ZonedDateTimePickerSpec = DateTimeZonePickerSpec;
pub use types::{
    resolve_semantic_control_size, AccordionItemSpec, AccordionSelectionValue, Alignment,
    BadgeVariant, ButtonTone, ButtonVariant, CalendarWeekStart, CheckState, ChoiceOption,
    ControlDensity, ControlSize, DateRangeValue, DateTimeRangeValue, DateTimeValue, DialogKind,
    DialogWidth, Dimension, Direction, DrawerEdge, FormActionAlign, FormActionDangerItem,
    HistoryContinuation, HistoryEntry, HistoryEntryPosition, HistoryPathPage,
    InlineTypographyMode, Inset, MenuEntry, MenuItemKind, MenubarEntry, NavigationMenuEntry,
    Orientation, Overflow, OverlayPlacement, PaddingScale, PopoverInitialFocus,
    PopoverSurfaceWidth, RuleTone, SemanticControlSizeRole, SeparatorOrientation, StatusTone,
    SurfaceBorder, SurfaceRole, SurfaceTone, TabActivationMode, TabDefinition, TabStripItem,
    TabVariant, TimeZoneOption, TriStateValue, ValidationState, ZonedDateTimeValue,
    ToneFill,
};

// Composite exports
pub use action_discovery_panel::ActionDiscoveryPanelSpec;
pub use app_header::AppHeaderSpec;
pub use audio_player::AudioPlayerSpec;
pub use block_editor::{BlockEditorMode, BlockEditorSpec, BlockTypeDefinition, EditorBlock};
pub use card_radio_group::CardRadioGroupSpec;
pub use command_palette::CommandPaletteSpec;
pub use composite_types::{
    ActionDiscoverySection, AnnouncementMode, AspectRatio, BrowseState, CommandActionItem,
    DiscoveryState, DockEdge, EmptyStateVariant, FormActionLayout, FormFieldState, FormSectionSpec,
    FormStatusSummary, MediaKind, MediaState, MinColumnWidth, PanelTabItem, ParsedEmbed,
    PickerItemSpec, PickerVariant, RemediationAction, ScrollOwner, SelectionMode,
    SelectionSummaryItem, SplitOrientation, SplitToggleVisibility, TableColumnSpec, TableFilter,
    TablePagination, TableRowSpec, TableSortDirection, ValidationSummaryEntry,
};
pub use confirm_action::ConfirmActionSpec;
pub use data_table::DataTableSpec;
pub use debug_dialog::DebugDialogSpec;
pub use detail_section::DetailSectionSpec;
pub use detail_section_group::{DetailSectionGroupLayout, DetailSectionGroupSpec};
pub use detail_shell::{DetailShellSpec, DetailState};
pub use dock_region::{
    DockCollapsedPosture, DockEmphasis, DockRegionSpec, DockSizing, DockTabsPlacement,
};
pub use editable_list::{EditableListItem, EditableListSpec};
pub use embed_input::EmbedInputSpec;
pub use embed_preview::EmbedPreviewSpec;
pub use empty_state::{EmptyStateSize, EmptyStateSpec};
pub use error_boundary::ErrorBoundarySpec;
pub use filter_builder::{
    default_operators_for_kind, FilterBuilderPicker, FilterBuilderSpec, FilterClause,
    FilterCombinator, FilterDraft, FilterExpression, FilterFieldDefinition, FilterFieldKind,
    FilterOperand, FilterOperandKind, FilterOperatorDefinition, FilterOption,
};
pub use filter_toolbar::FilterToolbarSpec;
pub use form_shell::FormShellSpec;
pub use inline_list_section::InlineListSectionSpec;
pub use inline_remediation::InlineRemediationSpec;
pub use list_container::{ListContainerSpec, ListContainerState};
pub use log_list::{
    AuditLogEntry, LogActor, LogEntry, LogFilter, LogFilterKind, LogFilterOption, LogLevel,
    LogListSpec, StreamLogEntry,
};
pub use markdown_editor::MarkdownEditorSpec;
pub use media_browse_panel::{MediaBrowseItem, MediaBrowsePanelSpec};
pub use media_picker::{MediaPickerItem, MediaPickerSpec, MediaPickerTab};
pub use media_preview::MediaPreviewSpec;
pub use media_thumbnail::{MediaFit, MediaFrameWidth, MediaPresentation, MediaThumbnailSpec};
pub use message_center::{MessageCenterItem, MessageCenterItemProgress, MessageCenterSpec};
pub use metric_tile::{MetricTileSpec, MetricTrend};
pub use page_header::{PageHeaderAlign, PageHeaderPosture, PageHeaderSpec};
pub use page_loading::{PageLoadingPresentation, PageLoadingSpec};
pub use pagination_summary::PaginationSummarySpec;
pub use picker_shell::PickerShellSpec;
pub use relation_picker::{
    DrillDownConfig, DrillDownItem, DrillDownLeafGroup, DrillDownLevel, PickerFilterConfig,
    PickerFilterOption, RelationPickerSpec,
};
pub use remediation_banner::RemediationBannerSpec;
pub use selection_summary::SelectionSummarySpec;
pub use settings_shell::SettingsShellSpec;
pub use shell_status_bar::ShellStatusBarSpec;
pub use sidebar_nav::{SidebarNavGroup, SidebarNavItem, SidebarNavSpec};
pub use split_view::SplitViewSpec;
pub use state_tile::StateTileSpec;
pub use theme_select::{ThemeOption, ThemeSelectSpec, ThemeSwatch};
pub use toast_host::{ToastHostPlacement, ToastHostSpec};
pub use toast_stack::{Toast, ToastPosition, ToastStackSpec, ToastTone};
pub use tree::{
    compute_selection, reorder_nodes, DropPosition, TreeNode, TreeSelectionMode,
    TreeSelectionResult, TreeSpec, TreeVisibleRow,
};
pub use update_center::UpdateCenterSpec;
pub use update_status::UpdateStatusSpec;
pub use validation_summary::ValidationSummarySpec;
pub use video_player::VideoPlayerSpec;

pub const CURRENT_GENERATION: &str = "g10.004";
pub const STRUCTURAL_EXPORTS: &[&str] = &[
    "BoxSpec",
    "StackSpec",
    "GridSpec",
    "SurfaceSpec",
    "SeparatorSpec",
    "ScrollShellSpec",
];
pub const ACTION_FIELD_EXPORTS: &[&str] = &[
    "ButtonSpec",
    "IconButtonSpec",
    "FieldSpec",
    "FieldRelationships",
    "TextInputSpec",
    "FormActionsSpec",
];
pub const SELECTION_FEEDBACK_DATE_EXPORTS: &[&str] = &[
    "CheckboxSpec",
    "RadioGroupSpec",
    "SwitchSpec",
    "SelectSpec",
    "SegmentedControlSpec",
    "SliderSpec",
    "ProgressSpec",
    "SpinnerSpec",
    "BadgeSpec",
    "StatusIndicatorSpec",
    "CalendarSpec",
    "DatePickerSpec",
    "DateRangePickerSpec",
    "TimeFieldSpec",
    "DateTimePickerSpec",
    "DateTimeRangePickerSpec",
];
pub const OVERLAY_NAVIGATION_EXPORTS: &[&str] = &[
    "AccordionSpec",
    "CollapsibleSpec",
    "DialogSpec",
    "DrawerSpec",
    "PopoverSpec",
    "TooltipSpec",
    "MenuSpec",
    "ContextMenuSpec",
    "TabsSpec",
    "NavigationMenuSpec",
    "MenubarSpec",
    "TabStripSpec",
];
pub const FORM_VALIDATION_REMEDIATION_EXPORTS: &[&str] = &[
    "FormShellSpec",
    "ValidationSummarySpec",
    "RemediationBannerSpec",
    "InlineRemediationSpec",
];
pub const DATA_BROWSE_DETAIL_PICKER_MEDIA_EXPORTS: &[&str] = &[
    "DataTableSpec",
    "DetailShellSpec",
    "FilterToolbarSpec",
    "PaginationSummarySpec",
    "EmptyStateSpec",
    "PickerShellSpec",
    "RelationPickerSpec",
    "SelectionSummarySpec",
    "MediaThumbnailSpec",
    "MediaPreviewSpec",
];

#[cfg(test)]
mod tests {
    use poodle_tokens::semantic;

    use super::{
        AccordionItemSpec, AccordionSelectionValue, AccordionSpec, BadgeSpec, BadgeVariant,
        BoxSpec, ButtonSpec, ButtonVariant, CalendarMode, CalendarSpec, CalendarWeekStart,
        CheckState, CheckboxSpec, ChoiceOption, CollapsibleSpec, ContextMenuSpec, ControlSize,
        DatePickerSpec, DateRangePickerSpec, DateRangeValue, DateTimePickerSpec,
        DateTimeRangePickerSpec, DateTimeRangeValue, DateTimeValue, DialogKind, DialogSpec,
        Direction, DrawerEdge, DrawerSpec, FieldSpec, FormActionAlign, FormActionDangerItem,
        FormActionsSpec, GridSpec, IconButtonSpec, MenuEntry, MenuItemKind, MenuSpec, MenubarEntry,
        MenubarSpec, NavigationMenuEntry, NavigationMenuSpec, Orientation, OverlayPlacement,
        PaddingScale, PopoverInitialFocus, PopoverSpec, ProgressSpec, RadioGroupSpec,
        ScrollShellSpec, SegmentedControlSpec, SelectSpec, SeparatorSpec, SliderSpec, StackSpec,
        StatusIndicatorSpec, StatusTone, SurfaceSpec, SurfaceTone, SwitchSpec, TabActivationMode,
        TabDefinition, TabStripItem, TabStripSpec, TabsSpec, TextInputSpec, TimeFieldSpec,
        TooltipSpec, ValidationState,
    };

    use super::{
        AnnouncementMode, AspectRatio, BrowseState, DataTableSpec, DetailShellSpec, DetailState,
        EmbedInputSpec, EmbedPreviewSpec, EmptyStateSpec, EmptyStateVariant, FilterToolbarSpec,
        FormFieldState, FormSectionSpec, FormShellSpec, InlineRemediationSpec, MediaKind,
        MediaPreviewSpec, MediaState, MediaThumbnailSpec, PaginationSummarySpec, ParsedEmbed,
        PickerItemSpec, PickerShellSpec, PickerVariant, RelationPickerSpec, RemediationAction,
        RemediationBannerSpec, SelectionMode, SelectionSummaryItem, SelectionSummarySpec,
        StateTileSpec, TableColumnSpec, TableRowSpec, TableSortDirection, ValidationSummaryEntry,
        ValidationSummarySpec,
    };

    #[test]
    fn box_defaults_stay_neutral() {
        let spec = BoxSpec::default();
        assert_eq!(spec.resolved_padding().horizontal, None);
        assert_eq!(spec.resolved_padding().vertical, None);
    }

    #[test]
    fn stack_defaults_use_md_gap() {
        let spec = StackSpec::default();
        assert_eq!(spec.resolved_gap(), Some(semantic::SPACE_STACK_MD));
    }

    #[test]
    fn grid_defaults_match_single_track_baseline() {
        let spec = GridSpec::default();
        assert_eq!(spec.columns.as_str(), "1fr");
        // Contract §8 Grid SpaceScale: gap `md` → `space-panel-y`, single value
        // on both axes (CSS `gap` is one value).
        assert_eq!(spec.resolved_column_gap(), Some(semantic::SPACE_PANEL_Y));
        assert_eq!(spec.resolved_row_gap(), Some(semantic::SPACE_PANEL_Y));
    }

    #[test]
    fn elevated_surface_uses_elevated_background_and_surface_shadow() {
        let spec = SurfaceSpec::new().with_tone(SurfaceTone::Elevated);
        assert_eq!(
            spec.resolved_background_token(),
            semantic::COLOR_BACKGROUND_ELEVATED
        );
        // Contract §8 elevated targets `--poodle-elevation-surface` (Svelte
        // `Surface.svelte:86`), not the overlay elevation.
        assert_eq!(spec.resolved_shadow_token(), semantic::ELEVATION_SURFACE);
    }

    #[test]
    fn separator_uses_default_border_width() {
        let spec = SeparatorSpec::default();
        assert_eq!(spec.resolved_stroke_width(), semantic::BORDER_WIDTH_DEFAULT);
    }

    #[test]
    fn scroll_shell_exposes_focus_tokens() {
        let spec = ScrollShellSpec::new()
            .with_direction(Direction::Both)
            .with_padding(PaddingScale::Md)
            .with_focusable(true);
        assert_eq!(
            spec.resolved_padding().horizontal,
            Some(semantic::SPACE_PANEL_X)
        );
        assert_eq!(
            spec.resolved_padding().vertical,
            Some(semantic::SPACE_PANEL_Y)
        );
        assert_eq!(
            spec.focus_ring_color_token(),
            semantic::COLOR_ACCENT_FOCUS_RING
        );
        assert_eq!(spec.focus_ring_width_token(), semantic::BORDER_WIDTH_FOCUS);
    }

    #[test]
    fn primary_button_uses_accent_and_suppresses_activation_while_loading() {
        let spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Primary)
            .with_size(ControlSize::Lg)
            .with_label("Publish")
            .with_loading(true);

        assert_eq!(spec.resolved_fill_token(), semantic::COLOR_ACCENT_BASE);
        assert_eq!(spec.resolved_text_token(), semantic::COLOR_TEXT_INVERSE);
        // Contract: icons in buttons always render at size="sm"
        assert_eq!(spec.icon_size_token(), semantic::SIZE_ICON_SM);
        assert!(!spec.activation_allowed());
        assert!(!spec.requires_aria_label());
    }

    #[test]
    fn icon_button_requires_icon_and_accessible_name() {
        let spec = IconButtonSpec::new()
            .with_variant(ButtonVariant::Ghost)
            .with_icon("close")
            .with_aria_label("Close panel")
            .with_expanded(true)
            .with_controls("panel")
            .with_pressed(true);

        assert!(spec.has_required_icon());
        assert!(spec.has_required_accessible_name());
        assert!(spec.uses_pressed_semantics());
        assert_eq!(spec.is_expanded, Some(true));
        assert_eq!(spec.controls.as_deref(), Some("panel"));
        assert_eq!(spec.control_height_token(), semantic::SIZE_CONTROL_HEIGHT);
    }

    #[test]
    fn field_relationships_prefer_invalid_message_over_pending() {
        let spec = FieldSpec::new("project-title", "Project title")
            .with_description("Used in review notes.")
            .with_error("Title is required.")
            .with_pending_message("Validating title.")
            .with_validation_state(ValidationState::Invalid);

        let relationships = spec.relationships();
        assert_eq!(
            relationships.description_id.as_deref(),
            Some("project-title-description")
        );
        assert_eq!(
            relationships.error_id.as_deref(),
            Some("project-title-error")
        );
        assert_eq!(
            relationships.message_id.as_deref(),
            Some("project-title-error")
        );
        assert_eq!(
            relationships.described_by.as_deref(),
            Some("project-title-description project-title-error")
        );
    }

    #[test]
    fn text_input_combines_description_and_invalid_message() {
        let spec = TextInputSpec::new()
            .with_default_value("Aura mix review")
            .with_description_id("project-title-description")
            .with_error_message_id("project-title-error")
            .with_validation_state(ValidationState::Invalid)
            .with_submit_enabled(true)
            .with_cancel_enabled(true);

        assert!(!spec.is_controlled());
        assert_eq!(spec.current_value(), "Aura mix review");
        assert_eq!(
            spec.described_by().as_deref(),
            Some("project-title-description project-title-error")
        );
        assert_eq!(spec.aria_invalid(), Some("true"));
        assert_eq!(spec.border_token(), semantic::COLOR_STATUS_DANGER);
    }

    #[test]
    fn text_input_multiline_reports_pending_busy_state() {
        let spec = TextInputSpec::new()
            .with_rows(4)
            .with_validation_state(ValidationState::Pending);

        assert_eq!(spec.rows, Some(4));
        assert!(spec.is_multiline());
        assert_eq!(spec.aria_busy(), Some("true"));
        assert_eq!(spec.border_token(), semantic::COLOR_ACCENT_BASE);
    }

    #[test]
    fn text_input_search_type_preserves_current_value() {
        let spec = TextInputSpec::new()
            .with_input_type("search")
            .with_default_value("track lane")
            .with_leading_icon("search");

        assert_eq!(spec.input_type, "search");
        assert_eq!(spec.current_value(), "track lane");
        assert_eq!(spec.leading_icon.as_deref(), Some("search"));
    }

    #[test]
    fn form_actions_default_to_end_alignment_with_wrap_behavior() {
        let spec = FormActionsSpec::new()
            .with_align(FormActionAlign::Between)
            .with_top_separation(false)
            .with_top_border(true);

        assert_eq!(spec.align, FormActionAlign::Between);
        assert!(!spec.shows_top_separation());
        assert!(spec.shows_top_border());
        assert_eq!(spec.action_gap_token(), semantic::SPACE_INLINE_MD);
        assert_eq!(spec.stack_separation_token(), semantic::SPACE_STACK_SM);
        assert_eq!(spec.border_token(), semantic::COLOR_BORDER_SUBTLE);
        assert!(spec.wraps_on_narrow_widths());
    }

    #[test]
    fn form_actions_danger_items_model() {
        // Default: no overflow danger menu.
        let spec = FormActionsSpec::new();
        assert!(spec.danger_items.is_empty());
        assert!(!spec.has_danger_menu());

        // Additive danger items, with value fallback to `index:label`.
        let spec = FormActionsSpec::new()
            .with_danger_item(FormActionDangerItem::new("Delete account"))
            .with_danger_item(
                FormActionDangerItem::new("Archive")
                    .with_value("archive")
                    .with_disabled(true),
            );
        assert!(spec.has_danger_menu());
        assert_eq!(spec.danger_items.len(), 2);
        // First item has no explicit value -> `0:Delete account`.
        assert_eq!(spec.danger_items[0].resolved_value(0), "0:Delete account");
        assert!(!spec.danger_items[0].disabled);
        // Second item keeps its explicit value and disabled flag.
        assert_eq!(spec.danger_items[1].resolved_value(1), "archive");
        assert!(spec.danger_items[1].disabled);
        // Inline danger gap mirrors the root action gap.
        assert_eq!(spec.danger_inline_gap_token(), spec.action_gap_token());
        assert_eq!(spec.danger_inline_gap_rem(), spec.gap_rem());
    }

    #[test]
    fn checkbox_mixed_state_overrides_checked_state() {
        let spec = CheckboxSpec::new()
            .with_checked(true)
            .with_mixed(true)
            .with_label("Select all");

        assert_eq!(spec.current_state(), CheckState::Mixed);
        assert_eq!(spec.current_state().aria_checked(), "mixed");
        assert_eq!(spec.indicator_fill_token(), semantic::COLOR_ACCENT_BASE);
    }

    #[test]
    fn radio_group_selects_default_option() {
        let options = vec![
            ChoiceOption::new("single", "Single"),
            ChoiceOption::new("multiple", "Multiple"),
        ];
        let spec = RadioGroupSpec::new(options.clone()).with_default_value("multiple");

        assert_eq!(spec.current_value(), Some("multiple"));
        assert_eq!(spec.selected_option(), Some(&options[1]));
        assert_eq!(spec.option_gap_token(), semantic::SPACE_STACK_SM);
    }

    #[test]
    fn select_prefers_selected_label_over_placeholder() {
        let spec = SelectSpec::new(vec![
            ChoiceOption::new("light", "Light"),
            ChoiceOption::new("dark", "Dark"),
        ])
        .with_placeholder("Pick theme")
        .with_value("dark")
        .with_open(true);

        assert_eq!(spec.trigger_text(), Some("Dark"));
        assert!(spec.current_open());
        assert_eq!(
            spec.overlay_fill_token(),
            semantic::COLOR_BACKGROUND_ELEVATED
        );
    }

    #[test]
    fn switch_uses_accent_track_when_checked() {
        let spec = SwitchSpec::new().with_default_checked(true);

        assert!(spec.current_checked());
        assert_eq!(spec.track_fill_token(), semantic::COLOR_ACCENT_BASE);
    }

    #[test]
    fn segmented_control_uses_selected_fill() {
        let spec = SegmentedControlSpec::new(vec![
            ChoiceOption::new("grid", "Grid"),
            ChoiceOption::new("list", "List"),
        ])
        .with_default_value("grid");

        assert_eq!(spec.current_value(), Some("grid"));
        assert_eq!(spec.selected_fill_token(), semantic::COLOR_ACCENT_BASE);
    }

    #[test]
    fn slider_normalizes_progress_within_bounds() {
        let spec = SliderSpec::new(75.0)
            .with_bounds(0.0, 100.0)
            .with_orientation(Orientation::Horizontal);

        assert_eq!(spec.clamped_value(), 75.0);
        assert_eq!(spec.normalized_progress(), 0.75);
        assert_eq!(spec.range_fill_token(), semantic::COLOR_ACCENT_BASE);
    }

    #[test]
    fn progress_reports_none_when_indeterminate() {
        let spec = ProgressSpec::new()
            .with_value(40.0)
            .with_indeterminate(true);

        assert_eq!(spec.normalized_progress(), None);
        assert_eq!(spec.indicator_fill_token(), semantic::COLOR_ACCENT_BASE);
    }

    #[test]
    fn badge_and_status_indicator_resolve_tokens() {
        let badge = BadgeSpec::new().with_variant(BadgeVariant::Muted);
        let status = StatusIndicatorSpec::new().with_status(StatusTone::Warning);

        assert_eq!(badge.fill_token(), semantic::COLOR_BACKGROUND_SURFACE);
        assert_eq!(status.status_color_token(), semantic::COLOR_STATUS_WARNING);
    }

    #[test]
    fn calendar_and_picker_surface_date_values() {
        let calendar = CalendarSpec::new()
            .with_value("2026-03-12")
            .with_visible_month("2026-03")
            .with_week_start(CalendarWeekStart::Monday);
        let picker = DatePickerSpec::new()
            .with_default_value("2026-03-12")
            .with_default_open(true);

        assert_eq!(calendar.current_value(), Some("2026-03-12"));
        assert_eq!(calendar.effective_visible_month(), Some("2026-03"));
        assert_eq!(picker.current_value(), Some("2026-03-12"));
        assert!(picker.current_open());
    }

    #[test]
    fn range_calendar_and_picker_surface_range_values() {
        let range = DateRangeValue::new(
            Some(String::from("2026-03-12")),
            Some(String::from("2026-03-18")),
        );
        let calendar = CalendarSpec::new()
            .with_mode(CalendarMode::Range)
            .with_default_range_value(range.clone());
        let picker = DateRangePickerSpec::new()
            .with_default_value(range.clone())
            .with_open(true);

        assert_eq!(calendar.current_range_value(), &range);
        assert_eq!(picker.current_value(), &range);
        assert!(picker.current_open());
    }

    #[test]
    fn time_and_date_time_specs_preserve_public_values() {
        let time = TimeFieldSpec::new()
            .with_default_value("09:30")
            .with_step(300);
        let date_time = DateTimeValue::new(
            Some(String::from("2026-03-12")),
            Some(String::from("09:30")),
        );
        let picker = DateTimePickerSpec::new().with_default_value(date_time.clone());
        let range_value = DateTimeRangeValue::new(
            date_time.clone(),
            DateTimeValue::new(
                Some(String::from("2026-03-13")),
                Some(String::from("18:00")),
            ),
        );
        let range_picker = DateTimeRangePickerSpec::new().with_default_value(range_value.clone());

        assert_eq!(time.current_value(), Some("09:30"));
        assert_eq!(picker.current_value(), &date_time);
        assert_eq!(range_picker.current_value(), &range_value);
    }

    #[test]
    fn accordion_and_collapsible_surface_disclosure_state() {
        let accordion = AccordionSpec::new(vec![
            AccordionItemSpec::new("filters", "Filters"),
            AccordionItemSpec::new("reference", "Reference"),
        ])
        .with_allow_multiple(true)
        .with_collapsible(true)
        .with_default_value(AccordionSelectionValue::Multiple(vec![
            String::from("filters"),
            String::from("reference"),
        ]));
        let collapsible = CollapsibleSpec::new()
            .with_title("Reference")
            .with_default_open(true);

        assert!(accordion.allow_multiple);
        assert_eq!(accordion.expanded_item_count(), 2);
        assert_eq!(accordion.item_gap_token(), semantic::SPACE_STACK_SM);
        assert!(collapsible.current_open());
        assert!(collapsible.activation_allowed());
    }

    #[test]
    fn dialog_and_drawer_keep_native_dismissal_posture_explicit() {
        let dialog = DialogSpec::new()
            .with_role(DialogKind::AlertDialog)
            .with_default_open(true)
            .with_title("Delete review");
        let drawer = DrawerSpec::new()
            .with_default_open(true)
            .with_edge(DrawerEdge::Left)
            .with_modal(true);

        assert!(dialog.current_open());
        assert!(dialog.is_alert_dialog());
        // The role does not suppress backdrop dismissal — `AlertDialog.svelte`
        // passes `dismissOnBackdrop={!working}`, so only the working state does.
        assert!(dialog.effective_dismiss_on_backdrop());
        assert!(!dialog
            .clone()
            .with_dismiss_on_backdrop(false)
            .effective_dismiss_on_backdrop());
        assert_eq!(
            dialog.backdrop_fill_token(),
            semantic::COLOR_BACKGROUND_OVERLAY
        );
        assert!(drawer.current_open());
        assert!(drawer.shows_backdrop());
        assert_eq!(drawer.shadow_token(), semantic::ELEVATION_DIALOG);
    }

    #[test]
    fn popover_and_tooltip_hold_overlay_state_and_placement() {
        let popover = PopoverSpec::new()
            .with_default_open(true)
            .with_placement(OverlayPlacement::RightStart)
            .with_initial_focus(PopoverInitialFocus::Content);
        let tooltip = TooltipSpec::new()
            .with_content("Open the command palette")
            .with_default_open(true)
            .with_placement(OverlayPlacement::TopEnd);

        assert!(popover.current_open());
        assert_eq!(popover.placement, OverlayPlacement::RightStart);
        assert_eq!(popover.initial_focus, PopoverInitialFocus::Content);
        assert_eq!(popover.shadow_token(), semantic::ELEVATION_OVERLAY);
        assert!(tooltip.current_open());
        assert!(tooltip.has_content());
    }

    #[test]
    fn menu_and_context_menu_report_actionable_entries() {
        let items = vec![
            MenuEntry::new("open", "Open"),
            MenuEntry::new("autosave", "Autosave")
                .with_kind(MenuItemKind::Checkbox)
                .with_checked(true),
            MenuEntry::new("divider", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("danger", "Delete").with_disabled(true),
        ];
        let menu = MenuSpec::new(items.clone()).with_default_open(true);
        let context_menu = ContextMenuSpec::new(items)
            .with_open(true)
            .with_anchor_point(200, 120);

        assert!(menu.current_open());
        assert_eq!(menu.actionable_item_count(), 2);
        assert_eq!(menu.checked_item_count(), 1);
        assert!(context_menu.current_open());
        assert_eq!(context_menu.actionable_item_count(), 2);
    }

    #[test]
    fn tabs_navigation_menu_menubar_and_tab_strip_surface_selection() {
        let tabs = TabsSpec::new(vec![
            TabDefinition::new("overview", "Overview"),
            TabDefinition::new("activity", "Activity"),
        ])
        .with_activation_mode(TabActivationMode::Manual)
        .with_default_value("activity");
        let navigation = NavigationMenuSpec::new(vec![
            NavigationMenuEntry::new("docs", "Docs"),
            NavigationMenuEntry::new("contracts", "Contracts"),
        ]);
        let menubar = MenubarSpec::new(vec![
            MenubarEntry::new(
                "file",
                "File",
                vec![MenuEntry::new("new", "New"), MenuEntry::new("open", "Open")],
            ),
            MenubarEntry::new("view", "View", vec![MenuEntry::new("zoom-in", "Zoom in")]),
        ]);
        let tab_strip = TabStripSpec::new(vec![
            TabStripItem::new("review", "Review").with_closable(true),
            TabStripItem::new("history", "History"),
        ])
        .with_default_value("review")
        .with_reorderable(true);

        assert_eq!(tabs.current_value(), Some("activity"));
        assert!(tabs.uses_manual_activation());
        assert_eq!(navigation.current_value(), Some("docs"));
        assert_eq!(menubar.current_value(), Some("file"));
        assert_eq!(tab_strip.current_value(), Some("review"));
        assert_eq!(tab_strip.closable_item_count(), 1);
    }

    // Composite tests

    #[test]
    fn form_shell_counts_invalid_and_pending_fields_and_blocks_submit() {
        let spec = FormShellSpec::new("mix-review")
            .with_sections(vec![FormSectionSpec::new(
                "main",
                "Main",
                vec![String::from("title"), String::from("search")],
            )])
            .with_fields(vec![
                FormFieldState::new("title", "Title")
                    .with_validation_state(ValidationState::Invalid)
                    .with_message("Title is required."),
                FormFieldState::new("search", "Search")
                    .with_validation_state(ValidationState::Pending)
                    .with_message("Checking references."),
            ])
            .with_actions(FormActionAlign::End, 2);

        assert_eq!(spec.invalid_field_count(), 1);
        assert_eq!(spec.pending_field_count(), 1);
        assert!(spec.blocks_submission());
        assert_eq!(spec.resolved_status_tone(), StatusTone::Danger);
        assert_eq!(spec.section_gap_token(), semantic::SPACE_STACK_MD);
    }

    #[test]
    fn validation_summary_filters_to_active_entries_and_exposes_alert_role() {
        let spec = ValidationSummarySpec::new(vec![
            ValidationSummaryEntry::new(
                "title",
                "Project title",
                "Title is required.",
                ValidationState::Invalid,
            ),
            ValidationSummaryEntry::new(
                "search",
                "Asset search",
                "Checking references.",
                ValidationState::Pending,
            ),
        ])
        .with_title("Fix the following fields")
        .with_announce_mode(AnnouncementMode::Assertive)
        .with_include_pending(true);

        assert_eq!(spec.active_entries().len(), 2);
        assert_eq!(spec.blocking_entry_count(), 1);
        assert_eq!(spec.accessibility_role(), Some("alert"));
        assert_eq!(spec.border_token(), semantic::COLOR_STATUS_DANGER);
    }

    #[test]
    fn remediation_banner_reports_actions_and_urgency() {
        let spec = RemediationBannerSpec::new(
            "Review attention needed",
            "Resolve the blocking validation before publishing.",
        )
        .with_tone(StatusTone::Warning)
        .with_announce_mode(AnnouncementMode::Polite)
        .with_primary_action(
            RemediationAction::new("resolve", "Resolve").with_variant(ButtonVariant::Primary),
        )
        .with_secondary_action(RemediationAction::new("inspect", "Inspect"))
        .with_dismissible(true);

        assert_eq!(spec.action_count(), 2);
        assert_eq!(spec.accessibility_role(), Some("status"));
        assert_eq!(spec.border_token(), semantic::COLOR_STATUS_WARNING);
        assert_eq!(spec.background_token(), semantic::COLOR_BACKGROUND_PANEL);
        // Additive dimension/color token methods (contract §6).
        assert_eq!(spec.radius_token(), semantic::RADIUS_SURFACE);
        assert_eq!(spec.border_width_token(), semantic::BORDER_WIDTH_DEFAULT);
        assert_eq!(spec.icon_color_token(), semantic::COLOR_STATUS_WARNING);
        assert_eq!(spec.title_color_token(), semantic::COLOR_TEXT_PRIMARY);
        assert_eq!(spec.message_color_token(), semantic::COLOR_TEXT_SECONDARY);
        assert_eq!(spec.tone_icon_name(), "alert-triangle");
    }

    #[test]
    fn remediation_banner_tone_icon_map_covers_all_tones() {
        let cases = [
            (StatusTone::Neutral, "info"),
            (StatusTone::Info, "info"),
            (StatusTone::Success, "check-circle"),
            (StatusTone::Warning, "alert-triangle"),
            (StatusTone::Danger, "x-circle"),
            (StatusTone::Pending, "loader"),
        ];
        for (tone, icon) in cases {
            let spec = RemediationBannerSpec::new("t", "m").with_tone(tone);
            assert_eq!(spec.tone_icon_name(), icon, "tone {tone:?} icon");
            assert_eq!(
                spec.icon_color_token(),
                tone.color_token(),
                "tone {tone:?} icon color"
            );
        }
    }

    #[test]
    fn state_tile_resolves_dimension_and_trend_tokens() {
        // Default: no trend → neutral color + right-arrow glyph.
        let base = StateTileSpec::new("Active Users", "1,284");
        assert_eq!(base.fill_token(), semantic::COLOR_BACKGROUND_PANEL);
        assert_eq!(base.border_token(), semantic::COLOR_BORDER_SUBTLE);
        assert_eq!(base.radius_token(), semantic::RADIUS_SURFACE);
        assert_eq!(base.border_width_token(), semantic::BORDER_WIDTH_DEFAULT);
        assert_eq!(base.label_color_token(), semantic::COLOR_TEXT_SECONDARY);
        assert_eq!(base.value_color_token(), semantic::COLOR_TEXT_PRIMARY);
        assert_eq!(
            base.value_font_size_token(),
            semantic::TYPOGRAPHY_HEADING_SIZE
        );
        assert_eq!(
            base.label_font_size_token(),
            semantic::TYPOGRAPHY_LABEL_SIZE
        );
        assert_eq!(base.trend_color_token(), semantic::COLOR_TEXT_SECONDARY);
        assert_eq!(base.trend_glyph(), "\u{2192}");

        // up / down map to success / danger with directional glyphs.
        let up = StateTileSpec::new("x", "1").with_trend("up");
        assert_eq!(up.trend_color_token(), semantic::COLOR_STATUS_SUCCESS);
        assert_eq!(up.trend_glyph(), "\u{2191}");
        let down = StateTileSpec::new("x", "1").with_trend("down");
        assert_eq!(down.trend_color_token(), semantic::COLOR_STATUS_DANGER);
        assert_eq!(down.trend_glyph(), "\u{2193}");

        // arbitrary string → neutral color + right-arrow glyph.
        let neutral = StateTileSpec::new("x", "1").with_trend("flat");
        assert_eq!(neutral.trend_color_token(), semantic::COLOR_TEXT_SECONDARY);
        assert_eq!(neutral.trend_glyph(), "\u{2192}");
    }

    #[test]
    fn inline_remediation_tracks_field_references_and_actionability() {
        let spec = InlineRemediationSpec::new("Validation is failing.")
            .with_tone(StatusTone::Danger)
            .with_title("Invalid form state")
            .with_referenced_field_ids(vec![String::from("title"), String::from("search")])
            .with_action(RemediationAction::new("jump", "Jump to field"));

        assert_eq!(spec.reference_count(), 2);
        assert!(spec.is_actionable());
        assert_eq!(spec.border_token(), semantic::COLOR_STATUS_DANGER);
        assert_eq!(spec.gap_token(), semantic::SPACE_STACK_SM);
    }

    #[test]
    fn data_table_reports_visible_scope_selection_and_sortability() {
        let spec = DataTableSpec::new(
            vec![
                TableColumnSpec::new("name", "Name").with_sortable(true),
                TableColumnSpec::new("owner", "Owner"),
            ],
            vec![
                TableRowSpec::new(
                    "a",
                    vec![(String::from("name"), String::from("Approval still"))],
                ),
                TableRowSpec::new(
                    "b",
                    vec![(String::from("name"), String::from("Stem waveform"))],
                ),
            ],
        )
        .with_selected_row_ids(vec![String::from("a")])
        .with_sort("name", TableSortDirection::Asc);

        assert_eq!(spec.visible_row_count(), 2);
        assert_eq!(spec.selected_visible_row_count(), 1);
        assert_eq!(spec.select_all_state(), CheckState::Mixed);
        assert_eq!(spec.sortable_column_count(), 1);
        assert_eq!(spec.header_fill_token(), semantic::COLOR_BACKGROUND_SURFACE);
    }

    #[test]
    fn detail_and_empty_state_surface_state_posture() {
        let detail = DetailShellSpec::new()
            .with_title("Delivery brief")
            .with_state(DetailState::Error);
        let empty = EmptyStateSpec::new("No assets yet")
            .with_variant(EmptyStateVariant::FirstRun)
            .with_actions(vec![RemediationAction::new("add", "Add first asset")]);

        assert!(!detail.has_ready_content());
        assert_eq!(detail.body_fill_token(), semantic::COLOR_BACKGROUND_PANEL);
        assert_eq!(empty.action_count(), 1);
        assert_eq!(empty.layout_gap_token(), semantic::SPACE_STACK_MD);
    }

    #[test]
    fn filter_toolbar_and_pagination_summary_report_result_state() {
        let toolbar = FilterToolbarSpec::new()
            .with_summary_text("Showing 8 of 24 items, 2 filters active")
            .with_collapsible(true)
            .with_collapsed(false);
        let summary = PaginationSummarySpec::new(2, 25, 67);

        assert!(toolbar.is_grid_visible());
        // Default density resolves the contract §8 density-table root gap
        // (space.inline.sm), not the un-scoped base (space.stack.sm).
        assert_eq!(toolbar.gap_token(), semantic::SPACE_INLINE_SM);
        assert_eq!(summary.start_index(), 26);
        assert_eq!(summary.end_index(), 50);
    }

    #[test]
    fn picker_and_relation_picker_preserve_selection_and_variant() {
        let picker = PickerShellSpec::new("Attach asset")
            .with_variant(PickerVariant::Modal)
            .with_selection_mode(SelectionMode::Multiple)
            .with_state(BrowseState::Ready)
            .with_query("stem")
            .with_result_count(3)
            .with_selected_count(2);
        let relation = RelationPickerSpec::new(vec![
            PickerItemSpec::new("a", "Approval still"),
            PickerItemSpec::new("b", "Stem waveform"),
        ])
        .with_selected_ids(vec![String::from("b")])
        .with_variant(PickerVariant::Popover)
        .with_selection_mode(SelectionMode::Single);

        assert!(picker.is_modal_like());
        assert_eq!(picker.summary_tone(), StatusTone::Neutral);
        assert_eq!(relation.selected_item_count(), 1);
        assert_eq!(relation.as_picker_shell().selected_count, 1);
    }

    #[test]
    fn selection_summary_and_media_specs_preserve_fallback_posture() {
        let summary = SelectionSummarySpec::new(vec![
            SelectionSummaryItem::new("a", "Approval still"),
            SelectionSummaryItem::new("b", "Stem waveform"),
        ])
        .with_clear_action(RemediationAction::new("clear", "Clear"));
        let thumbnail = MediaThumbnailSpec::new(MediaKind::Image)
            .with_state(MediaState::Error)
            .with_aspect_ratio(AspectRatio::Video)
            .with_title("Approval still")
            .with_state_title("Preview unavailable")
            .with_state_message("This file cannot be previewed.");
        let preview = MediaPreviewSpec::new(MediaKind::Audio, "Stem waveform")
            .with_state(MediaState::Empty)
            .with_aspect_ratio(AspectRatio::Landscape)
            .with_badge("WIP")
            .with_thumbnail_meta("Waveform")
            .with_state_title("No preview")
            .with_state_message("No audio render has been generated yet.")
            .with_metadata(vec![String::from("01:42"), String::from("WAV")])
            .with_footer_actions(vec![RemediationAction::new("open", "Open external")]);
        let embed = EmbedInputSpec::new()
            .with_value("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
            .with_detected_parse();

        assert_eq!(summary.selected_count(), 2);
        assert!(summary.has_clear_action());
        assert!(thumbnail.shows_fallback_copy());
        assert!(thumbnail.caption_visible());
        assert_eq!(thumbnail.resolved_state_title(), "Preview unavailable");
        assert_eq!(
            thumbnail.resolved_state_message(),
            Some("This file cannot be previewed.")
        );
        assert_eq!(
            thumbnail.frame_fill_token(),
            semantic::COLOR_BACKGROUND_SURFACE
        );
        assert_eq!(preview.metadata_count(), 2);
        assert!(preview.has_footer_actions());
        assert!(preview.shows_fallback_copy());
        assert_eq!(preview.badge.as_deref(), Some("WIP"));
        assert_eq!(
            embed.parsed.as_ref().map(|parsed| parsed.provider.as_str()),
            Some("youtube")
        );
        assert_eq!(embed.error.as_deref(), None);
    }

    #[test]
    fn parsed_embed_detection_matches_supported_patterns() {
        let youtube_short = ParsedEmbed::detect("https://youtu.be/dQw4w9WgXcQ").unwrap();
        let youtube = ParsedEmbed::detect("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap();
        let vimeo = ParsedEmbed::detect("https://vimeo.com/123456").unwrap();
        let iframe = ParsedEmbed::detect(
            r#"<iframe src="https://example.com/embed/1" width="640" height="480"></iframe>"#,
        )
        .unwrap();
        let restricted = EmbedInputSpec::new()
            .with_value("https://example.com/file.zip")
            .with_providers(vec![String::from("youtube"), String::from("vimeo")])
            .with_detected_parse();

        assert_eq!(youtube_short.provider, "youtube");
        assert_eq!(youtube_short.id, "dQw4w9WgXcQ");
        assert_eq!(youtube.provider, "youtube");
        assert_eq!(youtube.id, "dQw4w9WgXcQ");
        assert_eq!(vimeo.provider, "vimeo");
        assert_eq!(vimeo.id, "123456");
        assert_eq!(iframe.provider, "generic");
        assert_eq!(
            iframe.original_url.as_deref(),
            Some("https://example.com/embed/1")
        );
        assert_eq!(iframe.width, Some(640));
        assert_eq!(iframe.height, Some(480));
        assert_eq!(restricted.parsed, None);
        assert_eq!(
            restricted.error.as_deref(),
            Some("Provider \"generic\" is not allowed")
        );
    }

    #[test]
    fn embed_preview_spec_uses_public_parse_surface() {
        let youtube = EmbedPreviewSpec::new().with_parsed(
            ParsedEmbed::new("youtube", "dQw4w9WgXcQ")
                .with_original_url("https://youtube.com/watch?v=dQw4w9WgXcQ"),
        );
        let generic = EmbedPreviewSpec::new().with_parsed(
            ParsedEmbed::new("generic", "https://example.com/demo")
                .with_original_url("https://example.com/demo")
                .with_original_embed("<iframe></iframe>"),
        );
        let auto = EmbedPreviewSpec::new().with_auto_aspect_ratio();

        assert_eq!(
            youtube.embed_url().as_deref(),
            Some("https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ")
        );
        assert!(generic.has_raw_embed());
        assert_eq!(auto.effective_aspect_ratio(), None);
    }

    #[test]
    fn format_file_size_thresholds() {
        use super::format_file_size;
        assert_eq!(format_file_size(512), "512 bytes");
        assert_eq!(format_file_size(2048), "2.0 KB");
        assert_eq!(format_file_size(5 * 1024 * 1024), "5.0 MB");
    }

    #[test]
    fn file_upload_item_progress_drives_status() {
        use super::{FileUploadItem, FileUploadStatus};
        let mid = FileUploadItem::new("a", "photo.png", 1024).with_progress(40);
        assert_eq!(mid.status, FileUploadStatus::Uploading);
        assert!(mid.is_uploading());

        let done = FileUploadItem::new("b", "doc.pdf", 2048).with_progress(100);
        assert_eq!(done.status, FileUploadStatus::Complete);
        assert!(done.is_complete());

        let bad = FileUploadItem::new("c", "big.zip", 9999).with_error("Too large");
        assert!(bad.is_error());
        assert_eq!(bad.error.as_deref(), Some("Too large"));
    }

    #[test]
    fn file_upload_spec_files_drive_list() {
        use super::{FileUploadItem, FileUploadSpec};
        let spec = FileUploadSpec::new()
            .with_file(FileUploadItem::new("a", "one.png", 10).with_preview(true))
            .with_file(FileUploadItem::new("b", "two.txt", 20));
        assert!(spec.has_files());
        assert_eq!(spec.files.len(), 2);
        assert!(spec.files[0].has_preview);
    }

    #[test]
    fn media_picker_spec_tab_and_items() {
        use super::{MediaKind, MediaPickerItem, MediaPickerSpec, MediaPickerTab};
        let spec = MediaPickerSpec::new("Pick")
            .with_item(MediaPickerItem::new("i1", "Banner", MediaKind::Image).with_thumbnail(true))
            .with_item(MediaPickerItem::new("d1", "Readme", MediaKind::Document))
            .with_active_tab(MediaPickerTab::Upload);
        assert!(spec.has_items());
        assert_eq!(spec.items.len(), 2);
        assert!(!spec.is_browsing());
        assert!(spec.items[0].has_thumbnail);
        assert!(!spec.items[1].has_thumbnail);
    }
}
