//! Per-component specimen renderers.
//!
//! Each module renders an interactive specimen demo for a single component,
//! mirroring the Svelte preview's per-component specimen pages.

// ── Structural ────────────────────────────────────────────
pub(crate) mod specimen_layout;

mod bx;
mod stack;
mod grid;
mod surface;
mod separator;
mod scroll_shell;
mod callout;
mod spacer;

// ── Foundation ────────────────────────────────────────────
mod icon;

// ── Action ────────────────────────────────────────────────
mod button;
mod icon_button;
mod split_button;
mod text_input;
mod field;
mod form_actions;
mod number_input;
mod code_input;
mod toolbar;
mod editable_label;

// ── Selection ─────────────────────────────────────────────
mod checkbox;
mod radio_group;
mod switch;
mod tri_state_switch;
mod select;
mod segmented_control;
mod toggle_group;
mod slider;
mod range_slider;

// ── Date/Time ────────────────────────────────────────────
mod calendar;
mod date_picker;
mod date_range_picker;
mod time_field;
mod date_time_picker;
mod date_time_range_picker;
mod time_zone_select;
mod date_time_zone_picker;

// ── Feedback ──────────────────────────────────────────────
mod progress;
mod status_indicator;
mod meter;
mod rating;
mod skeleton;
mod spinner;
mod pill;
mod eyebrow;
mod code;
mod color_picker;
mod file_upload;

// ── Overlay ───────────────────────────────────────────────
mod accordion;
mod collapsible;
mod alert_dialog;
mod dialog;
mod drawer;
mod popover;
mod hover_card;
mod tooltip;
mod menu;
mod context_menu;
mod tabs;
mod navigation_menu;
mod menubar;

// ── Composites ────────────────────────────────────────────
mod data_table;
mod table;
mod card_radio_group_specimen;
mod card_specimen;
mod detail_item_specimen;
mod detail_section_specimen;
mod detail_shell;
mod breadcrumbs_specimen;
mod duration_input_specimen;
mod empty_state_specimen;
mod list_card;
mod list_container_specimen;
mod media_browse_panel_specimen;
mod media_upload_status_panel_specimen;
mod metric_tile_specimen;
mod page_header_specimen;
mod page_loading_specimen;
mod order_by_specimen;
mod pagination_summary_specimen;
mod picker_shell_specimen;
mod relation_picker_specimen;
mod selection_summary_specimen;
mod pagination;
mod state_display;
mod time_ago_specimen;
mod audio_player_specimen;
mod block_editor_specimen;
mod bulk_action_bar_specimen;
mod confirm_action_specimen;
mod editable_list_specimen;
mod embed_input_specimen;
mod embed_preview_specimen;
mod field_set_specimen;
mod filter_toolbar_specimen;
mod form_dialog_specimen;
mod log_list_specimen;
mod markdown_editor_specimen;
mod media_picker_specimen;
mod media_preview_specimen;
mod media_thumbnail_specimen;
mod split_view_specimen;
mod video_player_specimen;
mod form_layout;

// ── Layout Helpers ───────────────────────────────────────
mod collapse_toggle;
mod region;
mod resize_handle;

// ── App Shell ─────────────────────────────────────────────
mod app_header;
mod command_palette;
mod dock_split;
mod status_bar;
mod action_discovery;

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;
use crate::app_state::AppState;
use crate::PreviewRoot;

/// Render a specimen card wrapper with title.
pub fn specimen_card(
    title: &str,
    theme: &GpuiThemeProvider,
    content: impl IntoElement,
) -> Div {
    let elevated_bg = theme.resolve_color("color.background.elevated");
    let border_default = theme.resolve_color("color.border.default");
    let text_secondary = theme.resolve_color("color.text.secondary");

    // Match Svelte app.css .panel treatment:
    //   fill: color-mix(elevated 94%, transparent)
    //   border: color-mix(border-default 22%, transparent)
    //   shadow: elevation-surface
    let bg = color_to_hsla(elevated_bg);
    let bg = Hsla { a: bg.a * 0.94, ..bg };
    let border = color_to_hsla(border_default);
    let border = Hsla { a: border.a * 0.22, ..border };

    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .p(px(12.0))
        .rounded(px(6.0))
        .bg(bg)
        .border_1()
        .border_color(border)
        .shadow(vec![
            gpui::BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.08),
                offset: point(px(0.0), px(2.0)),
                blur_radius: px(8.0),
                spread_radius: px(0.0),
            },
            gpui::BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.04),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            },
        ])
        .child(
            div()
                .text_xs()
                .text_color(color_to_hsla(text_secondary))
                .child(title.to_string()),
        )
        .child(content)
}

