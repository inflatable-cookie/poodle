//! HoverCard — Jetstream hover card backed by HoverCardSpec.
//!
//! Contract: `docs/contracts/components/hover-card.md`
//! Renders the open surface (contract §8). The trigger relationship, delay
//! timers, anchored placement and viewport clamping live in the preview event
//! loop (contract §12 Known Delta), so this renders the surface at its current
//! open state — every surface visual property resolves from tokens.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::HoverCardSpec;

use crate::theme_ext::{color_mix, elevation_overlay, resolve_color, resolve_px, resolve_radius, tint};

pub fn js_hover_card(
    spec: &HoverCardSpec,
    theme: &JetstreamThemeProvider,
    content: Option<JsEl>,
) -> JsEl {
    // Contract §8 background: color-mix(elevated 98%, panel).
    let elevated = resolve_color(theme, spec.fill_token());
    let panel = resolve_color(theme, "color.background.panel");
    let fill = color_mix(elevated, panel, 0.98);

    // Contract §8 border: color-mix(border-default 72%, transparent) → 0.72 alpha
    // (matches the GPUI multiplier).
    let border = tint(resolve_color(theme, "color.border.default"), 0.72);
    let radius = resolve_radius(theme, "radius.surface");

    // Contract §8 padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x).
    let pad_x = resolve_px(theme, "space.panel.x");
    let pad_y = resolve_px(theme, "space.panel.y");

    // Contract §7 sizing: min-width 14rem, max-width min(22rem, 90vw).
    // The 90vw clamp is host/viewport-driven (preview loop); the component
    // resolves the 14rem / 22rem token bounds.
    let min_w = resolve_px(theme, "size.menu.minWidth");
    let max_w = resolve_px(theme, "size.hoverCard.maxWidth");

    // Token-accurate `elevation-overlay` (spec.shadow_token()) resolved from the
    // typed semantic token via the runtime shadow builder (single layer, spread
    // 0; matches GPUI's mapping).
    let mut el = elevation_overlay(
        ui_element::div()
            .bg(fill)
            .border(1.0)
            .border_color(border)
            .rounded(radius)
            .pl(pad_x)
            .pr(pad_x)
            .pt(pad_y)
            .pb(pad_y)
            .min_w(min_w)
            .max_w(max_w),
    )
    .overlay();

    if let Some(c) = content {
        el = el.child(c);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use crate::theme_ext::color_mix;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn open_spec() -> HoverCardSpec {
        HoverCardSpec::new().with_open(true)
    }

    #[test]
    fn renders_surface_with_content() {
        let th = theme();
        let content = ui_element::label("Hover preview");
        let el = js_hover_card(&open_spec(), &th, Some(content));
        let tree = probe(&el, 480.0, 320.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(
            tree.has_text("Hover preview"),
            "surface content missing: {:?}",
            tree.texts()
        );
        // Root surface laid out with positive size.
        let root = &tree.nodes[0];
        assert!(root.w > 0.0 && root.h > 0.0, "surface not laid out: {root:?}");
    }

    #[test]
    fn surface_respects_min_width_token() {
        let th = theme();
        // Narrow content; surface should still reach the 14rem min-width.
        let el = js_hover_card(&open_spec(), &th, Some(ui_element::label("x")));
        let tree = probe(&el, 800.0, 320.0);
        let min_w = resolve_px(&th, "size.menu.minWidth");
        let root = &tree.nodes[0];
        assert!(
            root.w >= min_w - 0.5,
            "surface width {} below min-width token {}",
            root.w,
            min_w
        );
    }

    #[test]
    fn surface_fill_is_elevated_mixed_with_panel() {
        let th = theme();
        let el = js_hover_card(&open_spec(), &th, None);
        let tree = probe(&el, 480.0, 320.0);

        let elevated = resolve_color(&th, HoverCardSpec::new().fill_token());
        let panel = resolve_color(&th, "color.background.panel");
        let mixed = color_mix(elevated, panel, 0.98);
        let expected = ProbeColor {
            r: mixed.x,
            g: mixed.y,
            b: mixed.z,
            a: mixed.w,
        };
        assert!(
            tree.has_background(expected, 0.02),
            "surface fill not the elevated/panel mix; backgrounds present: {:?}",
            tree.nodes.iter().filter_map(|n| n.bg).collect::<Vec<_>>()
        );
    }
}
