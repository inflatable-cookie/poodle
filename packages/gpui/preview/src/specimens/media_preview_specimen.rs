use crate::node_compat::{Eyebrow, IntoCompatNode, MediaPreview, Surface};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node};
use poodle_specs::{
    AspectRatio, CardVariant, ControlDensity, ControlSize, EyebrowSpec, MediaKind,
    MediaPreviewSpec, MediaState, SurfaceSpec, SurfaceTone,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // ── Image preview: eyebrow + description + meta chips + slot content ──
        .child(group(
            "Image preview",
            theme,
            MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Image, "Hero banner")
                    .with_eyebrow("Image")
                    .with_description("Main landing page banner image for the product launch.")
                    .with_aspect_ratio(AspectRatio::Landscape)
                    .with_metadata(vec!["1920 x 1080".into(), "245 KB".into(), "PNG".into()]),
                theme,
            )
            .with_media_content(media_slot(theme, "Image placeholder")),
        ))
        // ── Video preview: video ratio + duration/size meta + badge ──
        .child(group(
            "Video preview",
            theme,
            MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Video, "Onboarding walkthrough")
                    .with_eyebrow("Video")
                    .with_badge("HD")
                    .with_aspect_ratio(AspectRatio::Video)
                    .with_metadata(vec!["3:42".into(), "48 MB".into()]),
                theme,
            )
            .with_media_content(media_slot(theme, "Video placeholder")),
        ))
        // ── File chrome: document kind, caption body, thumbnail meta chip ──
        .child(group(
            "Document preview (caption + thumbnail meta)",
            theme,
            MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Document, "Q3 report.pdf")
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
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(MediaPreview::from_spec(
                    MediaPreviewSpec::new(MediaKind::Image, "Rendering preview")
                        .with_eyebrow("Image")
                        .with_state(MediaState::Loading)
                        .with_state_message("Preview is being generated.")
                        .with_aspect_ratio(AspectRatio::Landscape),
                    theme,
                ))
                .child(MediaPreview::from_spec(
                    MediaPreviewSpec::new(MediaKind::Document, "Corrupted file")
                        .with_eyebrow("Document")
                        .with_state(MediaState::Error)
                        .with_state_title("Preview unavailable")
                        .with_state_message("This file cannot be previewed.")
                        .with_aspect_ratio(AspectRatio::Landscape),
                    theme,
                ))
                .child(MediaPreview::from_spec(
                    MediaPreviewSpec::new(MediaKind::Image, "Empty slot")
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
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    MediaPreview::from_spec(
                        MediaPreviewSpec::new(MediaKind::Image, "Default card")
                            .with_eyebrow("Image")
                            .with_variant(CardVariant::Default)
                            .with_aspect_ratio(AspectRatio::Landscape),
                        theme,
                    )
                    .with_media_content(media_slot(theme, "Image placeholder")),
                )
                .child(
                    MediaPreview::from_spec(
                        MediaPreviewSpec::new(MediaKind::Image, "Elevated card")
                            .with_eyebrow("Image")
                            .with_variant(CardVariant::Elevated)
                            .with_aspect_ratio(AspectRatio::Landscape),
                        theme,
                    )
                    .with_media_content(media_slot(theme, "Image placeholder")),
                )
                .child(
                    MediaPreview::from_spec(
                        MediaPreviewSpec::new(MediaKind::Image, "Outlined card")
                            .with_eyebrow("Image")
                            .with_variant(CardVariant::Outlined)
                            .with_aspect_ratio(AspectRatio::Landscape),
                        theme,
                    )
                    .with_media_content(media_slot(theme, "Image placeholder")),
                ),
        ))
        // ── Size axis (eyebrow/title/body/meta scale) ──
        .child(group(
            "Sizes",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(size_preview(theme, ControlSize::Xs, "xs"))
                .child(size_preview(theme, ControlSize::Sm, "sm"))
                .child(size_preview(theme, ControlSize::Md, "md"))
                .child(size_preview(theme, ControlSize::Lg, "lg"))
                .child(size_preview(theme, ControlSize::Xl, "xl")),
        ))
        // ── Density axis (header/section gaps) ──
        .child(group(
            "Densities",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(density_preview(theme, ControlDensity::Compact, "compact"))
                .child(density_preview(theme, ControlDensity::Default, "default"))
                .child(density_preview(
                    theme,
                    ControlDensity::Comfortable,
                    "comfortable",
                )),
        ))
}

fn size_preview(theme: &GpuiThemeProvider, size: ControlSize, label: &str) -> MediaPreview {
    MediaPreview::from_spec(
        MediaPreviewSpec::new(MediaKind::Image, format!("Hero banner ({label})"))
            .with_eyebrow("Image")
            .with_description("Main landing page banner image for the product launch.")
            .with_aspect_ratio(AspectRatio::Landscape)
            .with_metadata(vec!["1920 x 1080".into(), "245 KB".into(), "PNG".into()])
            .with_size(size),
        theme,
    )
    .with_media_content(media_slot(theme, "Image placeholder"))
}

fn density_preview(
    theme: &GpuiThemeProvider,
    density: ControlDensity,
    label: &str,
) -> MediaPreview {
    MediaPreview::from_spec(
        MediaPreviewSpec::new(MediaKind::Image, format!("Hero banner ({label})"))
            .with_eyebrow("Image")
            .with_description("Main landing page banner image for the product launch.")
            .with_aspect_ratio(AspectRatio::Landscape)
            .with_metadata(vec!["1920 x 1080".into(), "245 KB".into(), "PNG".into()])
            .with_density(density),
        theme,
    )
    .with_media_content(media_slot(theme, "Image placeholder"))
}

/// Real Surface-based media slot content (token-resolved tone + radius).
fn media_slot(theme: &GpuiThemeProvider, text: &str) -> Node {
    let mut content = Node::container();
    content.style.descriptor.layout.direction = LayoutDirection::Row;
    content.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    content.style.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    content.style.descriptor.layout.height = LayoutSizing::Constrained {
        min: Some(140.0),
        max: None,
    };
    content = content.child(Node::text(text));
    Surface::from_spec(SurfaceSpec::new().with_tone(SurfaceTone::Elevated), theme)
        .with_content(content)
        .into_compat_node()
}

fn group(label: &str, theme: &GpuiThemeProvider, content: impl IntoElement) -> Div {
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
