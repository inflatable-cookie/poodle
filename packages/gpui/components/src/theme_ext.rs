//! Theme extension utilities for bridging pug token values to gpui types.

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;

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
