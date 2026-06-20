//! ColorPicker — real Jetstream component backed by ColorPickerSpec.
//!
//! Contract: `docs/contracts/components/color-picker.md`. Svelte reference:
//! `packages/svelte/components/src/ColorPicker.svelte`. Structure mirrors the
//! GPUI build-out (`packages/gpui/components/src/primitives/color_picker.rs`).
//!
//! Anatomy rendered here: trigger swatch (the spec's ACTUAL current color) +
//! optional inline hex input, and — when open — the surface popover containing
//! the 2D saturation/value gradient pad with thumb, hue slider, optional alpha
//! slider, mode toggle (SegmentedControl), channel inputs (hex/RGB/HSL), and
//! optional preset swatch grid.
//!
//! # Color source
//! The gradient/swatch/thumb COLORS are computed from the picker's value via
//! `theme_ext` color math — the one legitimate non-token color source (per
//! contract). All chrome (sizes, radii, borders, surface fill) resolves from
//! tokens.
//!
//! # Jetstream notes (probe-verified layout; render structure, not interactivity)
//! - `JsEl`/`NodeStyle` gradients (`bg_gradient_linear`) support an arbitrary
//!   number of color stops, so the hue strip renders as a SINGLE 7-stop rainbow
//!   gradient (vs GPUI's six stacked two-stop segments). The sat-value pad's two
//!   CSS overlays (white→transparent, transparent→black) are two absolutely
//!   positioned children each carrying a single two-stop gradient — mirroring
//!   the CSS `::before`/`::after`. The alpha strip checkerboard has no native
//!   repeating-conic-gradient, so it is a neutral surface base with a
//!   transparent→color overlay (same posture as GPUI/Svelte fallback).
//! - Interaction (gradient drag, slider drag, mode switch, hex/channel edit,
//!   swatch click) is preview-event-loop bound. Controls render at the current
//!   value; the embedded segmented-control / number-input carry their own
//!   preview wiring. No interactivity is faked here.
//! - No ARIA channel on Jetstream elements (role/aria-valuetext not emitted).

use glam::Vec4;
use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ChoiceOption, ColorInputMode, ColorPickerSpec, NumberInputSpec, SegmentedControlSpec,
};

use crate::number_input::js_number_input;
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::segmented_control::js_segmented_control;
use crate::theme_ext::{
    hex_to_rgb255, hsv_to_hsl, pure_hue_vec4, resolve_color, resolve_opacity, resolve_px,
    resolve_radius, rgb255_to_vec4, rgb_to_hsv, tint, Hsv, Rgb255,
};

/// Default fallback color when the spec value is missing/malformed (#6366f1).
const FALLBACK_RGB: Rgb255 = Rgb255 {
    r: 99,
    g: 102,
    b: 241,
    a: 1.0,
};

