//! Menu — Jetstream vertical menu backed by MenuSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::MenuSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_menu(spec: &MenuSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .pt(4.0).pb(4.0)
        .min_w(160.0)
        .shadow_md()
        .overlay();

    for entry in &spec.items {
        let mut item = ui_element::div()
            .flex_row().items_center().gap(8.0)
            .pl(12.0).pr(12.0).pt(6.0).pb(6.0)
            .cursor_pointer()
            .focusable();

        item = item.child(
            ui_element::label(&entry.label).text_color(text_color).text_size(13.0).grow()
        );

        el = el.child(item);
    }

    el
}
