//! Menu — Jetstream vertical menu backed by MenuSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::MenuSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_menu(spec: &MenuSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let item_px = rem_to_px(control_space_x_rem(spec.density));
    let item_py = rem_to_px(panel_space_y_rem(spec.density) - 0.375);
    let menu_py = rem_to_px(0.25);
    let item_gap = rem_to_px(0.5);

    let fill = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");
    let text_color = resolve_color(theme, "color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .pt(menu_py).pb(menu_py)
        .min_w(rem_to_px(10.0))
        .shadow_md()
        .overlay();

    for entry in &spec.items {
        let mut item = ui_element::div()
            .flex_row().items_center().gap(item_gap)
            .pl(item_px).pr(item_px).pt(item_py).pb(item_py)
            .cursor_pointer()
            .focusable();

        item = item.child(
            ui_element::label(&entry.label).text_color(text_color).text_size(font_size).grow()
        );

        el = el.child(item);
    }

    el
}
