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

        // Determine if we have any content
        let has_content = self.spec.title.is_some()
            || self.spec.description.is_some()
            || self.spec.provider.is_some()
            || self.spec.thumbnail_url.is_some();

        // Empty state: placeholder text
        if !has_content {
            el = el.child(
                div()
                    .py(px(16.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(desc_color)
                            .child("No embed to preview"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(desc_color)
                            .child("Paste a URL above to see a preview"),
                    ),
            );
            return el.into_any_element();
        }

        // Success / loaded state

        // Iframe placeholder area when a thumbnail URL is available
        if self.spec.thumbnail_url.is_some() {
            // Play icon overlay for video previews
            let play_overlay = div()
                .flex().items_center().justify_center()
                .w(px(48.0)).h(px(48.0))
                .rounded(px(24.0))
                .bg(hsla(0.0, 0.0, 0.0, 0.6))
                .child(
                    Icon::from_spec(
                        IconSpec::new("play").with_size(IconSize::Md),
                        theme,
                    ).with_color(gpui::white())
                );

            el = el.child(
                div()
                    .w_full()
                    // 16:9 aspect ratio approximation (56.25% of width)
                    .min_h(px(200.0))
                    .rounded(radius)
                    .bg(subtle_bg)
                    .border_1()
                    .border_color(border)
                    .flex()
                    .items_center()
                    .justify_center()
                    .overflow_hidden()
                    .child(play_overlay),
            );
        }

        // Provider badge
        if let Some(ref provider) = self.spec.provider {
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
                            .child(provider.clone()),
                    ),
            );
        }

        // Title
        if let Some(ref title) = self.spec.title {
            el = el.child(
                div()
                    .text_size(px(14.0))
                    .text_color(title_color)
                    .font_weight(FontWeight::MEDIUM)
                    .child(title.clone()),
            );
        }

        // Description
        if let Some(ref desc) = self.spec.description {
            el = el.child(
                div()
                    .text_size(px(12.0))
                    .line_height(relative(1.4))
                    .text_color(desc_color)
                    .child(desc.clone()),
            );
        }

        el.into_any_element()
    }
}
