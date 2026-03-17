//! Specimen framework — renders live component specimens in the content area.
//!
//! Each component can implement the `Specimen` trait to provide a live preview.
//! The framework wraps specimens in a consistent card layout with a hero header
//! showing component metadata.

mod accordion;
mod action_discovery;
mod alert_dialog;
mod app_header;
mod audio_player;
mod autonomous_list;
mod badge;
mod banner;
mod block_editor;
mod box_spec;
mod breadcrumbs;
mod bulk_action_bar;
mod button;
mod calendar;
mod callout;
mod card;
mod card_radio_group;
mod checkbox;
mod code;
mod collapsible;
mod color_picker;
mod command_palette;
mod command_palette_shell;
mod combobox;
mod confirm_action;
mod context_menu;
mod data_table;
pub mod demos;
mod date_picker;
mod date_range_picker;
mod date_time_picker;
mod date_time_range_picker;
mod detail_row;
mod detail_section;
mod detail_shell;
mod dialog;
mod dock_region;
mod drawer;
mod duration_input;
mod editable_label;
mod embed_input;
mod embed_preview;
mod embed_shell;
mod empty_state;
mod eyebrow;
mod field;
mod file_upload;
mod filter_toolbar;
mod form_actions;
mod form_dialog;
mod grid;
mod grid_shell;
mod hover_card;
mod icon;
mod icon_button;
mod inline;
mod inline_editable_field;
mod list_card;
mod list_shell;
mod log_list;
mod markdown_editor;
mod media_picker;
mod media_preview;
mod media_thumbnail;
mod menu;
mod menubar;
mod meter;
mod nav_card;
mod nav_card_grid;
mod navigation_menu;
mod number_entry;
mod order_by;
mod page_header;
mod page_loading;
mod pagination;
mod pagination_summary;
mod panel_header;
mod panel_surface;
mod panel_tabs;
mod picker_shell;
mod pill;
mod pin_input;
mod popover;
mod progress;
mod project_header;
mod radio_group;
mod range_calendar;
mod range_slider;
mod rating;
mod relation_picker;
mod reorderable_list;
mod scroll_shell;
mod search_field;
mod segmented_control;
mod select;
mod selection_summary;
mod separator;
mod shell_status_bar;
mod skeleton;
mod slider;
mod slug_field;
mod spacer;
mod split_button;
mod split_view;
mod stack;
mod state_tile;
mod status_indicator;
mod surface;
mod surface_tabs;
mod switch;
mod table;
mod tabs;
mod text_area;
mod text_input;
mod time_ago;
mod time_field;
mod time_zone_select;
mod toast_stack;
mod toggle;
mod toggle_group;
mod toolbar;
mod token_view;
mod tooltip;
mod tri_state_switch;
mod video_player;
mod workspace_shell;
mod zoned_date_time_picker;

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::app_state::{AppState, Section};
use crate::component_registry::{self, ComponentEntry};
use crate::theme_bridge;

// ── Specimen trait ──