/// Build the ColorPicker element (trigger + optional surface) for a spec.
pub fn js_color_picker(spec: &ColorPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let trigger_size = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    // ── Resolved chrome tokens ────────────────────────────────────
    let border = resolve_color(theme, spec.border_token());
    let trigger_radius = resolve_radius(theme, spec.trigger_radius_token());
    let surface_radius = resolve_radius(theme, spec.surface_radius_token());
    let radius_control = resolve_radius(theme, "radius.control");
    let surface_bg = resolve_color(theme, "color.background.surface");
    let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    // Stack gap (controls row → surface) and surface internal gaps.
    let stack_gap = resolve_px(theme, "space.stack.sm");
    let surface_gap = resolve_px(theme, "space.stack.md");

    // Trigger border is 62% opacity of border-default per contract.
    let trigger_border = tint(border, 0.62);

    // ── Current color (the legitimate non-token color source) ─────
    let current = spec.current_value().unwrap_or("#6366f1").to_string();
    let rgb = hex_to_rgb255(&current).unwrap_or(FALLBACK_RGB);
    let hsv: Hsv = rgb_to_hsv(rgb);
    let alpha = if spec.show_alpha { rgb.a } else { 1.0 };
    let current_color: Vec4 = rgb255_to_vec4(rgb, alpha);

    // ── Trigger swatch — fills with the ACTUAL current color ──────
    let preview = ui_element::div()
        .grow()
        .rounded((trigger_radius - 1.0).max(0.0))
        .bg(current_color);

    let trigger = ui_element::div()
        .id("color-picker-trigger")
        .w(trigger_size)
        .h(trigger_size)
        .border(1.0)
        .border_color(trigger_border)
        .rounded(trigger_radius)
        .overflow_hidden()
        .cursor_pointer()
        .focusable()
        .child(preview);

    // ── Controls row: trigger + optional inline hex input ─────────
    let mut controls_row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(stack_gap)
        .child(trigger);

    if spec.show_input {
        let input_display = spec.current_value().unwrap_or("#6366f1");
        let input_color = if spec.current_value().is_some() {
            text_primary
        } else {
            text_secondary
        };
        // Contract §8 inline input: 6.5rem wide, control height, code font.
        controls_row = controls_row.child(
            ui_element::div()
                .w(rem_to_px(6.5))
                .h(trigger_size)
                .pl(pad_x)
                .pr(pad_x)
                .border(1.0)
                .border_color(border)
                .rounded(trigger_radius)
                .bg(surface_bg)
                .flex_row()
                .items_center()
                .child(
                    ui_element::label(input_display)
                        .text_color(input_color)
                        .text_size(font_size),
                ),
        );
    }

    let mut root = ui_element::div().flex_col().gap(stack_gap).child(controls_row);

    // ── Surface popover ───────────────────────────────────────────
    if spec.current_open() && !spec.is_disabled {
        // Contract: width 24rem, padding 0.75rem, gap 0.625rem (stack.md ≈),
        // border-subtle, radius-surface, elevated bg.
        let surface_pad = rem_to_px(0.75);
        let mut surface = ui_element::div()
            .id("color-picker-surface")
            .w(rem_to_px(24.0))
            .rounded(surface_radius)
            .bg(elevated_bg)
            .border(1.0)
            .border_color(border_subtle)
            .p(surface_pad)
            .flex_col()
            .gap(surface_gap);

        // ── Picker area: gradient pad (left) + controls (right) ───
        let gradient_pad = build_gradient_pad(theme, hsv, current_color, radius_control);
        let controls_panel =
            build_controls_panel(spec, theme, &current, rgb, hsv, alpha, current_color);

        let picker_area = ui_element::div()
            .flex_row()
            .gap(rem_to_px(0.625))
            .items_start()
            .child(gradient_pad)
            .child(controls_panel);

        surface = surface.child(picker_area);

        // ── Swatch grid (opt-in) ──────────────────────────────────
        if !spec.swatches.is_empty() {
            surface = surface.child(build_swatch_grid(
                theme,
                &spec.swatches,
                &current,
                text_primary,
                border_subtle,
            ));
        }

        root = root.child(surface);
    }

    // ── Disabled state ────────────────────────────────────────────
    if spec.is_disabled {
        root = root.opacity(disabled_opacity).disabled(true);
    }

    root
}

/// 2D saturation/value gradient pad. Base = pure hue `hsl(h,100%,50%)`. Two
/// absolutely positioned overlay children carry the CSS `::before`/`::after`
/// gradients (white→transparent left→right, transparent→black top→bottom). A
/// thumb ring sits at the current S/V.
fn build_gradient_pad(
    theme: &JetstreamThemeProvider,
    hsv: Hsv,
    current_color: Vec4,
    radius_control: f32,
) -> JsEl {
    let pad_size = rem_to_px(10.0);
    let thumb_d = rem_to_px(0.875);
    let pure_hue: Color = pure_hue_vec4(hsv.h).into();

    let white = Color::WHITE;
    let white_t = Color::WHITE.with_alpha(0.0);
    let black = Color::BLACK;
    let black_t = Color::BLACK.with_alpha(0.0);

    // Thumb position within the pad (s → x, v inverted → y), centered.
    let thumb_x = pad_size * (hsv.s / 100.0) - thumb_d / 2.0;
    let thumb_y = pad_size * (1.0 - hsv.v / 100.0) - thumb_d / 2.0;

    // White → transparent, left to right (CSS `to right` == 90deg).
    let before = ui_element::div()
        .absolute()
        .inset_0()
        .bg_gradient_linear(90.0, vec![(white, 0.0), (white_t, 1.0)]);

    // Transparent → black, top to bottom (CSS `to bottom` == 0deg here).
    let after = ui_element::div()
        .absolute()
        .inset_0()
        .bg_gradient_linear(0.0, vec![(black_t, 0.0), (black, 1.0)]);

    // Thumb ring at current S/V.
    let thumb = ui_element::div()
        .absolute()
        .left(thumb_x)
        .top(thumb_y)
        .w(thumb_d)
        .h(thumb_d)
        .rounded(thumb_d / 2.0)
        .border(2.0)
        .border_color(white)
        .bg(current_color);

    let _ = theme; // chrome here is contract-fixed rem (no dedicated tokens)

    ui_element::div()
        .id("color-picker-gradient")
        .relative()
        .flex_shrink_0()
        .w(pad_size)
        .h(pad_size)
        .rounded(radius_control)
        .overflow_hidden()
        .bg(pure_hue)
        .child(before)
        .child(after)
        .child(thumb)
}

