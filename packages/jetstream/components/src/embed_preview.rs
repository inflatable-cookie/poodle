//! EmbedPreview — Jetstream embed display backed by EmbedPreviewSpec.
//!
//! Contract: `docs/contracts/components/embed-preview.md`
//! Reference: Svelte `EmbedPreview.svelte` (parity authority); GPUI `composites/embed_preview.rs`.
//!
//! Composes the real `js_skeleton` (block) loading primitive and `js_text_link`
//! fallback. Live iframes are a runtime gap; the parsed-media state renders a
//! contract-sanctioned placeholder panel (GPUI Notes §iframe) honoring the
//! effective aspect ratio. ZERO hardcoded hsla; dimensions resolve from named
//! tokens where the contract rem aligns (exact rem noted where no token maps).
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{EmbedPreviewSpec, SkeletonSpec, TextLinkSpec};

use crate::presentation::rem_to_px;
use crate::skeleton::js_skeleton;
use crate::text_link::js_text_link;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_embed_preview(spec: &EmbedPreviewSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let panel_bg = resolve_color(theme, spec.fill_token()); // background-panel
    let radius = resolve_radius(theme, "radius.surface");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_tertiary = resolve_color(theme, "color.text.tertiary");
    let danger_color = resolve_color(theme, "color.status.danger");

    // Contract §7 sizing. gap 0.5rem → space.inline.sm; text 0.8125rem →
    // typography.label.size; fallback padding 0.75rem/1rem → space.panel.y / .x.
    // min-h 8rem, padding 1.5rem, icon 2rem have no exact named token — exact rem.
    let state_gap = resolve_px(theme, "space.inline.sm");
    let text_size = resolve_px(theme, "typography.label.size");
    let state_min_h = rem_to_px(8.0);
    let state_pad = rem_to_px(1.5);
    let icon_2rem = rem_to_px(2.0);
    let fallback_pad_y = resolve_px(theme, "space.panel.y");
    let fallback_pad_x = resolve_px(theme, "space.panel.x");

    // Root: radius-surface + overflow hidden; state children carry the panel bg.
    let root = || ui_element::div().rounded(radius).overflow_hidden();

    // Centered state column shared by loading / error / empty.
    let state_column = || {
        ui_element::div()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(state_gap)
            .min_h(state_min_h)
            .pl(state_pad)
            .pr(state_pad)
            .pt(state_pad)
            .pb(state_pad)
            .bg(panel_bg)
            .rounded(radius)
    };

    // ── Render priority: loading > error > empty > iframe > raw > trusted > fallback ──

    // Loading: real Skeleton primitive (block) + loading text.
    if spec.is_loading {
        return root().child(
            state_column()
                .child(js_skeleton(&SkeletonSpec::new().with_shape("block"), theme))
                .child(
                    ui_element::label("Loading preview...")
                        .text_color(text_secondary)
                        .text_size(text_size),
                ),
        );
    }

    // Error: alert-circle icon (text-danger) + error text.
    if let Some(ref error) = spec.error {
        return root().child(
            state_column()
                .child(
                    ui_element::icon("alert-circle")
                        .w(icon_2rem)
                        .h(icon_2rem)
                        .text_color(danger_color),
                )
                .child(
                    ui_element::label(error)
                        .text_color(text_secondary)
                        .text_size(text_size),
                ),
        );
    }

    // Empty: play-rectangle icon (text-tertiary) + empty message.
    if spec.is_empty_state() {
        return root().child(
            state_column()
                .child(
                    ui_element::icon("monitor-play")
                        .w(icon_2rem)
                        .h(icon_2rem)
                        .text_color(text_tertiary),
                )
                .child(
                    ui_element::label(&spec.empty_message)
                        .text_color(text_secondary)
                        .text_size(text_size),
                ),
        );
    }

    // Aspect-ratio container. No live iframe → contract-sanctioned placeholder
    // panel. Min-height derives from the effective aspect ratio against a nominal
    // reference width; "auto" (audio / ratio None) falls back to the static 10rem
    // media height from the contract.
    let container = |child: JsEl| -> JsEl {
        let mut frame = ui_element::div().bg(panel_bg).rounded(radius).grow();
        match spec.effective_aspect_ratio() {
            Some(ratio) => {
                let ref_width = rem_to_px(28.0);
                frame = frame.min_h((ref_width / ratio).max(rem_to_px(8.0)));
            }
            None => {
                frame = frame.h(rem_to_px(10.0));
            }
        }
        frame.child(child)
    };

    // Iframe / embed-URL state → placeholder panel with the derived embed URL.
    if let Some(url) = spec.embed_url() {
        let provider = spec
            .parsed
            .as_ref()
            .map(|p| p.provider.clone())
            .unwrap_or_default();
        return root().child(container(
            ui_element::div()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(state_gap)
                .pl(state_pad)
                .pr(state_pad)
                .pt(state_pad)
                .pb(state_pad)
                .grow()
                .child(
                    ui_element::icon("monitor-play")
                        .w(icon_2rem)
                        .h(icon_2rem)
                        .text_color(text_tertiary),
                )
                .child(
                    ui_element::label(format!("{provider} embed"))
                        .text_color(text_secondary)
                        .text_size(text_size),
                )
                .child(
                    ui_element::label(&url)
                        .text_color(text_tertiary)
                        .text_size(text_size),
                ),
        ));
    }

    // Raw embed (parsed.originalEmbed, no embed URL).
    if spec.has_raw_embed() {
        let raw = spec
            .parsed
            .as_ref()
            .and_then(|p| p.original_embed.clone())
            .unwrap_or_default();
        return root().child(container(
            ui_element::div()
                .pl(state_pad)
                .pr(state_pad)
                .pt(state_pad)
                .pb(state_pad)
                .grow()
                .child(
                    ui_element::label(raw)
                        .text_color(text_secondary)
                        .text_size(text_size),
                ),
        ));
    }

    // Trusted HTML (caller-sanitized) — rendered in the same container.
    if spec.has_trusted_html() {
        let html = spec.trusted_html.clone().unwrap_or_default();
        return root().child(container(
            ui_element::div()
                .pl(state_pad)
                .pr(state_pad)
                .pt(state_pad)
                .pb(state_pad)
                .grow()
                .child(
                    ui_element::label(html)
                        .text_color(text_secondary)
                        .text_size(text_size),
                ),
        ));
    }

    // Fallback: real TextLink to the original URL.
    let href = spec
        .parsed
        .as_ref()
        .and_then(|p| p.original_url.clone().or_else(|| Some(p.id.clone())))
        .unwrap_or_default();
    root().child(
        ui_element::div()
            .bg(panel_bg)
            .rounded(radius)
            .pl(fallback_pad_x)
            .pr(fallback_pad_x)
            .pt(fallback_pad_y)
            .pb(fallback_pad_y)
            .child(js_text_link(
                &TextLinkSpec::new(href.clone()).with_href(href),
                theme,
            )),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::ParsedEmbed;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn loading_uses_skeleton_block_and_loading_text() {
        let th = theme();
        let spec = EmbedPreviewSpec::new().with_loading(true);
        let tree = probe(&js_embed_preview(&spec, &th), 320.0, 200.0);
        assert!(tree.has_text("Loading preview..."), "loading text missing: {:?}", tree.texts());
        // Skeleton(block) resolves the contract 6rem block height.
        assert!(
            tree.nodes.iter().any(|n| (n.h - rem_to_px(6.0)).abs() < 0.5),
            "missing 6rem skeleton block: {}",
            tree.to_json()
        );
    }

    #[test]
    fn error_state_renders_icon_and_message() {
        let th = theme();
        let spec = EmbedPreviewSpec::new().with_error("Failed to load embed.");
        let tree = probe(&js_embed_preview(&spec, &th), 320.0, 200.0);
        assert!(tree.has_text("Failed to load embed."), "error text missing: {:?}", tree.texts());
        assert!(tree.count_kind("Icon") >= 1, "error icon missing: {}", tree.to_json());
        // No parsed content leaks into the error state.
        assert!(!tree.has_text("youtube embed"));
    }

    #[test]
    fn empty_state_shows_custom_message() {
        let th = theme();
        let spec = EmbedPreviewSpec::new().with_empty_message("Paste a URL above");
        let tree = probe(&js_embed_preview(&spec, &th), 320.0, 200.0);
        assert!(tree.has_text("Paste a URL above"), "empty message missing: {:?}", tree.texts());
        assert!(tree.count_kind("Icon") >= 1, "empty icon missing");
    }

    #[test]
    fn youtube_embed_shows_provider_title_and_nocookie_url() {
        let th = theme();
        let spec = EmbedPreviewSpec::new().with_parsed(ParsedEmbed::new("youtube", "abc123"));
        let tree = probe(&js_embed_preview(&spec, &th), 320.0, 240.0);
        // Provider title "youtube embed" + derived privacy-enhanced URL.
        assert!(tree.has_text("youtube embed"), "provider title missing: {:?}", tree.texts());
        assert!(
            tree.texts()
                .iter()
                .any(|t| t.contains("youtube-nocookie.com/embed/abc123")),
            "derived embed url missing: {:?}",
            tree.texts()
        );
        // No empty/loading text in the parsed state.
        assert!(!tree.has_text("No embed to preview"));
    }

    #[test]
    fn trusted_html_renders_in_container() {
        let th = theme();
        let spec = EmbedPreviewSpec::new().with_trusted_html("<b>stored embed</b>");
        let tree = probe(&js_embed_preview(&spec, &th), 320.0, 200.0);
        // trustedHtml renders even with no parsed embed (not the empty state).
        assert!(tree.has_text("<b>stored embed</b>"), "trusted html missing: {:?}", tree.texts());
        assert!(!tree.has_text("No embed to preview"), "trusted html must not show empty state");
    }

    #[test]
    fn fallback_renders_text_link_when_no_embed_url() {
        let th = theme();
        // Parsed embed with no derivable embed URL and no raw embed → fallback link
        // to the original id/url (embed_url() is None for generic without a URL).
        let spec =
            EmbedPreviewSpec::new().with_parsed(ParsedEmbed::new("generic", "embed-id-xyz"));
        let tree = probe(&js_embed_preview(&spec, &th), 320.0, 120.0);
        assert!(
            tree.has_text("embed-id-xyz"),
            "fallback link missing: {:?}",
            tree.texts()
        );
        assert!(!tree.has_text("No embed to preview"), "fallback must not show empty state");
    }
}
