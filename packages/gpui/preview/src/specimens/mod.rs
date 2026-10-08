//! Per-component specimen renderers.
//!
//! Each module renders an interactive specimen demo for a single component,
//! mirroring the Svelte preview's per-component specimen pages.

// ── Structural ────────────────────────────────────────────
pub(crate) mod scene_specimen;
pub(crate) mod specimen_axes;
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
mod motion_policy_provider;
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
mod agent_question;
mod agent_transcript;
mod audio_controls;
mod checkbox;
mod radio;
mod radio_group;
mod range_slider;
mod segmented_control;
mod select;
mod slider;
// `pub(crate)` for its test-only probe markers (g15.042); the render entry
// point is still reached through `render_single_specimen` like every other.
pub(crate) mod stepper;
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
mod time_input;
mod time_zone_select;

// ── Feedback ──────────────────────────────────────────────
mod code;
mod color_picker;
mod eyebrow;
mod file_upload;
mod meta_item;
mod meter;
mod pill;
mod progress;
mod rating;
mod remediation_banner;
mod skeleton;
mod spinner;
mod state_tile;
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
mod agent_chat_input_specimen;
mod agent_message;
mod agent_plan;
mod agent_plan_record;
mod agent_question_record;
mod agent_subagent;
mod audio_player_specimen;
mod block_editor_specimen;
mod breadcrumbs_specimen;
mod bulk_action_bar_specimen;
mod card_radio_group_specimen;
mod card_specimen;
mod card_toggle_group_specimen;
mod changed_files;
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
mod empty_state;
mod error_boundary_specimen;
mod field_set_specimen;
mod filter_builder_specimen;
mod filter_toolbar_specimen;
mod form_dialog_specimen;
mod form_layout;
mod form_shell;
mod history_center_specimen;
mod inline_list_section_specimen;
mod inline_remediation_specimen;
mod licence_activation;
mod licence_seats;
mod licence_status;
mod list_card;
mod list_card_counter;
mod list_container_specimen;
mod list_grid;
mod listbox;
mod log_list_specimen;
mod markdown_editor_specimen;
mod media_browse_panel_specimen;
mod media_picker_specimen;
mod media_preview_specimen;
mod media_thumbnail_specimen;
mod message_center_specimen;
mod meta_bar;
mod metric_tile_specimen;
mod model_catalogue_editor_specimen;
mod model_connection_card_specimen;
mod model_connection_picker_specimen;
mod model_connection_setup_specimen;
mod model_picker_specimen;
mod nav_card;
mod order_by_specimen;
mod page_header_specimen;
mod page_loading_specimen;
mod pagination;
mod pagination_summary_specimen;
mod password_requirements;
mod picker_shell_specimen;
mod ref_select_specimen;
mod relation_picker_specimen;
mod selection_summary_specimen;
mod settings_shell;
mod sidebar_nav;
mod split_view_specimen;
mod table;
mod theme_select_specimen;
mod time_ago_specimen;
mod toast_host;
mod toast_stack_specimen;
mod tool_call;
mod tool_call_group;
mod tree;
mod update_center;
mod update_status;
mod validation_summary;
mod video_player_specimen;

// ── Layout Helpers ───────────────────────────────────────
mod collapse_toggle;
mod region;
mod resize_handle;

// ── App Shell ─────────────────────────────────────────────
mod action_discovery_panel;
mod app_header;
mod command_palette;
mod dock_region;
mod status_bar;

use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_render::audio_specimens::AudioSpecimen;
use poodle_render::RenderContext;

