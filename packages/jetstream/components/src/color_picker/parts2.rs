//! ColorPicker — swatch-grid builder. Split out of `color_picker/parts.rs`
//! (god-file decomposition); unchanged.

use glam::Vec4;
use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;

use crate::presentation::rem_to_px;
use crate::theme_ext::{
    hex_to_rgb255, rgb255_to_vec4, tint, Rgb255,
};



/// Preset swatch grid. Each swatch is a 1.25rem square at its hex color; the
/// active swatch (matching the current value) gets a text-primary border, the
/// rest a transparent border. Top divider = border-subtle@42%.
pub(super) fn build_swatch_grid(
    theme: &JetstreamThemeProvider,
    swatches: &[String],
    current: &str,
    text_primary: Vec4,
    border_subtle: Vec4,
) -> JsEl {
    let swatch_size = rem_to_px(1.25);
    let swatch_radius = rem_to_px(0.1875);
    let gap = rem_to_px(0.25);
    let divider = tint(border_subtle, 0.42);
    let text_primary: Color = text_primary.into();
    let _ = theme;

    let mut grid = ui_element::div()
        .id("color-picker-swatches")
        .flex_row()
        .flex_wrap()
        .gap(gap)
        .pt(gap)
        .border_t_1()
        .border_color_top(divider);

    for (idx, hex) in swatches.iter().enumerate() {
        let rgb = hex_to_rgb255(hex).unwrap_or(Rgb255 {
            r: 0,
            g: 0,
            b: 0,
            a: 1.0,
        });
        let swatch_color: Color = rgb255_to_vec4(rgb, 1.0).into();
        let is_active = hex.eq_ignore_ascii_case(current);

        let border_color = if is_active {
            text_primary
        } else {
            Color::TRANSPARENT
        };

        let swatch = ui_element::div()
            .id(format!("color-picker-swatch-{idx}"))
            .w(swatch_size)
            .h(swatch_size)
            .rounded(swatch_radius)
            .border(2.0)
            .border_color(border_color)
            .bg(swatch_color)
            .cursor_pointer()
            .focusable();

        grid = grid.child(swatch);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_jetstream::JetstreamThemeProvider;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    /// The parsed-value color as a ProbeColor (what the trigger swatch must show).
    fn expected_color(hex: &str) -> ProbeColor {
        let rgb = hex_to_rgb255(hex).unwrap();
        let v = rgb255_to_vec4(rgb, rgb.a);
        ProbeColor {
            r: v.x,
            g: v.y,
            b: v.z,
            a: v.w,
        }
    }

    #[test]
    fn trigger_swatch_uses_parsed_value_not_accent_base() {
        let th = theme();
        // A clearly non-accent color.
        let spec = ColorPickerSpec::new().with_value("#ff0000");
        let el = js_color_picker(&spec, &th);
        let tree = probe(&el, 400.0, 400.0);

        let want = expected_color("#ff0000");
        assert!(
            tree.has_background(want, 0.01),
            "trigger swatch missing the parsed value color {want:?}; tree: {}",
            tree.to_json()
        );

        // It must NOT be filled with accent-base (the old placeholder bug).
        let accent = resolve_color(&th, "color.accent.base");
        let accent_pc = ProbeColor {
            r: accent.x,
            g: accent.y,
            b: accent.z,
            a: accent.w,
        };
        // Accent-base differs from pure red; assert the swatch fill is red, and
        // that red is not accidentally equal to accent-base.
        assert!(
            !want.approx(accent_pc, 0.01),
            "test precondition: value color equals accent-base; pick a different value"
        );
    }

    #[test]
    fn closed_picker_renders_trigger_only() {
        let th = theme();
        let spec = ColorPickerSpec::new().with_value("#3b82f6");
        let el = js_color_picker(&spec, &th);
        let tree = probe(&el, 400.0, 400.0);

        assert!(tree.find_token("color-picker-trigger").is_some());
        // No surface when closed.
        assert!(
            tree.find_token("color-picker-surface").is_none(),
            "surface should not render when picker is closed"
        );
        assert!(tree.find_token("color-picker-gradient").is_none());
    }

    #[test]
    fn open_surface_renders_pad_hue_mode_and_channels() {
        let th = theme();
        let spec = ColorPickerSpec::new()
            .with_value("#3b82f6")
            .with_open(true);
        let el = js_color_picker(&spec, &th);
        let tree = probe(&el, 600.0, 600.0);

        // Surface and the defining controls are present.
        assert!(
            tree.find_token("color-picker-surface").is_some(),
            "surface missing"
        );
        assert!(
            tree.find_token("color-picker-gradient").is_some(),
            "gradient sat-value pad missing"
        );
        assert!(
            tree.find_token("color-picker-hue").is_some(),
            "hue slider missing"
        );

        // Mode toggle SegmentedControl text present.
        let texts = tree.texts();
        assert!(tree.has_text("Hex"), "mode toggle Hex missing: {texts:?}");
        assert!(tree.has_text("RGB"), "mode toggle RGB missing: {texts:?}");
        assert!(tree.has_text("HSL"), "mode toggle HSL missing: {texts:?}");

        // Hex mode (default): hex channel field + uppercase HEX label.
        assert!(
            tree.find_token("color-picker-hex-input").is_some(),
            "hex channel input missing"
        );
        assert!(tree.has_text("HEX"), "HEX channel label missing: {texts:?}");

        // The current value renders somewhere (hex field / inline input).
        assert!(
            tree.has_text("#3b82f6"),
            "current value text missing: {texts:?}"
        );

        // Trigger swatch still reflects the parsed value.
        let want = expected_color("#3b82f6");
        assert!(
            tree.has_background(want, 0.01),
            "trigger swatch not the parsed value: {}",
            tree.to_json()
        );
    }

    #[test]
    fn open_with_alpha_renders_alpha_slider_and_channel() {
        let th = theme();
        let spec = ColorPickerSpec::new()
            .with_value("#3b82f6")
            .with_open(true)
            .with_show_alpha(true);
        let el = js_color_picker(&spec, &th);
        let tree = probe(&el, 600.0, 600.0);

        assert!(
            tree.find_token("color-picker-alpha").is_some(),
            "alpha slider missing when show_alpha=true"
        );
        // Alpha channel label "A" present in hex mode.
        assert!(tree.has_text("A"), "alpha channel label missing");
    }

    #[test]
    fn rgb_mode_renders_three_channel_labels() {
        let th = theme();
        let spec = ColorPickerSpec::new()
            .with_value("#3b82f6")
            .with_open(true)
            .with_default_mode(ColorInputMode::Rgb);
        let el = js_color_picker(&spec, &th);
        let tree = probe(&el, 600.0, 600.0);

        for label in ["R", "G", "B"] {
            assert!(
                tree.has_text(label),
                "RGB channel label {label} missing: {:?}",
                tree.texts()
            );
        }
    }

    #[test]
    fn swatch_grid_renders_swatches_with_active_token() {
        let th = theme();
        let spec = ColorPickerSpec::new()
            .with_value("#3b82f6")
            .with_open(true)
            .with_swatches(vec![
                "#ef4444".to_string(),
                "#3b82f6".to_string(),
                "#22c55e".to_string(),
            ]);
        let el = js_color_picker(&spec, &th);
        let tree = probe(&el, 600.0, 600.0);

        assert!(tree.find_token("color-picker-swatches").is_some());
        for idx in 0..3 {
            assert!(
                tree.find_token(&format!("color-picker-swatch-{idx}")).is_some(),
                "swatch {idx} missing"
            );
        }
        // The active swatch (#3b82f6) is filled with that color.
        let active = expected_color("#3b82f6");
        assert!(tree.has_background(active, 0.01));
    }

    #[test]
    fn malformed_value_falls_back_without_panicking() {
        let th = theme();
        let spec = ColorPickerSpec::new().with_value("not-a-hex").with_open(true);
        let el = js_color_picker(&spec, &th);
        let tree = probe(&el, 600.0, 600.0);
        // Renders the surface using the fallback color, no panic.
        assert!(tree.find_token("color-picker-surface").is_some());
        let fallback = expected_color("#6366f1");
        assert!(tree.has_background(fallback, 0.01));
    }
}


