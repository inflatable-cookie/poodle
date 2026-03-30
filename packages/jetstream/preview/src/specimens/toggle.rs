//! Toggle specimen — pressed/unpressed, variants, sizes, disabled.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::toggle::js_toggle;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{ButtonVariant, ControlSize, ToggleSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Pressed / unpressed
        .child(group("States", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_toggle(&ToggleSpec::new().with_label("Unpressed").with_icon("B"), theme))
                .child(js_toggle(&ToggleSpec::new().with_label("Pressed").with_icon("B").with_pressed(true), theme))
        ))
        // Variants
        .child(group("Variants", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_toggle(&ToggleSpec::new().with_variant(ButtonVariant::Ghost).with_label("Ghost").with_pressed(true), theme))
                .child(js_toggle(&ToggleSpec::new().with_variant(ButtonVariant::Secondary).with_label("Secondary").with_pressed(true), theme))
                .child(js_toggle(&ToggleSpec::new().with_variant(ButtonVariant::Primary).with_label("Primary").with_pressed(true), theme))
        ))
        // Sizes
        .child(group("Sizes", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_toggle(&ToggleSpec::new().with_size(ControlSize::Sm).with_label("Small").with_pressed(true), theme))
                .child(js_toggle(&ToggleSpec::new().with_size(ControlSize::Md).with_label("Medium").with_pressed(true), theme))
                .child(js_toggle(&ToggleSpec::new().with_size(ControlSize::Lg).with_label("Large").with_pressed(true), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().flex_row().gap(8.0).items_center()
                .child(js_toggle(&ToggleSpec::new().with_label("Disabled off").with_disabled(true), theme))
                .child(js_toggle(&ToggleSpec::new().with_label("Disabled on").with_pressed(true).with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
