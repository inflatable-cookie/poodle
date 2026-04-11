//! StatusIndicator specimen.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::status_indicator::js_status_indicator;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{StatusIndicatorSpec, StatusTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    fn indicator(tone: StatusTone, label: &str) -> StatusIndicatorSpec {
        let mut s = StatusIndicatorSpec::new().with_status(tone);
        s.label = Some(label.into());
        s
    }

    div().flex_col().gap(24.0)
        .child(group("Tones", secondary,
            div().flex_col().gap(12.0)
                .child(js_status_indicator(&indicator(StatusTone::Neutral, "Neutral"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Info, "Info"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Success, "Success"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Warning, "Warning"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Danger, "Danger"), theme))
                .child(js_status_indicator(&indicator(StatusTone::Pending, "Pending"), theme))
        ))
        .child(group("Without label", secondary,
            div().flex_row().gap(12.0)
                .child(js_status_indicator(&StatusIndicatorSpec::new().with_status(StatusTone::Success), theme))
                .child(js_status_indicator(&StatusIndicatorSpec::new().with_status(StatusTone::Warning), theme))
                .child(js_status_indicator(&StatusIndicatorSpec::new().with_status(StatusTone::Danger), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
