//! Theme extension utilities for bridging poodle token values to gpui types.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

/// Resolve a color token string through the theme and convert to Hsla.
pub fn resolve_color(theme: &GpuiThemeProvider, token: &str) -> Hsla {
    let cv = theme.resolve_color(token);
    let rgba = gpui::Rgba {
        r: cv.0,
        g: cv.1,
        b: cv.2,
        a: cv.3,
    };
    rgba.into()
}

/// Resolve a space/size token string through the theme to Pixels.
pub fn resolve_px(theme: &GpuiThemeProvider, token: &str) -> Pixels {
    px(theme.resolve_space(token))
}

/// Resolve a radius token through the theme to Pixels.
pub fn resolve_radius(theme: &GpuiThemeProvider, token: &str) -> Pixels {
    px(theme.resolve_radius(token))
}

/// Resolve an opacity token through the theme to f32.
pub fn resolve_opacity(theme: &GpuiThemeProvider, token: &str) -> f32 {
    theme.resolve_opacity(token)
}

/// Mix two colors in sRGB space, matching CSS `color-mix(in srgb, a ratio%, b)`.
///
/// `ratio` is the proportion of `a` to keep (0.0–1.0).
/// For example, `color_mix(fill, elevated, 0.84)` = 84% fill + 16% elevated,
/// matching `color-mix(in srgb, fill 84%, elevated)`.
pub fn color_mix(a: Hsla, b: Hsla, ratio: f32) -> Hsla {
    // Convert to Rgba for linear interpolation in sRGB space
    let a_rgba: gpui::Rgba = a.into();
    let b_rgba: gpui::Rgba = b.into();
    let mixed = gpui::Rgba {
        r: a_rgba.r * ratio + b_rgba.r * (1.0 - ratio),
        g: a_rgba.g * ratio + b_rgba.g * (1.0 - ratio),
        b: a_rgba.b * ratio + b_rgba.b * (1.0 - ratio),
        a: a_rgba.a * ratio + b_rgba.a * (1.0 - ratio),
    };
    mixed.into()
}

/// Mix a color with black at the given ratio. Useful for darkened borders.
/// `color_mix_black(color, 0.84)` = 84% color + 16% black.
pub fn color_mix_black(color: Hsla, ratio: f32) -> Hsla {
    let rgba: gpui::Rgba = color.into();
    let mixed = gpui::Rgba {
        r: rgba.r * ratio,
        g: rgba.g * ratio,
        b: rgba.b * ratio,
        a: rgba.a,
    };
    mixed.into()
}

/// Contract §8 button hover/active shadow (secondary `--poodle-button-shadow`):
/// `inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent)`. GPUI has no
/// CSS `white` keyword/token, so the contract-literal white is expressed via
/// `gpui::white()`; the offset uses `rem_to_px(0.0625)` rather than a raw px.
pub fn button_hover_shadow() -> Vec<gpui::BoxShadow> {
    let white = gpui::white();
    let inset_y = px(crate::presentation::rem_to_px(0.0625));
    vec![gpui::BoxShadow {
        // color-mix(white 8%, transparent) → white at 8% alpha
        color: Hsla { a: 0.08, ..white },
        offset: point(px(0.0), inset_y),
        blur_radius: px(0.0),
        spread_radius: px(0.0),
    }]
}

/// Build the standard Svelte focus ring shadow: 0 0 0 2px focusRing@28%.
/// Use with `.focus(move |s| s.border_color(fr).shadow(focus_ring_shadow(fr)))`.
pub fn focus_ring_shadow(focus_ring_color: Hsla) -> Vec<gpui::BoxShadow> {
    vec![gpui::BoxShadow {
        color: Hsla {
            a: focus_ring_color.a * 0.28,
            ..focus_ring_color
        },
        offset: gpui::point(gpui::px(0.0), gpui::px(0.0)),
        blur_radius: gpui::px(0.0),
        spread_radius: gpui::px(2.0),
    }]
}

// ── Elevation shadows ─────────────────────────────────────────────
//
// Token-resolved drop shadows for elevated surfaces. These come from the
// typed semantic token table (`poodle_tokens::typed::semantic`), so the
// offset/blur/color are never hardcoded in component code.

/// Convert a typed `ShadowValue` into a single `gpui::BoxShadow`.
fn shadow_value_to_box(s: &poodle_tokens::typed::ShadowValue) -> gpui::BoxShadow {
    let rgba = gpui::Rgba {
        r: s.color.0,
        g: s.color.1,
        b: s.color.2,
        a: s.color.3,
    };
    gpui::BoxShadow {
        color: rgba.into(),
        offset: gpui::point(gpui::px(s.offset_x), gpui::px(s.offset_y)),
        blur_radius: gpui::px(s.blur),
        spread_radius: gpui::px(0.0),
    }
}

/// Token-resolved `elevation.surface` shadow — low-elevation cards/panels.
pub fn elevation_surface_shadow() -> Vec<gpui::BoxShadow> {
    vec![shadow_value_to_box(
        &poodle_tokens::typed::semantic::ELEVATION_SURFACE,
    )]
}

/// Token-resolved `elevation.overlay` shadow — popovers, menus, dropdowns.
pub fn elevation_overlay_shadow() -> Vec<gpui::BoxShadow> {
    vec![shadow_value_to_box(
        &poodle_tokens::typed::semantic::ELEVATION_OVERLAY,
    )]
}

