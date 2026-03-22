//! Specimen framework — renders live component specimens in the content area.
//!
//! Each component returns a `JsEl` value describing its specimen.
//! The framework wraps specimens in a consistent card layout with a hero header.

pub mod accordion;
pub mod action_discovery_panel;
pub mod alert_dialog;
pub mod app_header;
pub mod badge;
pub mod banner;
pub mod breadcrumbs;
pub mod bulk_action_bar;
pub mod button;
pub mod bx;
pub mod calendar;
pub mod callout;
pub mod card;
pub mod checkbox;
pub mod code;
pub mod collapse_toggle;
pub mod collapsible;
pub mod combobox;
pub mod command_palette;
pub mod context_menu;
pub mod data_table;
pub mod date_picker;
pub mod detail_row;
pub mod detail_section;
pub mod detail_shell;
pub mod dialog;
pub mod dock_region;
pub mod drawer;
pub mod editable_label;
pub mod empty_state;
pub mod eyebrow;
pub mod field;
pub mod filter_toolbar;
pub mod form_actions;
pub mod grid;
pub mod hover_card;
pub mod icon;
pub mod icon_button;
pub mod list_card;
pub mod media_preview;
pub mod media_thumbnail;
pub mod menu;
pub mod menubar;
pub mod meter;
pub mod metric_tile;
pub mod nav_card;
pub mod nav_card_grid;
pub mod navigation_menu;
pub mod number_entry;
pub mod order_by;
pub mod page_header;
pub mod pagination;
pub mod pagination_summary;
pub mod picker_shell;
pub mod pill;
pub mod pin_input;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod range_slider;
pub mod rating;
pub mod region;
pub mod relation_picker;
pub mod resize_handle;
pub mod scroll_shell;
pub mod search_field;
pub mod segmented_control;
pub mod select;
pub mod selection_summary;
pub mod separator;
pub mod skeleton;
pub mod slider;
pub mod split_button;
pub mod split_view;
pub mod stack;
pub mod status_indicator;
pub mod surface;
pub mod switch;
pub mod tab_strip;
pub mod tabs;
pub mod text_area;
pub mod text_input;
pub mod time_ago;
pub mod toast_stack;
pub mod toolbar;
pub mod tooltip;

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::theme_ext::*;

use crate::app_state::{AppState, Section};
use crate::component_registry::{self, ComponentEntry};

// ── Content routing ──

/// Build the content for the specimen area based on current app state.
pub fn build_content(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
    match state.section {
        Section::Primitives | Section::Composites => {
            let components = component_registry::components_for_section(state.section);
            match state.active_component() {
                Some(idx) if idx < components.len() => {
                    build_specimen_page(&components[idx], theme)
                }
                _ => build_catalogue_landing(state, theme),
            }
        }
        Section::Demo => {
            // Placeholder until demo screens are implemented as real components
            let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
            label("Demo screens — coming soon")
                .text_color(text_secondary).text_size(13.0)
        }
        Section::Tokens => {
            let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
            label("Token inspector — coming soon")
                .text_color(text_secondary).text_size(13.0)
        }
    }
}

// ── Specimen page ──

