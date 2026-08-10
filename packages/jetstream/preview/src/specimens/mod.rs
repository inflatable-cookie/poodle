//! Specimen framework — renders live component specimens in the content area.
//!
//! Each component returns a `JsEl` value describing its specimen.
//! The framework wraps specimens in a consistent card layout with a hero header.

pub mod accordion;
pub mod action_discovery_panel;
pub mod agent_chat_input;
pub mod alert_dialog;
pub mod app_header;
pub mod audio_player;
pub mod avatar;
pub mod badge;
pub mod banner;
pub mod block_editor;
pub mod breadcrumbs;
pub mod bulk_action_bar;
pub mod button;
pub mod bx;
pub mod calendar;
pub mod callout;
pub mod card;
pub mod card_radio_group;
pub mod card_toggle_group;
pub mod checkbox;
pub mod code;
pub mod code_input;
pub mod collapse_toggle;
pub mod collapsible;
pub mod color_picker;
pub mod command_palette;
pub mod confirm_action;
pub mod context_menu;
pub mod data_table;
pub mod date_picker;
pub mod date_range_picker;
pub mod date_time_picker;
pub mod date_time_range_picker;
pub mod debug_dialog;
pub mod detail_item;
pub mod detail_section;
pub mod detail_section_group;
pub mod detail_shell;
pub mod dialog;
pub mod dock_region;
pub mod drawer;
pub mod duration_input;
pub mod editable_label;
pub mod editable_list;
pub mod embed_input;
pub mod embed_preview;
pub mod empty_state;
pub mod error_boundary;
pub mod eyebrow;
pub mod field;
pub mod field_set;
pub mod file_upload;
pub mod filter_builder;
pub mod filter_toolbar;
pub mod floating_overlay;
pub mod form_actions;
pub mod form_dialog;
pub mod form_layout;
pub mod form_shell;
pub mod grid;
pub mod hover_card;
pub mod icon;
pub mod icon_button;
pub mod icon_provider;
pub mod inline_list_section;
pub mod inline_remediation;
pub mod list_card;
pub mod list_card_counter;
pub mod list_container;
pub mod list_grid;
pub mod log_list;
pub mod markdown_editor;
pub mod media_browse_panel;
pub mod media_picker;
pub mod media_preview;
pub mod media_thumbnail;
pub mod menu;
pub mod menubar;
pub mod meta_bar;
pub mod meta_item;
pub mod meter;
pub mod metric_tile;
pub mod model_picker;
pub mod nav_card;
pub mod navigation_menu;
pub mod number_input;
pub mod order_by;
pub mod page_header;
pub mod page_loading;
pub mod pagination;
pub mod pagination_summary;
pub mod password_requirements;
pub mod picker_shell;
pub mod pill;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod range_slider;
pub mod rating;
pub mod ref_select;
pub mod region;
pub mod relation_picker;
pub mod remediation_banner;
pub mod theme_select;
// reorderable_list merged into editable_list on the contract side.
// pub mod reorderable_list;
pub mod agent_question;
pub mod agent_transcript;
pub mod audio_controls;
pub mod date_time_zone_picker;
pub mod resize_handle;
pub mod scroll_shell;
pub mod segmented_control;
pub mod select;
pub mod selection_summary;
pub mod separator;
pub mod shell_status_bar;
pub mod sidebar_nav;
pub mod skeleton;
pub mod slider;
pub mod spacer;
pub mod spinner;
pub mod split_button;
pub mod split_view;
pub mod stack;
pub mod state_tile;
pub mod status_indicator;
pub mod stepper;
pub mod surface;
pub mod switch;
pub mod tab_strip;
pub mod table;
pub mod tabs;
pub mod text;
pub mod text_input;
pub mod text_link;
pub mod time_ago;
pub mod time_field;
pub mod time_zone_select;
pub mod toast_host;
pub mod toast_stack;
pub mod toggle_group;
pub mod token_input;
pub mod toolbar;
pub mod tooltip;
pub mod tree;
pub mod tri_state_switch;
pub mod ui_presentation_provider;
pub mod validation_summary;
pub mod video_player;

