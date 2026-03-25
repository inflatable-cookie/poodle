use gpui::*;
use poodle_composites::{MediaPreviewSpec, MediaKind, MediaState};
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{MediaPreview, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Image preview"), theme))
            .child(MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Image, "Hero banner")
                    .with_description("Main landing page banner image for the product launch.")
                    .with_metadata(vec!["1920 x 1080".into(), "245 KB".into(), "PNG".into()]),
                theme,
            )))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Video preview"), theme))
            .child(MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Video, "Onboarding walkthrough")
                    .with_metadata(vec!["3:42".into(), "48 MB".into()]),
                theme,
            )))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Error state"), theme))
            .child(MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Document, "Corrupted file")
                    .with_state(MediaState::Error),
                theme,
            )))
}
