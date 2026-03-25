//! Per-component specimen renderers.
//!
//! Each module renders an interactive specimen demo for a single component,
//! mirroring the Svelte preview's per-component specimen pages.

// ── Structural ────────────────────────────────────────────
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
mod text_area;
mod search_field;
mod field;
mod form_actions;
mod number_entry;
mod pin_input;
mod toolbar;
mod editable_label;
mod toggle;

// ── Selection ─────────────────────────────────────────────
mod checkbox;
mod radio_group;
mod switch;
mod tri_state_switch;
mod select;
mod combobox;
mod segmented_control;
mod toggle_group;
mod slider;
mod range_slider;

// ── Date/Time ────────────────────────────────────────────
mod calendar;
mod range_calendar;
mod date_picker;
mod date_range_picker;
mod time_field;
mod date_time_picker;
mod date_time_range_picker;
mod time_zone_select;
mod zoned_date_time_picker;

// ── Feedback ──────────────────────────────────────────────
mod progress;
mod status_indicator;
mod meter;
mod rating;
mod skeleton;
mod pill;
mod eyebrow;
mod temporal;
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
mod detail_row_specimen;
mod detail_section_specimen;
mod detail_shell;
mod list_card;
mod picker;
mod page_structure;
mod pagination;
mod state_display;
mod misc_composites;
mod form_layout;
mod reorderable_list;
mod media;
mod editors;

