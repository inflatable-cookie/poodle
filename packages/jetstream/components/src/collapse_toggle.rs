//! CollapseToggle — Jetstream collapse toggle backed by CollapseToggleSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::CollapseToggleSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_collapse_toggle(spec: &CollapseToggleSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let icon_size = resolve_px(theme, spec.icon_size_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_color = resolve_color(theme, spec.text_color_token());

    let chevron_icon = if spec.is_collapsed { "chevron-right" } else { "chevron-down" };

    let mut el = ui_element::button("")
        .rounded(radius)
        .w(24.0).h(24.0)
        .flex_row().items_center().justify_center()
        .focusable()
        .cursor_pointer()
        .child(
            ui_element::icon(chevron_icon)
                .w(icon_size).h(icon_size)
                .text_color(text_color)
        );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity);
    }

    el
}
