//! SearchInput specimen — search inputs with default and value states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::search_input::js_search_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::SearchInputSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Default
        .child(group("Default", secondary,
            div().w(300.0)
                .child(js_search_input(&SearchInputSpec::new(), theme))
        ))
        // With value
        .child(group("With value", secondary,
            div().w(300.0)
                .child(js_search_input(&SearchInputSpec::new().with_value("search query"), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(300.0)
                .child(js_search_input(&SearchInputSpec::new().with_value("locked search").with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