/// Controls panel (right of the gradient): hue slider, optional alpha slider,
/// mode toggle (SegmentedControl), channel inputs.
fn build_controls_panel(
    spec: &ColorPickerSpec,
    theme: &JetstreamThemeProvider,
    current: &str,
    rgb: Rgb255,
    hsv: Hsv,
    alpha: f32,
    current_color: Vec4,
) -> JsEl {
    let surface_bg = resolve_color(theme, "color.background.surface");

    let mut panel = ui_element::div()
        .flex_1()
        .flex_col()
        .gap(rem_to_px(0.5))
        .min_w_0();

    // Hue slider — full 7-stop rainbow track + thumb at current hue.
    panel = panel.child(build_hue_strip(theme, hsv.h));

    // Alpha slider (opt-in) — checkerboard stand-in + color overlay + thumb.
    if spec.show_alpha {
        panel = panel.child(build_alpha_strip(theme, alpha, current_color, surface_bg));
    }

    // Mode toggle (SegmentedControl: Hex / RGB / HSL).
    let mode_value = match spec.default_mode {
        ColorInputMode::Hex => "hex",
        ColorInputMode::Rgb => "rgb",
        ColorInputMode::Hsl => "hsl",
    };
    let mode_spec = SegmentedControlSpec::new(vec![
        ChoiceOption::new("hex", "Hex"),
        ChoiceOption::new("rgb", "RGB"),
        ChoiceOption::new("hsl", "HSL"),
    ])
    .with_default_value(mode_value)
    .with_size(spec.size)
    .with_density(spec.density);
    panel = panel.child(js_segmented_control(&mode_spec, theme));

    // Channel inputs for the current mode.
    panel = panel.child(build_channel_inputs(
        spec, theme, current, rgb, hsv, alpha,
    ));

    panel
}

/// Hue slider with a full rainbow track. `JsEl` gradients allow arbitrary
/// stops, so the seven CSS stops render as a single linear gradient. A thumb
/// sits at the current hue.
fn build_hue_strip(theme: &JetstreamThemeProvider, hue: f32) -> JsEl {
    let track_h = rem_to_px(0.375);
    let thumb_d = resolve_px(theme, "size.icon.md");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");

    // Seven CSS rainbow stops (#f00 0%, #ff0 17%, #0f0 33%, #0ff 50%,
    // #00f 67%, #f0f 83%, #f00 100%).
    let stops: Vec<(Color, f32)> = vec![
        (Color::from_u8(255, 0, 0, 255), 0.0),
        (Color::from_u8(255, 255, 0, 255), 0.17),
        (Color::from_u8(0, 255, 0, 255), 0.33),
        (Color::from_u8(0, 255, 255, 255), 0.50),
        (Color::from_u8(0, 0, 255, 255), 0.67),
        (Color::from_u8(255, 0, 255, 255), 0.83),
        (Color::from_u8(255, 0, 0, 255), 1.0),
    ];

    let track = ui_element::div()
        .id("color-picker-hue")
        .w_full()
        .h(track_h)
        .rounded(track_h / 2.0)
        .overflow_hidden()
        .bg_gradient_linear(90.0, stops);

    slider_wrap(track, thumb_d, track_h, (hue / 360.0).clamp(0.0, 1.0), elevated, border)
}

/// Alpha slider. CSS layers a transparent→color gradient over a checkerboard;
/// Jetstream has no repeating-conic-gradient, so the checkerboard is a neutral
/// surface base with a transparent→color overlay. A thumb sits at current alpha.
fn build_alpha_strip(
    theme: &JetstreamThemeProvider,
    alpha: f32,
    color: Vec4,
    surface_bg: Vec4,
) -> JsEl {
    let track_h = rem_to_px(0.375);
    let thumb_d = resolve_px(theme, "size.icon.md");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");

    let color: Color = color.into();
    let opaque = color.with_alpha(1.0);
    let transparent = color.with_alpha(0.0);

    let overlay = ui_element::div()
        .absolute()
        .inset_0()
        .bg_gradient_linear(90.0, vec![(transparent, 0.0), (opaque, 1.0)]);

    let track = ui_element::div()
        .id("color-picker-alpha")
        .relative()
        .w_full()
        .h(track_h)
        .rounded(track_h / 2.0)
        .overflow_hidden()
        // Neutral checkerboard stand-in.
        .bg(surface_bg)
        .child(overlay);

    slider_wrap(track, thumb_d, track_h, alpha.clamp(0.0, 1.0), elevated, border)
}

