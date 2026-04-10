//! Rating specimen — ratings at different values and disabled state.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::rating::js_rating;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::RatingSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Low rating
        .child(group("1 of 5", secondary,
            js_rating(&RatingSpec::new().with_value(1.0).with_max(5), theme)
        ))
        // Medium rating
        .child(group("3 of 5", secondary,
            js_rating(&RatingSpec::new().with_value(3.0).with_max(5), theme)
        ))
        // Full rating
        .child(group("5 of 5", secondary,
            js_rating(&RatingSpec::new().with_value(5.0).with_max(5), theme)
        ))
        // Disabled
        .child(group("Disabled (3 of 5)", secondary,
            js_rating(&RatingSpec::new().with_value(3.0).with_max(5).with_disabled(true), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
