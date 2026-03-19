use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_composites::{
    MediaPreviewSpec, MediaThumbnailSpec, AspectRatio, MediaKind, MediaState,
};
use pug_gpui_components::{PugMediaPreview, PugMediaThumbnail};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // ── MediaPreview ──

        // --- Image preview ---
        .child(section_label("MEDIA PREVIEW: IMAGE PREVIEW", text_secondary))
        .child(
            PugMediaPreview::new(
                MediaPreviewSpec::new(MediaKind::Image, "Hero banner")
                    .with_description("Main landing page banner image for the product launch.")
                    .with_metadata(vec![
                        "1920 x 1080".into(),
                        "245 KB".into(),
                        "PNG".into(),
                    ]),
                theme,
            )
        )
        // --- Video preview ---
        .child(section_label("MEDIA PREVIEW: VIDEO PREVIEW", text_secondary))
        .child(
            PugMediaPreview::new(
                MediaPreviewSpec::new(MediaKind::Video, "Onboarding walkthrough")
                    .with_metadata(vec![
                        "3:42".into(),
                        "48 MB".into(),
                    ]),
                theme,
            )
        )
        // --- Error state ---
        .child(section_label("MEDIA PREVIEW: ERROR STATE", text_secondary))
        .child(
            PugMediaPreview::new(
                MediaPreviewSpec::new(MediaKind::Document, "Corrupted file")
                    .with_state(MediaState::Error),
                theme,
            )
        )

        // ── MediaThumbnail ──

        // --- Image thumbnails ---
        .child(section_label("MEDIA THUMBNAIL: IMAGE THUMBNAILS", text_secondary))
        .child(
            div().flex().gap(px(8.0)).flex_wrap()
                .child(
                    PugMediaThumbnail::new(
                        MediaThumbnailSpec::new(MediaKind::Image)
                            .with_title("Photo 1")
                            .with_badge_label("New")
                            .with_aspect_ratio(AspectRatio::Square),
                        theme,
                    )
                )
                .child(
                    PugMediaThumbnail::new(
                        MediaThumbnailSpec::new(MediaKind::Image)
                            .with_title("Photo 2")
                            .with_meta("2.4 MB")
                            .with_aspect_ratio(AspectRatio::Square),
                        theme,
                    )
                )
                .child(
                    PugMediaThumbnail::new(
                        MediaThumbnailSpec::new(MediaKind::Video)
                            .with_title("Clip")
                            .with_badge_label("HD")
                            .with_meta("1:24")
                            .with_aspect_ratio(AspectRatio::Square),
                        theme,
                    )
                )
        )
        // --- Compact presentation ---
        .child(section_label("MEDIA THUMBNAIL: COMPACT PRESENTATION", text_secondary))
        .child(
            div().flex().gap(px(8.0)).flex_wrap()
                .child(
                    PugMediaThumbnail::new(
                        MediaThumbnailSpec::new(MediaKind::Document)
                            .with_title("Report.pdf")
                            .with_aspect_ratio(AspectRatio::Landscape),
                        theme,
                    )
                )
                .child(
                    PugMediaThumbnail::new(
                        MediaThumbnailSpec::new(MediaKind::Audio)
                            .with_title("Interview.mp3")
                            .with_aspect_ratio(AspectRatio::Landscape),
                        theme,
                    )
                )
        )
        // --- Loading state ---
        .child(section_label("MEDIA THUMBNAIL: LOADING STATE", text_secondary))
        .child(
            PugMediaThumbnail::new(
                MediaThumbnailSpec::new(MediaKind::Image)
                    .with_state(MediaState::Loading)
                    .with_aspect_ratio(AspectRatio::Square),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