use crate::jsx::*;
use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;

use crate::app_state::SpecimenView;
use crate::app_state::{AppState, Section};
use crate::component_registry::{self, ComponentEntry};

// ── Content routing ──

/// Build the content for the specimen area based on current app state.
pub fn build_content(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
    match state.section {
        Section::Components => {
            let components = component_registry::ALL_COMPONENTS;
            match state.active_component() {
                Some(idx) if idx < components.len() => {
                    build_specimen_page(&components[idx], theme, state)
                }
                _ => build_catalogue_landing(state, theme),
            }
        }
        Section::Demo => {
            // Placeholder until demo screens are implemented as real components
            let text_secondary = resolve_color(theme, "color.text.secondary");
            label("Demo screens — coming soon")
                .text_color(text_secondary)
                .text_size(13.0)
        }
        Section::Tokens => {
            let text_secondary = resolve_color(theme, "color.text.secondary");
            label("Token inspector — coming soon")
                .text_color(text_secondary)
                .text_size(13.0)
        }
    }
}

// ── Specimen view filtering (Examples/Sizes/Densities tabs) ──

/// A top-level specimen section's title, when the child follows the shared
/// `group(title, …)` pattern (a panel whose first child is the title label).
fn section_title(el: &jetstream_ui::ui_element::JsEl) -> Option<&str> {
    el.children.first().and_then(|c| match &c.kind {
        jetstream_ui::ui_element::WidgetKind::Label(t) => Some(t.as_str()),
        _ => None,
    })
}

fn is_sizes_section(el: &jetstream_ui::ui_element::JsEl) -> bool {
    section_title(el).is_some_and(|t| t.starts_with("Sizes") || t.starts_with("Size "))
}

fn is_densities_section(el: &jetstream_ui::ui_element::JsEl) -> bool {
    section_title(el).is_some_and(|t| t.starts_with("Densities") || t.starts_with("Density"))
}

/// Filter a rendered specimen to the active view. Specimens are uniformly a
/// column of `group(...)` panels; Sizes/Densities sections move to their own
/// tabs (mirroring the Svelte SpecimenLayout), everything else is Examples.
/// Returns the filtered element plus whether sizes/densities sections exist
/// (drives which tabs are offered).
fn filter_specimen_view(
    mut specimen: jetstream_ui::ui_element::JsEl,
    view: SpecimenView,
) -> (jetstream_ui::ui_element::JsEl, bool, bool) {
    let has_sizes = specimen.children.iter().any(is_sizes_section);
    let has_densities = specimen.children.iter().any(is_densities_section);
    if has_sizes || has_densities {
        specimen.children.retain(|c| match view {
            SpecimenView::Examples => !is_sizes_section(c) && !is_densities_section(c),
            SpecimenView::Sizes => is_sizes_section(c),
            SpecimenView::Densities => is_densities_section(c),
        });
    }
    (specimen, has_sizes, has_densities)
}

// ── Specimen page ──

