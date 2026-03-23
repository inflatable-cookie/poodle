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
mod collapse_toggle;
mod collapsible;
mod color_picker;
mod combobox;
mod context_menu;
mod date_picker;
mod date_range_picker;
mod date_time_picker;
mod date_time_range_picker;
mod detail_row;
mod dialog;
mod drawer;
mod duration_input;
mod editable_label;
mod eyebrow;
mod field;
mod file_upload;
mod form_actions;
mod grid;
mod hover_card;
mod icon;
mod icon_button;
mod list_card;
mod menu;
mod menubar;
mod meter;
mod nav_card;
mod nav_card_grid;
mod navigation_menu;
mod number_entry;
mod order_by;
mod pagination;
mod pill;
mod pin_input;
mod popover;
mod progress;
mod radio_group;
mod range_calendar;
mod range_slider;
mod rating;
mod region;
mod resize_handle;
mod scroll_shell;
mod search_field;
mod segmented_control;
mod select;
mod separator;
mod skeleton;
mod slider;
mod split_button;
mod stack;
mod status_indicator;
mod surface;
mod switch;
mod tab_strip;
mod tabs;
mod table;
mod text_area;
mod text_input;
mod time_ago;
mod time_field;
mod time_zone_select;
mod toolbar;
mod toggle;
mod toggle_group;
mod tooltip;
mod tri_state_switch;
mod types;
mod zoned_date_time_picker;

pub use accordion::AccordionSpec;
pub use alert_dialog::{AlertDialogSpec, AlertDialogTone};
pub use badge::BadgeSpec;
pub use banner::BannerSpec;
pub use breadcrumbs::{BreadcrumbItem, BreadcrumbsSpec};
pub use bulk_action_bar::{BulkAction, BulkActionBarSpec, BulkActionTone};
pub use button::ButtonSpec;
pub use calendar::CalendarSpec;
pub use call_out::CallOutSpec;
pub use card::{CardLayout, CardSpec, CardVariant};
pub use checkbox::CheckboxSpec;
pub use code::CodeSpec;
pub use collapse_toggle::{CollapseDirection, CollapseToggleSpec};
pub use collapsible::CollapsibleSpec;
pub use color_picker::{ColorPickerSpec, ColorInputMode};
pub use combobox::{ComboboxOption, ComboboxSpec};
pub use context_menu::ContextMenuSpec;
pub use date_picker::DatePickerSpec;
pub use date_range_picker::DateRangePickerSpec;
pub use date_time_picker::DateTimePickerSpec;
pub use date_time_range_picker::DateTimeRangePickerSpec;
pub use detail_row::DetailRowSpec;
pub use dialog::DialogSpec;
pub use drawer::DrawerSpec;
pub use duration_input::DurationInputSpec;
pub use editable_label::EditableLabelSpec;
pub use eyebrow::EyebrowSpec;
pub use field::{FieldRelationships, FieldSpec};
pub use file_upload::FileUploadSpec;
pub use form_actions::FormActionsSpec;
pub use grid::GridSpec;
pub use hover_card::HoverCardSpec;
pub use icon::{IconSize, IconSpec};
pub use icon_button::IconButtonSpec;
pub use list_card::{LeadingFill, LeadingShape, ListCardSpec};
pub use menu::MenuSpec;
pub use menubar::MenubarSpec;
pub use meter::MeterSpec;
pub use nav_card::NavCardSpec;
pub use nav_card_grid::NavCardGridSpec;
pub use navigation_menu::NavigationMenuSpec;
pub use number_entry::NumberEntrySpec;
pub use order_by::{ActiveSort, OrderBySpec, SortDirection, SortField};
pub use pagination::{PageItem, PaginationSpec};
pub use pill::{PillAppearance, PillFont, PillSize, PillSpec, PillTone};
pub use pin_input::PinInputSpec;
pub use popover::PopoverSpec;
pub use progress::ProgressSpec;
pub use r#box::BoxSpec;
pub use radio_group::RadioGroupSpec;
pub use range_calendar::RangeCalendarSpec;
pub use range_slider::RangeSliderSpec;
pub use rating::RatingSpec;
pub use region::RegionSpec;
pub use resize_handle::ResizeHandleSpec;
pub use scroll_shell::ScrollShellSpec;
pub use search_field::SearchFieldSpec;
pub use segmented_control::SegmentedControlSpec;
pub use select::SelectSpec;
pub use separator::SeparatorSpec;
pub use skeleton::SkeletonSpec;
pub use slider::SliderSpec;
pub use split_button::{SplitButtonSpec, SplitMenuItem};
pub use stack::{LayoutJustify, StackDirection, StackSpec};
pub use status_indicator::StatusIndicatorSpec;
pub use surface::SurfaceSpec;
pub use switch::SwitchSpec;
pub use tab_strip::TabStripSpec;
pub use tabs::TabsSpec;
pub use table::{ColumnAlign, TableColumn, TableRow, TableSpec};
pub use text_area::TextAreaSpec;
pub use text_input::TextInputSpec;
pub use time_ago::TimeAgoSpec;
pub use time_field::TimeFieldSpec;
pub use time_zone_select::TimeZoneSelectSpec;
pub use toolbar::ToolbarSpec;
pub use toggle::{ToggleLayout, ToggleSpec};
pub use toggle_group::{ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};
pub use tooltip::TooltipSpec;
pub use tri_state_switch::TriStateSwitchSpec;
pub use zoned_date_time_picker::ZonedDateTimePickerSpec;
pub use types::{
    AccordionItemSpec, AccordionSelectionValue, Alignment, BadgeVariant, ButtonTone, ButtonVariant,
    CalendarWeekStart, CheckState, ChoiceOption, ControlSize, DateRangeValue, DateTimeRangeValue,
    DateTimeValue, DialogKind, Dimension, Direction, DrawerEdge, FormActionAlign, Inset, MenuEntry,
    MenuItemKind, MenubarEntry, NavigationMenuEntry, Orientation, Overflow, OverlayPlacement,
    PaddingScale, PopoverInitialFocus, RuleTone, SeparatorOrientation, StatusTone, SurfaceBorder,
    SurfaceRole, SurfaceTone, TabActivationMode, TabDefinition, TabStripItem, TabVariant,
    ValidationState,
};