/// Build the full specimen page for a selected component.
fn build_specimen_page(entry: &ComponentEntry, theme: &JetstreamThemeProvider) -> JsEl {
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let bg_elevated = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let accent = resolve_color(theme, "semantic.color.accent.base");

    let mut page = div().flex_col().gap(16.0);

    // Hero header
    page = page.child(
        div().flex_col().p(20.0).gap(8.0)
            .bg(bg_elevated).border_1().border_color(border).rounded(8.0)
            .child(
                div().flex_row().gap(8.0).items_center()
                    .child(
                        label(entry.tier.label())
                            .px(8.0).py(2.0).rounded(4.0)
                            .bg(tint(accent, 0.20))
                            .text_color(accent).text_size(10.0)
                    )
                    .child(
                        label(entry.display_name)
                            .text_color(text_primary).text_size(20.0)
                    )
            )
            .child(
                label(entry.description)
                    .text_color(text_secondary).text_size(13.0)
            )
    );

    // Specimen section
    if let Some(specimen) = render_specimen(entry.slug, theme) {
        page = page.child(
            div().flex_col().gap(8.0)
                .child(
                    label("Specimen").pl(2.0)
                        .text_color(text_secondary).text_size(11.0)
                )
                .child(
                    div().flex_col().p(24.0).gap(12.0)
                        .bg(bg_elevated).border_1().border_color(border).rounded(8.0)
                        .child(specimen)
                )
        );
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
fn render_specimen(slug: &str, theme: &JetstreamThemeProvider) -> Option<JsEl> {
    match slug {
        "accordion" => Some(accordion::render(theme)),
        "action-discovery-panel" => Some(action_discovery_panel::render(theme)),
        "alert-dialog" => Some(alert_dialog::render(theme)),
        "app-header" => Some(app_header::render(theme)),
        "badge" => Some(badge::render(theme)),
        "banner" => Some(banner::render(theme)),
        "box" => Some(bx::render(theme)),
        "breadcrumbs" => Some(breadcrumbs::render(theme)),
        "bulk-action-bar" => Some(bulk_action_bar::render(theme)),
        "button" => Some(button::render(theme)),
        "calendar" => Some(calendar::render(theme)),
        "callout" => Some(callout::render(theme)),
        "card" => Some(card::render(theme)),
        "checkbox" => Some(checkbox::render(theme)),
        "code" => Some(code::render(theme)),
        "collapse-toggle" => Some(collapse_toggle::render(theme)),
        "collapsible" => Some(collapsible::render(theme)),
        "combobox" => Some(combobox::render(theme)),
        "command-palette" => Some(command_palette::render(theme)),
        "context-menu" => Some(context_menu::render(theme)),
        "data-table" => Some(data_table::render(theme)),
        "date-picker" => Some(date_picker::render(theme)),
        "detail-row" => Some(detail_row::render(theme)),
        "detail-section" => Some(detail_section::render(theme)),
        "detail-shell" => Some(detail_shell::render(theme)),
        "dialog" => Some(dialog::render(theme)),
        "dock-region" => Some(dock_region::render(theme)),
        "drawer" => Some(drawer::render(theme)),
        "editable-label" => Some(editable_label::render(theme)),
        "empty-state" => Some(empty_state::render(theme)),
        "eyebrow" => Some(eyebrow::render(theme)),
        "field" => Some(field::render(theme)),
        "filter-toolbar" => Some(filter_toolbar::render(theme)),
        "form-actions" => Some(form_actions::render(theme)),
        "grid" => Some(grid::render(theme)),
        "hover-card" => Some(hover_card::render(theme)),
        "icon" => Some(icon::render(theme)),
        "icon-button" => Some(icon_button::render(theme)),
        "list-card" => Some(list_card::render(theme)),
        "media-preview" => Some(media_preview::render(theme)),
        "media-thumbnail" => Some(media_thumbnail::render(theme)),
        "menu" => Some(menu::render(theme)),
        "menubar" => Some(menubar::render(theme)),
        "meter" => Some(meter::render(theme)),
        "metric-tile" => Some(metric_tile::render(theme)),
        "nav-card" => Some(nav_card::render(theme)),
        "nav-card-grid" => Some(nav_card_grid::render(theme)),
        "navigation-menu" => Some(navigation_menu::render(theme)),
        "number-entry" => Some(number_entry::render(theme)),
        "order-by" => Some(order_by::render(theme)),
        "page-header" => Some(page_header::render(theme)),
        "pagination" => Some(pagination::render(theme)),
        "pagination-summary" => Some(pagination_summary::render(theme)),
        "picker-shell" => Some(picker_shell::render(theme)),
        "pill" => Some(pill::render(theme)),
        "pin-input" => Some(pin_input::render(theme)),
        "popover" => Some(popover::render(theme)),
        "progress" => Some(progress::render(theme)),
        "radio-group" => Some(radio_group::render(theme)),
        "range-slider" => Some(range_slider::render(theme)),
        "rating" => Some(rating::render(theme)),
        "region" => Some(region::render(theme)),
        "relation-picker" => Some(relation_picker::render(theme)),
        "resize-handle" => Some(resize_handle::render(theme)),
        "scroll-shell" => Some(scroll_shell::render(theme)),
        "search-field" => Some(search_field::render(theme)),
        "segmented-control" => Some(segmented_control::render(theme)),
        "select" => Some(select::render(theme)),
        "selection-summary" => Some(selection_summary::render(theme)),
        "separator" => Some(separator::render(theme)),
        "skeleton" => Some(skeleton::render(theme)),
        "slider" => Some(slider::render(theme)),
        "split-button" => Some(split_button::render(theme)),
        "split-view" => Some(split_view::render(theme)),
        "stack" => Some(stack::render(theme)),
        "status-indicator" => Some(status_indicator::render(theme)),
        "surface" => Some(surface::render(theme)),
        "switch" => Some(switch::render(theme)),
        "tab-strip" => Some(tab_strip::render(theme)),
        "tabs" => Some(tabs::render(theme)),
        "text-area" => Some(text_area::render(theme)),
        "text-input" => Some(text_input::render(theme)),
        "time-ago" => Some(time_ago::render(theme)),
        "toast-stack" => Some(toast_stack::render(theme)),
        "toolbar" => Some(toolbar::render(theme)),
        "tooltip" => Some(tooltip::render(theme)),
        _ => None,
    }
}

// ── Catalogue landing ──

/// Build the catalogue landing page when no component is selected.
fn build_catalogue_landing(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let bg_elevated = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let accent = resolve_color(theme, "semantic.color.accent.base");

    let components = component_registry::components_for_section(state.section);
    let specimen_count = component_registry::specimen_count(state.section);

    let mut landing = div().flex_col().gap(24.0);

    landing = landing
        .child(label(state.section.label()).text_color(text_primary).text_size(20.0))
        .child(label(format!(
            "{} components registered, {} with live specimens",
            components.len(), specimen_count,
        )).text_color(text_secondary).text_size(13.0));

    // Component overview by first letter
    let categories = categorize_components(components);
    let mut grid = div().flex_col().gap(8.0);
    for (letter, count) in &categories {
        grid = grid.child(
            div().flex_row().px(12.0).py(6.0).gap(8.0).items_center()
                .bg(bg_elevated).border_1().border_color(border).rounded(6.0)
                .child(label(letter.to_string()).text_color(accent).text_size(14.0))
                .child(label(format!(
                    "{} component{}", count, if *count == 1 { "" } else { "s" }
                )).text_color(text_secondary).text_size(12.0))
        );
    }
    landing = landing.child(grid);

    landing = landing.child(
        label("Select a component from the sidebar to view its specimen.")
            .pt(8.0).text_color(tint(text_secondary, 0.7)).text_size(12.0)
    );

    landing
}

fn categorize_components(components: &[ComponentEntry]) -> Vec<(char, usize)> {
    let mut categories: Vec<(char, usize)> = Vec::new();
    for entry in components {
        let first = entry.display_name.chars().next().unwrap_or('?');
        if let Some(cat) = categories.iter_mut().find(|(c, _)| *c == first) {
            cat.1 += 1;
        } else {
            categories.push((first, 1));
        }
    }
    categories
}