/// Build the full specimen page for a selected component.
fn build_specimen_page(
    entry: &ComponentEntry,
    theme: &JetstreamThemeProvider,
    state: &AppState,
) -> JsEl {
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let bg_elevated = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.subtle");
    let accent = resolve_color(theme, "color.accent.base");

    let mut page = div().flex_col().gap(16.0);

    // Hero header
    page = page.child(
        div()
            .flex_col()
            .p(20.0)
            .gap(8.0)
            .bg(bg_elevated)
            .border_1()
            .border_color(border)
            .rounded(8.0)
            .child(
                div()
                    .flex_row()
                    .gap(8.0)
                    .items_center()
                    .child(
                        label(entry.tag.label())
                            .px(8.0)
                            .py(2.0)
                            .rounded(4.0)
                            .bg(tint(accent, 0.20))
                            .text_color(accent)
                            .text_size(10.0),
                    )
                    .child(
                        label(entry.display_name)
                            .text_color(text_primary)
                            .text_size(20.0),
                    )
                    .child(div().grow())
                    .child(crate::shell::build_state_probes(state, theme)),
            )
            .child(
                label(entry.description)
                    .text_color(text_secondary)
                    .text_size(13.0),
            ),
    );

    // Specimen section — filtered to the active Examples/Sizes/Densities view,
    // with a text-variant Tabs switcher when the specimen has those sections
    // (mirrors the Svelte SpecimenLayout).
    if let Some(specimen) = render_specimen(entry.slug, theme, state) {
        let (filtered, has_sizes, has_densities) =
            filter_specimen_view(specimen, state.specimen_view);

        let mut section = div().flex_col().gap(12.0);
        if has_sizes || has_densities {
            let mut items = vec![poodle_specs::TabDefinition::new("examples", "Examples")];
            if has_sizes {
                items.push(poodle_specs::TabDefinition::new("sizes", "Sizes"));
            }
            if has_densities {
                items.push(poodle_specs::TabDefinition::new("densities", "Densities"));
            }
            let value = match state.specimen_view {
                SpecimenView::Examples => "examples",
                SpecimenView::Sizes => "sizes",
                SpecimenView::Densities => "densities",
            };
            section = section.child(crate::jsx::jel(crate::compat::js_tabs(
                &poodle_specs::TabsSpec::new(items)
                    .with_variant(poodle_specs::TabVariant::Underline)
                    .with_size(poodle_specs::ControlSize::Sm)
                    .with_value(value),
                theme,
            )));
        }
        section = section.child(
            div()
                .flex_col()
                .p(24.0)
                .gap(12.0)
                .bg(bg_elevated)
                .border_1()
                .border_color(border)
                .rounded(8.0)
                .child(filtered),
        );
        page = page.child(section);
    } else {
        page = page.child(
            div().flex_col().gap(8.0).child(
                label(format!("{} — specimen coming soon", entry.display_name))
                    .text_color(text_secondary)
                    .text_size(13.0),
            ),
        );
    }

    page
}

/// Convert a node-tier specimen (El) to the framework's JsEl at the boundary.
fn el(e: crate::nel::El) -> JsEl {
    jetstream_poodle::to_js_el(&e.0)
}

