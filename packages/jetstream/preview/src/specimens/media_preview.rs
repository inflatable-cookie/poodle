//! MediaPreview specimen — Card-composed media preview surface.

use crate::compat::js_media_preview;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    AspectRatio, CardVariant, ControlDensity, ControlSize, MediaKind, MediaPreviewSpec, MediaState,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // ── Image preview: eyebrow + description + meta chips ──
        .child(group(
            "Image preview",
            secondary,
            js_media_preview(
                &MediaPreviewSpec::new(MediaKind::Image, "Hero banner")
                    .with_eyebrow("Image")
                    .with_description("Main landing page banner image for the product launch.")
                    .with_aspect_ratio(AspectRatio::Landscape)
                    .with_metadata(vec!["1920 x 1080".into(), "245 KB".into(), "PNG".into()]),
                theme,
            ),
        ))
        // ── Video preview: video ratio + duration/size meta + badge ──
        .child(group(
            "Video preview",
            secondary,
            js_media_preview(
                &MediaPreviewSpec::new(MediaKind::Video, "Onboarding walkthrough")
                    .with_eyebrow("Video")
                    .with_badge("HD")
                    .with_aspect_ratio(AspectRatio::Video)
                    .with_metadata(vec!["3:42".into(), "48 MB".into()]),
                theme,
            ),
        ))
        // ── Document chrome: caption body + thumbnail meta chip ──
        .child(group(
            "Document preview (caption + thumbnail meta)",
            secondary,
            js_media_preview(
                &MediaPreviewSpec::new(MediaKind::Document, "Q3 report.pdf")
                    .with_eyebrow("Document")
                    .with_caption("Final draft shared with the leadership team.")
                    .with_thumbnail_meta("12 pages")
                    .with_aspect_ratio(AspectRatio::Landscape)
                    .with_metadata(vec!["1.1 MB".into(), "PDF".into()]),
                theme,
            ),
        ))
        // ── State postures: loading / error / empty ──
        .child(group(
            "States (loading, error, empty)",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(js_media_preview(
                    &MediaPreviewSpec::new(MediaKind::Image, "Rendering preview")
                        .with_eyebrow("Image")
                        .with_state(MediaState::Loading)
                        .with_state_message("Preview is being generated.")
                        .with_aspect_ratio(AspectRatio::Landscape),
                    theme,
                ))
                .child(js_media_preview(
                    &MediaPreviewSpec::new(MediaKind::Document, "Corrupted file")
                        .with_eyebrow("Document")
                        .with_state(MediaState::Error)
                        .with_state_title("Preview unavailable")
                        .with_state_message("This file cannot be previewed.")
                        .with_aspect_ratio(AspectRatio::Landscape),
                    theme,
                ))
                .child(js_media_preview(
                    &MediaPreviewSpec::new(MediaKind::Image, "Empty slot")
                        .with_eyebrow("Image")
                        .with_state(MediaState::Empty)
                        .with_state_message("No preview available yet.")
                        .with_aspect_ratio(AspectRatio::Landscape),
                    theme,
                )),
        ))
        // ── Card variants ──
        .child(group(
            "Variants",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(js_media_preview(
                    &MediaPreviewSpec::new(MediaKind::Image, "Default card")
                        .with_eyebrow("Image")
                        .with_variant(CardVariant::Default)
                        .with_aspect_ratio(AspectRatio::Landscape),
                    theme,
                ))
                .child(js_media_preview(
                    &MediaPreviewSpec::new(MediaKind::Image, "Elevated card")
                        .with_eyebrow("Image")
                        .with_variant(CardVariant::Elevated)
                        .with_aspect_ratio(AspectRatio::Landscape),
                    theme,
                ))
                .child(js_media_preview(
                    &MediaPreviewSpec::new(MediaKind::Image, "Outlined card")
                        .with_eyebrow("Image")
                        .with_variant(CardVariant::Outlined)
                        .with_aspect_ratio(AspectRatio::Landscape),
                    theme,
                )),
        ))
        // ── Size axis ──
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(size_preview(theme, ControlSize::Xs, "xs"))
                .child(size_preview(theme, ControlSize::Sm, "sm"))
                .child(size_preview(theme, ControlSize::Md, "md"))
                .child(size_preview(theme, ControlSize::Lg, "lg"))
                .child(size_preview(theme, ControlSize::Xl, "xl")),
        ))
        // ── Density axis ──
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(density_preview(theme, ControlDensity::Compact, "compact"))
                .child(density_preview(theme, ControlDensity::Default, "default"))
                .child(density_preview(
                    theme,
                    ControlDensity::Comfortable,
                    "comfortable",
                )),
        ))
}

fn size_preview(theme: &JetstreamThemeProvider, size: ControlSize, label: &str) -> El {
    js_media_preview(
        &MediaPreviewSpec::new(MediaKind::Image, format!("Hero banner ({label})"))
            .with_eyebrow("Image")
            .with_description("Main landing page banner image for the product launch.")
            .with_aspect_ratio(AspectRatio::Landscape)
            .with_metadata(vec!["1920 x 1080".into(), "245 KB".into(), "PNG".into()])
            .with_size(size),
        theme,
    )
}

fn density_preview(theme: &JetstreamThemeProvider, density: ControlDensity, label: &str) -> El {
    js_media_preview(
        &MediaPreviewSpec::new(MediaKind::Image, format!("Hero banner ({label})"))
            .with_eyebrow("Image")
            .with_description("Main landing page banner image for the product launch.")
            .with_aspect_ratio(AspectRatio::Landscape)
            .with_metadata(vec!["1920 x 1080".into(), "245 KB".into(), "PNG".into()])
            .with_density(density),
        theme,
    )
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
