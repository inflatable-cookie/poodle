//! MediaPicker specimen — media asset selection dialog.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::media_picker::js_media_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::MediaPickerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Browse tab", secondary,
            js_media_picker(
                &MediaPickerSpec::new("Select Media")
                    .with_open(true),
                theme,
            )
        ))
        .child(group("With selections", secondary,
            js_media_picker(
                &MediaPickerSpec::new("Attach Assets")
                    .with_open(true)
                    .with_multiple(true)
                    .with_selected_count(2),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
