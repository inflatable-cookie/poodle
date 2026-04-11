//! PageHeader specimen — page-level title and subtitle header.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::page_header::js_page_header;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::PageHeaderSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Title only", secondary,
            js_page_header(&PageHeaderSpec::new("Dashboard"), theme)
        ))
        .child(group("With subtitle", secondary,
            js_page_header(
                &PageHeaderSpec::new("Settings")
                    .with_subtitle("Manage your account preferences."),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
