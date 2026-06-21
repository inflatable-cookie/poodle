//! MediaPreview — real GPUI component backed by MediaPreviewSpec.
//!
//! Composes the `Card` primitive (contract §10) with a nested `MediaThumbnail`
//! in the media slot, a header (eyebrow / title / description + pill metadata),
//! and a body (caption). Size and density resolve from the spec's contract-exact
//! rem tables.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CardSpec, ControlDensity, MediaKind, MediaPreviewSpec, MediaState, MediaThumbnailSpec,
};

use crate::composites::MediaThumbnail;
use crate::presentation::rem_to_px;
use crate::primitives::Card;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI media preview component backed by `MediaPreviewSpec`.
pub struct MediaPreview {
    spec: MediaPreviewSpec,
    theme: GpuiThemeProvider,
    media_content: Option<AnyElement>,
}

impl std::ops::Deref for MediaPreview {
    type Target = MediaPreviewSpec;
    fn deref(&self) -> &MediaPreviewSpec {
        &self.spec
    }
}

impl MediaPreview {
    pub fn new(kind: MediaKind, title: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: MediaPreviewSpec::new(kind, title),
            theme: theme.clone(),
            media_content: None,
        }
    }

    pub fn from_spec(spec: MediaPreviewSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            media_content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn kind(mut self, v: MediaKind) -> Self {
        self.spec.kind = v;
        self
    }
    pub fn state(mut self, v: MediaState) -> Self {
        self.spec.state = v;
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = v.into();
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn eyebrow(mut self, v: impl Into<String>) -> Self {
        self.spec.eyebrow = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.spec.caption = Some(v.into());
        self
    }
    pub fn metadata(mut self, v: Vec<String>) -> Self {
        self.spec.metadata = v;
        self
    }
    pub fn aspect_ratio(mut self, v: poodle_specs::AspectRatio) -> Self {
        self.spec.aspect_ratio = v;
        self
    }
    pub fn badge(mut self, v: impl Into<String>) -> Self {
        self.spec.badge = Some(v.into());
        self
    }
    pub fn thumbnail_meta(mut self, v: impl Into<String>) -> Self {
        self.spec.thumbnail_meta = Some(v.into());
        self
    }
    pub fn state_title(mut self, v: impl Into<String>) -> Self {
        self.spec.state_title = Some(v.into());
        self
    }
    pub fn state_message(mut self, v: impl Into<String>) -> Self {
        self.spec.state_message = Some(v.into());
        self
    }
    pub fn variant(mut self, v: poodle_specs::CardVariant) -> Self {
        self.spec.variant = v;
        self
    }
    pub fn size(mut self, v: poodle_specs::ControlSize) -> Self {
        self.spec.size = Some(v);
        self
    }
    pub fn density(mut self, v: ControlDensity) -> Self {
        self.spec.density = Some(v);
        self
    }

    pub fn with_media_content(mut self, content: impl IntoElement) -> Self {
        self.media_content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for MediaPreview {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_primary = resolve_color(theme, spec.title_color_token());
        let text_secondary = resolve_color(theme, spec.secondary_text_token());
        let meta_fill = resolve_color(theme, spec.meta_fill_token()).opacity(0.70);
        let meta_radius = resolve_radius(theme, spec.meta_radius_token());
        let inline_gap = resolve_px(theme, "space.inline.sm");

        let eyebrow_size = px(rem_to_px(spec.eyebrow_size_rem()));
        let title_size = px(rem_to_px(spec.title_size_rem()));
        let body_size = px(rem_to_px(spec.body_size_rem()));
        let (meta_pad_y, meta_pad_x) = spec.meta_padding_rem();
        let header_gap = px(rem_to_px(spec.header_gap_rem()));
        let section_gap = px(rem_to_px(spec.section_gap_rem()));

        // ── Media slot: nested MediaThumbnail (contract §3/§10) ──
        let mut thumbnail = MediaThumbnail::from_spec(
            MediaThumbnailSpec::new(spec.kind)
                .with_state(spec.state)
                .with_aspect_ratio(spec.aspect_ratio)
                .with_show_caption(false),
            theme,
        )
        .title(spec.title.clone());

        if let Some(ref state_title) = spec.state_title {
            thumbnail = thumbnail.state_title(state_title);
        }
        if let Some(ref state_message) = spec.state_message {
            thumbnail = thumbnail.state_message(state_message);
        }
        if let Some(ref badge) = spec.badge {
            thumbnail = thumbnail.badge_label(badge);
        }
        if let Some(media) = self.media_content {
            thumbnail = thumbnail.with_image(media);
        }

        // ── Header: heading block + pill metadata ──────────────
        let mut heading = div().flex().flex_col().gap(section_gap);

        if let Some(ref eyebrow) = spec.eyebrow {
            heading = heading.child(
                div()
                    .text_size(eyebrow_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_secondary)
                    .child(eyebrow.to_uppercase()),
            );
        }

        heading = heading.child(
            div()
                .text_size(title_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(spec.title.clone()),
        );

        if let Some(ref description) = spec.description {
            heading = heading.child(
                div()
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        let mut header = div().flex().flex_col().gap(header_gap).child(heading);

        if spec.thumbnail_meta.is_some() || !spec.metadata.is_empty() {
            let mut meta_list = div().flex().flex_wrap().gap(inline_gap);

            let items = spec
                .thumbnail_meta
                .iter()
                .cloned()
                .chain(spec.metadata.iter().cloned());

            for item in items {
                meta_list = meta_list.child(
                    div()
                        .px(px(rem_to_px(meta_pad_x)))
                        .py(px(rem_to_px(meta_pad_y)))
                        .rounded(meta_radius)
                        .bg(meta_fill)
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child(item),
                );
            }

            header = header.child(meta_list);
        }

        // ── Body: caption ──────────────────────────────────────
        let body = spec.caption.as_ref().map(|caption| {
            div()
                .flex()
                .flex_col()
                .gap(section_gap)
                .child(
                    div()
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child(caption.clone()),
                )
        });

        // ── Compose Card (contract §10) ────────────────────────
        let card = Card::from_spec(
            CardSpec::new()
                .with_variant(spec.variant)
                .with_density(spec.resolved_density())
                .with_media(true)
                .with_aria_label(spec.title.clone()),
            theme,
        )
        .with_media(thumbnail)
        .with_header(header)
        .when_some(body, |card, body| card.with_body(body));

        card.into_any_element()
    }
}
