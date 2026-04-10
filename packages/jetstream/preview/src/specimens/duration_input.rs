//! DurationInput specimen — with value, disabled, sizes.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::duration_input::js_duration_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{ControlSize, DurationInputSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // With value
        .child(group("With value", secondary,
            div().w(240.0)
                .child(js_duration_input(&DurationInputSpec::new().with_value("02:30"), theme))
        ))
        // With seconds
        .child(group("With seconds", secondary,
            div().w(240.0)
                .child(js_duration_input(&DurationInputSpec::new().with_value("01:15:45").with_show_seconds(true), theme))
        ))
        // Sizes
        .child(group("Sizes", secondary,
            div().flex_col().gap(8.0)
                .child(div().w(240.0).child(js_duration_input(&DurationInputSpec::new().with_value("00:30").with_size(ControlSize::Sm), theme)))
                .child(div().w(240.0).child(js_duration_input(&DurationInputSpec::new().with_value("00:30").with_size(ControlSize::Md), theme)))
                .child(div().w(240.0).child(js_duration_input(&DurationInputSpec::new().with_value("00:30").with_size(ControlSize::Lg), theme)))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(240.0)
                .child(js_duration_input(&DurationInputSpec::new().with_value("01:00").with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
