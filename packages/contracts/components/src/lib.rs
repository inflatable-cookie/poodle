mod accordion;
mod alert_dialog;
mod badge;
mod banner;
mod r#box;
mod breadcrumbs;
mod bulk_action_bar;
mod button;
mod calendar;
mod call_out;
mod card;
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
mod form_actions;
mod form_dialog;
mod form_layout;
mod grid;
mod hover_card;
mod icon;
mod icon_button;
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
mod surface;
mod switch;
mod tab_strip;
mod table;
mod tabs;
mod text_input;
mod time_ago;
mod time_field;
mod time_zone_select;
mod toggle_group;
mod toolbar;
mod tooltip;
mod tri_state_switch;
mod types;

// Composite modules
mod action_discovery_panel;
mod app_header;
mod audio_player;
mod block_editor;
mod card_radio_group;
mod command_palette;
mod composite_types;
mod confirm_action;
mod data_table;
mod detail_section;
mod detail_shell;
mod dock_region;
mod editable_list;
mod embed_input;
mod embed_preview;
mod empty_state;
mod filter_toolbar;
mod form_shell;
mod inline_remediation;
mod list_container;
mod log_list;
mod markdown_editor;
mod media_browse_panel;
mod media_picker;
mod media_preview;
mod media_thumbnail;
mod metric_tile;
mod page_header;
mod page_loading;
mod pagination_summary;
mod picker_shell;
mod relation_picker;
mod remediation_banner;
mod selection_summary;
mod shell_status_bar;
mod sidebar_nav;
mod split_view;
mod state_tile;
mod toast_host;
mod toast_stack;
mod validation_summary;
mod video_player;

