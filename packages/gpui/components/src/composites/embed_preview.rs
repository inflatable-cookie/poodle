//! EmbedPreview — preview of embedded content backed by EmbedPreviewSpec.

use crate::presentation::rem_to_px;
use crate::primitives::{Icon, Skeleton, TextLink};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{IconSpec, SkeletonSpec, TextLinkSpec};
use poodle_specs::EmbedPreviewSpec;

pub struct EmbedPreview {
    spec: EmbedPreviewSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for EmbedPreview {
    type Target = EmbedPreviewSpec;
    fn deref(&self) -> &EmbedPreviewSpec {
        &self.spec
    }
}

impl EmbedPreview {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: EmbedPreviewSpec::new(),
            theme: theme.clone(),
        }
    }
    pub fn from_spec(spec: EmbedPreviewSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for EmbedPreview {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Contract §8 tokens.
        let panel_bg = resolve_color(theme, spec.fill_token()); // background-panel
        let radius = resolve_radius(theme, "radius.surface");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_tertiary = resolve_color(theme, "color.text.tertiary");
        let danger_color = resolve_color(theme, "color.status.danger");

        // Contract §7 sizing — loading/error/empty: gap 0.5rem, min-h 8rem,
        // padding 1.5rem. Text 0.8125rem. Icon 2rem. Fallback padding 0.75rem 1rem.
        let state_gap = resolve_px(theme, "space.inline.sm"); // 0.5rem
        let text_size = resolve_px(theme, "typography.label.size"); // 0.8125rem
        // No named token maps min-h 8rem / padding 1.5rem / icon 2rem / fallback
        // padding-y 0.75rem(panel.y) padding-x 1rem(panel.x) — exact rem from contract.
        let state_min_h = px(rem_to_px(8.0));
        let state_pad = px(rem_to_px(1.5));
        let icon_2rem = rem_to_px(2.0);
        let fallback_pad_y = resolve_px(theme, "space.panel.y"); // 0.75rem
        let fallback_pad_x = resolve_px(theme, "space.panel.x"); // 1rem

        // Root: radius-surface + overflow hidden (contract §8). No border/padding —
        // each state child carries the panel background per the Svelte anatomy.
        let root = || div().rounded(radius).overflow_hidden();

        // Centered state column (loading / error / empty share this layout).
        let state_column = || {
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(state_gap)
                .min_h(state_min_h)
                .p(state_pad)
                .bg(panel_bg)
                .rounded(radius)
        };

        // ── Render priority: loading > error > empty > iframe > raw > trusted > fallback ──

        // Loading: Skeleton primitive (shape="block") + LoadingText.
        if spec.is_loading {
            return root()
                .child(
                    state_column()
                        .child(Skeleton::from_spec(
                            SkeletonSpec::new().with_shape("block"),
                            theme,
                        ))
                        .child(
                            div()
                                .text_size(text_size)
                                .text_color(text_secondary)
                                .child("Loading preview..."),
                        ),
                )
                .into_any_element();
        }

        // Error: alert-circle icon (text-danger) + error text.
        if let Some(error) = spec.error.clone() {
            return root()
                .child(
                    state_column()
                        .child(
                            Icon::from_spec(IconSpec::new("alert-circle"), theme)
                                .with_px_size(icon_2rem)
                                .with_color(danger_color),
                        )
                        .child(
                            div()
                                .text_size(text_size)
                                .text_color(text_secondary)
                                .child(error),
                        ),
                )
                .into_any_element();
        }

        // Empty: play-rectangle icon (text-tertiary) + empty message.
        if spec.is_empty_state() {
            return root()
                .child(
                    state_column()
                        .child(
                            Icon::from_spec(IconSpec::new("monitor-play"), theme)
                                .with_px_size(icon_2rem)
                                .with_color(text_tertiary),
                        )
                        .child(
                            div()
                                .text_size(text_size)
                                .text_color(text_secondary)
                                .child(spec.empty_message.clone()),
                        ),
                )
                .into_any_element();
        }

        // Aspect-ratio container for media states. GPUI renders a contract-sanctioned
        // placeholder panel (GPUI Notes §iframe) rather than a live web view; the
        // panel honors the effective aspect ratio via a nominal reference width.
        let container = |child: AnyElement| {
            let mut frame = div().w_full().bg(panel_bg).rounded(radius);
            if let Some(ratio) = spec.effective_aspect_ratio() {
                // Aspect-ratio layout: derive min-height from the ratio against a
                // nominal content width (no live width at build time). For "auto"
                // (audio / ratio None) the panel falls back to the contract's
                // static 10rem media height.
                let ref_width = rem_to_px(28.0);
                frame = frame.min_h(px((ref_width / ratio).max(rem_to_px(8.0))));
            } else {
                frame = frame.h(px(rem_to_px(10.0)));
            }
            frame.child(child)
        };

        // Iframe / embed-URL state → placeholder panel with the derived embed URL.
        if let Some(embed_url) = spec.embed_url() {
            let provider = spec
                .parsed
                .as_ref()
                .map(|p| p.provider.clone())
                .unwrap_or_default();
            return root()
                .child(container(
                    div()
                        .size_full()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(state_gap)
                        .p(state_pad)
                        .child(
                            Icon::from_spec(IconSpec::new("monitor-play"), theme)
                                .with_px_size(icon_2rem)
                                .with_color(text_tertiary),
                        )
                        .child(
                            div()
                                .text_size(text_size)
                                .text_color(text_secondary)
                                .child(format!("{provider} embed")),
                        )
                        .child(
                            div()
                                .text_size(text_size)
                                .text_color(text_tertiary)
                                .child(embed_url),
                        )
                        .into_any_element(),
                ))
                .into_any_element();
        }

        // Raw embed (parsed.originalEmbed, no embed URL) → raw HTML in the container.
        if spec.has_raw_embed() {
            let raw = spec
                .parsed
                .as_ref()
                .and_then(|p| p.original_embed.clone())
                .unwrap_or_default();
            return root()
                .child(container(
                    div()
                        .size_full()
                        .p(state_pad)
                        .text_size(text_size)
                        .text_color(text_secondary)
                        .child(raw)
                        .into_any_element(),
                ))
                .into_any_element();
        }

        // Trusted HTML (no parsed iframe/raw output) → caller-sanitized HTML.
        if spec.has_trusted_html() {
            let html = spec.trusted_html.clone().unwrap_or_default();
            return root()
                .child(container(
                    div()
                        .size_full()
                        .p(state_pad)
                        .text_size(text_size)
                        .text_color(text_secondary)
                        .child(html)
                        .into_any_element(),
                ))
                .into_any_element();
        }

        // Fallback: TextLink to the original URL.
        let href = spec
            .parsed
            .as_ref()
            .and_then(|p| p.original_url.clone().or_else(|| Some(p.id.clone())))
            .unwrap_or_default();
        root()
            .child(
                div()
                    .bg(panel_bg)
                    .rounded(radius)
                    .py(fallback_pad_y)
                    .px(fallback_pad_x)
                    .text_size(text_size)
                    .child(TextLink::from_spec(
                        TextLinkSpec::new(href.clone()).with_href(href),
                        theme,
                    )),
            )
            .into_any_element()
    }
}
