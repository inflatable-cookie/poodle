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
