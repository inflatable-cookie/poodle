//! Theme bridge — resolves Pug semantic tokens to Jetstream-compatible Vec4 colors.
//!
//! Converts from `JetstreamThemeProvider` (which resolves string token paths to
//! `ColorValue` tuples) to `glam::Vec4` values used by `game_ui::UiStyle`.
//!
//! **Color space**: Pug tokens are stored in sRGB gamma space (CSS hex values).
//! The Jetstream renderer uses an sRGB surface format, which auto-applies gamma
//! encoding on write. Therefore, colors must be converted from sRGB to linear
//! before passing to the GPU to avoid double-gamma (washed-out appearance).

use glam::Vec4;
use pug_adapter::ThemeProvider;

/// Convert a single sRGB component to linear light.
///
/// Uses the official sRGB transfer function (IEC 61966-2-1).
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Resolve a Pug semantic token path to a Vec4 color in **linear** space.
///
/// The Pug token system stores colors in sRGB gamma space (matching CSS).
/// This function converts RGB channels to linear for correct rendering on
/// an sRGB surface. Alpha is left unchanged.
pub fn resolve_vec4(theme: &dyn ThemeProvider, token: &str) -> Vec4 {
    let c = theme.resolve_color(token);
    Vec4::new(
        srgb_to_linear(c.0),
        srgb_to_linear(c.1),
        srgb_to_linear(c.2),
        c.3, // alpha is linear already
    )
}

// ── Common token shortcuts ──

pub fn panel_background(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.background.panel")
}

pub fn surface_background(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.background.surface")
}

pub fn canvas_background(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.background.canvas")
}

pub fn elevated_background(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.background.elevated")
}

pub fn text_primary(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.text.primary")
}

pub fn text_secondary(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.text.secondary")
}

pub fn text_inverse(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.text.inverse")
}

pub fn border_default(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.border.default")
}

pub fn border_subtle(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.border.subtle")
}

pub fn accent_base(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.accent.base")
}

pub fn accent_hover(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.accent.hover")
}

pub fn icon_primary(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.icon.primary")
}

pub fn icon_muted(theme: &dyn ThemeProvider) -> Vec4 {
    resolve_vec4(theme, "semantic.color.icon.muted")
}

/// Mix a color with transparency to create a tinted version.
/// This emulates CSS `color-mix(in srgb, color XX%, transparent)`.
pub fn tint(color: Vec4, opacity_fraction: f32) -> Vec4 {
    Vec4::new(color.x, color.y, color.z, color.w * opacity_fraction)
}
