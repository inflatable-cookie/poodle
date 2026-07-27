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
use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ColorPickerSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{
    hex_to_rgb255, resolve_color, resolve_opacity, resolve_px,
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
                    // Contract §8 `.color-picker__input`: code-family hex value.
                    ui_element::label(input_display)
                        .text_color(input_color)
                        .text_size(font_size)
                        .font_family(FontFamily::Mono),
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

    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
}

mod parts;
mod parts2;
use parts::{build_controls_panel, build_gradient_pad};
use parts2::build_swatch_grid;
