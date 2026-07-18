//! MediaThumbnail specimen — framed media preview with state posture.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::media_thumbnail::js_media_thumbnail;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    AspectRatio, MediaFit, MediaFrameWidth, MediaKind, MediaPresentation, MediaState,
    MediaThumbnailSpec,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // ── Kinds: each resolves its own fallback icon + play overlay ──
        .child(group(
            "Kinds (fallback icons)",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Image)
                        .with_title("Image")
                        .with_aspect_ratio(AspectRatio::Square),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Video)
                        .with_title("Video")
                        .with_aspect_ratio(AspectRatio::Square),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Audio)
                        .with_title("Audio")
                        .with_aspect_ratio(AspectRatio::Square),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Document)
                        .with_title("Document")
                        .with_aspect_ratio(AspectRatio::Square),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Embed)
                        .with_title("Embed")
                        .with_aspect_ratio(AspectRatio::Square),
                )),
        ))
        // ── Badge + duration chip + play indicator ──
        .child(group(
            "Badge, meta, play indicator",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Image)
                        .with_title("Photo 1")
                        .with_badge_label("New")
                        .with_aspect_ratio(AspectRatio::Square),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Image)
                        .with_title("Photo 2")
                        .with_meta("2.4 MB")
                        .with_aspect_ratio(AspectRatio::Square),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Video)
                        .with_title("Clip")
                        .with_badge_label("HD")
                        .with_meta("1:24")
                        .with_aspect_ratio(AspectRatio::Square),
                )),
        ))
        // ── Aspect ratios (size axis) ──
        .child(group(
            "Aspect ratios",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Image)
                        .with_title("Square")
                        .with_aspect_ratio(AspectRatio::Square),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Image)
                        .with_title("Landscape")
                        .with_aspect_ratio(AspectRatio::Landscape),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Image)
                        .with_title("Portrait")
                        .with_aspect_ratio(AspectRatio::Portrait),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Video)
                        .with_title("Video 16:9")
                        .with_aspect_ratio(AspectRatio::Video),
                )),
        ))
        // ── Compact presentation (caption hidden) ──
        .child(group(
            "Compact presentation (caption hidden)",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Document)
                        .with_title("Report.pdf")
                        .with_presentation(MediaPresentation::Compact)
                        .with_aspect_ratio(AspectRatio::Landscape),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Audio)
                        .with_title("Interview.mp3")
                        .with_presentation(MediaPresentation::Compact)
                        .with_aspect_ratio(AspectRatio::Landscape),
                )),
        ))
        // ── State postures: loading / error / empty ──
        .child(group(
            "States (loading, error, empty)",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Image)
                        .with_state(MediaState::Loading)
                        .with_aspect_ratio(AspectRatio::Square)
                        .with_state_message("Preview data is still being prepared."),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Document)
                        .with_state(MediaState::Error)
                        .with_aspect_ratio(AspectRatio::Square)
                        .with_state_message("This file cannot be previewed."),
                ))
                .child(thumb(
                    theme,
                    MediaThumbnailSpec::new(MediaKind::Image)
                        .with_state(MediaState::Empty)
                        .with_aspect_ratio(AspectRatio::Square)
                        .with_state_message("Nothing to show yet."),
                )),
        ))
        // ── Fit: contained image in a constrained frame ──
        .child(group(
            "Fit: contain",
            secondary,
            thumb(
                theme,
                MediaThumbnailSpec::new(MediaKind::Image)
                    .with_title("Question diagram")
                    .with_aspect_ratio(AspectRatio::Landscape)
                    .with_fit(MediaFit::Contain)
                    .with_frame_width(MediaFrameWidth::Xl)
                    .with_frame_min_height(160.0)
                    .with_frame_max_height(224.0),
            ),
        ))
}

fn thumb(theme: &JetstreamThemeProvider, spec: MediaThumbnailSpec) -> JsEl {
    js_media_thumbnail(&spec, theme)
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
