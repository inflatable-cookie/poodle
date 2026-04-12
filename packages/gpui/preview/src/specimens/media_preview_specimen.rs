use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, MediaPreview};
use poodle_specs::EyebrowSpec;
use poodle_specs::{AspectRatio, MediaKind, MediaPreviewSpec, MediaState};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Image preview"),
                    theme,
                ))
                .child(MediaPreview::from_spec(
                    MediaPreviewSpec::new(MediaKind::Image, "Hero banner")
                        .with_description("Main landing page banner image for the product launch.")
                        .with_aspect_ratio(AspectRatio::Landscape)
                        .with_metadata(vec!["1920 x 1080".into(), "245 KB".into(), "PNG".into()]),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Video preview"),
                    theme,
                ))
                .child(MediaPreview::from_spec(
                    MediaPreviewSpec::new(MediaKind::Video, "Onboarding walkthrough")
                        .with_aspect_ratio(AspectRatio::Video)
                        .with_metadata(vec!["3:42".into(), "48 MB".into()]),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Error state"),
                    theme,
                ))
                .child(MediaPreview::from_spec(
                    MediaPreviewSpec::new(MediaKind::Document, "Corrupted file")
                        .with_state(MediaState::Error)
                        .with_state_title("Preview unavailable")
                        .with_state_message("This file cannot be previewed.")
                        .with_aspect_ratio(AspectRatio::Landscape),
                    theme,
                )),
        )
}
