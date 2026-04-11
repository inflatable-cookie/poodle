//! LogList specimen — timestamped log viewer.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::log_list::js_log_list;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::LogListSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("With entries", secondary,
            js_log_list(
                &LogListSpec::new()
                    .with_entry_count(8)
                    .with_auto_scroll(true),
                theme,
            )
        ))
        .child(group("Filtered to errors", secondary,
            js_log_list(
                &LogListSpec::new()
                    .with_entry_count(12)
                    .with_filter_level("error"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