pub const CURRENT_GENERATION: &str = "g06.002";
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
    "TextAreaSpec",
    "SearchFieldSpec",
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
    "BadgeSpec",
    "StatusIndicatorSpec",
    "CalendarSpec",
    "RangeCalendarSpec",
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

#[cfg(test)]
mod tests {
    use poodle_tokens::semantic;

    use super::{
        AccordionItemSpec, AccordionSelectionValue, AccordionSpec, BadgeSpec, BadgeVariant,
        BoxSpec, ButtonSpec, ButtonVariant, CalendarSpec, CalendarWeekStart, CheckState,
        CheckboxSpec, ChoiceOption, CollapsibleSpec, ContextMenuSpec, ControlSize, DatePickerSpec,
        DateRangePickerSpec, DateRangeValue, DateTimePickerSpec, DateTimeRangePickerSpec,
        DateTimeRangeValue, DateTimeValue, DialogKind, DialogSpec, Direction, DrawerEdge,
        DrawerSpec, FieldSpec, FormActionAlign, FormActionsSpec, GridSpec, IconButtonSpec,
        MenuEntry, MenuItemKind, MenuSpec, MenubarEntry, MenubarSpec, NavigationMenuEntry,
        NavigationMenuSpec, Orientation, OverlayPlacement, PaddingScale, PopoverInitialFocus,
        PopoverSpec, ProgressSpec, RadioGroupSpec, RangeCalendarSpec, ScrollShellSpec,
        SearchFieldSpec, SegmentedControlSpec, SelectSpec, SeparatorSpec, SliderSpec, StackSpec,
        StatusIndicatorSpec, StatusTone, SurfaceSpec, SurfaceTone, SwitchSpec, TabActivationMode,
        TabDefinition, TabStripItem, TabStripSpec, TabsSpec, TextAreaSpec, TextInputSpec,
        TimeFieldSpec, TooltipSpec, ValidationState,
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
    fn text_area_defaults_to_four_rows_and_reports_pending_busy_state() {
        let spec = TextAreaSpec::new().with_validation_state(ValidationState::Pending);

        assert_eq!(spec.rows, 4);
        assert_eq!(spec.aria_busy(), Some("true"));
        assert_eq!(spec.border_token(), semantic::COLOR_ACCENT_BASE);
    }

    #[test]
    fn search_field_exposes_clear_action_only_when_query_exists() {
        let spec = SearchFieldSpec::new().with_default_value("track lane");
        let input = spec.as_text_input_spec();

        assert!(spec.shows_clear_action());
        assert_eq!(input.input_type, "search");
        assert_eq!(input.current_value(), "track lane");
        assert_eq!(input.leading_icon.as_deref(), Some("search"));
        assert_eq!(input.trailing_icon.as_deref(), Some("clear"));
    }

    #[test]
    fn form_actions_default_to_end_alignment_with_wrap_behavior() {
        let spec = FormActionsSpec::new().with_align(FormActionAlign::Between);

        assert_eq!(spec.align, FormActionAlign::Between);
        assert_eq!(spec.action_gap_token(), semantic::SPACE_INLINE_MD);
        assert_eq!(spec.stack_separation_token(), semantic::SPACE_STACK_SM);
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
        let calendar = RangeCalendarSpec::new().with_default_value(range.clone());
        let picker = DateRangePickerSpec::new()
            .with_default_value(range.clone())
            .with_open(true);

        assert_eq!(calendar.current_value(), &range);
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
}
