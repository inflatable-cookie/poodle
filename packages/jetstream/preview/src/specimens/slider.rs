//! Slider specimen — sliders at different values and disabled state.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::slider::js_slider;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::SliderSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Default (50%)
        .child(group("Default (50%)", secondary,
            div().w(300.0)
                .child(js_slider(&SliderSpec::new(50.0).with_bounds(0.0, 100.0), theme))
        ))
        // Low value
        .child(group("Low value (10%)", secondary,
            div().w(300.0)
                .child(js_slider(&SliderSpec::new(10.0).with_bounds(0.0, 100.0), theme))
        ))
        // High value
        .child(group("High value (90%)", secondary,
            div().w(300.0)
                .child(js_slider(&SliderSpec::new(90.0).with_bounds(0.0, 100.0), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = SliderSpec::new(50.0).with_bounds(0.0, 100.0);
            spec.is_disabled = true;
            div().w(300.0).child(js_slider(&spec, theme))
        }))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
