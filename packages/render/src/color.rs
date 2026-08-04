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
