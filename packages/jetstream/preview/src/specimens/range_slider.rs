//! RangeSlider specimen — dual-thumb range selection slider.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::range_slider::js_range_slider;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::RangeSliderSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Default range: 20–80
        .child(group("Default range (20–80)", secondary,
            div().w(300.0).child(
                js_range_slider(&RangeSliderSpec::new(20.0, 80.0), theme)
            )
        ))
        // Narrow range near centre
        .child(group("Narrow range (45–55)", secondary,
            div().w(300.0).child(
                js_range_slider(&RangeSliderSpec::new(45.0, 55.0), theme)
            )
        ))
        // Full range
        .child(group("Full range (0–100)", secondary,
            div().w(300.0).child(
                js_range_slider(&RangeSliderSpec::new(0.0, 100.0), theme)
            )
        ))
        // Low end only
        .child(group("Low end (0–25)", secondary,
            div().w(300.0).child(
                js_range_slider(&RangeSliderSpec::new(0.0, 25.0), theme)
            )
        ))
        // High end only
        .child(group("High end (75–100)", secondary,
            div().w(300.0).child(
                js_range_slider(&RangeSliderSpec::new(75.0, 100.0), theme)
            )
        ))
        // Disabled
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
