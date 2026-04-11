//! FileUpload specimen — default dropzone, dragging, disabled.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::file_upload::js_file_upload;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::FileUploadSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Default
        .child(group("Default dropzone", secondary,
            div().w(400.0)
                .child(js_file_upload(&FileUploadSpec::new(), theme))
        ))
        // With accept filter
        .child(group("Accept images only", secondary,
            div().w(400.0)
                .child(js_file_upload(&FileUploadSpec::new().with_accept("image/*"), theme))
        ))
        // Dragging state
        .child(group("Dragging over", secondary,
            div().w(400.0)
                .child(js_file_upload(&FileUploadSpec::new().with_dragging(true), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(400.0)
                .child(js_file_upload(&FileUploadSpec::new().with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
