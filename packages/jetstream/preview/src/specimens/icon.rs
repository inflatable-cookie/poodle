//! Icon specimen — shows icons at different sizes and with color inheritance.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::icon::js_icon;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{IconSize, IconSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let danger = resolve_color(theme, "color.status.danger");

    div().flex_col().gap(24.0)
        // Sizes
        .child(group("Sizes", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_icon(&IconSpec::new("star").with_size(IconSize::Sm), theme))
                .child(js_icon(&IconSpec::new("star").with_size(IconSize::Md), theme))
                .child(js_icon(&IconSpec::new("star").with_size(IconSize::Lg), theme))
        ))
        // Color inheritance
        .child(group("Color inheritance", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(div().text_color(accent).child(js_icon(&IconSpec::new("check-circle"), theme)))
                .child(div().text_color(danger).child(js_icon(&IconSpec::new("alert-triangle"), theme)))
                .child(js_icon(&IconSpec::new("settings"), theme))
        ))
        // Common icons
        .child(group("Common icons", secondary,
            div().flex_row().gap(12.0).items_center().flex_wrap()
                .child(js_icon(&IconSpec::new("chevron-down"), theme))
                .child(js_icon(&IconSpec::new("chevron-right"), theme))
                .child(js_icon(&IconSpec::new("plus"), theme))
                .child(js_icon(&IconSpec::new("x"), theme))
                .child(js_icon(&IconSpec::new("search"), theme))
                .child(js_icon(&IconSpec::new("edit"), theme))
                .child(js_icon(&IconSpec::new("trash"), theme))
                .child(js_icon(&IconSpec::new("loader"), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