/// Route a component slug to its specimen renderer.
pub fn render_specimen(
    slug: &str,
    theme: &JetstreamThemeProvider,
    state: &AppState,
) -> Option<JsEl> {
    match slug {
        "accordion" => Some(el(accordion::render(theme))),
        "action-discovery-panel" => Some(el(action_discovery_panel::render(theme))),
        "alert-dialog" => Some(el(alert_dialog::render(theme))),
        "app-header" => Some(el(app_header::render(theme))),
        "audio-player" => Some(el(audio_player::render(theme))),
        "avatar" => Some(el(avatar::render(theme))),
        "badge" => Some(el(badge::render(theme))),
        "banner" => Some(el(banner::render(theme))),
        "block-editor" => Some(el(block_editor::render(theme))),
        "box" => Some(el(bx::render(theme))),
        "breadcrumbs" => Some(el(breadcrumbs::render(theme))),
        "bulk-action-bar" => Some(el(bulk_action_bar::render(theme))),
        "button" => Some(el(button::render(theme))),
        "calendar" => Some(el(calendar::render(theme))),
        "callout" => Some(el(callout::render(theme))),
        "card" => Some(el(card::render(theme))),
        "card-radio-group" => Some(el(card_radio_group::render(theme))),
        "card-toggle-group" => Some(el(card_toggle_group::render(theme))),
        "checkbox" => Some(el(checkbox::render(theme))),
        "code" => Some(el(code::render(theme))),
        "collapse-toggle" => Some(el(collapse_toggle::render(theme))),
        "collapsible" => Some(el(collapsible::render(theme))),
        "color-picker" => Some(el(color_picker::render(theme))),
        "command-palette" => Some(el(command_palette::render(theme))),
        "confirm-action" => Some(el(confirm_action::render(theme))),
        "context-menu" => Some(el(context_menu::render(theme))),
        "data-table" => Some(el(data_table::render(theme))),
        "debug-dialog" => Some(el(debug_dialog::render(theme))),
        "date-picker" => Some(el(date_picker::render(theme))),
        "date-range-picker" => Some(el(date_range_picker::render(theme))),
        "date-time-picker" => Some(el(date_time_picker::render(theme))),
        "date-time-range-picker" => Some(el(date_time_range_picker::render(theme))),
        "detail-item" => Some(el(detail_item::render(theme))),
        "detail-section" => Some(el(detail_section::render(theme))),
        "detail-section-group" => Some(el(detail_section_group::render(theme))),
        "detail-shell" => Some(el(detail_shell::render(theme))),
        "dialog" => Some(el(dialog::render(theme))),
        "dock-region" => Some(el(dock_region::render(theme))),
        "drawer" => Some(el(drawer::render(theme))),
        "duration-input" => Some(el(duration_input::render(theme))),
        "editable-label" => Some(el(editable_label::render(theme))),
        "editable-list" => Some(el(editable_list::render(theme))),
        "embed-input" => Some(el(embed_input::render(theme))),
        "embed-preview" => Some(el(embed_preview::render(theme))),
        "empty-state" => Some(el(empty_state::render(theme))),
        "error-boundary" => Some(el(error_boundary::render(theme))),
        "eyebrow" => Some(el(eyebrow::render(theme))),
        "field" => Some(el(field::render(theme))),
        "field-set" => Some(el(field_set::render(theme))),
        "file-upload" => Some(el(file_upload::render(theme))),
        "filter-toolbar" => Some(el(filter_toolbar::render(theme))),
        "floating-overlay" => Some(el(floating_overlay::render(theme))),
        "form-actions" => Some(el(form_actions::render(theme))),
        "form-dialog" => Some(el(form_dialog::render(theme))),
        "form-layout" => Some(el(form_layout::render(theme))),
        "form-shell" => Some(el(form_shell::render(theme))),
        "grid" => Some(el(grid::render(theme))),
        "hover-card" => Some(el(hover_card::render(theme))),
        "icon" => Some(el(icon::render(theme))),
        "icon-button" => Some(el(icon_button::render(theme))),
        "icon-provider" => Some(el(icon_provider::render(theme))),
        "inline-list-section" => Some(el(inline_list_section::render(theme))),
        "inline-remediation" => Some(el(inline_remediation::render(theme))),
        "list-card" => Some(el(list_card::render(theme))),
        "list-card-counter" => Some(el(list_card_counter::render(theme))),
        "list-container" => Some(el(list_container::render(theme))),
        "list-grid" => Some(el(list_grid::render(theme))),
        "log-list" => Some(el(log_list::render(theme))),
        "markdown-editor" => Some(el(markdown_editor::render(theme))),
        "meta-bar" => Some(el(meta_bar::render(theme))),
        "meta-item" => Some(el(meta_item::render(theme))),
        "media-browse-panel" => Some(el(media_browse_panel::render(theme))),
        "media-picker" => Some(el(media_picker::render(theme))),
        "media-preview" => Some(el(media_preview::render(theme))),
        "media-thumbnail" => Some(el(media_thumbnail::render(theme))),
        "menu" => Some(el(menu::render(theme))),
        "menubar" => Some(el(menubar::render(theme))),
        "meter" => Some(el(meter::render(theme))),
        "metric-tile" => Some(el(metric_tile::render(theme))),
        "nav-card" => Some(el(nav_card::render(theme))),
        "navigation-menu" => Some(el(navigation_menu::render(theme))),
        "number-input" => Some(el(number_input::render(theme))),
        "filter-builder" => Some(el(filter_builder::render(theme))),
        "model-picker" => Some(el(model_picker::render(theme))),
        "ref-select" => Some(el(ref_select::render(theme))),
        "agent-chat-input" => Some(el(agent_chat_input::render(theme))),
        "theme-select" => Some(el(theme_select::render(theme))),
        "order-by" => Some(el(order_by::render(theme))),
        "page-header" => Some(el(page_header::render(theme))),
        "page-loading" => Some(el(page_loading::render(theme))),
        "pagination" => Some(el(pagination::render(theme))),
        "pagination-summary" => Some(el(pagination_summary::render(theme))),
        "password-requirements" => Some(el(password_requirements::render(theme))),
        "picker-shell" => Some(el(picker_shell::render(theme))),
        "pill" => Some(el(pill::render(theme))),
        "code-input" => Some(el(code_input::render(theme))),
        "popover" => Some(el(popover::render(theme))),
        "progress" => Some(el(progress::render(theme))),
        "radio-group" => Some(el(radio_group::render(theme))),
        "range-slider" => Some(el(range_slider::render(theme))),
        "rating" => Some(el(rating::render(theme))),
        "region" => Some(el(region::render(theme))),
        "relation-picker" => Some(el(relation_picker::render(theme))),
        "remediation-banner" => Some(el(remediation_banner::render(theme))),
        // "reorderable-list" => merged into "editable-list"
        "resize-handle" => Some(el(resize_handle::render(theme))),
        "scroll-shell" => Some(el(scroll_shell::render(theme))),
        "segmented-control" => Some(el(segmented_control::render(theme))),
        "select" => Some(el(select::render(theme))),
        "selection-summary" => Some(el(selection_summary::render(theme))),
        "separator" => Some(el(separator::render(theme))),
        "shell-status-bar" => Some(el(shell_status_bar::render(theme))),
        "sidebar-nav" => Some(el(sidebar_nav::render(theme))),
        "skeleton" => Some(el(skeleton::render(theme))),
        "slider" => Some(el(slider::render(theme))),
        "knob" => Some(el(audio_controls::knob(theme))),
        "fader" => Some(el(audio_controls::fader(theme))),
        "audio-meter" => Some(el(audio_controls::audio_meter(theme))),
        "value-readout" => Some(el(audio_controls::value_readout(theme))),
        "drag-number-field" => Some(el(audio_controls::drag_number_field(theme))),
        "envelope-editor" => Some(el(audio_controls::envelope_editor(theme))),
        "xy-pad" => Some(el(audio_controls::xy_pad(theme))),
        "audio-switch" => Some(el(audio_controls::audio_switch(theme))),
        "gain-reduction-meter" => Some(el(audio_controls::gain_reduction_meter(theme))),
        "spacer" => Some(el(spacer::render(theme))),
        "spinner" => Some(el(spinner::render(theme))),
        "split-button" => Some(el(split_button::render(theme))),
        "split-view" => Some(el(split_view::render(theme))),
        "stack" => Some(el(stack::render(theme))),
        "state-tile" => Some(el(state_tile::render(theme))),
        "status-indicator" => Some(el(status_indicator::render(theme))),
        "surface" => Some(el(surface::render(theme))),
        "stepper" => Some(el(stepper::render(state, theme))),
        "agent-transcript" => Some(el(agent_transcript::render(theme))),
        "agent-question" => Some(el(agent_question::render(theme))),
        "switch" => Some(el(switch::render(theme))),
        "tab-strip" => Some(el(tab_strip::render(theme))),
        "table" => Some(el(table::render(theme))),
        "tabs" => Some(el(tabs::render(state, theme))),
        "text" => Some(el(text::render(theme))),
        "text-input" => Some(el(text_input::render(theme))),
        "text-link" => Some(el(text_link::render(theme))),
        "time-ago" => Some(el(time_ago::render(theme))),
        "time-field" => Some(el(time_field::render(theme))),
        "time-zone-select" => Some(el(time_zone_select::render(theme))),
        "toast-host" => Some(el(toast_host::render(theme))),
        "toast-stack" => Some(el(toast_stack::render(theme))),
        "toggle-group" => Some(el(toggle_group::render(theme))),
        "token-input" => Some(el(token_input::render(theme))),
        "toolbar" => Some(el(toolbar::render(theme))),
        "tooltip" => Some(el(tooltip::render(theme))),
        "tree" => Some(el(tree::render(state, theme))),
        "tri-state-switch" => Some(el(tri_state_switch::render(theme))),
        "ui-presentation-provider" => Some(el(ui_presentation_provider::render(theme))),
        "validation-summary" => Some(el(validation_summary::render(theme))),
        "video-player" => Some(el(video_player::render(theme))),
        "date-time-zone-picker" => Some(el(date_time_zone_picker::render(theme))),
        _ => None,
    }
}

