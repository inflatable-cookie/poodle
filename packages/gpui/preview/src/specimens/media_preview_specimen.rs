use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, MediaPreview};
use poodle_specs::EyebrowSpec;
use poodle_specs::{
    AspectRatio, ButtonVariant, MediaKind, MediaPreviewSpec, MediaState, RemediationAction,
};

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
                        .with_badge("Approved")
                        .with_thumbnail_meta("Featured")
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
                        .with_description("Product onboarding video used in account setup flows.")
                        .with_badge("Ready")
                        .with_thumbnail_meta("3:42")
                        .with_aspect_ratio(AspectRatio::Video)
                        .with_metadata(vec!["3:42".into(), "48 MB".into(), "1080p".into()])
                        .with_footer_actions(vec![
                            RemediationAction::new("replace", "Replace"),
                            RemediationAction::new("download", "Download")
                                .with_variant(ButtonVariant::Ghost),
                        ]),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading state"),
                    theme,
                ))
                .child(MediaPreview::from_spec(
                    MediaPreviewSpec::new(MediaKind::Audio, "Narration mix")
                        .with_state(MediaState::Loading)
                        .with_state_title("Generating preview")
                        .with_state_message("Waveform and metadata are still loading.")
                        .with_aspect_ratio(AspectRatio::Landscape),
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
                        .with_aspect_ratio(AspectRatio::Landscape)
                        .with_footer_actions(vec![
                            RemediationAction::new("retry", "Retry"),
                            RemediationAction::new("replace", "Replace")
                                .with_variant(ButtonVariant::Ghost),
                        ]),
                    theme,
                )),
        )
}
