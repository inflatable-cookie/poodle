use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, IntoCompatNode, MediaPreview, Surface};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node};
use poodle_specs::{
    AspectRatio, ControlDensity, ControlSize, EyebrowSpec, MediaKind, MediaPreviewSpec, MediaState,
    SurfaceSpec, SurfaceTone,
};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
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
        // ── Error state ──
        .child(group(
            "Error state",
            theme,
            MediaPreview::from_spec(
                MediaPreviewSpec::new(MediaKind::Document, "Corrupted file")
                    .with_eyebrow("Document")
                    .with_state(MediaState::Error)
                    .with_state_title("Preview unavailable")
                    .with_state_message("This file cannot be previewed.")
                    .with_aspect_ratio(AspectRatio::Landscape),
                theme,
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "media-preview",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                size_preview(theme, size).into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                density_preview(theme, density).into_any_element()
            }),
    )
}

fn size_preview(theme: &GpuiThemeProvider, size: ControlSize) -> MediaPreview {
    let label = size_key(size);
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

fn density_preview(theme: &GpuiThemeProvider, density: ControlDensity) -> MediaPreview {
    let label = density_key(density);
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