// ── Layout Helpers ───────────────────────────────────────
mod collapse_toggle;
mod region;
mod resize_handle;
mod nav_card_grid;

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
    let elevated_bg = theme.resolve_color("semantic.color.background.elevated");
    let border_default = theme.resolve_color("semantic.color.border.default");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

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
    let accent = theme.resolve_color("semantic.color.accent.base");
    let border = theme.resolve_color("semantic.color.border.default");

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
        "icon" => specimen_card("Icon", theme, icon::render(theme)),

        // ── Action ──────────────────────────────────────────────
        "button" => specimen_card("Button", theme, button::render(state, cx)),
        "icon-button" => specimen_card("IconButton", theme, icon_button::render(state, cx)),
        "split-button" => specimen_card("SplitButton", theme, split_button::render(state, cx)),
        "text-input" => specimen_card("TextInput", theme, text_input::render(state, cx)),
        "text-area" => specimen_card("TextArea", theme, text_area::render(state, cx)),
        "search-field" => specimen_card("SearchField", theme, search_field::render(state, cx)),
        "field" => specimen_card("Field", theme, field::render(state, cx)),
        "form-actions" => specimen_card("FormActions", theme, form_actions::render(state, cx)),
        "number-entry" => specimen_card("NumberEntry", theme, number_entry::render(state, cx)),
        "pin-input" => specimen_card("PinInput", theme, pin_input::render(state, cx)),
        "toolbar" => specimen_card("Toolbar", theme, toolbar::render(state, cx)),
        "time-field" => specimen_card("TimeField", theme, time_field::render(state, cx)),
        "editable-label" => specimen_card("EditableLabel", theme, editable_label::render(state, cx)),
        "toggle" => specimen_card("Toggle", theme, toggle::render(state, cx)),
        "toggle-group" => specimen_card("ToggleGroup", theme, toggle_group::render(state, cx)),

        // ── Selection ───────────────────────────────────────────
        "checkbox" => specimen_card("Checkbox", theme, checkbox::render(state, cx)),
        "radio-group" => specimen_card("RadioGroup", theme, radio_group::render(state, cx)),
        "switch" => specimen_card("Switch", theme, switch::render(state, cx)),
        "tri-state-switch" => specimen_card("TriStateSwitch", theme, tri_state_switch::render(state, cx)),
        "select" => specimen_card("Select", theme, select::render(state, cx)),
        "combobox" => specimen_card("Combobox", theme, combobox::render(state, cx)),
        "segmented-control" => specimen_card("SegmentedControl", theme, segmented_control::render(state, cx)),
        "slider" => specimen_card("Slider", theme, slider::render(state, cx)),
        "range-slider" => specimen_card("RangeSlider", theme, range_slider::render(theme)),

        // ── Date/Time ───────────────────────────────────────────
        "calendar" => specimen_card("Calendar", theme, calendar::render(state, cx)),
        "range-calendar" => specimen_card("RangeCalendar", theme, range_calendar::render(theme)),
        "date-picker" => specimen_card("DatePicker", theme, date_picker::render(state, cx)),
        "date-range-picker" => specimen_card("DateRangePicker", theme, date_range_picker::render(state, cx)),
        "date-time-picker" => specimen_card("DateTimePicker", theme, date_time_picker::render(state, cx)),
        "date-time-range-picker" => specimen_card("DateTimeRangePicker", theme, date_time_range_picker::render(state, cx)),
        "time-zone-select" => specimen_card("TimeZoneSelect", theme, time_zone_select::render(state, cx)),
        "zoned-date-time-picker" => specimen_card("ZonedDateTimePicker", theme, zoned_date_time_picker::render(state, cx)),

        // ── Feedback ────────────────────────────────────────────
        "progress" => specimen_card("Progress", theme, progress::render(theme)),
        "pill" => specimen_card("Pill", theme, pill::render(theme)),
        "status-indicator" => specimen_card("StatusIndicator", theme, status_indicator::render(theme)),
        "meter" => specimen_card("Meter", theme, meter::render(theme)),
        "rating" => specimen_card("Rating", theme, rating::render(state, cx)),
        "skeleton" => specimen_card("Skeleton", theme, skeleton::render(theme)),
        "eyebrow" => specimen_card("Eyebrow", theme, eyebrow::render(theme)),
        "time-ago" | "duration-input" => specimen_card("TimeAgo + DurationInput", theme, temporal::render(theme)),
        "code" => specimen_card("Code", theme, code::render(theme)),
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
        "pagination" => specimen_card("Pagination", theme, pagination::render(theme)),
        "form-layout" => specimen_card("FormLayout", theme, form_layout::render(state, cx)),
        "reorderable-list" => specimen_card("ReorderableList", theme, reorderable_list::render(theme)),
        "detail-shell" => specimen_card("DetailShell", theme, detail_shell::render(theme)),
        "detail-row" => specimen_card("DetailRow", theme, detail_row_specimen::render(theme)),
        "detail-section" => specimen_card("DetailSection", theme, detail_section_specimen::render(theme)),
        "card" => specimen_card("Card", theme, card_specimen::render(theme)),
        "card-radio-group" => specimen_card("CardRadioGroup", theme, card_radio_group_specimen::render(state, cx)),
        "nav-card" => specimen_card("NavCard", theme, nav_card_grid::render(theme)),
        "picker-shell" | "relation-picker" | "selection-summary" | "order-by" => specimen_card("PickerShell", theme, picker::render(state, cx)),
        "page-header" | "breadcrumbs" | "page-loading" | "pagination-summary" => specimen_card("Page Structure", theme, page_structure::render(theme)),
        "metric-tile" | "state-tile" | "empty-state" | "toast-stack" => specimen_card("State Display", theme, state_display::render(state, cx)),
        "confirm-action" | "form-dialog" | "filter-toolbar" | "bulk-action-bar"
        | "inline-editable-field" | "log-list" | "editable-list" | "autonomous-list" | "embed-input"
        | "embed-preview" => specimen_card("Misc", theme, misc_composites::render(state, cx)),
        "audio-player" | "video-player" | "media-picker" | "media-preview" | "media-thumbnail" => {
            specimen_card("Media", theme, media::render(state, cx))
        }
        "markdown-editor" | "block-editor" => specimen_card("Editors", theme, editors::render(state, cx)),

        // ── Layout Helpers ─────────────────────────────────────
        "collapse-toggle" => specimen_card("CollapseToggle", theme, collapse_toggle::render(state, cx)),
        "region" => specimen_card("Region", theme, region::render(theme)),
        "resize-handle" => specimen_card("ResizeHandle", theme, resize_handle::render(theme)),
        "nav-card-grid" => specimen_card("NavCardGrid", theme, nav_card_grid::render(theme)),

        // ── App Shell ───────────────────────────────────────────
        "app-header" => specimen_card("AppHeader", theme, app_header::render(theme)),
        "command-palette" => specimen_card("CommandPalette", theme, command_palette::render(state, cx)),
        "dock-region" | "split-view" => specimen_card("Dock + SplitView", theme, dock_split::render(state, cx)),
        "status-bar" => specimen_card("StatusBar", theme, status_bar::render(state, cx)),
        "action-discovery-panel" => specimen_card("ActionDiscovery", theme, action_discovery::render(state, cx)),

        // Fallback
        _ => simple_specimen(slug, theme),
    }
}
