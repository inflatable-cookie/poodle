//! Specimen framework — renders live component specimens in the content area.
//!
//! Each component returns a `JsEl` value describing its specimen.
//! The framework wraps specimens in a consistent card layout with a hero header.

pub mod accordion;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod progress;
pub mod separator;
pub mod status_indicator;
pub mod switch;

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
        "badge" => Some(badge::render(theme)),
        "button" => Some(button::render(theme)),
        "checkbox" => Some(checkbox::render(theme)),
        "progress" => Some(progress::render(theme)),
        "separator" => Some(separator::render(theme)),
        "status-indicator" => Some(status_indicator::render(theme)),
        "switch" => Some(switch::render(theme)),
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
