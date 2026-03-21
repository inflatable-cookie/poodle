//! IconButton specimen — icon-only buttons with variants and states.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::icon_button::js_icon_button;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::IconButtonSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Default
        .child(group("Default", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_icon_button(&IconButtonSpec::new().with_icon("plus"), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("edit"), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("trash"), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("settings"), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_icon_button(&IconButtonSpec::new().with_icon("plus").with_disabled(true), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("edit").with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
