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

/// Build the standard Svelte focus ring shadow: 0 0 0 2px focusRing@28%.
/// Use with `.focus(move |s| s.border_color(fr).shadow(focus_ring_shadow(fr)))`.
pub fn focus_ring_shadow(focus_ring_color: Hsla) -> Vec<gpui::BoxShadow> {
    vec![gpui::BoxShadow {
        color: Hsla { a: focus_ring_color.a * 0.28, ..focus_ring_color },
        offset: gpui::point(gpui::px(0.0), gpui::px(0.0)),
        blur_radius: gpui::px(0.0),
        spread_radius: gpui::px(2.0),
    }]
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
