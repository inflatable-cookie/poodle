//! color_picker — helper builders. Split out of `color_picker/mod.rs` (god-file
//! decomposition); unchanged.

use glam::Vec4;
use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ChoiceOption, ColorInputMode, ColorPickerSpec, NumberInputSpec, SegmentedControlSpec,
};

use crate::number_input::js_number_input;
use crate::presentation::rem_to_px;
use crate::segmented_control::js_segmented_control;
use crate::theme_ext::{
    hsv_to_hsl, pure_hue_vec4, resolve_color, resolve_px,
    resolve_radius, Hsv, Rgb255,
};


/// 2D saturation/value gradient pad. Base = pure hue `hsl(h,100%,50%)`. Two
/// absolutely positioned overlay children carry the CSS `::before`/`::after`
/// gradients (white→transparent left→right, transparent→black top→bottom). A
/// thumb ring sits at the current S/V.
pub(super) fn build_gradient_pad(
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
pub(super) fn build_controls_panel(
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
pub(super) fn build_hue_strip(theme: &JetstreamThemeProvider, hue: f32) -> JsEl {
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

    // A gradient packs only its first + last stop, so render the multi-stop rainbow
    // as one 2-stop segment per adjacent pair, each flex-grown to its stop spacing.
    // Visually identical to a single 7-stop gradient; overflow_hidden + rounded clip
    // the row to the track shape.
    let mut track = ui_element::div()
        .id("color-picker-hue")
        .w_full()
        .h(track_h)
        .rounded(track_h / 2.0)
        .overflow_hidden()
        .flex_row();
    for pair in stops.windows(2) {
        let (c0, p0) = pair[0];
        let (c1, p1) = pair[1];
        let mut seg = ui_element::div()
            .h(track_h)
            .bg_gradient_linear(90.0, vec![(c0, 0.0), (c1, 1.0)]);
        seg.layout.flex_grow = (p1 - p0).max(0.0001);
        track = track.child(seg);
    }

    slider_wrap("Hue", track, thumb_d, track_h, (hue / 360.0).clamp(0.0, 1.0), elevated, border)
}

/// Alpha slider. CSS layers a transparent→color gradient over a checkerboard;
/// Jetstream has no repeating-conic-gradient, so the checkerboard is a neutral
/// surface base with a transparent→color overlay. A thumb sits at current alpha.
pub(super) fn build_alpha_strip(
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

    slider_wrap("Opacity", track, thumb_d, track_h, alpha.clamp(0.0, 1.0), elevated, border)
}

/// Wrap a slider track in a relative container with a thumb at `progress`.
pub(super) fn slider_wrap(
    // What this channel controls. A slider announced as "slider, 40%" says
    // nothing about which quantity moved, and this picker has several.
    channel: &str,
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

    // Contract: each channel track is a `slider` reporting its own value, not
    // a decorative bar with a dot on it.
    ui_element::div()
        .aria_role(jetstream_ui::accesskit::Role::Slider)
        .aria_label(channel)
        .aria_value(progress as f64, 0.0, 1.0)
        .relative()
        .w_full()
        .child(track)
        .child(thumb)
}

/// Channel inputs row for the current mode (hex / RGB / HSL). Each is a
/// stacked field: control over an uppercase label, matching Svelte. The
/// optional alpha channel appends when `show_alpha`.
pub(super) fn build_channel_inputs(
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
                    // Contract §8 `.color-picker__text-input`: code-family hex value.
                    ui_element::label(current.to_string())
                        .text_size(rem_to_px(0.75))
                        .text_color(text_primary)
                        .font_family(FontFamily::Mono),
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
