//! Colour recipes components use to derive states from resolved tokens.
//!
//! Everything here operates in **sRGB**, the vocabulary's colour space. The
//! CSS `color-mix` behaviour both old tiers implemented was "lerp in sRGB":
//! they resolved tokens to linear first, so they had to encode, lerp, decode.
//! Node colours stay in sRGB until a backend converts at its own edge, so the
//! same recipe is a plain componentwise lerp — bit-equivalent, asserted by
//! the Jetstream adapter's draw-command parity suite.

use poodle_node::ColorValue;

/// sRGB-space mix: `fraction` weights `a`. Alpha lerps the same way.
pub fn mix_srgb(a: ColorValue, b: ColorValue, fraction: f32) -> ColorValue {
    let f = fraction.clamp(0.0, 1.0);
    let inv = 1.0 - f;
    ColorValue(
        a.0 * f + b.0 * inv,
        a.1 * f + b.1 * inv,
        a.2 * f + b.2 * inv,
        a.3 * f + b.3 * inv,
    )
}

/// Copy with the alpha channel replaced.
pub fn with_alpha(c: ColorValue, alpha: f32) -> ColorValue {
    ColorValue(c.0, c.1, c.2, alpha)
}

/// Parse `#rgb` / `#rrggbb` / `#rrggbbaa` into an sRGB [`ColorValue`].
///
/// Note: the old Jetstream tier fed hex overrides into its linear pipeline
/// *without* conversion while token colours were converted — custom-coloured
/// controls rendered brighter than intended. Here the hex lands in sRGB like
/// every other colour and converts at the backend edge, which fixes that. The
/// parity suite documents this as an intentional divergence.
pub fn hex_color(hex: &str) -> Option<ColorValue> {
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
    Some(ColorValue(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a,
    ))
}

/// Fully transparent.
pub const TRANSPARENT: ColorValue = ColorValue(0.0, 0.0, 0.0, 0.0);
/// Pure black, for the contract's darkening mixes.
pub const BLACK: ColorValue = ColorValue(0.0, 0.0, 0.0, 1.0);

/// Pure white, for the contract's lightening mixes.
pub const WHITE: ColorValue = ColorValue(1.0, 1.0, 1.0, 1.0);

fn to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn srgb_encode(c: f32) -> f32 {
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// Linear-space mix: `fraction` weights `a`. Decode → lerp → encode.
///
/// Distinct from [`mix_srgb`]: the old tier had BOTH recipes — `color_mix`
/// lerps in sRGB gamma (most state recipes), while `tabs::blend` lerps the
/// linear values directly (card/block selected fills). A first `mix_linear`
/// was deleted when nothing used it; tabs is the component that proved the
/// second recipe real.
pub fn mix_linear(a: ColorValue, b: ColorValue, fraction: f32) -> ColorValue {
    let f = fraction.clamp(0.0, 1.0);
    let inv = 1.0 - f;
    let mix_c = |x: f32, y: f32| srgb_encode(to_linear(x) * f + to_linear(y) * inv);
    ColorValue(
        mix_c(a.0, b.0),
        mix_c(a.1, b.1),
        mix_c(a.2, b.2),
        a.3 * f + b.3 * inv,
    )
}

// ── Picker color math (ColorPicker) ─────────────────────────────────────────
// Ported from the old tier's theme_ext; values stay sRGB like everything in
// the node vocabulary (backends convert at their edge).

/// HSV color components. `h` in 0–360, `s`/`v` in 0–100, `a` 0–1.
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

/// Convert RGB (0–255) into an sRGB `ColorValue`, applying the given alpha.
pub fn rgb255_to_color(rgb: Rgb255, alpha: f32) -> ColorValue {
    ColorValue(
        rgb.r as f32 / 255.0,
        rgb.g as f32 / 255.0,
        rgb.b as f32 / 255.0,
        alpha,
    )
}

/// The pure-hue base color `hsl(h, 100%, 50%)` used as the gradient-pad
/// background. Returns an opaque sRGB `ColorValue`.
pub fn pure_hue_color(h: f32) -> ColorValue {
    // hsl(h,100%,50%) == hsv(h,100,100) in RGB terms.
    rgb255_to_color(hsv_to_rgb255(h, 100.0, 100.0, 1.0), 1.0)
}
