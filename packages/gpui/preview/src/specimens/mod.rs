//! Per-component specimen renderers.
//!
//! Each module renders an interactive specimen demo for a single component,
//! mirroring the Svelte preview's per-component specimen pages.

// ── Structural ────────────────────────────────────────────
pub(crate) mod specimen_layout;

mod bx;
mod callout;
mod grid;
mod scroll_shell;
mod separator;
mod spacer;
mod stack;
mod surface;

// ── Foundation ────────────────────────────────────────────
mod avatar;
mod icon;
mod icon_provider;
mod ui_presentation_provider;

// ── Action ────────────────────────────────────────────────
mod button;
mod code_input;
mod editable_label;
mod field;
mod form_actions;
mod icon_button;
mod number_input;
mod split_button;
mod text;
mod text_input;
mod text_link;
mod token_input;
mod toolbar;

// ── Selection ─────────────────────────────────────────────
mod checkbox;
mod radio_group;
mod range_slider;
mod segmented_control;
mod agent_transcript;
mod stepper;
mod select;
mod slider;
mod switch;
mod toggle_group;
mod tri_state_switch;

// ── Date/Time ────────────────────────────────────────────
mod calendar;
mod date_picker;
mod date_range_picker;
mod date_time_picker;
mod date_time_range_picker;
mod date_time_zone_picker;
mod time_field;
mod time_zone_select;

// ── Feedback ──────────────────────────────────────────────
mod code;
mod color_picker;
mod eyebrow;
mod file_upload;
mod meter;
mod pill;
mod progress;
mod rating;
mod skeleton;
mod spinner;
mod status_indicator;

// ── Overlay ───────────────────────────────────────────────
mod accordion;
mod alert_dialog;
mod collapsible;
mod context_menu;
mod dialog;
mod drawer;
mod hover_card;
mod menu;
mod menubar;
mod navigation_menu;
mod overlay_state;
mod popover;
mod tab_strip;
mod tabs;
mod tooltip;

// ── Composites ────────────────────────────────────────────
mod audio_player_specimen;
mod block_editor_specimen;
mod breadcrumbs_specimen;
mod bulk_action_bar_specimen;
mod card_radio_group_specimen;
mod card_specimen;
mod card_toggle_group_specimen;
mod confirm_action_specimen;
mod data_table;
mod debug_dialog_specimen;
mod detail_item_specimen;
mod detail_section_group_specimen;
mod detail_section_specimen;
mod detail_shell;
mod duration_input_specimen;
mod editable_list_specimen;
mod embed_input_specimen;
mod embed_preview_specimen;
mod empty_state_specimen;
mod error_boundary_specimen;
mod field_set_specimen;
mod filter_toolbar_specimen;
mod form_dialog_specimen;
mod form_layout;
mod form_shell;
mod inline_list_section_specimen;
mod inline_remediation_specimen;
mod list_card;
mod list_card_counter;
mod list_container_specimen;
mod list_grid;
mod log_list_specimen;
mod markdown_editor_specimen;
mod media_browse_panel_specimen;
mod media_picker_specimen;
mod media_preview_specimen;
mod media_thumbnail_specimen;
mod meta_bar;
mod metric_tile_specimen;
mod nav_card;
mod agent_chat_input_specimen;
mod filter_builder_specimen;
mod model_picker_specimen;
mod ref_select_specimen;
mod theme_select_specimen;
mod order_by_specimen;
mod page_header_specimen;
mod page_loading_specimen;
mod pagination;
mod pagination_summary_specimen;
mod password_requirements;
mod picker_shell_specimen;
mod relation_picker_specimen;
mod selection_summary_specimen;
mod sidebar_nav;
mod split_view_specimen;
mod table;
mod time_ago_specimen;
mod toast_host;
mod toast_stack_specimen;
mod tree;
mod validation_summary;
mod video_player_specimen;

// ── Layout Helpers ───────────────────────────────────────
mod collapse_toggle;
mod region;
mod resize_handle;

// ── App Shell ─────────────────────────────────────────────
mod action_discovery;
mod app_header;
mod command_palette;
mod dock_split;
mod status_bar;

use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

