//! Framework-side helpers for the JsEl chrome that wraps specimens (shell,
//! hero cards, snap demo scenes). Chrome renders directly through the engine's
//! linear pipeline, so colours here resolve to LINEAR — unlike specimen chrome
//! (`crate::nel`), which is node-tier sRGB and converts at the adapter edge.

#![allow(dead_code)]

use glam::Vec4;
use jetstream_ui::ui_element::JsEl;
use poodle_jetstream::JetstreamThemeProvider;

/// Convert a node-tier element to JsEl for embedding in framework chrome.
pub fn jel(e: crate::nel::El) -> JsEl {
    jetstream_poodle::to_js_el(&e.0)
}

/// Resolve a colour token to LINEAR (the engine pipeline's working space).
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

/// Resolve an opacity token.
pub fn resolve_opacity(theme: &JetstreamThemeProvider, token: &str) -> f32 {
    theme.resolve_opacity_value(token)
}

/// Alpha tint.
pub fn tint(c: Vec4, fraction: f32) -> Vec4 {
    Vec4::new(c.x, c.y, c.z, c.w * fraction)
}

/// Apply a typed `ShadowValue` as a single token-accurate shadow (spread 0).
pub fn with_elevation(el: JsEl, sv: &poodle_tokens::typed::ShadowValue) -> JsEl {
    el.shadow(
        sv.offset_x,
        sv.offset_y,
        sv.blur,
        0.0,
        Vec4::new(sv.color.0, sv.color.1, sv.color.2, sv.color.3),
    )
}

/// Token-resolved `elevation.surface` shadow.
pub fn elevation_surface(el: JsEl) -> JsEl {
    with_elevation(el, &poodle_tokens::typed::semantic::ELEVATION_SURFACE)
}

/// Token-resolved `elevation.overlay` shadow.
pub fn elevation_overlay(el: JsEl) -> JsEl {
    with_elevation(el, &poodle_tokens::typed::semantic::ELEVATION_OVERLAY)
}

/// Token-resolved `elevation.dialog` shadow.
pub fn elevation_dialog(el: JsEl) -> JsEl {
    with_elevation(el, &poodle_tokens::typed::semantic::ELEVATION_DIALOG)
}

/// 1rem = 16 logical px.
pub fn rem_to_px(rem: f32) -> f32 {
    rem * 16.0
}
