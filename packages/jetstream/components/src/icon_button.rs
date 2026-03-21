//! IconButton — Jetstream icon-only button backed by IconButtonSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::IconButtonSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_icon_button(spec: &IconButtonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let height = resolve_px(theme, spec.control_height_token());
    let icon_size = resolve_px(theme, spec.icon_size_token());
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let icon_text = spec.icon.as_deref().unwrap_or("?");

    let mut el = ui_element::button(icon_text)
        .h(height).w(height) // square
        .rounded(radius)
        .text_size(icon_size)
        .text_color(text_color)
        .flex_row().items_center().justify_center()
        .focusable();

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}
