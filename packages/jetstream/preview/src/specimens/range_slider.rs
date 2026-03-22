//! RangeSlider specimen — dual-thumb range selection slider.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::range_slider::js_range_slider;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::RangeSliderSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Default range", secondary,
            div().w(300.0).child(
                js_range_slider(&RangeSliderSpec::new(20.0, 80.0), theme)
            )
        ))
        .child(group("Disabled", secondary,
            div().w(300.0).child(
                js_range_slider(
                    &RangeSliderSpec::new(30.0, 70.0).with_disabled(true),
                    theme,
                )
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
