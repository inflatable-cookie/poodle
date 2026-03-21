//! MediaPreview — real GPUI component backed by MediaPreviewSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::{MediaKind, MediaPreviewSpec, MediaState, RemediationAction};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI media preview component backed by `MediaPreviewSpec`.
///
/// Renders a full media preview with a viewport area, title, description,
/// metadata row, and optional footer actions.
pub struct MediaPreview {
    spec: MediaPreviewSpec,
    theme: GpuiThemeProvider,
    media_content: Option<AnyElement>,
    on_action: Option<Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for MediaPreview {
    type Target = MediaPreviewSpec;
    fn deref(&self) -> &MediaPreviewSpec { &self.spec }
}

impl MediaPreview {
    pub fn new(kind: MediaKind, title: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self { spec: MediaPreviewSpec::new(kind, title), theme: theme.clone(), media_content: None, on_action: None }
    }

    pub fn from_spec(spec: MediaPreviewSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            media_content: None,
            on_action: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn kind(mut self, v: MediaKind) -> Self { self.spec.kind = v; self }
    pub fn state(mut self, v: MediaState) -> Self { self.spec.state = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn metadata(mut self, v: Vec<String>) -> Self { self.spec.metadata = v; self }
    pub fn footer_actions(mut self, v: Vec<RemediationAction>) -> Self { self.spec.footer_actions = v; self }


    pub fn with_media_content(mut self, content: impl IntoElement) -> Self {
        self.media_content = Some(content.into_any_element());
        self
    }

    pub fn on_action(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_action = Some(Box::new(handler));
        self
    }
}

impl IntoElement for MediaPreview {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let frame_bg = resolve_color(theme, spec.frame_fill_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let border = resolve_color(theme, "semantic.color.border.subtle");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let mut container = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .border_1()
            .border_color(border)
            .rounded(control_radius)
            .overflow_hidden();

        // Media viewport
        let mut viewport = div()
            .w_full()
            .min_h(px(200.0))
            .flex()
            .items_center()
            .justify_center()
            .bg(frame_bg);

        if spec.shows_fallback_copy() {
            let fallback_msg = match spec.state {
                MediaState::Loading => "Loading media\u{2026}",
                MediaState::Error => "Failed to load media.",
                MediaState::Empty => "No media available.",
                MediaState::Ready => "",
            };
            let kind_label = match spec.kind {
                MediaKind::Image => "Image",
                MediaKind::Audio => "Audio",
                MediaKind::Video => "Video",
                MediaKind::Document => "Document",
                MediaKind::Embed => "Embed",
            };
            viewport = viewport.child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary)
                            .child(format!("[{}]", kind_label)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_secondary)
                            .child(String::from(fallback_msg)),
                    ),
            );
        } else if let Some(media) = self.media_content {
            viewport = viewport.child(media);
        }

        container = container.child(viewport);

        // Info section
        let mut info = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .px(inline_padding)
            .py(px(10.0));

        // Title
        info = info.child(
            div()
                .text_base()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(spec.title.clone()),
        );

        // Description
        if let Some(ref description) = spec.description {
            info = info.child(
                div()
                    .text_sm()
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        // Metadata row
        if !spec.metadata.is_empty() {
            let mut meta_row = div()
                .flex()
                .items_center()
                .gap(inline_gap);

            for (i, meta) in spec.metadata.iter().enumerate() {
                if i > 0 {
                    meta_row = meta_row.child(
                        div()
                            .text_xs()
                            .text_color(text_secondary.opacity(0.5))
                            .child("\u{00B7}"),
                    );
                }
                meta_row = meta_row.child(
                    div()
                        .text_xs()
                        .text_color(text_secondary)
                        .child(meta.clone()),
                );
            }

            info = info.child(meta_row);
        }

        container = container.child(info);

        // Footer actions
        if spec.has_footer_actions() {
            let mut footer = div()
                .w_full()
                .flex()
                .items_center()
                .gap(inline_gap)
                .px(inline_padding)
                .py(px(8.0))
                .border_t_1()
                .border_color(border);

            for action in &spec.footer_actions {
                let action_id = SharedString::from(format!("media-preview-{}", action.id));
                let btn = div()
                    .id(action_id)
                    .cursor_pointer()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(accent)
                    .when(action.is_disabled, |el| el.opacity(0.5))
                    .child(action.label.clone());

                footer = footer.child(btn);
            }

            container = container.child(footer);
        }

        container.into_any_element()
    }
}
