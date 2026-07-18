//! FileUpload specimen — dropzone states + populated file list.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::file_upload::js_file_upload;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, ControlSize, FileUploadItem, FileUploadSpec, FileUploadStatus};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Default
        .child(group(
            "Default dropzone",
            secondary,
            wrap(js_file_upload(&FileUploadSpec::new(), theme)),
        ))
        // Accept filter + max size
        .child(group(
            "Accept images only",
            secondary,
            wrap(js_file_upload(
                &FileUploadSpec::new()
                    .with_accept("image/*")
                    .with_max_size(5 * 1024 * 1024),
                theme,
            )),
        ))
        // Populated image list with preview + progress
        .child(group(
            "Image upload with list",
            secondary,
            wrap(js_file_upload(
                &FileUploadSpec::new()
                    .with_accept("image/*")
                    .with_multiple(true)
                    .with_file(
                        FileUploadItem::new("a", "hero-banner.png", 1_887_436)
                            .with_preview(true)
                            .with_status(FileUploadStatus::Complete),
                    )
                    .with_file(
                        FileUploadItem::new("b", "profile-photo.jpg", 524_288)
                            .with_preview(true)
                            .with_progress(60),
                    ),
                theme,
            )),
        ))
        // Document (no preview) + error row
        .child(group(
            "Document upload + error",
            secondary,
            wrap(js_file_upload(
                &FileUploadSpec::new()
                    .with_accept(".pdf,.doc,.docx,.txt")
                    .with_show_preview(false)
                    .with_file(FileUploadItem::new("c", "spec-sheet.pdf", 3_355_443))
                    .with_file(
                        FileUploadItem::new("d", "raw-capture.tiff", 99_614_720)
                            .with_error("Exceeds 10 MB limit"),
                    ),
                theme,
            )),
        ))
        // Validation error line
        .child(group(
            "Validation error",
            secondary,
            wrap(js_file_upload(
                &FileUploadSpec::new()
                    .with_accept("image/*")
                    .with_validation_error("Only image files are accepted"),
                theme,
            )),
        ))
        // Dragging state
        .child(group(
            "Dragging over",
            secondary,
            wrap(js_file_upload(&FileUploadSpec::new().with_dragging(true), theme)),
        ))
        // Size + density
        .child(group(
            "Small / compact",
            secondary,
            wrap(js_file_upload(
                &FileUploadSpec::new()
                    .with_size(ControlSize::Sm)
                    .with_density(ControlDensity::Compact),
                theme,
            )),
        ))
        .child(group(
            "Large / comfortable",
            secondary,
            wrap(js_file_upload(
                &FileUploadSpec::new()
                    .with_size(ControlSize::Lg)
                    .with_density(ControlDensity::Comfortable),
                theme,
            )),
        ))
        // Disabled
        .child(group(
            "Disabled",
            secondary,
            wrap(js_file_upload(&FileUploadSpec::new().with_disabled(true), theme)),
        ))
}

fn wrap(content: JsEl) -> JsEl {
    div().w(400.0).child(content)
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