/// Render the interactive preview's specimen card, or a bare wrapper for capture.
pub fn specimen_card(
    capture_mode: bool,
    title: &str,
    theme: &GpuiThemeProvider,
    content: impl IntoElement,
) -> Div {
    if capture_mode {
        return div()
            .w_full()
            .debug_selector(|| "specimen-card".to_string())
            .child(content);
    }

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
        // Test-only marker for the headless specimen probe (g15.026); a no-op
        // outside GPUI's `test-support` builds.
        .debug_selector(|| "specimen-card".to_string())
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
                inset: false,
            },
            gpui::BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.04),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                inset: false,
            },
        ])
        .child(
            div()
                .debug_selector(|| "specimen-card-title".to_string())
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
        // Test-only marker for the headless specimen probe (g15.026); a no-op
        // outside GPUI's `test-support` builds.
        .debug_selector(|| "specimen-missing".to_string())
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
pub fn render_single_specimen(
    slug: &str,
    state: &AppState,
    cx: &mut Context<PreviewRoot>,
    motion_context: &RenderContext<'_>,
) -> Div {
    let theme = &state.theme;
    match slug {
        // ── Structural ──────────────────────────────────────────
        "box" => specimen_card(state.specimen_capture, "Box", theme, bx::render(theme)),
        "stack" => specimen_card(state.specimen_capture, "Stack", theme, stack::render(theme)),
        "grid" => specimen_card(state.specimen_capture, "Grid", theme, grid::render(theme)),
        "surface" => specimen_card(
            state.specimen_capture,
            "Surface",
            theme,
            surface::render(theme),
        ),
        "separator" => specimen_card(
            state.specimen_capture,
            "Separator",
            theme,
            separator::render(theme),
        ),
        "scroll-shell" => specimen_card(
            state.specimen_capture,
            "ScrollShell",
            theme,
            scroll_shell::render(theme),
        ),
        "callout" => specimen_card(
            state.specimen_capture,
            "Callout",
            theme,
            callout::render(state, cx),
        ),
        "spacer" => specimen_card(
            state.specimen_capture,
            "Spacer",
            theme,
            spacer::render(theme),
        ),

        // ── Foundation ──────────────────────────────────────────
        "avatar" => specimen_card(
            state.specimen_capture,
            "Avatar",
            theme,
            avatar::render(state, cx),
        ),
        "icon" => specimen_card(
            state.specimen_capture,
            "Icon",
            theme,
            icon::render(state, cx),
        ),
        "icon-provider" => specimen_card(
            state.specimen_capture,
            "IconProvider",
            theme,
            icon_provider::render(theme),
        ),
        "ui-presentation-provider" => specimen_card(
            state.specimen_capture,
            "UiPresentationProvider",
            theme,
            ui_presentation_provider::render(state, cx),
        ),
        "motion-policy-provider" => specimen_card(
            state.specimen_capture,
            "MotionPolicyProvider",
            theme,
            motion_policy_provider::render(state, cx, motion_context),
        ),

        // ── Action ──────────────────────────────────────────────
        "button" => specimen_card(
            state.specimen_capture,
            "Button",
            theme,
            button::render(state, cx),
        ),
        "icon-button" => specimen_card(
            state.specimen_capture,
            "IconButton",
            theme,
            icon_button::render(state, cx),
        ),
        "split-button" => specimen_card(
            state.specimen_capture,
            "SplitButton",
            theme,
            split_button::render(state, cx),
        ),
        "text" => specimen_card(
            state.specimen_capture,
            "Text",
            theme,
            text::render(state, cx),
        ),
        "text-input" => specimen_card(
            state.specimen_capture,
            "TextInput",
            theme,
            text_input::render(state, cx),
        ),
        "text-link" => specimen_card(
            state.specimen_capture,
            "TextLink",
            theme,
            text_link::render(theme),
        ),
        "token-input" => specimen_card(
            state.specimen_capture,
            "TokenInput",
            theme,
            token_input::render(state, cx),
        ),
        "field" => specimen_card(
            state.specimen_capture,
            "Field",
            theme,
            field::render(state, cx),
        ),
        "field-set" => specimen_card(
            state.specimen_capture,
            "FieldSet",
            theme,
            field_set_specimen::render(theme),
        ),
        "form-actions" => specimen_card(
            state.specimen_capture,
            "FormActions",
            theme,
            form_actions::render(state, cx),
        ),
        "password-requirements" => specimen_card(
            state.specimen_capture,
            "PasswordRequirements",
            theme,
            password_requirements::render(state, cx),
        ),
        "number-input" => specimen_card(
            state.specimen_capture,
            "NumberInput",
            theme,
            number_input::render(state, cx),
        ),
        "code-input" => specimen_card(
            state.specimen_capture,
            "CodeInput",
            theme,
            code_input::render(state, cx),
        ),
        "toolbar" => specimen_card(
            state.specimen_capture,
            "Toolbar",
            theme,
            toolbar::render(state, cx),
        ),
        "time-input" => specimen_card(
            state.specimen_capture,
            "TimeInput",
            theme,
            time_input::render(state, cx),
        ),
        "editable-label" => specimen_card(
            state.specimen_capture,
            "EditableLabel",
            theme,
            editable_label::render(state, cx),
        ),
        "toggle-group" => specimen_card(
            state.specimen_capture,
            "ToggleGroup",
            theme,
            toggle_group::render(state, cx),
        ),

        // ── Selection ───────────────────────────────────────────
        "checkbox" => specimen_card(
            state.specimen_capture,
            "Checkbox",
            theme,
            checkbox::render(state, cx),
        ),
        "radio" => specimen_card(
            state.specimen_capture,
            "Radio",
            theme,
            radio::render(state, cx),
        ),
        "radio-group" => specimen_card(
            state.specimen_capture,
            "RadioGroup",
            theme,
            radio_group::render(state, cx),
        ),
        "switch" => specimen_card(
            state.specimen_capture,
            "Switch",
            theme,
            switch::render(state, cx),
        ),
        "tri-state-switch" => specimen_card(
            state.specimen_capture,
            "TriStateSwitch",
            theme,
            tri_state_switch::render(state, cx),
        ),
        "select" => specimen_card(
            state.specimen_capture,
            "Select",
            theme,
            select::render(state, cx),
        ),
        "listbox" => specimen_card(
            state.specimen_capture,
            "Listbox",
            theme,
            listbox::render(state, cx),
        ),
        "segmented-control" => specimen_card(
            state.specimen_capture,
            "SegmentedControl",
            theme,
            segmented_control::render(state, cx),
        ),
        "stepper" => specimen_card(
            state.specimen_capture,
            "Stepper",
            theme,
            stepper::render(state, cx),
        ),
        "agent-question" => specimen_card(
            state.specimen_capture,
            "AgentQuestion",
            theme,
            agent_question::render(state, cx),
        ),
        "agent-transcript" => specimen_card(
            state.specimen_capture,
            "AgentTranscript",
            theme,
            agent_transcript::render(state, cx),
        ),
        "slider" => specimen_card(
            state.specimen_capture,
            "Slider",
            theme,
            slider::render(state, cx),
        ),
        "knob" => specimen_card(
            state.specimen_capture,
            "Knob",
            theme,
            audio_controls::render(AudioSpecimen::Knob, "knob", state, cx),
        ),
        "fader" => specimen_card(
            state.specimen_capture,
            "Fader",
            theme,
            audio_controls::render(AudioSpecimen::Fader, "fader", state, cx),
        ),
        "audio-meter" => specimen_card(
            state.specimen_capture,
            "AudioMeter",
            theme,
            audio_controls::render(AudioSpecimen::AudioMeter, "audio-meter", state, cx),
        ),
        "value-readout" => specimen_card(
            state.specimen_capture,
            "ValueReadout",
            theme,
            audio_controls::render(AudioSpecimen::ValueReadout, "value-readout", state, cx),
        ),
        "drag-number-field" => specimen_card(
            state.specimen_capture,
            "DragNumberField",
            theme,
            audio_controls::render(
                AudioSpecimen::DragNumberField,
                "drag-number-field",
                state,
                cx,
            ),
        ),
        "envelope-editor" => specimen_card(
            state.specimen_capture,
            "EnvelopeEditor",
            theme,
            audio_controls::render(AudioSpecimen::EnvelopeEditor, "envelope-editor", state, cx),
        ),
        "xy-pad" => specimen_card(
            state.specimen_capture,
            "XYPad",
            theme,
            audio_controls::render(AudioSpecimen::XyPad, "xy-pad", state, cx),
        ),
        "audio-switch" => specimen_card(
            state.specimen_capture,
            "AudioSwitch",
            theme,
            audio_controls::render(AudioSpecimen::AudioSwitch, "audio-switch", state, cx),
        ),
        "gain-reduction-meter" => specimen_card(
            state.specimen_capture,
            "GainReductionMeter",
            theme,
            audio_controls::render(
                AudioSpecimen::GainReductionMeter,
                "gain-reduction-meter",
                state,
                cx,
            ),
        ),
        "keyboard" => specimen_card(
            state.specimen_capture,
            "Keyboard",
            theme,
            audio_controls::render(AudioSpecimen::Keyboard, "keyboard", state, cx),
        ),
        "waveform-display" => specimen_card(
            state.specimen_capture,
            "WaveformDisplay",
            theme,
            audio_controls::render(
                AudioSpecimen::WaveformDisplay,
                "waveform-display",
                state,
                cx,
            ),
        ),
        "mod-matrix-grid" => specimen_card(
            state.specimen_capture,
            "ModMatrixGrid",
            theme,
            audio_controls::render(AudioSpecimen::ModMatrixGrid, "mod-matrix-grid", state, cx),
        ),
        "range-slider" => specimen_card(
            state.specimen_capture,
            "RangeSlider",
            theme,
            range_slider::render(state, cx),
        ),

        // ── Date/Time ───────────────────────────────────────────
        "calendar" => specimen_card(
            state.specimen_capture,
            "Calendar",
            theme,
            calendar::render(state, cx),
        ),
        "date-picker" => specimen_card(
            state.specimen_capture,
            "DatePicker",
            theme,
            date_picker::render(state, cx),
        ),
        "date-range-picker" => specimen_card(
            state.specimen_capture,
            "DateRangePicker",
            theme,
            date_range_picker::render(state, cx),
        ),
        "date-time-picker" => specimen_card(
            state.specimen_capture,
            "DateTimePicker",
            theme,
            date_time_picker::render(state, cx),
        ),
        "date-time-range-picker" => specimen_card(
            state.specimen_capture,
            "DateTimeRangePicker",
            theme,
            date_time_range_picker::render(state, cx),
        ),
        "time-zone-select" => specimen_card(
            state.specimen_capture,
            "TimeZoneSelect",
            theme,
            time_zone_select::render(state, cx),
        ),
        "date-time-zone-picker" => specimen_card(
            state.specimen_capture,
            "DateTimeZonePicker",
            theme,
            date_time_zone_picker::render(state, cx),
        ),

        // ── Feedback ────────────────────────────────────────────
        "progress" => specimen_card(
            state.specimen_capture,
            "Progress",
            theme,
            progress::render(state, cx),
        ),
        "pill" => specimen_card(
            state.specimen_capture,
            "Pill",
            theme,
            pill::render(state, cx),
        ),
        "status-indicator" => specimen_card(
            state.specimen_capture,
            "StatusIndicator",
            theme,
            status_indicator::render(state, cx),
        ),
        "meter" => specimen_card(
            state.specimen_capture,
            "Meter",
            theme,
            meter::render(state, cx),
        ),
        "meta-bar" => specimen_card(
            state.specimen_capture,
            "MetaBar",
            theme,
            meta_bar::render(theme),
        ),
        "meta-item" => specimen_card(
            state.specimen_capture,
            "MetaItem",
            theme,
            meta_item::render(theme),
        ),
        "rating" => specimen_card(
            state.specimen_capture,
            "Rating",
            theme,
            rating::render(state, cx),
        ),
        "skeleton" => specimen_card(
            state.specimen_capture,
            "Skeleton",
            theme,
            skeleton::render(theme, motion_context),
        ),
        "spinner" => specimen_card(
            state.specimen_capture,
            "Spinner",
            theme,
            spinner::render(state, cx, motion_context),
        ),
        "remediation-banner" => specimen_card(
            state.specimen_capture,
            "RemediationBanner",
            theme,
            remediation_banner::render(state, cx),
        ),
        "state-tile" => specimen_card(
            state.specimen_capture,
            "StateTile",
            theme,
            state_tile::render(theme),
        ),
        "eyebrow" => specimen_card(
            state.specimen_capture,
            "Eyebrow",
            theme,
            eyebrow::render(state, cx),
        ),
        "time-ago" => specimen_card(
            state.specimen_capture,
            "TimeAgo",
            theme,
            time_ago_specimen::render(theme),
        ),
        "duration-input" => specimen_card(
            state.specimen_capture,
            "DurationInput",
            theme,
            duration_input_specimen::render(state, cx),
        ),
        "code" => specimen_card(
            state.specimen_capture,
            "Code",
            theme,
            code::render(state, cx),
        ),
        "color-picker" => specimen_card(
            state.specimen_capture,
            "ColorPicker",
            theme,
            color_picker::render(state, cx),
        ),
        "file-upload" => specimen_card(
            state.specimen_capture,
            "FileUpload",
            theme,
            file_upload::render(state, cx),
        ),

        // ── Overlay ─────────────────────────────────────────────
        "accordion" => specimen_card(
            state.specimen_capture,
            "Accordion",
            theme,
            accordion::render(state, cx),
        ),
        "collapsible" => specimen_card(
            state.specimen_capture,
            "Collapsible",
            theme,
            collapsible::render(state, cx),
        ),
        "dialog" => specimen_card(
            state.specimen_capture,
            "Dialog",
            theme,
            dialog::render(state, cx),
        ),
        "alert-dialog" => specimen_card(
            state.specimen_capture,
            "AlertDialog",
            theme,
            alert_dialog::render(state, cx),
        ),
        "drawer" => specimen_card(
            state.specimen_capture,
            "Drawer",
            theme,
            drawer::render(state, cx),
        ),
        "popover" => specimen_card(
            state.specimen_capture,
            "Popover",
            theme,
            popover::render(state, cx),
        ),
        "hover-card" => specimen_card(
            state.specimen_capture,
            "HoverCard",
            theme,
            hover_card::render(state, cx),
        ),
        "tooltip" => specimen_card(
            state.specimen_capture,
            "Tooltip",
            theme,
            tooltip::render(state, cx),
        ),
        "menu" => specimen_card(
            state.specimen_capture,
            "Menu",
            theme,
            menu::render(state, cx),
        ),
        "context-menu" => specimen_card(
            state.specimen_capture,
            "ContextMenu",
            theme,
            context_menu::render(state, cx),
        ),
        "tabs" => specimen_card(
            state.specimen_capture,
            "Tabs",
            theme,
            tabs::render(state, cx),
        ),
        "tab-strip" => specimen_card(
            state.specimen_capture,
            "TabStrip",
            theme,
            tab_strip::render(state, cx),
        ),
        "navigation-menu" => specimen_card(
            state.specimen_capture,
            "NavigationMenu",
            theme,
            navigation_menu::render(state, cx),
        ),
        "menubar" => specimen_card(
            state.specimen_capture,
            "Menubar",
            theme,
            menubar::render(state, cx),
        ),

        // ── Composites ──────────────────────────────────────────
        "table" => specimen_card(
            state.specimen_capture,
            "Table",
            theme,
            table::render(state, cx),
        ),
        "data-table" => specimen_card(
            state.specimen_capture,
            "DataTable",
            theme,
            data_table::render(state, cx),
        ),
        "list-card" => specimen_card(
            state.specimen_capture,
            "ListCard",
            theme,
            list_card::render(state, cx),
        ),
        "list-card-counter" => specimen_card(
            state.specimen_capture,
            "ListCardCounter",
            theme,
            list_card_counter::render(theme),
        ),
        "list-grid" => specimen_card(
            state.specimen_capture,
            "ListGrid",
            theme,
            list_grid::render(state, cx),
        ),
        "nav-card" => specimen_card(
            state.specimen_capture,
            "NavCard",
            theme,
            nav_card::render(state, cx),
        ),
        "pagination" => specimen_card(
            state.specimen_capture,
            "Pagination",
            theme,
            pagination::render(state, cx),
        ),
        "form-layout" => specimen_card(
            state.specimen_capture,
            "FormLayout",
            theme,
            form_layout::render(state, cx),
        ),
        "form-shell" => specimen_card(
            state.specimen_capture,
            "FormShell",
            theme,
            form_shell::render(state, cx),
        ),
        "validation-summary" => specimen_card(
            state.specimen_capture,
            "ValidationSummary",
            theme,
            validation_summary::render(state, cx),
        ),
        "detail-shell" => specimen_card(
            state.specimen_capture,
            "DetailShell",
            theme,
            detail_shell::render(state, cx),
        ),
        "detail-item" => specimen_card(
            state.specimen_capture,
            "DetailItem",
            theme,
            detail_item_specimen::render(state, cx),
        ),
        "detail-section" => specimen_card(
            state.specimen_capture,
            "DetailSection",
            theme,
            detail_section_specimen::render(state, cx),
        ),
        "detail-section-group" => specimen_card(
            state.specimen_capture,
            "DetailSectionGroup",
            theme,
            detail_section_group_specimen::render(state, cx),
        ),
        "card" => specimen_card(
            state.specimen_capture,
            "Card",
            theme,
            card_specimen::render(state, cx),
        ),
        "card-radio-group" => specimen_card(
            state.specimen_capture,
            "CardRadioGroup",
            theme,
            card_radio_group_specimen::render(state, cx),
        ),
        "card-toggle-group" => specimen_card(
            state.specimen_capture,
            "CardToggleGroup",
            theme,
            card_toggle_group_specimen::render(state, cx),
        ),
        "picker-shell" => specimen_card(
            state.specimen_capture,
            "PickerShell",
            theme,
            picker_shell_specimen::render(theme),
        ),
        "relation-picker" => specimen_card(
            state.specimen_capture,
            "RelationPicker",
            theme,
            relation_picker_specimen::render(state, cx),
        ),
        "selection-summary" => specimen_card(
            state.specimen_capture,
            "SelectionSummary",
            theme,
            selection_summary_specimen::render(state, cx),
        ),
        "sidebar-nav" => specimen_card(
            state.specimen_capture,
            "SidebarNav",
            theme,
            sidebar_nav::render(state, cx),
        ),
        "tree" => specimen_card(
            state.specimen_capture,
            "Tree",
            theme,
            tree::render(state, cx),
        ),
        "filter-builder" => specimen_card(
            state.specimen_capture,
            "FilterBuilder",
            theme,
            filter_builder_specimen::render(state, cx),
        ),
        "model-connection-picker" => specimen_card(
            state.specimen_capture,
            "ModelConnectionPicker",
            theme,
            model_connection_picker_specimen::render(state, cx),
        ),
        "model-connection-setup" => specimen_card(
            state.specimen_capture,
            "ModelConnectionSetup",
            theme,
            model_connection_setup_specimen::render(state, cx),
        ),
        "model-connection-card" => specimen_card(
            state.specimen_capture,
            "ModelConnectionCard",
            theme,
            model_connection_card_specimen::render(state, cx),
        ),
        "model-catalogue-editor" => specimen_card(
            state.specimen_capture,
            "ModelCatalogueEditor",
            theme,
            model_catalogue_editor_specimen::render(state, cx),
        ),
        "model-picker" => specimen_card(
            state.specimen_capture,
            "ModelPicker",
            theme,
            model_picker_specimen::render(state, cx),
        ),
        "ref-select" => specimen_card(
            state.specimen_capture,
            "RefSelect",
            theme,
            ref_select_specimen::render(state, cx),
        ),
        "agent-chat-input" => specimen_card(
            state.specimen_capture,
            "AgentChatInput",
            theme,
            agent_chat_input_specimen::render(state, cx),
        ),
        "agent-message" => specimen_card(
            state.specimen_capture,
            "AgentMessage",
            theme,
            agent_message::render(state, cx),
        ),
        "agent-plan" => specimen_card(
            state.specimen_capture,
            "AgentPlan",
            theme,
            agent_plan::render(state, cx),
        ),
        "agent-plan-record" => specimen_card(
            state.specimen_capture,
            "AgentPlanRecord",
            theme,
            agent_plan_record::render(state, cx),
        ),
        "agent-question-record" => specimen_card(
            state.specimen_capture,
            "AgentQuestionRecord",
            theme,
            agent_question_record::render(state, cx),
        ),
        "agent-subagent" => specimen_card(
            state.specimen_capture,
            "AgentSubagent",
            theme,
            agent_subagent::render(state, cx),
        ),
        "changed-files" => specimen_card(
            state.specimen_capture,
            "ChangedFiles",
            theme,
            changed_files::render(state, cx),
        ),
        "tool-call" => specimen_card(
            state.specimen_capture,
            "ToolCall",
            theme,
            tool_call::render(state, cx),
        ),
        "tool-call-group" => specimen_card(
            state.specimen_capture,
            "ToolCallGroup",
            theme,
            tool_call_group::render(state, cx),
        ),
        "theme-select" => specimen_card(
            state.specimen_capture,
            "ThemeSelect",
            theme,
            theme_select_specimen::render(state, cx),
        ),
        "order-by" => specimen_card(
            state.specimen_capture,
            "OrderBy",
            theme,
            order_by_specimen::render(state, cx),
        ),
        "page-header" => specimen_card(
            state.specimen_capture,
            "PageHeader",
            theme,
            page_header_specimen::render(state, cx),
        ),
        "breadcrumbs" => specimen_card(
            state.specimen_capture,
            "Breadcrumbs",
            theme,
            breadcrumbs_specimen::render(state, cx),
        ),
        "page-loading" => specimen_card(
            state.specimen_capture,
            "PageLoading",
            theme,
            page_loading_specimen::render(theme),
        ),
        "pagination-summary" => specimen_card(
            state.specimen_capture,
            "PaginationSummary",
            theme,
            pagination_summary_specimen::render(theme),
        ),
        "metric-tile" => specimen_card(
            state.specimen_capture,
            "MetricTile",
            theme,
            metric_tile_specimen::render(state, cx),
        ),
        "empty-state" => specimen_card(
            state.specimen_capture,
            "EmptyState",
            theme,
            empty_state::render(state, cx),
        ),
        "error-boundary" => specimen_card(
            state.specimen_capture,
            "ErrorBoundary",
            theme,
            error_boundary_specimen::render(theme),
        ),
        "toast-stack" => specimen_card(
            state.specimen_capture,
            "ToastStack",
            theme,
            toast_stack_specimen::render(state, cx),
        ),
        "toast-host" => specimen_card(
            state.specimen_capture,
            "ToastHost",
            theme,
            toast_host::render(state, cx),
        ),
        "confirm-action" => specimen_card(
            state.specimen_capture,
            "ConfirmAction",
            theme,
            confirm_action_specimen::render(state, cx),
        ),
        "form-dialog" => specimen_card(
            state.specimen_capture,
            "FormDialog",
            theme,
            form_dialog_specimen::render(state, cx),
        ),
        "inline-list-section" => specimen_card(
            state.specimen_capture,
            "InlineListSection",
            theme,
            inline_list_section_specimen::render(theme),
        ),
        "inline-remediation" => specimen_card(
            state.specimen_capture,
            "InlineRemediation",
            theme,
            inline_remediation_specimen::render(state, cx),
        ),
        "debug-dialog" => specimen_card(
            state.specimen_capture,
            "DebugDialog",
            theme,
            debug_dialog_specimen::render(theme),
        ),
        "filter-toolbar" => specimen_card(
            state.specimen_capture,
            "FilterToolbar",
            theme,
            filter_toolbar_specimen::render(state, cx),
        ),
        "bulk-action-bar" => specimen_card(
            state.specimen_capture,
            "BulkActionBar",
            theme,
            bulk_action_bar_specimen::render(state, cx),
        ),
        // inline-editable-field was merged into EditableLabel
        "log-list" => specimen_card(
            state.specimen_capture,
            "LogList",
            theme,
            log_list_specimen::render(state, cx),
        ),
        "editable-list" => specimen_card(
            state.specimen_capture,
            "EditableList",
            theme,
            editable_list_specimen::render(state, cx),
        ),
        "embed-input" => specimen_card(
            state.specimen_capture,
            "EmbedInput",
            theme,
            embed_input_specimen::render(state, cx),
        ),
        "embed-preview" => specimen_card(
            state.specimen_capture,
            "EmbedPreview",
            theme,
            embed_preview_specimen::render(theme),
        ),
        // autonomous-list was renamed to editable-list
        "audio-player" => specimen_card(
            state.specimen_capture,
            "AudioPlayer",
            theme,
            audio_player_specimen::render(state, cx),
        ),
        "video-player" => specimen_card(
            state.specimen_capture,
            "VideoPlayer",
            theme,
            video_player_specimen::render(state, cx),
        ),
        "media-picker" => specimen_card(
            state.specimen_capture,
            "MediaPicker",
            theme,
            media_picker_specimen::render(state, cx),
        ),
        "media-preview" => specimen_card(
            state.specimen_capture,
            "MediaPreview",
            theme,
            media_preview_specimen::render(state, cx),
        ),
        "media-thumbnail" => specimen_card(
            state.specimen_capture,
            "MediaThumbnail",
            theme,
            media_thumbnail_specimen::render(theme),
        ),
        "media-browse-panel" => specimen_card(
            state.specimen_capture,
            "MediaBrowsePanel",
            theme,
            media_browse_panel_specimen::render(state, cx),
        ),
        "licence-activation" => specimen_card(
            state.specimen_capture,
            "LicenceActivation",
            theme,
            licence_activation::render(state, cx),
        ),
        "licence-seats" => specimen_card(
            state.specimen_capture,
            "LicenceSeats",
            theme,
            licence_seats::render(state, cx),
        ),
        "licence-status" => specimen_card(
            state.specimen_capture,
            "LicenceStatus",
            theme,
            licence_status::render(state, cx),
        ),
        "list-container" => specimen_card(
            state.specimen_capture,
            "ListContainer",
            theme,
            list_container_specimen::render(state, cx),
        ),
        "markdown-editor" => specimen_card(
            state.specimen_capture,
            "MarkdownEditor",
            theme,
            markdown_editor_specimen::render(state, cx),
        ),
        "block-editor" => specimen_card(
            state.specimen_capture,
            "BlockEditor",
            theme,
            block_editor_specimen::render(state, cx),
        ),

        // ── Layout Helpers ─────────────────────────────────────
        "collapse-toggle" => specimen_card(
            state.specimen_capture,
            "CollapseToggle",
            theme,
            collapse_toggle::render(state, cx),
        ),
        "region" => specimen_card(
            state.specimen_capture,
            "Region",
            theme,
            region::render(theme),
        ),
        "resize-handle" => specimen_card(
            state.specimen_capture,
            "ResizeHandle",
            theme,
            resize_handle::render(state, cx),
        ),

        // ── App Shell ───────────────────────────────────────────
        "app-header" => specimen_card(
            state.specimen_capture,
            "AppHeader",
            theme,
            app_header::render(state, cx),
        ),
        "command-palette" => specimen_card(
            state.specimen_capture,
            "CommandPalette",
            theme,
            command_palette::render(state, cx),
        ),
        "dock-region" => specimen_card(
            state.specimen_capture,
            "DockRegion",
            theme,
            dock_region::render(state, cx),
        ),
        "split-view" => specimen_card(
            state.specimen_capture,
            "SplitView",
            theme,
            split_view_specimen::render(state, cx),
        ),
        "status-bar" => specimen_card(
            state.specimen_capture,
            "StatusBar",
            theme,
            status_bar::render(state, cx),
        ),
        "action-discovery-panel" => specimen_card(
            state.specimen_capture,
            "ActionDiscoveryPanel",
            theme,
            action_discovery_panel::render(state, cx),
        ),
        "history-center" => specimen_card(
            state.specimen_capture,
            "HistoryCenter",
            theme,
            history_center_specimen::render(state, cx),
        ),
        "message-center" => specimen_card(
            state.specimen_capture,
            "MessageCenter",
            theme,
            message_center_specimen::render(state, cx),
        ),
        "update-status" => specimen_card(
            state.specimen_capture,
            "UpdateStatus",
            theme,
            update_status::render(state, cx),
        ),
        "update-center" => specimen_card(
            state.specimen_capture,
            "UpdateCenter",
            theme,
            update_center::render(state, cx),
        ),
        "settings-shell" => specimen_card(
            state.specimen_capture,
            "SettingsShell",
            theme,
            settings_shell::render(state, cx),
        ),

        // Fallback
        _ => missing_specimen(slug, theme),
    }
}
