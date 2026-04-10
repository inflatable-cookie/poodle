use gpui::*;
use poodle_primitives::{FileUploadSpec, EyebrowSpec};
use poodle_gpui_components::{FileUpload, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0)).max_w(px(400.0))
        // --- Image upload with preview ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Image upload with preview"), theme))
                .child(
                    FileUpload::from_spec(
                        FileUploadSpec::new()
                            .with_accept("image/*")
                            .with_multiple(true)
                            .with_max_size(5 * 1024 * 1024),
                        theme,
                    )
                )
        )
        // --- Document upload (single file) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Document upload (single file)"), theme))
                .child(
                    FileUpload::from_spec(
                        FileUploadSpec::new()
                            .with_accept(".pdf,.doc,.docx,.txt")
                            .with_max_size(10 * 1024 * 1024),
                        theme,
                    )
                )
        )
        // --- Dragging state (active drop target) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Dragging state"), theme))
                .child(
                    FileUpload::from_spec(
                        FileUploadSpec::new()
                            .with_accept("image/*")
                            .with_multiple(true)
                            .with_dragging(true),
                        theme,
                    )
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    FileUpload::from_spec(
                        FileUploadSpec::new()
                            .with_disabled(true),
                        theme,
                    )
                )
        )
        // --- With max files cap ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With max files cap"), theme))
                .child(
                    FileUpload::from_spec(
                        FileUploadSpec::new()
                            .with_accept("image/*")
                            .with_multiple(true)
                            .with_max_files(5)
                            .with_max_size(5 * 1024 * 1024),
                        theme,
                    )
                )
        )
        // --- Image compression ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Image compression"), theme))
                .child(
                    FileUpload::from_spec(
                        FileUploadSpec::new()
                            .with_accept("image/*")
                            .with_multiple(true)
                            .with_compress(true)
                            .with_max_files(10),
                        theme,
                    )
                )
        )
        // --- Validation error ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Validation error"), theme))
                .child(
                    FileUpload::from_spec(
                        FileUploadSpec::new()
                            .with_accept(".pdf,.doc,.docx")
                            .with_max_size(2 * 1024 * 1024)
                            .with_validation_error("File \"contract.pdf\" exceeds the 2 MB limit."),
                        theme,
                    )
                )
        )
}