/// Trait for components that can render a live specimen in the preview app.
///
/// Each specimen receives the current theme and a mutable reference to the
/// UiTree, and returns the root node of its rendered output.
pub trait Specimen {
    fn render(&self, tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId;
}

/// Route a component slug to its specimen renderer.
///
/// Returns `Some(node_id)` if a specimen exists, `None` otherwise.
fn render_specimen(slug: &str, tree: &mut UiTree, theme: &dyn ThemeProvider) -> Option<UiNodeId> {
    match slug {
        // Structural (g10.003)
        "box" => Some(box_spec::render(tree, theme)),
        "stack" => Some(stack::render(tree, theme)),
        "grid" => Some(grid::render(tree, theme)),
        "surface" => Some(surface::render(tree, theme)),
        "separator" => Some(separator::render(tree, theme)),
        "scroll-shell" => Some(scroll_shell::render(tree, theme)),
        "banner" => Some(banner::render(tree, theme)),
        "callout" => Some(callout::render(tree, theme)),
        "inline" => Some(inline::render(tree, theme)),
        "spacer" => Some(spacer::render(tree, theme)),
        // Action & input (g10.004)
        "button" => Some(button::render(tree, theme)),
        "icon-button" => Some(icon_button::render(tree, theme)),
        "split-button" => Some(split_button::render(tree, theme)),
        "field" => Some(field::render(tree, theme)),
        "text-input" => Some(text_input::render(tree, theme)),
        "text-area" => Some(text_area::render(tree, theme)),
        "search-field" => Some(search_field::render(tree, theme)),
        "form-actions" => Some(form_actions::render(tree, theme)),
        "time-field" => Some(time_field::render(tree, theme)),
        "editable-label" => Some(editable_label::render(tree, theme)),
        "number-entry" => Some(number_entry::render(tree, theme)),
        "pin-input" => Some(pin_input::render(tree, theme)),
        "toolbar" => Some(toolbar::render(tree, theme)),
        // Selection & feedback (g10.005)
        "checkbox" => Some(checkbox::render(tree, theme)),
        "radio-group" => Some(radio_group::render(tree, theme)),
        "switch" => Some(switch::render(tree, theme)),
        "tri-state-switch" => Some(tri_state_switch::render(tree, theme)),
        "select" => Some(select::render(tree, theme)),
        "segmented-control" => Some(segmented_control::render(tree, theme)),
        "slider" => Some(slider::render(tree, theme)),
        "range-slider" => Some(range_slider::render(tree, theme)),
        "progress" => Some(progress::render(tree, theme)),
        "badge" => Some(badge::render(tree, theme)),
        "status-indicator" => Some(status_indicator::render(tree, theme)),
        "meter" => Some(meter::render(tree, theme)),
        "rating" => Some(rating::render(tree, theme)),
        "skeleton" => Some(skeleton::render(tree, theme)),
        "pill" => Some(pill::render(tree, theme)),
        "eyebrow" => Some(eyebrow::render(tree, theme)),
        "time-ago" => Some(time_ago::render(tree, theme)),
        "duration-input" => Some(duration_input::render(tree, theme)),
        "code" => Some(code::render(tree, theme)),
        "color-picker" => Some(color_picker::render(tree, theme)),
        "file-upload" => Some(file_upload::render(tree, theme)),
        // Overlay, datetime, and remaining primitives (g10.006)
        "accordion" => Some(accordion::render(tree, theme)),
        "alert-dialog" => Some(alert_dialog::render(tree, theme)),
        "bulk-action-bar" => Some(bulk_action_bar::render(tree, theme)),
        "calendar" => Some(calendar::render(tree, theme)),
        "card" => Some(card::render(tree, theme)),
        "collapsible" => Some(collapsible::render(tree, theme)),
        "combobox" => Some(combobox::render(tree, theme)),
        "context-menu" => Some(context_menu::render(tree, theme)),
        "date-picker" => Some(date_picker::render(tree, theme)),
        "date-range-picker" => Some(date_range_picker::render(tree, theme)),
        "date-time-picker" => Some(date_time_picker::render(tree, theme)),
        "date-time-range-picker" => Some(date_time_range_picker::render(tree, theme)),
        "detail-row" => Some(detail_row::render(tree, theme)),
        "dialog" => Some(dialog::render(tree, theme)),
        "drawer" => Some(drawer::render(tree, theme)),
        "hover-card" => Some(hover_card::render(tree, theme)),
        "icon" => Some(icon::render(tree, theme)),
        "list-card" => Some(list_card::render(tree, theme)),
        "menu" => Some(menu::render(tree, theme)),
        "menubar" => Some(menubar::render(tree, theme)),
        "nav-card" => Some(nav_card::render(tree, theme)),
        "nav-card-grid" => Some(nav_card_grid::render(tree, theme)),
        "navigation-menu" => Some(navigation_menu::render(tree, theme)),
        "order-by" => Some(order_by::render(tree, theme)),
        "pagination" => Some(pagination::render(tree, theme)),
        "pagination-summary" => Some(pagination_summary::render(tree, theme)),
        "popover" => Some(popover::render(tree, theme)),
        "range-calendar" => Some(range_calendar::render(tree, theme)),
        "table" => Some(table::render(tree, theme)),
        "tabs" => Some(tabs::render(tree, theme)),
        "time-zone-select" => Some(time_zone_select::render(tree, theme)),
        "toggle" => Some(toggle::render(tree, theme)),
        "toggle-group" => Some(toggle_group::render(tree, theme)),
        "tooltip" => Some(tooltip::render(tree, theme)),
        "zoned-date-time-picker" => Some(zoned_date_time_picker::render(tree, theme)),
        // Composites (g10.007)
        "confirm-action" => Some(confirm_action::render(tree, theme)),
        "data-table" => Some(data_table::render(tree, theme)),
        "detail-section" => Some(detail_section::render(tree, theme)),
        "detail-shell" => Some(detail_shell::render(tree, theme)),
        "filter-toolbar" => Some(filter_toolbar::render(tree, theme)),
        "form-dialog" => Some(form_dialog::render(tree, theme)),
        "grid-shell" => Some(grid_shell::render(tree, theme)),
        "list-shell" => Some(list_shell::render(tree, theme)),
        "picker-shell" => Some(picker_shell::render(tree, theme)),
        "relation-picker" => Some(relation_picker::render(tree, theme)),
        "selection-summary" => Some(selection_summary::render(tree, theme)),
        // Editing, media, and operational composites (g10.008)
        "audio-player" => Some(audio_player::render(tree, theme)),
        "autonomous-list" => Some(autonomous_list::render(tree, theme)),
        "block-editor" => Some(block_editor::render(tree, theme)),
        "breadcrumbs" => Some(breadcrumbs::render(tree, theme)),
        "card-radio-group" => Some(card_radio_group::render(tree, theme)),
        "embed-input" => Some(embed_input::render(tree, theme)),
        "embed-preview" => Some(embed_preview::render(tree, theme)),
        "embed-shell" => Some(embed_shell::render(tree, theme)),
        "empty-state" => Some(empty_state::render(tree, theme)),
        "inline-editable-field" => Some(inline_editable_field::render(tree, theme)),
        "log-list" => Some(log_list::render(tree, theme)),
        "markdown-editor" => Some(markdown_editor::render(tree, theme)),
        "media-picker" => Some(media_picker::render(tree, theme)),
        "media-preview" => Some(media_preview::render(tree, theme)),
        "media-thumbnail" => Some(media_thumbnail::render(tree, theme)),
        "page-header" => Some(page_header::render(tree, theme)),
        "page-loading" => Some(page_loading::render(tree, theme)),
        "reorderable-list" => Some(reorderable_list::render(tree, theme)),
        "slug-field" => Some(slug_field::render(tree, theme)),
        "state-tile" => Some(state_tile::render(tree, theme)),
        "toast-stack" => Some(toast_stack::render(tree, theme)),
        "video-player" => Some(video_player::render(tree, theme)),
        // Workstation surfaces (g10.012)
        "app-header" => Some(app_header::render(tree, theme)),
        "command-palette" => Some(command_palette::render(tree, theme)),
        "command-palette-shell" => Some(command_palette_shell::render(tree, theme)),
        "project-header" => Some(project_header::render(tree, theme)),
        "workspace-shell" => Some(workspace_shell::render(tree, theme)),
        "panel-header" => Some(panel_header::render(tree, theme)),
        "panel-surface" => Some(panel_surface::render(tree, theme)),
        "panel-tabs" => Some(panel_tabs::render(tree, theme)),
        "dock-region" => Some(dock_region::render(tree, theme)),
        "split-view" => Some(split_view::render(tree, theme)),
        "shell-status-bar" => Some(shell_status_bar::render(tree, theme)),
        "surface-tabs" => Some(surface_tabs::render(tree, theme)),
        "action-discovery" => Some(action_discovery::render(tree, theme)),
        _ => None,
    }
}

// ── Specimen page layout ──

/// Build the full specimen page for a selected component.
///
/// Layout:
/// ```text
/// SpecimenPage (Column, Grow, gap 16)
/// ├── HeroHeader (Column, gap 8, padding 20, elevated bg, border, rounded)
/// │   ├── TierBadge + ComponentName (Row, gap 8)
/// │   │   ├── Badge (Panel, accent bg, padded, rounded, label)
/// │   │   └── Name (Label, 20px, primary text)
/// │   └── Description (Label, 13px, secondary text)
/// ├── SpecimenSection (Column, gap 8)
/// │   ├── SectionTitle "Specimen" (Label, 11px, secondary text, uppercase)
/// │   └── SpecimenCard (Panel, elevated bg, border, rounded, padding 24)
/// │       └── [specimen content or placeholder]
/// ```
pub fn build_specimen_page(
    tree: &mut UiTree,
    entry: &ComponentEntry,
    theme: &dyn ThemeProvider,
) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    // Page root
    let page = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 16.0,
        ..UiStyle::default()
    });

    // ── Hero header ──
    let hero = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        padding: Edges::all(20.0),
        gap: 8.0,
        background: Some(bg_elevated),
        border_color: Some(border),
        border_width: 1.0,
        corner_radii: [8.0; 4],
        ..UiStyle::default()
    });
    tree.add_child(page, hero);

    // Title row: tier badge + component name
    let title_row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        gap: 8.0,
        align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(hero, title_row);

    // Tier badge
    let badge = tree.create(Widget::Label {
        text: entry.tier.label().to_string(),
    }, UiStyle {
        padding: Edges { top: 2.0, right: 8.0, bottom: 2.0, left: 8.0 },
        corner_radii: [4.0; 4],
        background: Some(theme_bridge::tint(accent, 0.20)),
        text_color: Some(accent),
        text_size: Some(10.0),
        ..UiStyle::default()
    });
    tree.add_child(title_row, badge);

    // Component name
    let name = tree.create(Widget::Label {
        text: entry.display_name.to_string(),
    }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(20.0),
        ..UiStyle::default()
    });
    tree.add_child(title_row, name);

    // Description
    let desc = tree.create(Widget::Label {
        text: entry.description.to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(13.0),
        ..UiStyle::default()
    });
    tree.add_child(hero, desc);

    // ── Specimen section ──
    let section = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(page, section);

    // Section title
    let section_title = tree.create(Widget::Label {
        text: "Specimen".to_string(),
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(11.0),
        padding: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 2.0 },
        ..UiStyle::default()
    });
    tree.add_child(section, section_title);

    // Specimen card container
    let card = build_specimen_card(tree, theme);
    tree.add_child(section, card);

    // Render live specimen or placeholder
    if let Some(specimen_node) = render_specimen(entry.slug, tree, theme) {
        tree.add_child(card, specimen_node);
    } else {
        let placeholder = tree.create(Widget::Label {
            text: format!("{} — specimen coming soon", entry.display_name),
        }, UiStyle {
            text_color: Some(text_secondary),
            text_size: Some(13.0),
            ..UiStyle::default()
        });
        tree.add_child(card, placeholder);
    }

    page
}

