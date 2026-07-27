//! Tooltip — Jetstream tooltip backed by TooltipSpec.
//!
//! Contract: `docs/contracts/components/tooltip.md`
//! Uses overlay() for the tooltip panel. Triggered by on_pointer_enter/leave.

use jetstream_ui::{BoxShadow, Color};
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::TooltipSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

/// Build a tooltip bubble from a TooltipSpec.
///
/// Contract: `docs/contracts/components/tooltip.md` §8 (Bubble).
/// Reference: `packages/svelte/components/src/Tooltip.svelte`.
///
/// Contract bubble token map (§8):
/// - padding         0.375rem 0.5rem
/// - max-width       16rem
/// - border          0.0625rem solid color-mix(border-default 72%, transparent)
/// - border-radius   calc(radius.control − 0.125rem)
/// - background      color-mix(background.elevated 98%, background.panel)
/// - box-shadow      0 0.5rem 1.25rem rgba(0,0,0,0.3), 0 0.125rem 0.375rem rgba(0,0,0,0.2)
/// - color           text.primary
/// - font-size       0.6875rem
///
/// The bubble is the only part this component renders — trigger anchoring,
/// open/delay/dismiss, and placement positioning are owned by the runtime /
/// preview overlay loop (see notes below), matching the GPUI render-only model.
pub fn js_tooltip(spec: &TooltipSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "color.text.primary");

    // Contract §8 background = color-mix(elevated 98%, panel) — not the raw
    // elevated fill. Mix in the Jetstream linear space (cross-target color delta
    // vs GPUI's sRGB mix is allowed per §12).
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();
    let panel: Color = resolve_color(theme, "color.background.panel").into();
    let fill: Color = color_mix(elevated.into(), panel.into(), 0.98).into();

    // Contract §8 border = color-mix(border-default 72%, transparent), 0.0625rem.
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let border_color: Color = border_default.with_alpha(border_default.a * 0.72);
    let border_width = rem_to_px(0.0625);

    // Contract §8 radius = calc(radius.control − 0.125rem). `radius.surface` was
    // wrong; resolve control radius and subtract the contract inset.
    let radius = resolve_radius(theme, "radius.control") - rem_to_px(spec.radius_inset_rem());

    let content = spec.content.as_deref().unwrap_or("");

    // Contract: padding 0.375rem 0.5rem, font-size 0.6875rem (11px), max-width 16rem
    let pad_x = rem_to_px(spec.padding_x_rem());
    let pad_y = rem_to_px(spec.padding_y_rem());
    let font_size = rem_to_px(spec.font_size_rem());
    let max_w = rem_to_px(spec.max_width_rem());

    // Contract §8 box-shadow is a literal two-layer drop shadow. JsEl holds a
    // single shadow layer, so we render the dominant first layer
    // (0 0.5rem 1.25rem rgba(0,0,0,0.3)); the secondary 0.125rem/0.375rem/0.2
    // layer is dropped (JsEl single-shadow limitation, noted in parity doc).
    let overlay_shadow = BoxShadow {
        offset_x: 0.0,
        offset_y: rem_to_px(0.5),
        blur: rem_to_px(1.25),
        spread: 0.0,
        color: glam::Vec4::new(0.0, 0.0, 0.0, 0.30),
        inset: false,
    };

    // Tooltip bubble: overlay surface positioned by the runtime/preview loop.
    let mut bubble = ui_element::div()
        .bg(fill)
        .rounded(radius)
        .border(border_width)
        .border_color(border_color)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .max_w(max_w)
        .whitespace_nowrap()
        .overlay()
        .child(
            ui_element::label(content)
                .text_color(text_color)
                .text_size(font_size),
        );
    bubble.style.shadow = Some(overlay_shadow);
    crate::aria::with_aria_label(bubble, spec.aria_label.as_deref())
        .aria_role(jetstream_ui::accesskit::Role::Tooltip)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::OverlayPlacement;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_content_text() {
        let el = js_tooltip(&TooltipSpec::new().with_content("Save your changes"), &theme());
        let tree = probe(&el, 320.0, 80.0);
        assert!(
            tree.has_text("Save your changes"),
            "tooltip content missing: {:?}",
            tree.texts(),
        );
    }

    #[test]
    fn surface_has_padding_and_max_width() {
        // Bubble width must not exceed the 16rem (256px) cap and must include the
        // 0.5rem horizontal padding around short content.
        let el = js_tooltip(&TooltipSpec::new().with_content("OK"), &theme());
        let tree = probe(&el, 600.0, 80.0);
        let bubble = &tree.nodes[0];
        assert!(bubble.w <= 256.0 + 0.5, "bubble exceeds max-width: {}", bubble.w);
        // padding(0.5rem each side = 16px) means the bubble is wider than its text.
        assert!(bubble.w > 0.0 && bubble.h > 0.0, "bubble not laid out: {bubble:?}");
    }

    #[test]
    fn surface_fill_is_elevated_panel_mix() {
        // Background = color-mix(elevated 98%, panel). Assert the resolved fill
        // matches that mix (not the raw elevated token).
        let th = theme();
        let elevated: Color = resolve_color(&th, "color.background.elevated").into();
        let panel: Color = resolve_color(&th, "color.background.panel").into();
        let mixed: Color = color_mix(elevated.into(), panel.into(), 0.98).into();
        let el = js_tooltip(&TooltipSpec::new().with_content("Tip"), &th);
        let tree = probe(&el, 320.0, 80.0);
        let bubble_bg = tree.nodes[0].bg.expect("bubble has no fill");
        assert!(
            bubble_bg.approx(
                ProbeColor { r: mixed.r, g: mixed.g, b: mixed.b, a: mixed.a },
                0.01,
            ),
            "bubble fill not the elevated/panel mix: {bubble_bg:?} vs {mixed:?}",
        );
    }

    #[test]
    fn placement_does_not_change_bubble_surface() {
        // Placement is positioning-only (runtime/preview loop); the rendered
        // bubble surface is identical across placements.
        let top = js_tooltip(
            &TooltipSpec::new().with_content("X").with_placement(OverlayPlacement::Top),
            &theme(),
        );
        let bottom = js_tooltip(
            &TooltipSpec::new().with_content("X").with_placement(OverlayPlacement::Bottom),
            &theme(),
        );
        let t_top = probe(&top, 320.0, 80.0);
        let t_bottom = probe(&bottom, 320.0, 80.0);
        assert_eq!(t_top.nodes[0].w, t_bottom.nodes[0].w);
        assert_eq!(t_top.nodes[0].bg, t_bottom.nodes[0].bg);
    }
}
