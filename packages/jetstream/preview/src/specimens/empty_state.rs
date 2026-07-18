//! EmptyState specimen — placeholder for empty content areas.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::empty_state::js_empty_state;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ButtonVariant, EmptyStateSpec, EmptyStateVariant, RemediationAction};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Neutral variant (default)
        .child(group("Neutral variant", secondary,
            js_empty_state(&EmptyStateSpec::new("Nothing here yet"), theme)
        ))
        // Search variant
        .child(group("Search variant", secondary,
            js_empty_state(
                &EmptyStateSpec::new("No results")
                    .with_variant(EmptyStateVariant::Search)
                    .with_message("Try adjusting your search or filters."),
                theme,
            )
        ))
        // FirstRun variant
        .child(group("First-run variant", secondary,
            js_empty_state(
                &EmptyStateSpec::new("Get started")
                    .with_variant(EmptyStateVariant::FirstRun)
                    .with_message("Create your first item to begin."),
                theme,
            )
        ))
        // With single action
        .child(group("With action", secondary,
            js_empty_state(
                &EmptyStateSpec::new("No items")
                    .with_message("Create an item to get started.")
                    .with_actions(vec![
                        RemediationAction::new("create", "New Item")
                            .with_variant(ButtonVariant::Primary),
                    ]),
                theme,
            )
        ))
        // With multiple actions
        .child(group("Multiple actions", secondary,
            js_empty_state(
                &EmptyStateSpec::new("Nothing found")
                    .with_message("Adjust your search or clear all filters.")
                    .with_actions(vec![
                        RemediationAction::new("clear", "Clear Filters"),
                        RemediationAction::new("new", "New Item")
                            .with_variant(ButtonVariant::Primary),
                    ]),
                theme,
            )
        ))
        // Compact mode
        .child(group("Compact", secondary,
            js_empty_state(
                &EmptyStateSpec::new("No results")
                    .with_variant(EmptyStateVariant::Search)
                    .with_compact(true),
                theme,
            )
        ))
        // Compact with message
        .child(group("Compact with message", secondary,
            js_empty_state(
                &EmptyStateSpec::new("No results")
                    .with_message("Try a different search term.")
                    .with_compact(true),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
