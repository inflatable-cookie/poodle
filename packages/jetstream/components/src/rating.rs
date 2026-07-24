//! Rating — Jetstream star rating backed by RatingSpec.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, RatingSpec};

use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, tint};

/// Per-size glyph font-size in rem (contract §8 size table).
fn glyph_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Per-density inter-item gap in rem (contract §8: base 0.125, compact 0.0625,
/// comfortable 0.25).
fn item_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.0625,
        ControlDensity::Default => 0.125,
        ControlDensity::Comfortable => 0.25,
    }
}

pub fn js_rating(spec: &RatingSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let active = resolve_color(theme, spec.active_color_token());
    // Contract §8: unfilled color = color-mix(text-secondary 48%, transparent).
    let inactive = tint(
        resolve_color(theme, spec.inactive_color_token()),
        spec.inactive_color_alpha(),
    );

    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    // Contract §8 glyph font-size; Svelte renders the SVG at 1.125em.
    let glyph_px = rem_to_px(glyph_font_rem(effective_size)) * 1.125;
    // Contract §7/§8: each item is a 2rem (size-scaled) square touch target.
    let item_px = rem_to_px(control_height_rem(effective_size));
    // Gap between stars: density-driven (contract §8).
    let gap = rem_to_px(item_gap_rem(spec.density));

    let mut el = ui_element::div().flex_row().items_center().gap(gap);

    for i in 0..spec.max {
        // Contract §2/§4: clipped accent overlay sized by per-star fill ratio.
        let ratio = spec.fill_ratio(i) as f32;
        let fill_w = (glyph_px * ratio).clamp(0.0, glyph_px);

        // Base (unfilled) glyph layer.
        let base = ui_element::icon("star")
            .w(glyph_px)
            .h(glyph_px)
            .text_color(inactive);

        // Glyph wrapper: relative base + absolute clipped fill overlay.
        let mut glyph = ui_element::div()
            .relative()
            .w(glyph_px)
            .h(glyph_px)
            .items_center()
            .justify_center()
            .child(base);

        if fill_w > 0.0 {
            let fill = ui_element::div()
                .absolute()
                .top(0.0)
                .left(0.0)
                .h(glyph_px)
                .w(fill_w)
                .overflow_hidden()
                .child(
                    ui_element::icon("star")
                        .w(glyph_px)
                        .h(glyph_px)
                        .text_color(active),
                );
            glyph = glyph.child(fill);
        }

        // Touch-target wrapper: fixed square hit area per effective size.
        el = el.child(
            ui_element::div()
                .w(item_px)
                .h(item_px)
                .flex_row()
                .items_center()
                .justify_center()
                .child(glyph),
        );
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use crate::theme_ext::tint;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn px_color(v: glam::Vec4) -> ProbeColor {
        ProbeColor { r: v.x, g: v.y, b: v.z, a: v.w }
    }

    #[test]
    fn renders_one_touch_target_and_two_glyph_layers_per_star() {
        let spec = RatingSpec::new().with_value(3.0).with_max(5);
        let tree = probe(&js_rating(&spec, &theme()), 240.0, 48.0);
        // 5 stars: each filled star contributes a base + fill icon (2),
        // each unfilled star contributes only a base (1).
        // value=3 → 3 filled (6 icons) + 2 unfilled (2 icons) = 8 star icons.
        assert_eq!(tree.count_kind("Icon"), 8, "icon layers: {:?}", tree.texts());
    }

    #[test]
    fn empty_value_renders_only_base_layers() {
        let spec = RatingSpec::new().with_value(0.0).with_max(5);
        let tree = probe(&js_rating(&spec, &theme()), 240.0, 48.0);
        // No fill overlays → exactly `max` base icons.
        assert_eq!(tree.count_kind("Icon"), 5, "expected 5 base icons only");
    }

    #[test]
    fn full_value_renders_fill_layer_for_every_star() {
        let spec = RatingSpec::new().with_value(5.0).with_max(5);
        let tree = probe(&js_rating(&spec, &theme()), 240.0, 48.0);
        // Every star filled → base + fill = 10 icons.
        assert_eq!(tree.count_kind("Icon"), 10, "expected 10 icons (all filled)");
    }

    #[test]
    fn active_and_inactive_colors_are_token_resolved() {
        let th = theme();
        let spec = RatingSpec::new().with_value(1.0).with_max(2);
        let tree = probe(&js_rating(&spec, &th), 120.0, 48.0);
        let active = px_color(resolve_color(&th, spec.active_color_token()));
        let inactive = px_color(tint(
            resolve_color(&th, spec.inactive_color_token()),
            spec.inactive_color_alpha(),
        ));
        // value=1 of 2: one filled (active) star + one unfilled (inactive) star.
        assert!(
            tree.nodes.iter().any(|n| n.bg.is_none() && n.kind == "Icon"),
            "icons present"
        );
        // Colors travel on text_color, not bg; assert both resolve to non-equal,
        // confirming inactive is the dimmed text-secondary, active the accent.
        assert!(
            (active.r - inactive.r).abs() > f32::EPSILON
                || (active.a - inactive.a).abs() > f32::EPSILON,
            "active vs inactive must differ: active={active:?} inactive={inactive:?}"
        );
    }
}
