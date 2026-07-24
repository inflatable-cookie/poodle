//! StateTile — Jetstream lightweight label-value tile backed by StateTileSpec.
//!
//! Contract: `docs/contracts/components/state-tile.md`
//! Reference: Svelte component NOT YET BUILT — the contract is the sole
//! authority (see `docs/parity/state-tile.md`). GPUI mirrors the same contract.
//!
//! Anatomy (contract §2):
//! ```text
//! [Root .state-tile]
//!   ├── [Label]      caption (typography-label, text-secondary)
//!   ├── [Value]      primary value (typography-heading)
//!   ├── [Trend]      optional: directional glyph + trend label
//!   └── [Sparkline]  optional reserved slot (host renders the chart)
//! ```
//!
//! ALL colors + radius/border/typography resolve from spec token methods.
//! Gaps/padding resolve from space tokens. ZERO hardcoded colors.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::StateTileSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_state_tile(spec: &StateTileSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // ── Colors (token-resolved via spec) ──
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let label_color = resolve_color(theme, spec.label_color_token());
    let value_color = resolve_color(theme, spec.value_color_token());
    let trend_color = resolve_color(theme, spec.trend_color_token());

    // ── Dimensions ──
    let radius = resolve_radius(theme, spec.radius_token());
    let border_width = resolve_px(theme, spec.border_width_token());
    // Padding / gaps resolve from space tokens (panel padding, stack/inline gaps).
    let pad_x = resolve_px(theme, "space.panel.x");
    let pad_y = resolve_px(theme, "space.panel.y");
    let gap = resolve_px(theme, "space.stack.sm"); // label↔value↔trend↔sparkline
    let trend_gap = resolve_px(theme, "space.inline.xs"); // glyph↔label
    // Typography from spec token methods (contract §2: Label→label, Value→heading).
    let label_size = resolve_px(theme, spec.label_font_size_token());
    let value_size = resolve_px(theme, spec.value_font_size_token());
    let trend_size = resolve_px(theme, spec.trend_font_size_token());

    let mut el = ui_element::div()
        .bg(fill)
        .border(border_width)
        .border_color(border)
        .rounded(radius)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .flex_col()
        .gap(gap);

    // ── Label (contract §2: caption, text-secondary) ──
    el = el.child(
        ui_element::label(&spec.label)
            .text_color(label_color)
            .text_size(label_size),
    );

    // ── Value (contract §2: primary value, typography-heading) ──
    el = el.child(
        ui_element::label(&spec.value)
            .text_color(value_color)
            .text_size(value_size)
            .text_weight(700),
    );

    // ── Trend row (optional) ──
    // Contract §7: trend meaning lives in `trend_label` text; the glyph is
    // decorative.
    if spec.trend.is_some() || spec.trend_label.is_some() {
        let mut trend_row = ui_element::div().flex_row().items_center().gap(trend_gap);

        if spec.trend.is_some() {
            trend_row = trend_row.child(
                ui_element::label(spec.trend_glyph())
                    .text_color(trend_color)
                    .text_size(trend_size)
                    .text_weight(600),
            );
        }

        if let Some(ref trend_label) = spec.trend_label {
            trend_row = trend_row.child(
                ui_element::label(trend_label)
                    .text_color(trend_color)
                    .text_size(trend_size),
            );
        }

        el = el.child(trend_row);
    }

    // ── Sparkline slot (optional) ──
    // Contract §1/§2/§3: the host owns the chart; StateTile only reserves the
    // slot. We render an empty reserved area (no synthetic chart data).
    if spec.has_sparkline {
        let sparkline_h = rem_to_px(2.0); // note: contract slot height, no token
        let slot_bg = resolve_color(theme, spec.sparkline_slot_token());

        el = el.child(
            ui_element::div()
                .id("state-tile-sparkline")
                .bg(slot_bg)
                .rounded(radius)
                .h(sparkline_h)
                .grow(),
        );
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use jetstream_ui::Color;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_label_and_value() {
        let th = theme();
        let el = js_state_tile(&StateTileSpec::new("Active Users", "1,284"), &th);
        let tree = probe(&el, 240.0, 160.0);
        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(tree.has_text("Active Users"), "label missing: {:?}", tree.texts());
        assert!(tree.has_text("1,284"), "value missing: {:?}", tree.texts());
    }

    #[test]
    fn default_state_has_no_trend_or_sparkline() {
        let th = theme();
        let el = js_state_tile(&StateTileSpec::new("Latency", "42ms"), &th);
        let tree = probe(&el, 240.0, 160.0);
        // No trend glyph, no sparkline slot.
        assert!(tree.find_token("state-tile-sparkline").is_none(), "sparkline should be absent");
        assert!(!tree.has_text("\u{2191}") && !tree.has_text("\u{2192}"), "no trend glyph expected");
    }

    #[test]
    fn up_trend_uses_success_color_and_glyph() {
        let th = theme();
        let success: Color = resolve_color(&th, "color.status.success").into();
        let spec = StateTileSpec::new("Revenue", "$42,800")
            .with_trend("up")
            .with_trend_label("+8.5%");
        let el = js_state_tile(&spec, &th);
        let tree = probe(&el, 240.0, 160.0);
        assert!(tree.has_text("\u{2191}"), "up glyph missing: {:?}", tree.texts());
        assert!(tree.has_text("+8.5%"), "trend label missing: {:?}", tree.texts());
        // Up trend resolves the success color via the spec's trend_color_token.
        let resolved: Color = resolve_color(&th, spec.trend_color_token()).into();
        assert!(
            (resolved.r - success.r).abs() < 0.02 && (resolved.g - success.g).abs() < 0.02,
            "up trend should resolve status-success, got {resolved:?}"
        );
    }

    #[test]
    fn down_trend_uses_danger_glyph() {
        let th = theme();
        let el = js_state_tile(
            &StateTileSpec::new("Error Rate", "0.3%").with_trend("down"),
            &th,
        );
        let tree = probe(&el, 240.0, 160.0);
        assert!(tree.has_text("\u{2193}"), "down glyph missing: {:?}", tree.texts());
    }

    #[test]
    fn neutral_string_trend_uses_arrow_glyph() {
        let th = theme();
        let el = js_state_tile(
            &StateTileSpec::new("Sessions", "9,001")
                .with_trend("flat")
                .with_trend_label("Stable"),
            &th,
        );
        let tree = probe(&el, 240.0, 160.0);
        // Any non up/down string → right arrow (neutral).
        assert!(tree.has_text("\u{2192}"), "neutral glyph missing: {:?}", tree.texts());
        assert!(tree.has_text("Stable"), "trend label missing: {:?}", tree.texts());
    }

    #[test]
    fn sparkline_reserves_slot_when_flagged() {
        let th = theme();
        let el = js_state_tile(
            &StateTileSpec::new("Revenue", "$42,800").with_sparkline(true),
            &th,
        );
        let tree = probe(&el, 240.0, 200.0);
        let slot = tree.find_token("state-tile-sparkline").expect("sparkline slot present");
        assert!(slot.h > 0.0, "sparkline slot should have height");
    }

    #[test]
    fn root_resolves_radius_and_border() {
        let th = theme();
        let el = js_state_tile(&StateTileSpec::new("X", "1"), &th);
        assert!(el.style.border_width > 0.0, "border width resolved");
        assert!(el.style.background.is_some(), "fill resolved");
    }
}
