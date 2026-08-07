use crate::node_compat::{Eyebrow, MediaThumbnail};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::EyebrowSpec;
use poodle_specs::{
    AspectRatio, MediaFit, MediaFrameWidth, MediaKind, MediaPresentation, MediaState,
    MediaThumbnailSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // ── Kinds: each kind resolves its own fallback icon + play overlay ──
        .child(group(
            "Kinds (fallback icons)",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
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
        // ── Badge + duration chip + play indicator (audio/video only) ──
        .child(group(
            "Badge, meta, play indicator",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
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
        // ── Aspect ratios (size axis: intrinsic frame dimensions) ──
        .child(group(
            "Aspect ratios",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
                .items_start()
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
        // ── Compact presentation hides the caption ──
        .child(group(
            "Compact presentation (caption hidden)",
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
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
            theme,
            div()
                .flex()
                .gap(px(8.0))
                .flex_wrap()
                .items_start()
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
        // ── Fit: contained image in an auto-height frame ──
        .child(group(
            "Fit: contain",
            theme,
            div().w(px(320.0)).child(thumb(
                theme,
                MediaThumbnailSpec::new(MediaKind::Image)
                    .with_title("Question diagram")
                    .with_aspect_ratio(AspectRatio::Landscape)
                    .with_fit(MediaFit::Contain)
                    .with_frame_width(MediaFrameWidth::Xl)
                    .with_frame_min_height(160.0)
                    .with_frame_max_height(224.0),
            )),
        ))
}

fn thumb(theme: &GpuiThemeProvider, spec: MediaThumbnailSpec) -> MediaThumbnail {
    MediaThumbnail::from_spec(spec, theme)
}

fn group(label: &str, theme: &GpuiThemeProvider, content: Div) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(content)
}
