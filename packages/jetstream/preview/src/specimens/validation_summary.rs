//! ValidationSummary specimen — validation error and warning list.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::validation_summary::js_validation_summary;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{ValidationSummaryEntry, ValidationSummarySpec};
use poodle_components::ValidationState;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let entries = vec![
        ValidationSummaryEntry::new(
            "title", "Project title", "Title is required.", ValidationState::Invalid,
        ),
        ValidationSummaryEntry::new(
            "slug", "URL slug", "Slug contains invalid characters.", ValidationState::Invalid,
        ),
        ValidationSummaryEntry::new(
            "search", "Asset search", "Checking references.", ValidationState::Pending,
        ),
    ];

    div().flex_col().gap(24.0)
        .child(group("With errors", secondary,
            js_validation_summary(
                &ValidationSummarySpec::new(entries)
                    .with_title("Fix the following fields")
                    .with_include_pending(true),
                theme,
            )
        ))
        .child(group("Clean (no entries)", secondary,
            js_validation_summary(
                &ValidationSummarySpec::new(vec![])
                    .with_title("Validation"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