/// Token-resolved `elevation.dialog` shadow — modal dialogs and drawers.
pub fn elevation_dialog_shadow() -> Vec<gpui::BoxShadow> {
    vec![shadow_value_to_box(
        &poodle_tokens::typed::semantic::ELEVATION_DIALOG,
    )]
}

/// Parse a CSS hex color string (#rrggbb or #rgb) to Hsla.
pub fn parse_hex_color(hex: &str) -> Option<Hsla> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let (r, g, b) = match hex.len() {
        6 => (
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
        ),
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            (r * 17, g * 17, b * 17)
        }
        _ => return None,
    };
    let rgba = gpui::Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    };
    Some(rgba.into())
}

// ── Brand-raised gradient helpers ────────────────────────────────



// ── Color-picker color math ───────────────────────────────────────
//
// The color-picker resolves its chrome from tokens, but the gradient/
// swatch/thumb COLORS are computed from the picker's own value — the one
// legitimate non-token color source (per the component contract). These
// helpers mirror the Svelte `color-utils.ts` conversions (HSV model) so
// the Rust render reflects the actual selected color.

/// HSV color components. `h` in 0–360, `s`/`v` in 0–100, `a` in 0–1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

/// RGB channels in 0–255 (rounded) plus alpha 0–1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb255 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

/// HSL color components. `h` in 0–360, `s`/`l` in 0–100.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
    pub h: u16,
    pub s: u8,
    pub l: u8,
}

/// Parse a hex string (#rgb / #rrggbb / #rrggbbaa) into RGB + alpha.
/// Returns `None` for malformed input. Delegates to the poodle-headless hex
/// codec (conformance-tested against the TS core).
pub fn hex_to_rgb255(hex: &str) -> Option<Rgb255> {
    if !poodle_headless::color::is_valid_hex(hex) && !poodle_headless::color::is_valid_hex(&format!("#{hex}")) {
        return None;
    }

    let (rgb, alpha) = poodle_headless::color::hex_to_rgb(hex)?;

    Some(Rgb255 {
        r: rgb.r,
        g: rgb.g,
        b: rgb.b,
        a: alpha.unwrap_or(1.0) as f32,
    })
}

/// Convert RGB (0–255) to HSV.
pub fn rgb_to_hsv(rgb: Rgb255) -> Hsv {
    let r = rgb.r as f32 / 255.0;
    let g = rgb.g as f32 / 255.0;
    let b = rgb.b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;

    let mut h = 0.0;
    if d > 0.0 {
        h = if max == r {
            ((g - b) / d).rem_euclid(6.0)
        } else if max == g {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        } * 60.0;
    }
    if h < 0.0 {
        h += 360.0;
    }
    let s = if max == 0.0 { 0.0 } else { d / max };
    Hsv {
        h: h.round(),
        s: (s * 100.0).round(),
        v: (max * 100.0).round(),
        a: rgb.a,
    }
}

/// Convert HSV to RGB (0–255). `h` 0–360, `s`/`v` 0–100.
pub fn hsv_to_rgb255(h: f32, s: f32, v: f32, a: f32) -> Rgb255 {
    let s = s / 100.0;
    let v = v / 100.0;
    let c = v * s;
    let hp = (h / 60.0).rem_euclid(6.0);
    let x = c * (1.0 - (hp.rem_euclid(2.0) - 1.0).abs());
    let (r1, g1, b1) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    Rgb255 {
        r: (((r1 + m) * 255.0).round()).clamp(0.0, 255.0) as u8,
        g: (((g1 + m) * 255.0).round()).clamp(0.0, 255.0) as u8,
        b: (((b1 + m) * 255.0).round()).clamp(0.0, 255.0) as u8,
        a,
    }
}

/// Convert HSV to HSL channels (for the HSL input-mode display).
pub fn hsv_to_hsl(h: f32, s: f32, v: f32) -> Hsl {
    let s = s / 100.0;
    let v = v / 100.0;
    let l = v * (1.0 - s / 2.0);
    let sl = if l == 0.0 || l == 1.0 {
        0.0
    } else {
        (v - l) / l.min(1.0 - l)
    };
    Hsl {
        h: h.round() as u16,
        s: (sl * 100.0).round() as u8,
        l: (l * 100.0).round() as u8,
    }
}

/// Convert RGB (0–255) into a `gpui::Hsla`, applying the given alpha.
pub fn rgb255_to_hsla(rgb: Rgb255, alpha: f32) -> Hsla {
    let rgba = gpui::Rgba {
        r: rgb.r as f32 / 255.0,
        g: rgb.g as f32 / 255.0,
        b: rgb.b as f32 / 255.0,
        a: alpha,
    };
    rgba.into()
}

/// The pure-hue base color `hsl(h, 100%, 50%)` used as the gradient-pad
/// background. Returns an opaque `gpui::Hsla`.
pub fn pure_hue_hsla(h: f32) -> Hsla {
    // hsl(h,100%,50%) == hsv(h,100,100) in RGB terms.
    rgb255_to_hsla(hsv_to_rgb255(h, 100.0, 100.0, 1.0), 1.0)
}

/// Format RGB (0–255) + alpha into a normalized hex string.
/// 6-digit when alpha is 1.0, 8-digit otherwise.
pub fn rgb255_to_hex(rgb: Rgb255) -> String {
    let alpha = if (rgb.a - 1.0).abs() < f32::EPSILON { None } else { Some(f64::from(rgb.a)) };

    poodle_headless::color::rgb_to_hex(
        poodle_headless::color::Rgb { r: rgb.r, g: rgb.g, b: rgb.b },
        alpha,
    )
}
