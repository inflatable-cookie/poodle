//! MediaUploadStatusPanel specimen — upload workflow status display.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::media_upload_status_panel::js_media_upload_status_panel;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{MediaUploadStatusPanelSpec, MediaUploadStep};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Uploading (65%)", secondary,
            js_media_upload_status_panel(
                &MediaUploadStatusPanelSpec::new()
                    .with_step(MediaUploadStep::Uploading)
                    .with_upload_progress(65.0),
                theme,
            )
        ))
        .child(group("Complete", secondary,
            js_media_upload_status_panel(
                &MediaUploadStatusPanelSpec::new()
                    .with_step(MediaUploadStep::Complete),
                theme,
            )
        ))
        .child(group("Error", secondary,
            js_media_upload_status_panel(
                &MediaUploadStatusPanelSpec::new()
                    .with_step(MediaUploadStep::Error)
                    .with_upload_error("Network timeout — please try again."),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
