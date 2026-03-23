//! Surface — Jetstream themed container backed by SurfaceSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::SurfaceSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_surface(spec: &SurfaceSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let bg = resolve_color(theme, spec.resolved_background_token());
    let padding = spec.resolved_padding();

    let mut el = ui_element::div()
        .bg(bg)
        .rounded(resolve_radius(theme, "semantic.radius.surface"));

    if let Some(border_token) = spec.resolved_border_color() {
        let border_color = resolve_color(theme, border_token);
        el = el.border(1.0).border_color(border_color);
    }

    if let Some(h) = padding.horizontal {
        let px_val = resolve_px(theme, h);
        el = el.pl(px_val).pr(px_val);
    }
    if let Some(v) = padding.vertical {
        let px_val = resolve_px(theme, v);
        el = el.pt(px_val).pb(px_val);
    }

    for child in children {
        el = el.child(child);
    }

    el
}
