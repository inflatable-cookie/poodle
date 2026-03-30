//! TimeZoneSelect specimen — with value, placeholder.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::time_zone_select::js_time_zone_select;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::TimeZoneSelectSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // With value
        .child(group("With value", secondary,
            div().w(280.0)
                .child(js_time_zone_select(&TimeZoneSelectSpec::new().with_value("America/New_York"), theme))
        ))
        // Placeholder
        .child(group("Placeholder", secondary,
            div().w(280.0)
                .child(js_time_zone_select(&TimeZoneSelectSpec::new().with_placeholder("Select timezone..."), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(280.0)
                .child(js_time_zone_select(&TimeZoneSelectSpec::new().with_value("Europe/London").with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
