//! ColorPicker — hue/alpha strip and channel-input builders.
//!
//! Split out of `color_picker/mod.rs` (god-file decomposition). These are
//! pure element builders (no `ColorPicker` self access); behavior unchanged.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ColorInputMode, ControlDensity, ControlSize, NumberInputSpec,
};

use crate::presentation::rem_to_px;
use crate::theme_ext::{
    hsv_to_hsl, resolve_color, resolve_px, rgb255_to_hsla, Hsv,
};
use crate::NumberInput;

pub(super) fn hue_strip(
    theme: &GpuiThemeProvider,
    id_base: &str,
    hue: f32,
    size: ControlSize,
    density: ControlDensity,
) -> AnyElement {
    let track_h = px(rem_to_px(0.375));
    let thumb_d = theme.resolve_space("size.icon.md");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");

    // Six rainbow stops at the segment boundaries.
    let stops = [
        hsla_from_rgb(255, 0, 0),   // red
        hsla_from_rgb(255, 255, 0), // yellow
        hsla_from_rgb(0, 255, 0),   // green
        hsla_from_rgb(0, 255, 255), // cyan
        hsla_from_rgb(0, 0, 255),   // blue
        hsla_from_rgb(255, 0, 255), // magenta
        hsla_from_rgb(255, 0, 0),   // red (wrap)
    ];

    let mut track = div()
        .id(SharedString::from(format!("{}-hue", id_base)))
        .relative()
        .w_full()
        .h(track_h)
        .rounded_full()
        .overflow_hidden()
        .flex();

    for i in 0..6 {
        track = track.child(
            div().flex_1().h_full().bg(gpui::linear_gradient(
                90.0,
                gpui::linear_color_stop(stops[i], 0.0),
                gpui::linear_color_stop(stops[i + 1], 1.0),
            )),
        );
    }

    let progress = (hue / 360.0).clamp(0.0, 1.0);
    let thumb = div()
        .absolute()
        .top(px(-(thumb_d - rem_to_px(0.375)) / 2.0))
        .left(relative(progress))
        .ml(px(-(thumb_d / 2.0)))
        .w(px(thumb_d))
        .h(px(thumb_d))
        .rounded_full()
        .bg(elevated)
        .border_1()
        .border_color(border)
        .shadow(vec![gpui::BoxShadow {
            color: hsla(0.0, 0.0, 0.0, 0.18),
            offset: point(px(0.0), px(2.0)),
            blur_radius: px(8.0),
            spread_radius: px(0.0),
        }]);

    let _ = (size, density);
    div()
        .relative()
        .w_full()
        .child(track)
        .child(thumb)
        .into_any_element()
}

/// Alpha slider. CSS layers a transparent→color gradient over a checkerboard;
/// GPUI 0.2.2 has no repeating-conic-gradient, so the checkerboard is
/// approximated by a neutral surface base with a transparent→color overlay.
/// A thumb sits at the current alpha.
#[allow(clippy::too_many_arguments)]

pub(super) fn alpha_strip(
    theme: &GpuiThemeProvider,
    id_base: &str,
    alpha: f32,
    color: Hsla,
    surface_bg: Hsla,
    size: ControlSize,
    density: ControlDensity,
) -> AnyElement {
    let track_h = px(rem_to_px(0.375));
    let thumb_d = theme.resolve_space("size.icon.md");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");

    let opaque = Hsla { a: 1.0, ..color };
    let transparent = Hsla { a: 0.0, ..color };

    let track = div()
        .id(SharedString::from(format!("{}-alpha", id_base)))
        .relative()
        .w_full()
        .h(track_h)
        .rounded_full()
        .overflow_hidden()
        // Neutral checkerboard stand-in.
        .bg(surface_bg)
        .child(
            div().absolute().inset_0().bg(gpui::linear_gradient(
                90.0,
                gpui::linear_color_stop(transparent, 0.0),
                gpui::linear_color_stop(opaque, 1.0),
            )),
        );

    let progress = alpha.clamp(0.0, 1.0);
    let thumb = div()
        .absolute()
        .top(px(-(thumb_d - rem_to_px(0.375)) / 2.0))
        .left(relative(progress))
        .ml(px(-(thumb_d / 2.0)))
        .w(px(thumb_d))
        .h(px(thumb_d))
        .rounded_full()
        .bg(elevated)
        .border_1()
        .border_color(border)
        .shadow(vec![gpui::BoxShadow {
            color: hsla(0.0, 0.0, 0.0, 0.18),
            offset: point(px(0.0), px(2.0)),
            blur_radius: px(8.0),
            spread_radius: px(0.0),
        }]);

    let _ = (size, density);
    div()
        .relative()
        .w_full()
        .child(track)
        .child(thumb)
        .into_any_element()
}

