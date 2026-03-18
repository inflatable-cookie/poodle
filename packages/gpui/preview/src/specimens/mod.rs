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
mod banner;
mod callout;
mod inline;
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
mod number_entry;
mod pin_input;
mod toolbar;
mod editable_label;

// ── Selection ─────────────────────────────────────────────
mod checkbox;
mod radio_group;
mod switch;
mod select;
mod segmented_control;
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

// ── Feedback ──────────────────────────────────────────────
mod progress;
mod badge;
mod status_indicator;
mod meter;
mod rating;
mod skeleton;
mod pill;
mod temporal;
mod code;
mod color_picker;
mod file_upload;

// ── Overlay ───────────────────────────────────────────────
mod accordion;
mod collapsible;
mod dialog;
mod drawer;
mod popover;
mod hover_card;
mod tooltip;
mod menu;
mod tabs;
mod navigation_menu;
mod menubar;

// ── Composites ────────────────────────────────────────────
mod data_table;
mod detail_shell;
mod cards;
mod picker;
mod page_structure;
mod state_display;
mod misc_composites;
mod media;
mod editors;

// ── Workstation ───────────────────────────────────────────
mod workspace;
mod project_header;
mod command_palette;
mod panel;
mod dock_split;
mod status_bar;
mod action_discovery;

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
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
    let border = theme.resolve_color("semantic.color.border.default");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .p(px(12.0))
        .rounded(px(6.0))
        .bg(color_to_hsla(elevated_bg))
        .border_1()
        .border_color(color_to_hsla(border))
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
        "banner" => specimen_card("Banner", theme, banner::render(theme)),
        "callout" => specimen_card("Callout", theme, callout::render(theme)),
        "inline" => specimen_card("Inline", theme, inline::render(theme)),
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
        "field" | "form-actions" => specimen_card("Field + FormActions", theme, field::render(state, cx)),
        "number-entry" => specimen_card("NumberEntry", theme, number_entry::render(state, cx)),
        "pin-input" => specimen_card("PinInput", theme, pin_input::render(theme)),
        "toolbar" => specimen_card("Toolbar", theme, toolbar::render(state, cx)),
        "time-field" => specimen_card("TimeField", theme, time_field::render(theme)),
        "editable-label" => specimen_card("EditableLabel", theme, editable_label::render(theme)),

        // ── Selection ───────────────────────────────────────────
        "checkbox" => specimen_card("Checkbox", theme, checkbox::render(state, cx)),
        "radio-group" => specimen_card("RadioGroup", theme, radio_group::render(state, cx)),
        "switch" | "tri-state-switch" => specimen_card("Switch", theme, switch::render(state, cx)),
        "select" => specimen_card("Select", theme, select::render(state, cx)),
        "segmented-control" => specimen_card("SegmentedControl", theme, segmented_control::render(state, cx)),
        "slider" => specimen_card("Slider", theme, slider::render(theme)),
        "range-slider" => specimen_card("RangeSlider", theme, range_slider::render(theme)),

        // ── Date/Time ───────────────────────────────────────────
        "calendar" => specimen_card("Calendar", theme, calendar::render(theme)),
        "range-calendar" => specimen_card("RangeCalendar", theme, range_calendar::render(theme)),
        "date-picker" => specimen_card("DatePicker", theme, date_picker::render(theme)),
        "date-range-picker" => specimen_card("DateRangePicker", theme, date_range_picker::render(theme)),
        "date-time-picker" => specimen_card("DateTimePicker", theme, date_time_picker::render(theme)),
        "date-time-range-picker" => specimen_card("DateTimeRangePicker", theme, date_time_range_picker::render(theme)),

        // ── Feedback ────────────────────────────────────────────
        "progress" => specimen_card("Progress", theme, progress::render(theme)),
        "badge" => specimen_card("Badge", theme, badge::render(theme)),
        "status-indicator" => specimen_card("StatusIndicator", theme, status_indicator::render(theme)),
        "meter" => specimen_card("Meter", theme, meter::render(theme)),
        "rating" => specimen_card("Rating", theme, rating::render(state, cx)),
        "skeleton" => specimen_card("Skeleton", theme, skeleton::render(theme)),
        "pill" | "eyebrow" => specimen_card("Pill + Eyebrow", theme, pill::render(theme)),
        "time-ago" | "duration-input" => specimen_card("TimeAgo + DurationInput", theme, temporal::render(theme)),
        "code" => specimen_card("Code", theme, code::render(theme)),
        "color-picker" => specimen_card("ColorPicker", theme, color_picker::render(state, cx)),
        "file-upload" => specimen_card("FileUpload", theme, file_upload::render(theme)),

        // ── Overlay ─────────────────────────────────────────────
        "accordion" => specimen_card("Accordion", theme, accordion::render(state, cx)),
        "collapsible" => specimen_card("Collapsible", theme, collapsible::render(state, cx)),
        "dialog" | "alert-dialog" => specimen_card("Dialog", theme, dialog::render(state, cx)),
        "drawer" => specimen_card("Drawer", theme, drawer::render(state, cx)),
        "popover" => specimen_card("Popover", theme, popover::render(state, cx)),
        "hover-card" => specimen_card("HoverCard", theme, hover_card::render(theme)),
        "tooltip" => specimen_card("Tooltip", theme, tooltip::render(theme)),
        "menu" | "context-menu" => specimen_card("Menu", theme, menu::render(state, cx)),
        "tabs" => specimen_card("Tabs", theme, tabs::render(state, cx)),
        "navigation-menu" => specimen_card("NavigationMenu", theme, navigation_menu::render(state, cx)),
        "menubar" => specimen_card("Menubar", theme, menubar::render(theme)),

        // ── Composites ──────────────────────────────────────────
        "data-table" => specimen_card("DataTable", theme, data_table::render(state, cx)),
        "detail-shell" | "detail-row" | "detail-section" => specimen_card("DetailShell", theme, detail_shell::render(theme)),
        "list-shell" | "list-card" | "reorderable-list" => specimen_card("ListShell", theme, detail_shell::render(theme)),
        "nav-card" | "nav-card-grid" | "card" | "card-radio-group" => specimen_card("Cards", theme, cards::render(state, cx)),
        "picker-shell" | "relation-picker" | "selection-summary" | "order-by" => specimen_card("PickerShell", theme, picker::render(state, cx)),
        "page-header" | "breadcrumbs" | "page-loading" | "pagination-summary" => specimen_card("Page Structure", theme, page_structure::render(theme)),
        "state-tile" | "empty-state" | "toast-stack" => specimen_card("State Display", theme, state_display::render(theme)),
        "confirm-action" | "form-dialog" | "filter-toolbar" | "bulk-action-bar" | "slug-field"
        | "inline-editable-field" | "log-list" | "autonomous-list" | "embed-input"
        | "embed-preview" | "embed-shell" | "grid-shell" => specimen_card("Misc", theme, misc_composites::render(theme)),
        "audio-player" | "video-player" | "media-picker" | "media-preview" | "media-thumbnail" => {
            specimen_card("Media", theme, media::render(state, cx))
        }
        "markdown-editor" | "block-editor" => specimen_card("Editors", theme, editors::render(theme)),

        // ── Workstation ─────────────────────────────────────────
        "workspace-shell" | "app-header" => specimen_card("WorkspaceShell", theme, workspace::render(theme)),
        "project-header" => specimen_card("ProjectHeader", theme, project_header::render(theme)),
        "command-palette" | "command-palette-shell" => specimen_card("CommandPalette", theme, command_palette::render(state, cx)),
        "panel-header" | "panel-surface" | "panel-tabs" => specimen_card("Panel", theme, panel::render(state, cx)),
        "dock-region" | "split-view" => specimen_card("Dock + SplitView", theme, dock_split::render(theme)),
        "shell-status-bar" | "surface-tabs" => specimen_card("StatusBar + Tabs", theme, status_bar::render(state, cx)),
        "action-discovery-panel" => specimen_card("ActionDiscovery", theme, action_discovery::render(state, cx)),

        // Fallback
        _ => simple_specimen(slug, theme),
    }
}