/// Wrap a slider track in a relative container with a thumb at `progress`.
fn slider_wrap(
    track: JsEl,
    thumb_d: f32,
    track_h: f32,
    progress: f32,
    thumb_fill: Vec4,
    thumb_border: Vec4,
) -> JsEl {
    // Center the thumb vertically on the track; horizontal position is a
    // fraction of (track width − thumb diameter). Track is full-width; the
    // wrap reserves its measured width via layout.
    let thumb_top = -(thumb_d - track_h) / 2.0;
    let thumb = ui_element::div()
        .absolute()
        .top(thumb_top)
        .left(progress * rem_to_px(10.0) - thumb_d / 2.0)
        .w(thumb_d)
        .h(thumb_d)
        .rounded(thumb_d / 2.0)
        .bg(thumb_fill)
        .border(1.0)
        .border_color(thumb_border);

    ui_element::div().relative().w_full().child(track).child(thumb)
}

/// Channel inputs row for the current mode (hex / RGB / HSL). Each is a
/// stacked field: control over an uppercase label, matching Svelte. The
/// optional alpha channel appends when `show_alpha`.
fn build_channel_inputs(
    spec: &ColorPickerSpec,
    theme: &JetstreamThemeProvider,
    current: &str,
    rgb: Rgb255,
    hsv: Hsv,
    alpha: f32,
) -> JsEl {
    let surface_bg = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, spec.border_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let radius_control = resolve_radius(theme, "radius.control");
    let label_size = rem_to_px(0.625); // contract: input-label font-size

    let mut row = ui_element::div().flex_row().gap(rem_to_px(0.25)).items_start();

    let labelled = |child: JsEl, label: &str| -> JsEl {
        ui_element::div()
            .flex_1()
            .flex_col()
            .gap(rem_to_px(0.125))
            .min_w_0()
            .child(child)
            .child(
                ui_element::label(label.to_uppercase())
                    .text_size(label_size)
                    .text_color(text_secondary),
            )
    };

    let number = |id: &str, value: f64, min: f64, max: f64, aria: &str| -> JsEl {
        let n = NumberInputSpec::new(value)
            .with_min(min)
            .with_max(max)
            .with_step(1.0)
            .with_aria_label(aria)
            .with_size(spec.size)
            .with_density(spec.density);
        js_number_input(&n, theme).id(id.to_string())
    };

    match spec.default_mode {
        ColorInputMode::Hex => {
            // Hex: a code-font text field (height 2rem) showing the value.
            let hex_field = ui_element::div()
                .id("color-picker-hex-input")
                .w_full()
                .h(rem_to_px(2.0))
                .pl(rem_to_px(0.375))
                .pr(rem_to_px(0.375))
                .rounded(radius_control)
                .bg(surface_bg)
                .border(1.0)
                .border_color(border)
                .flex_row()
                .items_center()
                .child(
                    ui_element::label(current.to_string())
                        .text_size(rem_to_px(0.75))
                        .text_color(text_primary),
                );
            row = row.child(labelled(hex_field, "Hex"));
            if spec.show_alpha {
                row = row.child(labelled(
                    number("color-picker-a", (alpha * 100.0).round() as f64, 0.0, 100.0, "Alpha"),
                    "A",
                ));
            }
        }
        ColorInputMode::Rgb => {
            row = row.child(labelled(
                number("color-picker-r", rgb.r as f64, 0.0, 255.0, "Red"),
                "R",
            ));
            row = row.child(labelled(
                number("color-picker-g", rgb.g as f64, 0.0, 255.0, "Green"),
                "G",
            ));
            row = row.child(labelled(
                number("color-picker-b", rgb.b as f64, 0.0, 255.0, "Blue"),
                "B",
            ));
            if spec.show_alpha {
                row = row.child(labelled(
                    number("color-picker-a", (alpha * 100.0).round() as f64, 0.0, 100.0, "Alpha"),
                    "A",
                ));
            }
        }
        ColorInputMode::Hsl => {
            let hsl = hsv_to_hsl(hsv.h, hsv.s, hsv.v);
            row = row.child(labelled(
                number("color-picker-h", hsl.h as f64, 0.0, 360.0, "Hue"),
                "H",
            ));
            row = row.child(labelled(
                number("color-picker-s", hsl.s as f64, 0.0, 100.0, "Saturation"),
                "S",
            ));
            row = row.child(labelled(
                number("color-picker-l", hsl.l as f64, 0.0, 100.0, "Lightness"),
                "L",
            ));
            if spec.show_alpha {
                row = row.child(labelled(
                    number("color-picker-a", (alpha * 100.0).round() as f64, 0.0, 100.0, "Alpha"),
                    "A",
                ));
            }
        }
    }

    row
}

/// Preset swatch grid. Each swatch is a 1.25rem square at its hex color; the
/// active swatch (matching the current value) gets a text-primary border, the
/// rest a transparent border. Top divider = border-subtle@42%.
fn build_swatch_grid(
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
