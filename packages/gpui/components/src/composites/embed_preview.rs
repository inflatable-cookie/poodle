//! EmbedPreview — preview of embedded content backed by EmbedPreviewSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::EmbedPreviewSpec;
use poodle_primitives::{IconSize, IconSpec};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct EmbedPreview {
    spec: EmbedPreviewSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for EmbedPreview {
    type Target = EmbedPreviewSpec;
    fn deref(&self) -> &EmbedPreviewSpec { &self.spec }
}

impl EmbedPreview {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: EmbedPreviewSpec::new(), theme: theme.clone() }
    }
    pub fn from_spec(spec: EmbedPreviewSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for EmbedPreview {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let fill = resolve_color(theme, self.spec.fill_token());
        let border = resolve_color(theme, self.spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.surface");
        let title_color = resolve_color(theme, "semantic.color.text.primary");
        let desc_color = resolve_color(theme, "semantic.color.text.secondary");
        let danger_color = resolve_color(theme, "semantic.color.status.danger");
        let success_color = resolve_color(theme, "semantic.color.status.success");
        let subtle_bg = resolve_color(theme, "semantic.color.background.subtle");
        let gap = resolve_px(theme, "semantic.space.inline.sm");
        let label_size = resolve_px(theme, "semantic.typography.label.size");

        // Surface container with border and radius
        let mut el = div()
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .px(px(16.0))
            .py(px(12.0))
            .flex()
            .flex_col()
            .gap(gap)
            .overflow_hidden();

        // Loading state: skeleton loading bar
        if self.spec.is_loading {
            let skeleton_bar = div()
                .w_full()
                .h(px(12.0))
                .rounded(px(4.0))
                .bg(subtle_bg);
            let skeleton_bar_short = div()
                .w(px(160.0))
                .h(px(12.0))
                .rounded(px(4.0))
                .bg(subtle_bg);
            let skeleton_bar_medium = div()
                .w(px(240.0))
                .h(px(12.0))
                .rounded(px(4.0))
                .bg(subtle_bg);

            el = el.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .py(px(4.0))
                    .child(skeleton_bar)
                    .child(skeleton_bar_medium)
                    .child(skeleton_bar_short),
            );
            return el.into_any_element();
        }

        if let Some(ref error) = self.spec.error {
            el = el.child(
                div()
                    .py(px(16.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    .child(
                        Icon::from_spec(IconSpec::new("alert-circle").with_size(IconSize::Lg), theme)
                            .with_color(danger_color),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(desc_color)
                            .child(error.clone()),
                    ),
            );
            return el.into_any_element();
        }

        if self.spec.parsed.is_none() {
            el = el.child(
                div()
                    .py(px(16.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    .child(
                        Icon::from_spec(IconSpec::new("monitor-play").with_size(IconSize::Lg), theme)
                            .with_color(desc_color.opacity(0.7)),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(desc_color)
                            .child(self.spec.empty_message.clone()),
                    ),
            );
            return el.into_any_element();
        }

        let parsed = self.spec.parsed.as_ref().unwrap();
        let embed_url = self.spec.embed_url();
        let provider_label = parsed.provider.clone();

        if let Some(embed_url) = embed_url {
            let preview_label = if parsed.provider == "youtube" || parsed.provider == "vimeo" {
                "Platform preview placeholder"
            } else {
                "External embed placeholder"
            };

            let media_frame = div()
                .w_full()
                .min_h(if self.spec.effective_aspect_ratio().is_some() { px(200.0) } else { px(160.0) })
                .rounded(radius)
                .bg(subtle_bg)
                .border_1()
                .border_color(border)
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(8.0))
                .px(px(16.0))
                .py(px(20.0))
                .child(
                    Icon::from_spec(
                        IconSpec::new("monitor-play").with_size(IconSize::Lg),
                        theme,
                    )
                    .with_color(success_color),
                )
                .child(
                    div()
                        .text_size(label_size)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(title_color)
                        .child(preview_label),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(desc_color)
                        .child(embed_url),
                );

            el = el.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        Icon::from_spec(
                            IconSpec::new("link").with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(success_color),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(success_color)
                            .bg(resolve_color(theme, "semantic.color.background.subtle"))
                            .rounded(px(4.0))
                            .px(px(6.0))
                            .py(px(2.0))
                            .child(provider_label),
                    ),
            ).child(media_frame);
            return el.into_any_element();
        }

        if self.spec.has_raw_embed() {
            el = el.child(
                div()
                    .w_full()
                    .rounded(radius)
                    .bg(subtle_bg)
                    .border_1()
                    .border_color(border)
                    .px(px(16.0))
                    .py(px(12.0))
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_size(label_size)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(title_color)
                            .child("Raw embed code"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(desc_color)
                            .child(parsed.original_embed.clone().unwrap_or_default()),
                    ),
            );
            return el.into_any_element();
        }

        el = el.child(
            div()
                .text_size(px(12.0))
                .text_color(success_color)
                .child(parsed.original_url.clone().unwrap_or_else(|| parsed.id.clone())),
        );

        el.into_any_element()
    }
}
