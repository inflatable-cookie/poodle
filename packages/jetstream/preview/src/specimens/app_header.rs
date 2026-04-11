//! AppHeader specimen — application header bar.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::app_header::js_app_header;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::AppHeaderSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("With title", secondary,
            js_app_header(&AppHeaderSpec::new().with_title("My Application"), theme)
        ))
        .child(group("Without title", secondary,
            js_app_header(&AppHeaderSpec::new(), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
