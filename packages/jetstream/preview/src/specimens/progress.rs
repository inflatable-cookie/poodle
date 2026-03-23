//! Progress specimen.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::progress::js_progress;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::ProgressSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Determinate", secondary,
            div().flex_col().gap(12.0).self_stretch()
                .child(js_progress(&ProgressSpec::new().with_value(0.25), theme))
                .child(js_progress(&ProgressSpec::new().with_value(0.50), theme))
                .child(js_progress(&ProgressSpec::new().with_value(0.75), theme))
                .child(js_progress(&ProgressSpec::new().with_value(1.0), theme))
        ))
        .child(group("Indeterminate", secondary,
            div().flex_col().gap(12.0).self_stretch()
                .child(js_progress(&ProgressSpec::new().with_indeterminate(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