/// Render a specimen card wrapper with title.
pub fn specimen_card(title: &str, theme: &GpuiThemeProvider, content: impl IntoElement) -> Div {
    let elevated_bg = theme.resolve_color("color.background.elevated");
    let border_default = theme.resolve_color("color.border.default");
    let text_secondary = theme.resolve_color("color.text.secondary");

    // Match Svelte app.css .panel treatment:
    //   fill: color-mix(elevated 94%, transparent)
    //   border: color-mix(border-default 22%, transparent)
    //   shadow: elevation-surface
    let bg = color_to_hsla(elevated_bg);
    let bg = Hsla {
        a: bg.a * 0.94,
        ..bg
    };
    let border = color_to_hsla(border_default);
    let border = Hsla {
        a: border.a * 0.22,
        ..border
    };

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

/// Render an honest placeholder when a registry entry has no specimen yet.
pub fn missing_specimen(display_name: &str, theme: &GpuiThemeProvider) -> Div {
    let border = theme.resolve_color("color.border.subtle");
    let text_secondary = theme.resolve_color("color.text.secondary");

    div()
        .p(px(32.0))
        .rounded(px(8.0))
        .border_1()
        .border_color(color_to_hsla(border).opacity(0.6))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(6.0))
        .child(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child(format!("Specimen not yet available for {}.", display_name)),
        )
        .child(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child("Check back as we build out interactive demos for each component."),
        )
}

