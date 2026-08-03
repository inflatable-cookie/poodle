//! ColorPicker — swatch-grid builder. Split out of `color_picker/parts.rs`
//! (god-file decomposition); unchanged.

use glam::Vec4;
use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_jetstream::JetstreamThemeProvider;

use crate::presentation::rem_to_px;
use crate::theme_ext::{hex_to_rgb255, rgb255_to_vec4, tint, Rgb255};

/// Preset swatch grid. Each swatch is a 1.25rem square at its hex color; the
/// active swatch (matching the current value) gets a text-primary border, the
/// rest a transparent border. Top divider = border-subtle@42%.
pub(super) fn build_swatch_grid(
    theme: &JetstreamThemeProvider,
    swatches: &[String],
    current: &str,
    text_primary: Vec4,
    border_subtle: Vec4,
    on_change: Option<&crate::element::Handler>,
) -> JsEl {
    let swatch_size = rem_to_px(1.25);
    let swatch_radius = rem_to_px(0.1875);
    let gap = rem_to_px(0.25);
    let divider = tint(border_subtle, 0.42);
    let text_primary: Color = text_primary.into();
    let _ = theme;

    // Contract: the preset swatches are a `listbox` of `option`s.
    let mut grid = ui_element::div()
        .aria_role(jetstream_ui::accesskit::Role::ListBox)
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

        let mut swatch = ui_element::div()
            .aria_role(jetstream_ui::accesskit::Role::ListBoxOption)
            .aria_label(hex.clone())
            .id(format!("color-picker-swatch-{idx}"))
            .w(swatch_size)
            .h(swatch_size)
            .rounded(swatch_radius)
            .border(2.0)
            .border_color(border_color)
            .bg(swatch_color)
            .cursor_pointer()
            .focusable();

        if let Some(handler) = on_change {
            let handler = std::sync::Arc::clone(handler);
            let hex = hex.clone();
            swatch = swatch.on_click(move |_event| handler(&hex));
        }

        grid = grid.child(swatch);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_picker::js_color_picker;
    use crate::render_probe::{probe, ProbeColor};
    use crate::theme_ext::resolve_color;
    use poodle_jetstream::JetstreamThemeProvider;
    use poodle_specs::{ColorInputMode, ColorPickerSpec};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
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
        let spec = ColorPickerSpec::new().with_value("#3b82f6").with_open(true);
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
                tree.find_token(&format!("color-picker-swatch-{idx}"))
                    .is_some(),
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
        let spec = ColorPickerSpec::new()
            .with_value("not-a-hex")
            .with_open(true);
        let el = js_color_picker(&spec, &th);
        let tree = probe(&el, 600.0, 600.0);
        // Renders the surface using the fallback color, no panic.
        assert!(tree.find_token("color-picker-surface").is_some());
        let fallback = expected_color("#6366f1");
        assert!(tree.has_background(fallback, 0.01));
    }

    #[test]
    fn the_trigger_reports_a_toggle() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = crate::color_picker::ColorPicker::from_spec(
            ColorPickerSpec::new().with_value("#3b82f6"),
            &theme(),
        )
        .on_toggle(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .into_js_el();

        // The trigger is the only element; click its centre.
        let tree = crate::render_probe::probe(&el, 200.0, 120.0);
        let trigger = tree
            .nodes
            .iter()
            .find(|n| n.token_key.as_deref() == Some("color-picker-trigger"))
            .expect("the trigger swatch");
        crate::element::click_probe::click_at(
            &el,
            200.0,
            120.0,
            trigger.x + trigger.w / 2.0,
            trigger.y + trigger.h / 2.0,
        );

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_toggle fired exactly once"
        );
    }

    /// Only the presets can report a colour: the gradient area would need
    /// drag-with-position to compute one, so it stays inert rather than lying.
    #[test]
    fn choosing_a_preset_reports_its_hex() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let hexes = Arc::clone(&seen);

        let spec = ColorPickerSpec::new()
            .with_value("#3b82f6")
            .with_open(true)
            .with_swatches(vec!["#ff0000".into(), "#00ff00".into()]);

        let el = crate::color_picker::ColorPicker::from_spec(spec, &theme())
            .on_change(move |hex| hexes.lock().unwrap().push(hex.to_string()))
            .into_js_el();

        let tree = crate::render_probe::probe(&el, 480.0, 640.0);
        let swatch = tree
            .nodes
            .iter()
            .find(|n| n.token_key.as_deref() == Some("color-picker-swatch-1"))
            .expect("the second preset");
        crate::element::click_probe::click_at(
            &el,
            480.0,
            640.0,
            swatch.x + swatch.w / 2.0,
            swatch.y + swatch.h / 2.0,
        );

        assert_eq!(seen.lock().unwrap().as_slice(), ["#00ff00"]);
    }
}
