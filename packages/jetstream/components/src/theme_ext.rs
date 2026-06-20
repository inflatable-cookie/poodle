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
