//! TimeAgo specimen — relative time labels.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::time_ago::js_time_ago;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::TimeAgoSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Timestamps", secondary,
            div().flex_col().gap(6.0)
                .child(js_time_ago(
                    &TimeAgoSpec::new().with_timestamp("2 minutes ago"),
                    theme,
                ))
                .child(js_time_ago(
                    &TimeAgoSpec::new().with_timestamp("3 hours ago"),
                    theme,
                ))
                .child(js_time_ago(
                    &TimeAgoSpec::new().with_timestamp("Yesterday"),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
