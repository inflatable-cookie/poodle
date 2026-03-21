//! CollapseToggle — Jetstream collapse toggle backed by CollapseToggleSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::CollapseToggleSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_collapse_toggle(spec: &CollapseToggleSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let icon_size = resolve_px(theme, spec.icon_size_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_color = resolve_color(theme, spec.text_color_token());

    let chevron = if spec.is_collapsed { "▸" } else { "▾" };

    let mut el = ui_element::button(chevron)
        .text_size(icon_size)
        .text_color(text_color)
        .rounded(radius)
        .w(24.0).h(24.0)
        .flex_row().items_center().justify_center()
        .focusable();

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity);
    }

    el
}