/// Channel inputs row for the current mode (hex / RGB / HSL), each a labelled
/// NumberInput (or hex text field) showing the current value. The optional
/// alpha channel appends when `show_alpha`.
#[allow(clippy::too_many_arguments)]

pub(super) fn channel_inputs(
    theme: &GpuiThemeProvider,
    id_base: &str,
    mode: ColorInputMode,
    current_hex: &str,
    rgb: crate::theme_ext::Rgb255,
    hsv: Hsv,
    alpha: f32,
    show_alpha: bool,
    size: ControlSize,
    density: ControlDensity,
    surface_bg: Hsla,
    border: Hsla,
    text_primary: Hsla,
    text_secondary: Hsla,
    radius_control: Pixels,
    label_size: Pixels,
) -> AnyElement {
    let gap = resolve_px(theme, "space.inline.xs");
    let mut row = div().flex().gap(gap).items_start();

    // Stacked field: control over an uppercase label, matching Svelte.
    let labelled = |child: AnyElement, label: &str| -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap(px(rem_to_px(0.125)))
            .min_w_0()
            .child(child)
            .child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(label.to_string().to_uppercase()),
            )
    };

    let number = |id: &str, value: f64, min: f64, max: f64, aria: &str| -> AnyElement {
        NumberInput::from_spec(
            NumberInputSpec::new(value)
                .with_min(min)
                .with_max(max)
                .with_step(1.0)
                .with_aria_label(aria),
            theme,
        )
        .with_id(id.to_string())
        .size(size)
        .density(density)
        .into_any_element()
    };

    match mode {
        ColorInputMode::Hex => {
            // Hex: a code-font text field showing the current value.
            let hex_field = div()
                .id(SharedString::from(format!("{}-hex-input", id_base)))
                .w_full()
                .h(px(rem_to_px(2.0)))
                .px(px(rem_to_px(0.375)))
                .rounded(radius_control)
                .bg(surface_bg)
                .border_1()
                .border_color(border)
                .flex()
                .items_center()
                .text_size(label_size)
                .text_color(text_primary)
                .font_family("monospace")
                .child(current_hex.to_string())
                .into_any_element();
            row = row.child(labelled(hex_field, "Hex"));
            if show_alpha {
                row = row.child(labelled(
                    number(
                        &format!("{}-a", id_base),
                        (alpha * 100.0).round() as f64,
                        0.0,
                        100.0,
                        "Alpha",
                    ),
                    "A",
                ));
            }
        }
        ColorInputMode::Rgb => {
            row = row.child(labelled(
                number(&format!("{}-r", id_base), rgb.r as f64, 0.0, 255.0, "Red"),
                "R",
            ));
            row = row.child(labelled(
                number(&format!("{}-g", id_base), rgb.g as f64, 0.0, 255.0, "Green"),
                "G",
            ));
            row = row.child(labelled(
                number(&format!("{}-b", id_base), rgb.b as f64, 0.0, 255.0, "Blue"),
                "B",
            ));
            if show_alpha {
                row = row.child(labelled(
                    number(
                        &format!("{}-a", id_base),
                        (alpha * 100.0).round() as f64,
                        0.0,
                        100.0,
                        "Alpha",
                    ),
                    "A",
                ));
            }
        }
        ColorInputMode::Hsl => {
            let hsl = hsv_to_hsl(hsv.h, hsv.s, hsv.v);
            row = row.child(labelled(
                number(&format!("{}-h", id_base), hsl.h as f64, 0.0, 360.0, "Hue"),
                "H",
            ));
            row = row.child(labelled(
                number(
                    &format!("{}-s", id_base),
                    hsl.s as f64,
                    0.0,
                    100.0,
                    "Saturation",
                ),
                "S",
            ));
            row = row.child(labelled(
                number(
                    &format!("{}-l", id_base),
                    hsl.l as f64,
                    0.0,
                    100.0,
                    "Lightness",
                ),
                "L",
            ));
            if show_alpha {
                row = row.child(labelled(
                    number(
                        &format!("{}-a", id_base),
                        (alpha * 100.0).round() as f64,
                        0.0,
                        100.0,
                        "Alpha",
                    ),
                    "A",
                ));
            }
        }
    }

    row.into_any_element()
}

/// Opaque `gpui::Hsla` from 8-bit RGB. Local convenience for rainbow stops.

fn hsla_from_rgb(r: u8, g: u8, b: u8) -> Hsla {
    rgb255_to_hsla(
        crate::theme_ext::Rgb255 {
            r,
            g,
            b,
            a: 1.0,
        },
        1.0,
    )
}

