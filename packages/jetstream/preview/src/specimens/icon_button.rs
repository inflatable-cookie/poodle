//! IconButton specimen — icon-only buttons with variants, tones, sizes, and disabled state.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::icon_button::js_icon_button;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ButtonTone, ButtonVariant, ControlSize, IconButtonSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Ghost (default)
        .child(group("Ghost (default)", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_icon_button(&IconButtonSpec::new().with_icon("plus"), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("edit"), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("trash"), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("settings"), theme))
        ))
        // Primary
        .child(group("Primary", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_icon_button(&IconButtonSpec::new().with_icon("plus").with_variant(ButtonVariant::Primary), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("check").with_variant(ButtonVariant::Primary), theme))
        ))
        // Secondary
        .child(group("Secondary", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_icon_button(&IconButtonSpec::new().with_icon("search").with_variant(ButtonVariant::Secondary), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("filter").with_variant(ButtonVariant::Secondary), theme))
        ))
        // Danger tone (Ghost + Danger)
        .child(group("Danger tone", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_icon_button(&IconButtonSpec::new().with_icon("trash").with_tone(ButtonTone::Danger), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("x").with_variant(ButtonVariant::Primary).with_tone(ButtonTone::Danger), theme))
        ))
        // Small size
        .child(group("Small size", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_icon_button(&IconButtonSpec::new().with_icon("plus").with_size(ControlSize::Sm), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("edit").with_size(ControlSize::Sm), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("trash").with_size(ControlSize::Sm).with_tone(ButtonTone::Danger), theme))
        ))
        // Large size
        .child(group("Large size", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_icon_button(&IconButtonSpec::new().with_icon("plus").with_size(ControlSize::Lg), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("settings").with_size(ControlSize::Lg), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_icon_button(&IconButtonSpec::new().with_icon("plus").with_disabled(true), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("edit").with_variant(ButtonVariant::Primary).with_disabled(true), theme))
                .child(js_icon_button(&IconButtonSpec::new().with_icon("trash").with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
