//! DetailSection specimen — labeled content section within a detail view.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::detail_section::js_detail_section;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::DetailSectionSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("With title and body", secondary,
            js_detail_section(
                &DetailSectionSpec::new()
                    .with_title("Overview")
                    .with_description("General information about this item."),
                theme,
                Some(label("Section body content goes here.").text_color(text_primary).text_size(13.0)),
            )
        ))
        .child(group("Empty body", secondary,
            js_detail_section(
                &DetailSectionSpec::new().with_title("Empty Section"),
                theme,
                None,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
