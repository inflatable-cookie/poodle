use gpui::*;
use poodle_specs::{MediaUploadStatusPanelSpec, MediaUploadStep};
use poodle_specs::{ControlSize, EyebrowSpec};
use poodle_gpui_components::{MediaUploadStatusPanel, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Upload status panel"), theme))
                .child(
                    div().flex().flex_col().gap(px(12.0))
                        .child(
                            MediaUploadStatusPanel::from_spec(
                                MediaUploadStatusPanelSpec::new()
                                    .with_step(MediaUploadStep::Checking),
                                theme,
                            )
                        )
                        .child(
                            MediaUploadStatusPanel::from_spec(
                                MediaUploadStatusPanelSpec::new()
                                    .with_step(MediaUploadStep::Duplicate)
                                    .with_duplicate_label("hero-banner.jpg"),
                                theme,
                            )
                        )
                        .child(
                            MediaUploadStatusPanel::from_spec(
                                MediaUploadStatusPanelSpec::new()
                                    .with_step(MediaUploadStep::Uploading)
                                    .with_upload_progress(62.0),
                                theme,
                            )
                        )
                        .child(
                            MediaUploadStatusPanel::from_spec(
                                MediaUploadStatusPanelSpec::new()
                                    .with_step(MediaUploadStep::Complete),
                                theme,
                            )
                        )
                        .child(
                            MediaUploadStatusPanel::from_spec(
                                MediaUploadStatusPanelSpec::new()
                                    .with_step(MediaUploadStep::Error)
                                    .with_upload_error("Upload failed because the file could not be finalised."),
                                theme,
                            )
                        )
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_col().gap(px(12.0))
                        .child(
                            MediaUploadStatusPanel::from_spec(
                                MediaUploadStatusPanelSpec::new()
                                    .with_step(MediaUploadStep::Uploading)
                                    .with_upload_progress(40.0)
                                    .with_size(ControlSize::Sm),
                                theme,
                            )
                        )
                        .child(
                            MediaUploadStatusPanel::from_spec(
                                MediaUploadStatusPanelSpec::new()
                                    .with_step(MediaUploadStep::Uploading)
                                    .with_upload_progress(40.0)
                                    .with_size(ControlSize::Md),
                                theme,
                            )
                        )
                        .child(
                            MediaUploadStatusPanel::from_spec(
                                MediaUploadStatusPanelSpec::new()
                                    .with_step(MediaUploadStep::Uploading)
                                    .with_upload_progress(40.0)
                                    .with_size(ControlSize::Lg),
                                theme,
                            )
                        )
                )
        )
}
