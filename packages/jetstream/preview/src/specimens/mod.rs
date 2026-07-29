//! Specimen framework — renders live component specimens in the content area.
//!
//! Each component returns a `JsEl` value describing its specimen.
//! The framework wraps specimens in a consistent card layout with a hero header.

pub mod accordion;
pub mod action_discovery_panel;
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
pub mod collapse_toggle;
pub mod collapsible;
pub mod color_picker;
pub mod command_palette;
pub mod confirm_action;
pub mod context_menu;
pub mod data_table;
pub mod debug_dialog;
pub mod date_picker;
pub mod date_range_picker;
pub mod date_time_picker;
pub mod date_time_range_picker;
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
pub mod meta_bar;
pub mod meta_item;
pub mod media_browse_panel;
pub mod media_picker;
pub mod media_preview;
pub mod media_thumbnail;
pub mod menu;
pub mod menubar;
pub mod meter;
pub mod metric_tile;
pub mod nav_card;
pub mod navigation_menu;
pub mod number_input;
pub mod agent_chat_input;
pub mod filter_builder;
pub mod model_picker;
pub mod ref_select;
pub mod theme_select;
pub mod order_by;
pub mod page_header;
pub mod page_loading;
pub mod pagination;
pub mod pagination_summary;
pub mod password_requirements;
pub mod picker_shell;
pub mod pill;
pub mod code_input;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod range_slider;
pub mod rating;
pub mod region;
pub mod relation_picker;
pub mod remediation_banner;
// reorderable_list merged into editable_list on the contract side.
// pub mod reorderable_list;
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
pub mod surface;
pub mod agent_transcript;
pub mod stepper;
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
pub mod date_time_zone_picker;

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::theme_ext::*;

use crate::app_state::{AppState, Section};
use crate::app_state::SpecimenView;
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
                .text_color(text_secondary).text_size(13.0)
        }
        Section::Tokens => {
            let text_secondary = resolve_color(theme, "color.text.secondary");
            label("Token inspector — coming soon")
                .text_color(text_secondary).text_size(13.0)
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
        div().flex_col().p(20.0).gap(8.0)
            .bg(bg_elevated).border_1().border_color(border).rounded(8.0)
            .child(
                div().flex_row().gap(8.0).items_center()
                    .child(
                        label(entry.tag.label())
                            .px(8.0).py(2.0).rounded(4.0)
                            .bg(tint(accent, 0.20))
                            .text_color(accent).text_size(10.0)
                    )
                    .child(
                        label(entry.display_name)
                            .text_color(text_primary).text_size(20.0)
                    )
                    .child(div().grow())
                    .child(crate::shell::build_state_probes(state, theme))
            )
            .child(
                label(entry.description)
                    .text_color(text_secondary).text_size(13.0)
            )
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
            section = section.child(poodle_jetstream_components::tabs::js_tabs(
                &poodle_specs::TabsSpec::new(items)
                    .with_variant(poodle_specs::TabVariant::Underline)
                    .with_size(poodle_specs::ControlSize::Sm)
                    .with_value(value),
                theme,
            ));
        }
        section = section.child(
            div().flex_col().p(24.0).gap(12.0)
                .bg(bg_elevated).border_1().border_color(border).rounded(8.0)
                .child(filtered),
        );
        page = page.child(section);
    } else {
        page = page.child(
            div().flex_col().gap(8.0)
                .child(
                    label(format!("{} — specimen coming soon", entry.display_name))
                        .text_color(text_secondary).text_size(13.0)
                )
        );
    }

    page
}

