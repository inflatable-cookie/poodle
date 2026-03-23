//! Pill specimen — compact inline labels.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::pill::js_pill;
use flint_jetstream_components::theme_ext::*;
use flint_primitives::PillSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Default", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_pill(&PillSpec::new().with_label("Label"), theme))
                .child(js_pill(&PillSpec::new().with_label("Status"), theme))
                .child(js_pill(&PillSpec::new().with_label("Tag"), theme))
        ))
        .child(group("Selected", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_pill(&PillSpec::new().with_label("Active").with_selected(true), theme))
        ))
        .child(group("Disabled", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_pill(&PillSpec::new().with_label("Disabled").with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