/// Render a simple specimen showing a styled box with a label.
pub fn simple_specimen(
    label: &str,
    theme: &GpuiThemeProvider,
) -> Div {
    let accent = theme.resolve_color("color.accent.base");
    let border = theme.resolve_color("color.border.default");

    div()
        .h(px(32.0))
        .px(px(10.0))
        .rounded(px(4.0))
        .border_1()
        .border_color(color_to_hsla(border))
        .flex()
        .items_center()
        .child(
            div()
                .text_xs()
                .text_color(color_to_hsla(accent))
                .child(label.to_string()),
        )
}

/// Render a single specimen by component slug.
pub fn render_single_specimen(
    slug: &str,
    state: &AppState,
    cx: &mut Context<PreviewRoot>,
) -> Div {
    let theme = &state.theme;
    match slug {
        // ── Structural ──────────────────────────────────────────
        "box" => specimen_card("Box", theme, bx::render(theme)),
        "stack" => specimen_card("Stack", theme, stack::render(theme)),
        "grid" => specimen_card("Grid", theme, grid::render(theme)),
        "surface" => specimen_card("Surface", theme, surface::render(theme)),
        "separator" => specimen_card("Separator", theme, separator::render(theme)),
        "scroll-shell" => specimen_card("ScrollShell", theme, scroll_shell::render(theme)),
        "callout" => specimen_card("Callout", theme, callout::render(state, cx)),
        "spacer" => specimen_card("Spacer", theme, spacer::render(theme)),

        // ── Foundation ──────────────────────────────────────────
        "icon" => specimen_card("Icon", theme, icon::render(state, cx)),

        // ── Action ──────────────────────────────────────────────
        "button" => specimen_card("Button", theme, button::render(state, cx)),
        "icon-button" => specimen_card("IconButton", theme, icon_button::render(state, cx)),
        "split-button" => specimen_card("SplitButton", theme, split_button::render(state, cx)),
        "text-input" => specimen_card("TextInput", theme, text_input::render(state, cx)),
        "field" => specimen_card("Field", theme, field::render(state, cx)),
        "field-set" => specimen_card("FieldSet", theme, field_set_specimen::render(theme)),
        "form-actions" => specimen_card("FormActions", theme, form_actions::render(state, cx)),
        "number-input" => specimen_card("NumberInput", theme, number_input::render(state, cx)),
        "code-input" => specimen_card("CodeInput", theme, code_input::render(state, cx)),
        "toolbar" => specimen_card("Toolbar", theme, toolbar::render(state, cx)),
        "time-field" => specimen_card("TimeField", theme, time_field::render(state, cx)),
        "editable-label" => specimen_card("EditableLabel", theme, editable_label::render(state, cx)),
        "toggle-group" => specimen_card("ToggleGroup", theme, toggle_group::render(state, cx)),

        // ── Selection ───────────────────────────────────────────
        "checkbox" => specimen_card("Checkbox", theme, checkbox::render(state, cx)),
        "radio-group" => specimen_card("RadioGroup", theme, radio_group::render(state, cx)),
        "switch" => specimen_card("Switch", theme, switch::render(state, cx)),
        "tri-state-switch" => specimen_card("TriStateSwitch", theme, tri_state_switch::render(state, cx)),
        "select" => specimen_card("Select", theme, select::render(state, cx)),
        "segmented-control" => specimen_card("SegmentedControl", theme, segmented_control::render(state, cx)),
        "slider" => specimen_card("Slider", theme, slider::render(state, cx)),
        "range-slider" => specimen_card("RangeSlider", theme, range_slider::render(state, cx)),

        // ── Date/Time ───────────────────────────────────────────
        "calendar" => specimen_card("Calendar", theme, calendar::render(state, cx)),
        "date-picker" => specimen_card("DatePicker", theme, date_picker::render(state, cx)),
        "date-range-picker" => specimen_card("DateRangePicker", theme, date_range_picker::render(state, cx)),
        "date-time-picker" => specimen_card("DateTimePicker", theme, date_time_picker::render(state, cx)),
        "date-time-range-picker" => specimen_card("DateTimeRangePicker", theme, date_time_range_picker::render(state, cx)),
        "time-zone-select" => specimen_card("TimeZoneSelect", theme, time_zone_select::render(state, cx)),
        "date-time-zone-picker" => specimen_card("DateTimeZonePicker", theme, date_time_zone_picker::render(state, cx)),

        // ── Feedback ────────────────────────────────────────────
        "progress" => specimen_card("Progress", theme, progress::render(state, cx)),
        "pill" => specimen_card("Pill", theme, pill::render(state, cx)),
        "status-indicator" => specimen_card("StatusIndicator", theme, status_indicator::render(theme)),
        "meter" => specimen_card("Meter", theme, meter::render(theme)),
        "rating" => specimen_card("Rating", theme, rating::render(state, cx)),
        "skeleton" => specimen_card("Skeleton", theme, skeleton::render(theme)),
        "spinner" => specimen_card("Spinner", theme, spinner::render(state, cx)),
        "eyebrow" => specimen_card("Eyebrow", theme, eyebrow::render(theme)),
        "time-ago" => specimen_card("TimeAgo", theme, time_ago_specimen::render(theme)),
        "duration-input" => specimen_card("DurationInput", theme, duration_input_specimen::render(state, cx)),
        "code" => specimen_card("Code", theme, code::render(state, cx)),
        "color-picker" => specimen_card("ColorPicker", theme, color_picker::render(state, cx)),
        "file-upload" => specimen_card("FileUpload", theme, file_upload::render(theme)),

        // ── Overlay ─────────────────────────────────────────────
        "accordion" => specimen_card("Accordion", theme, accordion::render(state, cx)),
        "collapsible" => specimen_card("Collapsible", theme, collapsible::render(state, cx)),
        "dialog" => specimen_card("Dialog", theme, dialog::render(state, cx)),
        "alert-dialog" => specimen_card("AlertDialog", theme, alert_dialog::render(state, cx)),
        "drawer" => specimen_card("Drawer", theme, drawer::render(state, cx)),
        "popover" => specimen_card("Popover", theme, popover::render(state, cx)),
        "hover-card" => specimen_card("HoverCard", theme, hover_card::render(theme)),
        "tooltip" => specimen_card("Tooltip", theme, tooltip::render(theme)),
        "menu" => specimen_card("Menu", theme, menu::render(state, cx)),
        "context-menu" => specimen_card("ContextMenu", theme, context_menu::render(state, cx)),
        "tabs" => specimen_card("Tabs", theme, tabs::render(state, cx)),
        "navigation-menu" => specimen_card("NavigationMenu", theme, navigation_menu::render(state, cx)),
        "menubar" => specimen_card("Menubar", theme, menubar::render(state, cx)),

        // ── Composites ──────────────────────────────────────────
        "table" => specimen_card("Table", theme, table::render(theme)),
        "data-table" => specimen_card("DataTable", theme, data_table::render(state, cx)),
        "list-card" => specimen_card("ListCard", theme, list_card::render(state, cx)),
        "pagination" => specimen_card("Pagination", theme, pagination::render(state, cx)),
        "form-layout" => specimen_card("FormLayout", theme, form_layout::render(state, cx)),
        "detail-shell" => specimen_card("DetailShell", theme, detail_shell::render(theme)),
        "detail-item" => specimen_card("DetailItem", theme, detail_item_specimen::render(theme)),
        "detail-section" => specimen_card("DetailSection", theme, detail_section_specimen::render(theme)),
        "card" => specimen_card("Card", theme, card_specimen::render(theme)),
        "card-radio-group" => specimen_card("CardRadioGroup", theme, card_radio_group_specimen::render(state, cx)),
        "picker-shell" => specimen_card("PickerShell", theme, picker_shell_specimen::render(theme)),
        "relation-picker" => specimen_card("RelationPicker", theme, relation_picker_specimen::render(state, cx)),
        "selection-summary" => specimen_card("SelectionSummary", theme, selection_summary_specimen::render(state, cx)),
        "order-by" => specimen_card("OrderBy", theme, order_by_specimen::render(state, cx)),
        "page-header" => specimen_card("PageHeader", theme, page_header_specimen::render(theme)),
        "breadcrumbs" => specimen_card("Breadcrumbs", theme, breadcrumbs_specimen::render(state, cx)),
        "page-loading" => specimen_card("PageLoading", theme, page_loading_specimen::render(theme)),
        "pagination-summary" => specimen_card("PaginationSummary", theme, pagination_summary_specimen::render(theme)),
        "metric-tile" | "state-tile" => specimen_card("MetricTile", theme, metric_tile_specimen::render(theme)),
        "empty-state" => specimen_card("EmptyState", theme, empty_state_specimen::render(theme)),
        "toast-stack" => specimen_card("ToastStack", theme, state_display::render(state, cx)),
        "confirm-action" => specimen_card("ConfirmAction", theme, confirm_action_specimen::render(state, cx)),
        "form-dialog" => specimen_card("FormDialog", theme, form_dialog_specimen::render(theme)),
        "filter-toolbar" => specimen_card("FilterToolbar", theme, filter_toolbar_specimen::render(theme)),
        "bulk-action-bar" => specimen_card("BulkActionBar", theme, bulk_action_bar_specimen::render(theme)),
        // inline-editable-field was merged into EditableLabel
        "log-list" => specimen_card("LogList", theme, log_list_specimen::render(theme)),
        "editable-list" => specimen_card("EditableList", theme, editable_list_specimen::render(theme)),
        "embed-input" => specimen_card("EmbedInput", theme, embed_input_specimen::render(theme)),
        "embed-preview" => specimen_card("EmbedPreview", theme, embed_preview_specimen::render(theme)),
        // autonomous-list was renamed to editable-list
        "audio-player" => specimen_card("AudioPlayer", theme, audio_player_specimen::render(theme)),
        "video-player" => specimen_card("VideoPlayer", theme, video_player_specimen::render(theme)),
        "media-picker" => specimen_card("MediaPicker", theme, media_picker_specimen::render(theme)),
        "media-preview" => specimen_card("MediaPreview", theme, media_preview_specimen::render(theme)),
        "media-thumbnail" => specimen_card("MediaThumbnail", theme, media_thumbnail_specimen::render(theme)),
        "media-browse-panel" => specimen_card("MediaBrowsePanel", theme, media_browse_panel_specimen::render(theme)),
        "media-upload-status-panel" => specimen_card("MediaUploadStatusPanel", theme, media_upload_status_panel_specimen::render(theme)),
        "list-container" => specimen_card("ListContainer", theme, list_container_specimen::render(state, cx)),
        "markdown-editor" => specimen_card("MarkdownEditor", theme, markdown_editor_specimen::render(state, cx)),
        "block-editor" => specimen_card("BlockEditor", theme, block_editor_specimen::render(theme)),

        // ── Layout Helpers ─────────────────────────────────────
        "collapse-toggle" => specimen_card("CollapseToggle", theme, collapse_toggle::render(state, cx)),
        "region" => specimen_card("Region", theme, region::render(theme)),
        "resize-handle" => specimen_card("ResizeHandle", theme, resize_handle::render(theme)),

        // ── App Shell ───────────────────────────────────────────
        "app-header" => specimen_card("AppHeader", theme, app_header::render(theme)),
        "command-palette" => specimen_card("CommandPalette", theme, command_palette::render(state, cx)),
        "dock-region" => specimen_card("DockRegion", theme, dock_split::render(state, cx)),
        "split-view" => specimen_card("SplitView", theme, split_view_specimen::render(theme)),
        "status-bar" => specimen_card("StatusBar", theme, status_bar::render(state, cx)),
        "action-discovery-panel" => specimen_card("ActionDiscovery", theme, action_discovery::render(state, cx)),

        // Fallback
        _ => simple_specimen(slug, theme),
    }
}