/// Build a specimen card container with elevated background, border, and padding.
pub fn build_specimen_card(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        padding: Edges::all(24.0),
        gap: 12.0,
        background: Some(bg_elevated),
        border_color: Some(border),
        border_width: 1.0,
        corner_radii: [8.0; 4],
        ..UiStyle::default()
    })
}

// ── Catalogue landing page ──

/// Build the catalogue landing page shown when no component is selected.
///
/// Layout:
/// ```text
/// CatalogueLanding (Column, Grow, gap 24)
/// ├── Title (Label, 20px, primary text)
/// ├── Description (Label, 13px, secondary text)
/// └── CategoryCards (Column, gap 12)
///     ├── PrimitivesCard (Panel, elevated bg, border, rounded)
///     │   ├── "Primitives" (Label, 14px)
///     │   └── "80 components, X with specimens" (Label, 12px, secondary)
///     └── CompositesCard (Panel, elevated bg, border, rounded)
///         ├── "Composites" (Label, 14px)
///         └── "33 components, X with specimens" (Label, 12px, secondary)
/// ```
pub fn build_catalogue_landing(
    tree: &mut UiTree,
    state: &AppState,
    theme: &dyn ThemeProvider,
) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    let landing = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 24.0,
        ..UiStyle::default()
    });

    // Section title
    let section_name = state.section.label();
    let title = tree.create(Widget::Label {
        text: section_name.to_string(),
    }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(20.0),
        ..UiStyle::default()
    });
    tree.add_child(landing, title);

    // Description
    let components = component_registry::components_for_section(state.section);
    let specimen_count = component_registry::specimen_count(state.section);
    let desc_text = format!(
        "{} components registered, {} with live specimens",
        components.len(),
        specimen_count,
    );
    let desc = tree.create(Widget::Label {
        text: desc_text,
    }, UiStyle {
        text_color: Some(text_secondary),
        text_size: Some(13.0),
        ..UiStyle::default()
    });
    tree.add_child(landing, desc);

    // Component overview grid
    let grid = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 8.0,
        ..UiStyle::default()
    });
    tree.add_child(landing, grid);

    // Show a summary card for each category within the current section
    if !components.is_empty() {
        // Group by first letter for a compact category overview
        let categories = categorize_components(components);
        for (letter, count) in &categories {
            let row = tree.create(Widget::Panel, UiStyle {
                direction: Direction::Row,
                width: Sizing::Grow(1.0),
                padding: Edges { top: 6.0, right: 12.0, bottom: 6.0, left: 12.0 },
                gap: 8.0,
                align: Align::Center,
                background: Some(bg_elevated),
                border_color: Some(border),
                border_width: 1.0,
                corner_radii: [6.0; 4],
                ..UiStyle::default()
            });
            tree.add_child(grid, row);

            let letter_label = tree.create(Widget::Label {
                text: letter.to_string(),
            }, UiStyle {
                text_color: Some(accent),
                text_size: Some(14.0),
                ..UiStyle::default()
            });
            tree.add_child(row, letter_label);

            let count_label = tree.create(Widget::Label {
                text: format!("{} component{}", count, if *count == 1 { "" } else { "s" }),
            }, UiStyle {
                text_color: Some(text_secondary),
                text_size: Some(12.0),
                ..UiStyle::default()
            });
            tree.add_child(row, count_label);
        }
    }

    // Instruction hint
    let hint = tree.create(Widget::Label {
        text: "Select a component from the sidebar to view its specimen.".to_string(),
    }, UiStyle {
        text_color: Some(theme_bridge::tint(text_secondary, 0.7)),
        text_size: Some(12.0),
        padding: Edges { top: 8.0, right: 0.0, bottom: 0.0, left: 0.0 },
        ..UiStyle::default()
    });
    tree.add_child(landing, hint);

    landing
}

/// Group components by first letter, returning (letter, count) pairs.
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

// ── Content area builder ──

/// Build the content for the specimen area based on current app state.
///
/// If a component is selected → specimen page.
/// If no component is selected → catalogue landing.
/// If Demo or Tokens section → section-specific placeholder.
pub fn build_content(
    tree: &mut UiTree,
    state: &AppState,
    theme: &dyn ThemeProvider,
) -> UiNodeId {
    match state.section {
        Section::Demo => demos::render_single_screen(state.active_demo_screen, tree, theme),
        Section::Tokens => token_view::render_token_inspector(tree, theme),
        Section::Primitives | Section::Composites => {
            let components = component_registry::components_for_section(state.section);
            match state.active_component() {
                Some(idx) if idx < components.len() => {
                    build_specimen_page(tree, &components[idx], theme)
                }
                _ => build_catalogue_landing(tree, state, theme),
            }
        }
    }
}

