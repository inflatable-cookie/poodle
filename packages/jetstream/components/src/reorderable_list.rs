//! ReorderableList — Jetstream draggable list items with grip handles backed by ReorderableListSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::ReorderableListSpec;

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_reorderable_list(
    spec: &ReorderableListSpec,
    theme: &JetstreamThemeProvider,
    items: Vec<JsEl>,
) -> JsEl {
    let _effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let item_gap = resolve_px(theme, spec.item_gap_token());
    let inline_gap = rem_to_px(control_space_x_rem(spec.density));

    let fill = resolve_color(theme, spec.fill_token());
    let handle_color = resolve_color(theme, spec.handle_color_token());
    let drag_bg = resolve_color(theme, "color.background.elevated");
    let focus_ring = resolve_color(theme, "color.accent.focusRing");

    let icon_size = rem_to_px(1.0);
    let row_py = rem_to_px(0.125);
    let row_px = rem_to_px(0.25);
    let row_radius = rem_to_px(0.25);

    let mut el = ui_element::div()
        .bg(fill)
        .flex_col().gap(item_gap);

    for (idx, child) in items.into_iter().enumerate() {
        let is_dragging = spec.active_drag_index == Some(idx);

        let handle = ui_element::icon("grip-vertical")
            .w(icon_size).h(icon_size)
            .text_color(handle_color);

        let mut row = ui_element::div()
            .flex_row().items_center().gap(inline_gap)
            .pl(row_px).pr(row_px).pt(row_py).pb(row_py)
            .rounded(row_radius)
            .child(handle)
            .child(child);

        if is_dragging {
            row = row.bg(drag_bg).border(1.0).border_color(focus_ring);
        }

        el = el.child(row);
    }

    if spec.is_disabled {
        el = el.opacity(0.5);
    }

    el
}
