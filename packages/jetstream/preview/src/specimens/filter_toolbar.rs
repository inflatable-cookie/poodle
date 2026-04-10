//! FilterToolbar specimen — search and filter controls bar.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::filter_toolbar::js_filter_toolbar;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::FilterToolbarSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("With search and filters", secondary,
            js_filter_toolbar(
                &FilterToolbarSpec::new()
                    .with_query("review")
                    .with_active_filter_count(2),
                theme,
                Some(label("Search field").text_color(text_primary).text_size(13.0)),
                Some(label("Filter chips").text_color(secondary).text_size(13.0)),
            )
        ))
        .child(group("Empty", secondary,
            js_filter_toolbar(&FilterToolbarSpec::new(), theme, None, None)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
