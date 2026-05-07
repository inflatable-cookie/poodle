//! Pill specimen — compact inline labels.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::pill::js_pill;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{InlineTypographyMode, PillSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

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
        .child(group("Inherit typography", secondary,
            div().flex_row().gap(8.0).items_center().text_size(20.0)
                .child(label("Release"))
                .child(js_pill(
                    &PillSpec::new()
                        .with_label("Beta")
                        .with_typography(InlineTypographyMode::Inherit),
                    theme,
                ))
                .child(label("channel"))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