/// Route a component slug to its specimen renderer.
pub fn render_specimen(
    slug: &str,
    theme: &JetstreamThemeProvider,
    state: &AppState,
) -> Option<JsEl> {
    match slug {
        "accordion" => Some(accordion::render(theme)),
        "action-discovery-panel" => Some(action_discovery_panel::render(theme)),
        "alert-dialog" => Some(alert_dialog::render(theme)),
        "app-header" => Some(app_header::render(theme)),
        "audio-player" => Some(audio_player::render(theme)),
        "avatar" => Some(avatar::render(theme)),
        "badge" => Some(badge::render(theme)),
        "banner" => Some(banner::render(theme)),
        "block-editor" => Some(block_editor::render(theme)),
        "box" => Some(bx::render(theme)),
        "breadcrumbs" => Some(breadcrumbs::render(theme)),
        "bulk-action-bar" => Some(bulk_action_bar::render(theme)),
        "button" => Some(button::render(theme)),
        "calendar" => Some(calendar::render(theme)),
        "callout" => Some(callout::render(theme)),
        "card" => Some(card::render(theme)),
        "card-radio-group" => Some(card_radio_group::render(theme)),
        "card-toggle-group" => Some(card_toggle_group::render(theme)),
        "checkbox" => Some(checkbox::render(theme)),
        "code" => Some(code::render(theme)),
        "collapse-toggle" => Some(collapse_toggle::render(theme)),
        "collapsible" => Some(collapsible::render(theme)),
        "color-picker" => Some(color_picker::render(theme)),
        "command-palette" => Some(command_palette::render(theme)),
        "confirm-action" => Some(confirm_action::render(theme)),
        "context-menu" => Some(context_menu::render(theme)),
        "data-table" => Some(data_table::render(theme)),
        "debug-dialog" => Some(debug_dialog::render(theme)),
        "date-picker" => Some(date_picker::render(theme)),
        "date-range-picker" => Some(date_range_picker::render(theme)),
        "date-time-picker" => Some(date_time_picker::render(theme)),
        "date-time-range-picker" => Some(date_time_range_picker::render(theme)),
        "detail-item" => Some(detail_item::render(theme)),
        "detail-section" => Some(detail_section::render(theme)),
        "detail-section-group" => Some(detail_section_group::render(theme)),
        "detail-shell" => Some(detail_shell::render(theme)),
        "dialog" => Some(dialog::render(theme)),
        "dock-region" => Some(dock_region::render(theme)),
        "drawer" => Some(drawer::render(theme)),
        "duration-input" => Some(duration_input::render(theme)),
        "editable-label" => Some(editable_label::render(theme)),
        "editable-list" => Some(editable_list::render(theme)),
        "embed-input" => Some(embed_input::render(theme)),
        "embed-preview" => Some(embed_preview::render(theme)),
        "empty-state" => Some(empty_state::render(theme)),
        "error-boundary" => Some(error_boundary::render(theme)),
        "eyebrow" => Some(eyebrow::render(theme)),
        "field" => Some(field::render(theme)),
        "field-set" => Some(field_set::render(theme)),
        "file-upload" => Some(file_upload::render(theme)),
        "filter-toolbar" => Some(filter_toolbar::render(theme)),
        "floating-overlay" => Some(floating_overlay::render(theme)),
        "form-actions" => Some(form_actions::render(theme)),
        "form-dialog" => Some(form_dialog::render(theme)),
        "form-layout" => Some(form_layout::render(theme)),
        "form-shell" => Some(form_shell::render(theme)),
        "grid" => Some(grid::render(theme)),
        "hover-card" => Some(hover_card::render(theme)),
        "icon" => Some(icon::render(theme)),
        "icon-button" => Some(icon_button::render(theme)),
        "icon-provider" => Some(icon_provider::render(theme)),
        "inline-list-section" => Some(inline_list_section::render(theme)),
        "inline-remediation" => Some(inline_remediation::render(theme)),
        "list-card" => Some(list_card::render(theme)),
        "list-card-counter" => Some(list_card_counter::render(theme)),
        "list-container" => Some(list_container::render(theme)),
        "list-grid" => Some(list_grid::render(theme)),
        "log-list" => Some(log_list::render(theme)),
        "markdown-editor" => Some(markdown_editor::render(theme)),
        "meta-bar" => Some(meta_bar::render(theme)),
        "meta-item" => Some(meta_item::render(theme)),
        "media-browse-panel" => Some(media_browse_panel::render(theme)),
        "media-picker" => Some(media_picker::render(theme)),
        "media-preview" => Some(media_preview::render(theme)),
        "media-thumbnail" => Some(media_thumbnail::render(theme)),
        "menu" => Some(menu::render(theme)),
        "menubar" => Some(menubar::render(theme)),
        "meter" => Some(meter::render(theme)),
        "metric-tile" => Some(metric_tile::render(theme)),
        "nav-card" => Some(nav_card::render(theme)),
        "navigation-menu" => Some(navigation_menu::render(theme)),
        "number-input" => Some(number_input::render(theme)),
        "filter-builder" => Some(filter_builder::render(theme)),
        "model-picker" => Some(model_picker::render(theme)),
        "ref-select" => Some(ref_select::render(theme)),
        "agent-chat-input" => Some(agent_chat_input::render(theme)),
        "theme-select" => Some(theme_select::render(theme)),
        "order-by" => Some(order_by::render(theme)),
        "page-header" => Some(page_header::render(theme)),
        "page-loading" => Some(page_loading::render(theme)),
        "pagination" => Some(pagination::render(theme)),
        "pagination-summary" => Some(pagination_summary::render(theme)),
        "password-requirements" => Some(password_requirements::render(theme)),
        "picker-shell" => Some(picker_shell::render(theme)),
        "pill" => Some(pill::render(theme)),
        "code-input" => Some(code_input::render(theme)),
        "popover" => Some(popover::render(theme)),
        "progress" => Some(progress::render(theme)),
        "radio-group" => Some(radio_group::render(theme)),
        "range-slider" => Some(range_slider::render(theme)),
        "rating" => Some(rating::render(theme)),
        "region" => Some(region::render(theme)),
        "relation-picker" => Some(relation_picker::render(theme)),
        "remediation-banner" => Some(remediation_banner::render(theme)),
        // "reorderable-list" => merged into "editable-list"
        "resize-handle" => Some(resize_handle::render(theme)),
        "scroll-shell" => Some(scroll_shell::render(theme)),
        "segmented-control" => Some(segmented_control::render(theme)),
        "select" => Some(select::render(theme)),
        "selection-summary" => Some(selection_summary::render(theme)),
        "separator" => Some(separator::render(theme)),
        "shell-status-bar" => Some(shell_status_bar::render(theme)),
        "sidebar-nav" => Some(sidebar_nav::render(theme)),
        "skeleton" => Some(skeleton::render(theme)),
        "slider" => Some(slider::render(theme)),
        "spacer" => Some(spacer::render(theme)),
        "spinner" => Some(spinner::render(theme)),
        "split-button" => Some(split_button::render(theme)),
        "split-view" => Some(split_view::render(theme)),
        "stack" => Some(stack::render(theme)),
        "state-tile" => Some(state_tile::render(theme)),
        "status-indicator" => Some(status_indicator::render(theme)),
        "surface" => Some(surface::render(theme)),
        "stepper" => Some(stepper::render(theme)),
        "agent-transcript" => Some(agent_transcript::render(theme)),
        "switch" => Some(switch::render(theme)),
        "tab-strip" => Some(tab_strip::render(theme)),
        "table" => Some(table::render(theme)),
        "tabs" => Some(tabs::render(state, theme)),
        "text" => Some(text::render(theme)),
        "text-input" => Some(text_input::render(theme)),
        "text-link" => Some(text_link::render(theme)),
        "time-ago" => Some(time_ago::render(theme)),
        "time-field" => Some(time_field::render(theme)),
        "time-zone-select" => Some(time_zone_select::render(theme)),
        "toast-host" => Some(toast_host::render(theme)),
        "toast-stack" => Some(toast_stack::render(theme)),
        "toggle-group" => Some(toggle_group::render(theme)),
        "token-input" => Some(token_input::render(theme)),
        "toolbar" => Some(toolbar::render(theme)),
        "tooltip" => Some(tooltip::render(theme)),
        "tree" => Some(tree::render(state, theme)),
        "tri-state-switch" => Some(tri_state_switch::render(theme)),
        "ui-presentation-provider" => Some(ui_presentation_provider::render(theme)),
        "validation-summary" => Some(validation_summary::render(theme)),
        "video-player" => Some(video_player::render(theme)),
        "date-time-zone-picker" => Some(date_time_zone_picker::render(theme)),
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
