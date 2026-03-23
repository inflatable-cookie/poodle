//! Theme bridge — resolves Poodle semantic tokens to Vec4 colors in linear space.
//!
//! Used by main.rs for the draw theme and clear color. Component code
//! uses `poodle_jetstream_components::theme_ext` instead.

use glam::Vec4;
use poodle_adapter::ThemeProvider;

/// Convert a single sRGB component to linear light.
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Resolve a Poodle semantic token path to a Vec4 color in **linear** space.
pub fn resolve_vec4(theme: &dyn ThemeProvider, token: &str) -> Vec4 {
    let c = theme.resolve_color(token);
    Vec4::new(
        srgb_to_linear(c.0),
        srgb_to_linear(c.1),
        srgb_to_linear(c.2),
        c.3,
    )
}

pub fn canvas_background(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.background.canvas")
}
