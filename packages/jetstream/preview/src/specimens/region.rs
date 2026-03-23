//! Region specimen — semantic regions with labels and borders.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::region::js_region;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::RegionSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("Labeled region", secondary,
            js_region(&RegionSpec::new().with_label("Section Title"), theme, vec![
                label("Region content goes here.").text_color(text_primary).text_size(13.0),
            ])
        ))
        .child(group("Unlabeled region", secondary,
            js_region(&RegionSpec::new(), theme, vec![
                label("Unlabeled region content.").text_color(text_primary).text_size(13.0),
            ])
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