pub use accordion::AccordionSpec;
pub use alert_dialog::{AlertDialogSpec, AlertDialogTone};
pub use badge::BadgeSpec;
pub use banner::BannerSpec;
pub use breadcrumbs::{BreadcrumbItem, BreadcrumbsSpec};
pub use bulk_action_bar::{BulkAction, BulkActionBarSpec, BulkActionTone};
pub use button::ButtonSpec;
pub use calendar::{CalendarMode, CalendarSpec};
pub use call_out::CallOutSpec;
pub use card::{CardLayout, CardSpec, CardVariant};
pub use checkbox::CheckboxSpec;
pub use code::CodeSpec;
pub use code_input::CodeInputSpec;
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
pub use eyebrow::EyebrowSpec;
pub use field::{FieldRelationships, FieldSpec};
pub use field_set::{FieldSetSpec, SpaceScale};
pub use file_upload::FileUploadSpec;
pub use form_actions::FormActionsSpec;
pub use form_dialog::FormDialogSpec;
pub use form_layout::FormLayoutSpec;
pub use grid::GridSpec;
pub use hover_card::HoverCardSpec;
pub use icon::{IconSize, IconSpec};
pub use icon_button::IconButtonSpec;
pub use list_card::{LeadingFill, LeadingShape, ListCardSpec};
pub use list_card_counter::ListCardCounterSpec;
pub use list_grid::{ListGridSpec, ListGridVariant};
pub use menu::MenuSpec;
pub use menubar::MenubarSpec;
pub use meta_bar::MetaBarSpec;
pub use meta_item::MetaItemSpec;
pub use meter::MeterSpec;
pub use nav_card::NavCardSpec;
pub use navigation_menu::NavigationMenuSpec;
pub use number_input::NumberInputSpec;
pub use order_by::{ActiveSort, OrderBySpec, SortDirection, SortField};
pub use pagination::{PageItem, PaginationSpec, PaginationVariant};
pub use password_requirements::{PasswordRequirementsPolicy, PasswordRequirementsSpec};
pub use pill::{PillAppearance, PillFont, PillSize, PillSpec, PillTone};
pub use popover::PopoverSpec;
pub use progress::ProgressSpec;
pub use r#box::BoxSpec;
pub use radio_group::RadioGroupSpec;
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
pub use slider::SliderSpec;
pub use spacer::SpacerSpec;
pub use spinner::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};
pub use split_button::{SplitButtonSpec, SplitMenuItem};
pub use stack::{LayoutJustify, StackDirection, StackSpec};
pub use status_indicator::StatusIndicatorSpec;
pub use surface::SurfaceSpec;
pub use switch::{SwitchSpec, SwitchTone};
pub use tab_strip::TabStripSpec;
pub use table::{ColumnAlign, TableColumn, TableRow, TableSpec};
pub use tabs::TabsSpec;
/// Deprecated: use `TextInputSpec` with `rows > 1` instead.
pub type TextAreaSpec = TextInputSpec;
pub use text_input::TextInputSpec;
pub use time_ago::TimeAgoSpec;
pub use time_field::TimeFieldSpec;
pub use time_zone_select::TimeZoneSelectSpec;
pub use toolbar::ToolbarSpec;
/// Deprecated: Toggle has been superseded by Button with `pressed` prop.
pub type ToggleSpec = ButtonSpec;
pub use date_time_zone_picker::DateTimeZonePickerSpec;
pub use toggle_group::{ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};
pub use tooltip::TooltipSpec;
pub use tri_state_switch::TriStateSwitchSpec;
/// Deprecated: Use `DateTimeZonePickerSpec` instead.
pub type ZonedDateTimePickerSpec = DateTimeZonePickerSpec;
pub use types::{
    AccordionItemSpec, AccordionSelectionValue, Alignment, BadgeVariant, ButtonTone, ButtonVariant,
    CalendarWeekStart, CheckState, ChoiceOption, ControlDensity, ControlSize, DateRangeValue,
    DateTimeRangeValue, DateTimeValue, DialogKind, DialogWidth, Dimension, Direction, DrawerEdge,
    FormActionAlign, Inset, MenuEntry, MenuItemKind, MenubarEntry, NavigationMenuEntry,
    Orientation, Overflow, OverlayPlacement, PaddingScale, PopoverInitialFocus, RuleTone,
    InlineTypographyMode, SemanticControlSizeRole, SeparatorOrientation, StatusTone, SurfaceBorder, SurfaceRole,
    SurfaceTone, TabActivationMode, TabDefinition, TabStripItem, TabVariant, ValidationState,
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
    SelectionSummaryItem, SplitOrientation, TableColumnSpec, TableFilter, TablePagination,
    TableRowSpec, TableSortDirection, ValidationSummaryEntry,
};
pub use confirm_action::ConfirmActionSpec;
pub use data_table::DataTableSpec;
pub use detail_section::DetailSectionSpec;
pub use detail_shell::{DetailShellSpec, DetailState};
pub use dock_region::{DockRegionSpec, DockTabsPlacement};
pub use editable_list::EditableListSpec;
pub use embed_input::EmbedInputSpec;
pub use embed_preview::EmbedPreviewSpec;
pub use empty_state::EmptyStateSpec;
pub use filter_toolbar::FilterToolbarSpec;
pub use form_shell::FormShellSpec;
pub use inline_remediation::InlineRemediationSpec;
pub use list_container::{ListContainerSpec, ListContainerState};
pub use log_list::LogListSpec;
pub use markdown_editor::MarkdownEditorSpec;
pub use media_browse_panel::{MediaBrowseItem, MediaBrowsePanelSpec};
pub use media_picker::MediaPickerSpec;
pub use media_preview::MediaPreviewSpec;
pub use media_thumbnail::MediaThumbnailSpec;
pub use metric_tile::{MetricTileSpec, MetricTrend};
pub use page_header::{PageHeaderAlign, PageHeaderSpec};
pub use page_loading::{PageLoadingPresentation, PageLoadingSpec};
pub use pagination_summary::PaginationSummarySpec;
pub use picker_shell::PickerShellSpec;
pub use relation_picker::{
    DrillDownConfig, DrillDownItem, DrillDownLeafGroup, DrillDownLevel, RelationPickerSpec,
};
pub use remediation_banner::RemediationBannerSpec;
pub use selection_summary::SelectionSummarySpec;
pub use shell_status_bar::ShellStatusBarSpec;
pub use sidebar_nav::{SidebarNavGroup, SidebarNavItem, SidebarNavSpec};
pub use split_view::SplitViewSpec;
pub use state_tile::StateTileSpec;
pub use toast_host::{ToastHostPlacement, ToastHostSpec};
pub use toast_stack::{Toast, ToastPosition, ToastStackSpec, ToastTone};
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
        Direction, DrawerEdge, DrawerSpec, FieldSpec, FormActionAlign, FormActionsSpec, GridSpec,
        IconButtonSpec, MenuEntry, MenuItemKind, MenuSpec, MenubarEntry, MenubarSpec,
        NavigationMenuEntry, NavigationMenuSpec, Orientation, OverlayPlacement, PaddingScale,
        PopoverInitialFocus, PopoverSpec, ProgressSpec, RadioGroupSpec, ScrollShellSpec,
        SegmentedControlSpec, SelectSpec, SeparatorSpec, SliderSpec, StackSpec,
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
        TableColumnSpec, TableRowSpec, TableSortDirection, ValidationSummaryEntry,
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
        assert_eq!(spec.resolved_column_gap(), Some(semantic::SPACE_INLINE_MD));
        assert_eq!(spec.resolved_row_gap(), Some(semantic::SPACE_STACK_MD));
    }

    #[test]
    fn elevated_surface_uses_elevated_background_and_overlay_shadow() {
        let spec = SurfaceSpec::new().with_tone(SurfaceTone::Elevated);
        assert_eq!(
            spec.resolved_background_token(),
            semantic::COLOR_BACKGROUND_ELEVATED
        );
        assert_eq!(spec.resolved_shadow_token(), semantic::ELEVATION_OVERLAY);
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
            .with_pressed(true);

        assert!(spec.has_required_icon());
        assert!(spec.has_required_accessible_name());
        assert!(spec.uses_pressed_semantics());
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

        assert_eq!(spec.rows, 4);
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
            .with_step(300)
            .with_validation_state(ValidationState::Pending);
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
            .with_kind(DialogKind::AlertDialog)
            .with_default_open(true)
            .with_title("Delete review");
        let drawer = DrawerSpec::new()
            .with_default_open(true)
            .with_edge(DrawerEdge::Left)
            .with_modal(true);

        assert!(dialog.current_open());
        assert!(dialog.is_alert_dialog());
        assert!(!dialog.effective_dismiss_on_backdrop());
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
        assert_eq!(toolbar.gap_token(), semantic::SPACE_STACK_SM);
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
        assert_eq!(relation.as_picker_shell("Attach").selected_count, 1);
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
}
