//! DatePicker specimen — date selection input.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::date_picker::js_date_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::DatePickerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Empty", secondary,
            js_date_picker(&DatePickerSpec::new(), theme)
        ))
        .child(group("With value", secondary,
            js_date_picker(
                &DatePickerSpec::new().with_default_value("2026-03-21"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
