//! Color conversion between Poodle tokens and GPUI.

use gpui::Hsla;
use poodle_tokens::typed::ColorValue;

/// Convert a Poodle `ColorValue` (RGBA f32) to gpui's `Hsla`.
pub fn color_to_hsla(c: ColorValue) -> Hsla {
    let rgba = gpui::Rgba {
        r: c.0,
        g: c.1,
        b: c.2,
        a: c.3,
    };
    rgba.into()
}

/// Convert a gpui `Hsla` back to a Poodle `ColorValue` (RGBA f32).
///
/// The inverse of [`color_to_hsla`], for specimen content authored in HSL that
/// has to reach the node vocabulary (which is sRGB).
pub fn hsla_to_color_value(c: Hsla) -> ColorValue {
    let rgba: gpui::Rgba = c.into();
    ColorValue(rgba.r, rgba.g, rgba.b, rgba.a)
}
