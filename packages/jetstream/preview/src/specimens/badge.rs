//! Badge specimen.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::badge::js_badge;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{BadgeSpec, BadgeVariant};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Variants", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_badge(&BadgeSpec::new().with_variant(BadgeVariant::Accent), theme))
                .child(js_badge(&BadgeSpec::new().with_variant(BadgeVariant::Muted), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
