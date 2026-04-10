//! EmptyState specimen — placeholder for empty content areas.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::empty_state::js_empty_state;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::EmptyStateSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Default", secondary,
            js_empty_state(&EmptyStateSpec::new("No items found"), theme)
        ))
        .child(group("With message", secondary,
            js_empty_state(
                &EmptyStateSpec::new("No results")
                    .with_message("Try adjusting your search or filters."),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
