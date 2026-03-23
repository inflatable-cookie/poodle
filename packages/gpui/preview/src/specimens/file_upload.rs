use gpui::*;
use pug_primitives::{FileUploadSpec, EyebrowSpec};
use pug_gpui_components::{FileUpload, Eyebrow};
use pug_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0)).max_w(px(400.0))
        // --- Image upload with preview ---
        .child(
            div().flex().flex_col().gap(px(10.0))
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
            div().flex().flex_col().gap(px(10.0))
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
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    FileUpload::from_spec(
                        FileUploadSpec::new()
                            .with_disabled(true),
                        theme,
                    )
                )
        )
}
