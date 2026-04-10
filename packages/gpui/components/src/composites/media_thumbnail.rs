//! MediaThumbnail — real GPUI component backed by MediaThumbnailSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{MediaThumbnailSpec, AspectRatio, MediaKind, MediaState};
use poodle_primitives::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::primitives::Spinner;
use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI media thumbnail component backed by `MediaThumbnailSpec`.
///
/// Renders a thumbnail frame with aspect ratio constraints, fallback content
/// when media is not ready, optional badge, and caption.
pub struct MediaThumbnail {
    spec: MediaThumbnailSpec,
    theme: GpuiThemeProvider,
    image_content: Option<AnyElement>,
}

impl std::ops::Deref for MediaThumbnail {
    type Target = MediaThumbnailSpec;
    fn deref(&self) -> &MediaThumbnailSpec { &self.spec }
}

impl MediaThumbnail {
    pub fn new(kind: MediaKind, theme: &GpuiThemeProvider) -> Self {
        Self { spec: MediaThumbnailSpec::new(kind), theme: theme.clone(), image_content: None }
    }

    pub fn from_spec(spec: MediaThumbnailSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            image_content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn kind(mut self, v: MediaKind) -> Self { self.spec.kind = v; self }
    pub fn state(mut self, v: MediaState) -> Self { self.spec.state = v; self }
    pub fn aspect_ratio(mut self, v: AspectRatio) -> Self { self.spec.aspect_ratio = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn meta(mut self, v: impl Into<String>) -> Self { self.spec.meta = Some(v.into()); self }
    pub fn badge_label(mut self, v: impl Into<String>) -> Self { self.spec.badge_label = Some(v.into()); self }
    pub fn state_title(mut self, v: impl Into<String>) -> Self { self.spec.state_title = Some(v.into()); self }
    pub fn state_message(mut self, v: impl Into<String>) -> Self { self.spec.state_message = Some(v.into()); self }
    pub fn show_caption(mut self, v: bool) -> Self { self.spec.show_caption = v; self }


    pub fn with_image(mut self, image: impl IntoElement) -> Self {
        self.image_content = Some(image.into_any_element());
        self
    }
}

impl IntoElement for MediaThumbnail {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let frame_bg = resolve_color(theme, spec.frame_fill_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let border = resolve_color(theme, "color.border.subtle");
        let accent = resolve_color(theme, "color.accent.base");
        let body_size = resolve_px(theme, "typography.body.size");

        // Aspect ratio sizing
        let (frame_w, frame_h) = match spec.aspect_ratio {
            AspectRatio::Square => (px(160.0), px(160.0)),
            AspectRatio::Landscape => (px(220.0), px(148.0)),
            AspectRatio::Portrait => (px(148.0), px(220.0)),
            AspectRatio::Video => (px(280.0), px(157.0)),
        };

        let mut container = div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .overflow_hidden();

        // Thumbnail frame
        let mut frame = div()
            .w(frame_w)
            .h(frame_h)
            .rounded(px(4.0))
            .bg(frame_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .relative();

        if spec.shows_fallback_copy() {
            // Fallback state
            let kind_label = match spec.kind {
                MediaKind::Image => "Image",
                MediaKind::Audio => "Audio",
                MediaKind::Video => "Video",
                MediaKind::Document => "Document",
                MediaKind::Embed => "Embed",
            };
            frame = frame.child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(4.0))
                    .when(spec.state == MediaState::Loading, |el| {
                        el.child(
                            Spinner::from_spec(
                                SpinnerSpec::new()
                                    .with_variant(SpinnerVariant::Grid)
                                    .with_size(if spec.show_caption { SpinnerSize::Md } else { SpinnerSize::Sm })
                                    .with_tone(SpinnerTone::Accent),
                                theme,
                            ),
                        )
                    })
                    .child(
                        div()
                            .text_size(body_size)
                            .text_color(text_primary)
                            .child(spec.resolved_state_title().to_string()),
                    )
                    .when(spec.resolved_state_message().is_some(), |el| {
                        el.child(
                            div()
                                .text_size(px(12.0))
                                .text_color(text_secondary.opacity(0.85))
                                .child(spec.resolved_state_message().unwrap().to_string()),
                        )
                    })
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(text_secondary.opacity(0.7))
                            .child(format!("[{}]", kind_label)),
                    ),
            );
        } else if let Some(image) = self.image_content {
            frame = frame.child(image);
        }

        // Badge overlay
        if let Some(ref badge_label) = spec.badge_label {
            frame = frame.child(
                div()
                    .absolute()
                    .top(px(6.0))
                    .right(px(6.0))
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .bg(accent)
                    .text_size(px(12.0))
                    .text_color(gpui::white())
                    .child(badge_label.clone()),
            );
        }

        container = container.child(frame);

        // Caption
        if spec.caption_visible() {
            let mut caption = div().w(frame_w).flex().flex_col().gap(px(2.0));

            if let Some(ref title) = spec.title {
                caption = caption.child(
                    div()
                        .text_size(body_size)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_primary)
                        .overflow_x_hidden()
                        .child(title.clone()),
                );
            }

            if let Some(ref meta) = spec.meta {
                caption = caption.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_secondary)
                        .child(meta.clone()),
                );
            }

            container = container.child(caption);
        }

        container.into_any_element()
    }
}
