//! Theme extension helpers — resolve Poodle tokens to Jetstream-compatible values.
//!
//! Mirrors `packages/gpui/components/src/theme_ext.rs` for API parity.

use glam::Vec4;
use poodle_jetstream::JetstreamThemeProvider;

/// Resolve a semantic color token to a linear-space Vec4.
pub fn resolve_color(theme: &JetstreamThemeProvider, token: &str) -> Vec4 {
    theme.resolve_linear_color(token)
}

/// Resolve a space/size token to logical pixels.
pub fn resolve_px(theme: &JetstreamThemeProvider, token: &str) -> f32 {
    theme.resolve_space_px(token)
}

/// Resolve a radius token to logical pixels.
pub fn resolve_radius(theme: &JetstreamThemeProvider, token: &str) -> f32 {
    theme.resolve_radius_px(token)
}

/// Resolve an opacity token to a float (0.0–1.0).
pub fn resolve_opacity(theme: &JetstreamThemeProvider, token: &str) -> f32 {
    theme.resolve_opacity_value(token)
}

/// Mix a color with transparency (emulates CSS `color-mix`).
pub fn tint(color: Vec4, opacity_fraction: f32) -> Vec4 {
    Vec4::new(color.x, color.y, color.z, color.w * opacity_fraction)
}

/// Blend two colors: `a * ratio + b * (1 - ratio)` (emulates CSS
/// `color-mix(a ratio%, b)`). Mixed in the linear space the Jetstream pipeline
/// uses; GPUI mixes in sRGB, a minor cross-target color delta.
pub fn color_mix(a: Vec4, b: Vec4, ratio: f32) -> Vec4 {
    a * ratio + b * (1.0 - ratio)
}

// ── Color-picker color math ───────────────────────────────────────
//
// The color-picker resolves its chrome from tokens, but the gradient/
// swatch/thumb COLORS are computed from the picker's own value — the one
// legitimate non-token color source (per the component contract). These
// helpers mirror the GPUI `theme_ext.rs` conversions (HSV model, which in
// turn mirror Svelte `color-utils.ts`) so the Jetstream render reflects the
// actual selected color. Colors are returned as `glam::Vec4` (the Jetstream
// `Color`/gradient stop type), with RGB channels held in sRGB 0–1.

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
/// Returns `None` for malformed input.
pub fn hex_to_rgb255(hex: &str) -> Option<Rgb255> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let (r, g, b, a) = match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
            (r * 17, g * 17, b * 17, 1.0)
        }
        6 => (
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
            1.0,
        ),
        8 => (
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
            u8::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0,
        ),
        _ => return None,
    };
    Some(Rgb255 { r, g, b, a })
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

/// Convert RGB (0–255) into a `glam::Vec4` color, applying the given alpha.
/// RGB channels are sRGB 0–1 (matching the Jetstream `Color` convention).
pub fn rgb255_to_vec4(rgb: Rgb255, alpha: f32) -> Vec4 {
    Vec4::new(
        rgb.r as f32 / 255.0,
        rgb.g as f32 / 255.0,
        rgb.b as f32 / 255.0,
        alpha,
    )
}

/// The pure-hue base color `hsl(h, 100%, 50%)` used as the gradient-pad
/// background. Returns an opaque `glam::Vec4`.
pub fn pure_hue_vec4(h: f32) -> Vec4 {
    // hsl(h,100%,50%) == hsv(h,100,100) in RGB terms.
    rgb255_to_vec4(hsv_to_rgb255(h, 100.0, 100.0, 1.0), 1.0)
}

/// Format RGB (0–255) + alpha into a normalized hex string.
/// 6-digit when alpha is 1.0, 8-digit otherwise.
pub fn rgb255_to_hex(rgb: Rgb255) -> String {
    if (rgb.a - 1.0).abs() < f32::EPSILON {
        format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
    } else {
        let a = (rgb.a * 255.0).round().clamp(0.0, 255.0) as u8;
        format!("#{:02x}{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b, a)
    }
}