/// Render a single specimen by component slug.
pub fn render_single_specimen(slug: &str, state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
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
        "avatar" => specimen_card("Avatar", theme, avatar::render(theme)),
        "icon" => specimen_card("Icon", theme, icon::render(state, cx)),
        "icon-provider" => specimen_card("IconProvider", theme, icon_provider::render(theme)),
        "ui-presentation-provider" => specimen_card(
            "UiPresentationProvider",
            theme,
            ui_presentation_provider::render(theme),
        ),

        // ── Action ──────────────────────────────────────────────
        "button" => specimen_card("Button", theme, button::render(state, cx)),
        "icon-button" => specimen_card("IconButton", theme, icon_button::render(state, cx)),
        "split-button" => specimen_card("SplitButton", theme, split_button::render(state, cx)),
        "text" => specimen_card("Text", theme, text::render(theme)),
        "text-input" => specimen_card("TextInput", theme, text_input::render(state, cx)),
        "text-link" => specimen_card("TextLink", theme, text_link::render(theme)),
        "token-input" => specimen_card("TokenInput", theme, token_input::render(state, cx)),
        "field" => specimen_card("Field", theme, field::render(state, cx)),
        "field-set" => specimen_card("FieldSet", theme, field_set_specimen::render(theme)),
        "form-actions" => specimen_card("FormActions", theme, form_actions::render(state, cx)),
        "password-requirements" => specimen_card(
            "PasswordRequirements",
            theme,
            password_requirements::render(theme),
        ),
        "number-input" => specimen_card("NumberInput", theme, number_input::render(state, cx)),
        "code-input" => specimen_card("CodeInput", theme, code_input::render(state, cx)),
        "toolbar" => specimen_card("Toolbar", theme, toolbar::render(state, cx)),
        "time-input" => specimen_card("TimeInput", theme, time_field::render(state, cx)),
        "editable-label" => {
            specimen_card("EditableLabel", theme, editable_label::render(state, cx))
        }
        "toggle-group" => specimen_card("ToggleGroup", theme, toggle_group::render(state, cx)),

        // ── Selection ───────────────────────────────────────────
        "checkbox" => specimen_card("Checkbox", theme, checkbox::render(state, cx)),
        "radio-group" => specimen_card("RadioGroup", theme, radio_group::render(state, cx)),
        "switch" => specimen_card("Switch", theme, switch::render(state, cx)),
        "tri-state-switch" => {
            specimen_card("TriStateSwitch", theme, tri_state_switch::render(state, cx))
        }
        "select" => specimen_card("Select", theme, select::render(state, cx)),
        "segmented-control" => specimen_card(
            "SegmentedControl",
            theme,
            segmented_control::render(state, cx),
        ),
        "stepper" => specimen_card("Stepper", theme, stepper::render(state, cx)),
        "agent-transcript" => specimen_card(
            "AgentTranscript",
            theme,
            agent_transcript::render(state, cx),
        ),
        "slider" => specimen_card("Slider", theme, slider::render(state, cx)),
        "range-slider" => specimen_card("RangeSlider", theme, range_slider::render(state, cx)),

        // ── Date/Time ───────────────────────────────────────────
        "calendar" => specimen_card("Calendar", theme, calendar::render(state, cx)),
        "date-picker" => specimen_card("DatePicker", theme, date_picker::render(state, cx)),
        "date-range-picker" => specimen_card(
            "DateRangePicker",
            theme,
            date_range_picker::render(state, cx),
        ),
        "date-time-picker" => {
            specimen_card("DateTimePicker", theme, date_time_picker::render(state, cx))
        }
        "date-time-range-picker" => specimen_card(
            "DateTimeRangePicker",
            theme,
            date_time_range_picker::render(state, cx),
        ),
        "time-zone-select" => {
            specimen_card("TimeZoneSelect", theme, time_zone_select::render(state, cx))
        }
        "date-time-zone-picker" => specimen_card(
            "DateTimeZonePicker",
            theme,
            date_time_zone_picker::render(state, cx),
        ),

        // ── Feedback ────────────────────────────────────────────
        "progress" => specimen_card("Progress", theme, progress::render(state, cx)),
        "pill" => specimen_card("Pill", theme, pill::render(state, cx)),
        "status-indicator" => {
            specimen_card("StatusIndicator", theme, status_indicator::render(theme))
        }
        "meter" => specimen_card("Meter", theme, meter::render(theme)),
        "meta-bar" => specimen_card("MetaBar", theme, meta_bar::render(theme)),
        "meta-item" => specimen_card("MetaItem", theme, meta_bar::render_meta_item(theme)),
        "rating" => specimen_card("Rating", theme, rating::render(state, cx)),
        "skeleton" => specimen_card("Skeleton", theme, skeleton::render(theme)),
        "spinner" => specimen_card("Spinner", theme, spinner::render(state, cx)),
        "eyebrow" => specimen_card("Eyebrow", theme, eyebrow::render(theme)),
        "time-ago" => specimen_card("TimeAgo", theme, time_ago_specimen::render(theme)),
        "duration-input" => specimen_card(
            "DurationInput",
            theme,
            duration_input_specimen::render(state, cx),
        ),
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
        "hover-card" => specimen_card("HoverCard", theme, hover_card::render(state, cx)),
        "tooltip" => specimen_card("Tooltip", theme, tooltip::render(state, cx)),
        "menu" => specimen_card("Menu", theme, menu::render(state, cx)),
        "context-menu" => specimen_card("ContextMenu", theme, context_menu::render(state, cx)),
        "tabs" => specimen_card("Tabs", theme, tabs::render(state, cx)),
        "tab-strip" => specimen_card("TabStrip", theme, tab_strip::render(state, cx)),
        "navigation-menu" => {
            specimen_card("NavigationMenu", theme, navigation_menu::render(state, cx))
        }
        "menubar" => specimen_card("Menubar", theme, menubar::render(state, cx)),

        // ── Composites ──────────────────────────────────────────
        "table" => specimen_card("Table", theme, table::render(theme)),
        "data-table" => specimen_card("DataTable", theme, data_table::render(state, cx)),
        "list-card" => specimen_card("ListCard", theme, list_card::render(state, cx)),
        "list-card-counter" => {
            specimen_card("ListCardCounter", theme, list_card_counter::render(theme))
        }
        "list-grid" => specimen_card("ListGrid", theme, list_grid::render(state, cx)),
        "nav-card" => specimen_card("NavCard", theme, nav_card::render(theme)),
        "pagination" => specimen_card("Pagination", theme, pagination::render(state, cx)),
        "form-layout" => specimen_card("FormLayout", theme, form_layout::render(state, cx)),
        "form-shell" => specimen_card("FormShell", theme, form_shell::render(state, cx)),
        "validation-summary" => specimen_card(
            "ValidationSummary",
            theme,
            validation_summary::render(state, cx),
        ),
        "detail-shell" => specimen_card("DetailShell", theme, detail_shell::render(theme)),
        "detail-item" => specimen_card("DetailItem", theme, detail_item_specimen::render(theme)),
        "detail-section" => specimen_card(
            "DetailSection",
            theme,
            detail_section_specimen::render(theme),
        ),
        "detail-section-group" => specimen_card(
            "DetailSectionGroup",
            theme,
            detail_section_group_specimen::render(theme),
        ),
        "card" => specimen_card("Card", theme, card_specimen::render(theme)),
        "card-radio-group" => specimen_card(
            "CardRadioGroup",
            theme,
            card_radio_group_specimen::render(state, cx),
        ),
        "card-toggle-group" => specimen_card(
            "CardToggleGroup",
            theme,
            card_toggle_group_specimen::render(theme),
        ),
        "picker-shell" => specimen_card("PickerShell", theme, picker_shell_specimen::render(theme)),
        "relation-picker" => specimen_card(
            "RelationPicker",
            theme,
            relation_picker_specimen::render(state, cx),
        ),
        "selection-summary" => specimen_card(
            "SelectionSummary",
            theme,
            selection_summary_specimen::render(state, cx),
        ),
        "sidebar-nav" => specimen_card("SidebarNav", theme, sidebar_nav::render(theme)),
        "tree" => specimen_card("Tree", theme, tree::render(state, cx)),
        "filter-builder" => {
            specimen_card("FilterBuilder", theme, filter_builder_specimen::render(state, cx))
        }
        "model-picker" => {
            specimen_card("ModelPicker", theme, model_picker_specimen::render(state, cx))
        }
        "ref-select" => {
            specimen_card("RefSelect", theme, ref_select_specimen::render(state, cx))
        }
        "agent-chat-input" => specimen_card(
            "AgentChatInput",
            theme,
            agent_chat_input_specimen::render(state, cx),
        ),
        "theme-select" => {
            specimen_card("ThemeSelect", theme, theme_select_specimen::render(state, cx))
        }
        "order-by" => specimen_card("OrderBy", theme, order_by_specimen::render(state, cx)),
        "page-header" => specimen_card("PageHeader", theme, page_header_specimen::render(theme)),
        "breadcrumbs" => specimen_card(
            "Breadcrumbs",
            theme,
            breadcrumbs_specimen::render(state, cx),
        ),
        "page-loading" => specimen_card("PageLoading", theme, page_loading_specimen::render(theme)),
        "pagination-summary" => specimen_card(
            "PaginationSummary",
            theme,
            pagination_summary_specimen::render(theme),
        ),
        "metric-tile" => specimen_card("MetricTile", theme, metric_tile_specimen::render(theme)),
        "empty-state" => specimen_card("EmptyState", theme, empty_state_specimen::render(theme)),
        "error-boundary" => specimen_card(
            "ErrorBoundary",
            theme,
            error_boundary_specimen::render(theme),
        ),
        "toast-stack" => specimen_card("ToastStack", theme, toast_stack_specimen::render(theme)),
        "toast-host" => specimen_card("ToastHost", theme, toast_host::render(theme)),
        "confirm-action" => specimen_card(
            "ConfirmAction",
            theme,
            confirm_action_specimen::render(state, cx),
        ),
        "form-dialog" => specimen_card("FormDialog", theme, form_dialog_specimen::render(theme)),
        "inline-list-section" => specimen_card(
            "InlineListSection",
            theme,
            inline_list_section_specimen::render(theme),
        ),
        "inline-remediation" => specimen_card(
            "InlineRemediation",
            theme,
            inline_remediation_specimen::render(state, cx),
        ),
        "debug-dialog" => specimen_card("DebugDialog", theme, debug_dialog_specimen::render(theme)),
        "filter-toolbar" => specimen_card(
            "FilterToolbar",
            theme,
            filter_toolbar_specimen::render(theme),
        ),
        "bulk-action-bar" => specimen_card(
            "BulkActionBar",
            theme,
            bulk_action_bar_specimen::render(theme),
        ),
        // inline-editable-field was merged into EditableLabel
        "log-list" => specimen_card("LogList", theme, log_list_specimen::render(theme)),
        "editable-list" => {
            specimen_card("EditableList", theme, editable_list_specimen::render(theme))
        }
        "embed-input" => specimen_card("EmbedInput", theme, embed_input_specimen::render(theme)),
        "embed-preview" => {
            specimen_card("EmbedPreview", theme, embed_preview_specimen::render(theme))
        }
        // autonomous-list was renamed to editable-list
        "audio-player" => specimen_card("AudioPlayer", theme, audio_player_specimen::render(theme)),
        "video-player" => specimen_card("VideoPlayer", theme, video_player_specimen::render(theme)),
        "media-picker" => specimen_card("MediaPicker", theme, media_picker_specimen::render(theme)),
        "media-preview" => {
            specimen_card("MediaPreview", theme, media_preview_specimen::render(theme))
        }
        "media-thumbnail" => specimen_card(
            "MediaThumbnail",
            theme,
            media_thumbnail_specimen::render(theme),
        ),
        "media-browse-panel" => specimen_card(
            "MediaBrowsePanel",
            theme,
            media_browse_panel_specimen::render(theme),
        ),
        "list-container" => specimen_card(
            "ListContainer",
            theme,
            list_container_specimen::render(state, cx),
        ),
        "markdown-editor" => specimen_card(
            "MarkdownEditor",
            theme,
            markdown_editor_specimen::render(state, cx),
        ),
        "block-editor" => specimen_card("BlockEditor", theme, block_editor_specimen::render(theme)),

        // ── Layout Helpers ─────────────────────────────────────
        "collapse-toggle" => {
            specimen_card("CollapseToggle", theme, collapse_toggle::render(state, cx))
        }
        "region" => specimen_card("Region", theme, region::render(theme)),
        "resize-handle" => specimen_card("ResizeHandle", theme, resize_handle::render(theme)),

        // ── App Shell ───────────────────────────────────────────
        "app-header" => specimen_card("AppHeader", theme, app_header::render(theme)),
        "command-palette" => {
            specimen_card("CommandPalette", theme, command_palette::render(state, cx))
        }
        "dock-region" => specimen_card("DockRegion", theme, dock_split::render(state, cx)),
        "split-view" => specimen_card("SplitView", theme, split_view_specimen::render(theme)),
        "status-bar" => specimen_card("StatusBar", theme, status_bar::render(state, cx)),
        "action-discovery-panel" => specimen_card(
            "ActionDiscoveryPanel",
            theme,
            action_discovery::render(state, cx),
        ),

        // Fallback
        _ => missing_specimen(slug, theme),
    }
}