// ── Catalogue landing ──

/// Build the catalogue landing page when no component is selected.
fn build_catalogue_landing(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let bg_elevated = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.subtle");
    let accent = resolve_color(theme, "color.accent.base");

    let components = component_registry::ALL_COMPONENTS;

    // Mirrors the Svelte preview's landing: title + blurb + per-tag sections,
    // each a three-column grid of clickable component cards.
    let mut landing = div().flex_col().gap(20.0);

    landing = landing.child(
        div().flex_col().gap(6.0)
            .child(
                label("Component catalogue")
                    .text_color(text_primary)
                    .text_size(24.0)
                    .text_weight(700),
            )
            .child(
                label("Browse the full Poodle component library. Each component handles accessibility, keyboard support, and theming.")
                    .text_color(text_secondary)
                    .text_size(13.0)
                    .text_wrap(true),
            )
            .child(
                label(format!("{} COMPONENTS", components.len()))
                    .text_color(text_secondary)
                    .text_size(11.0)
                    .text_weight(600)
                    .letter_spacing_em(0.08),
            ),
    );

    // Per-tag sections. Entries are pre-ordered by (tag order, name); a card's
    // id reuses the sidebar action ("sidebar:{i}") so clicking navigates.
    let mut current_tag = None;
    let mut section_grid: Option<JsEl> = None;
    for (i, entry) in components.iter().enumerate() {
        if !state.matches_search(entry.display_name) {
            continue;
        }
        if current_tag != Some(entry.tag) {
            if let Some(grid) = section_grid.take() {
                landing = landing.child(grid);
            }
            current_tag = Some(entry.tag);
            landing = landing.child(
                label(entry.tag.label().to_uppercase())
                    .pt(8.0)
                    .text_color(accent)
                    .text_size(11.0)
                    .text_weight(700)
                    .letter_spacing_em(0.08),
            );
            section_grid = Some(div().grid().grid_cols(3).gap(12.0).w_full());
        }
        let card = div()
            .flex_col()
            .gap(6.0)
            .p(16.0)
            .bg(bg_elevated)
            .border_1()
            .border_color(border)
            .rounded(8.0)
            .id(format!("sidebar:{i}"))
            .focusable()
            .cursor_pointer()
            .hover(move |st| st.border_color(tint(accent, 0.5)))
            .child(
                label(entry.display_name)
                    .text_color(text_primary)
                    .text_size(14.0)
                    .text_weight(600),
            )
            .child(
                label(entry.description)
                    .text_color(text_secondary)
                    .text_size(12.0)
                    .text_wrap(true),
            );
        section_grid = section_grid.map(|g| g.child(card));
    }
    if let Some(grid) = section_grid.take() {
        landing = landing.child(grid);
    }

    landing
}
