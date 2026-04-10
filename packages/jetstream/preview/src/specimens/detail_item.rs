//! DetailItem specimen — key-value pair rows.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::detail_item::js_detail_item;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::DetailItemSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Key-value rows", secondary,
            div().flex_col().gap(4.0)
                .child(js_detail_item(
                    &DetailItemSpec::new("Name").with_value("Jetstream Preview"),
                    theme,
                ))
                .child(js_detail_item(
                    &DetailItemSpec::new("Version").with_value("0.1.0"),
                    theme,
                ))
                .child(js_detail_item(
                    &DetailItemSpec::new("Status")
                        .with_value("Active")
                        .with_description("Running in development mode"),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
