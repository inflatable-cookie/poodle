//! RelationPicker — Jetstream relation picker backed by RelationPickerSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::RelationPickerSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::resolve_color;

pub fn js_relation_picker(spec: &RelationPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let indicator_size = rem_to_px(size_font_rem(effective_size) + 0.0625);
    let item_px = rem_to_px(control_space_x_rem(spec.density));
    let item_py = rem_to_px(panel_space_y_rem(spec.density) - 0.5);
    let gap = rem_to_px(0.375);
    let item_gap = rem_to_px(0.375);

    let text_color = resolve_color(theme, "color.text.primary");
    let accent = resolve_color(theme, "color.accent.base");

    let mut el = ui_element::div().flex_col().gap(gap);

    for item in &spec.items {
        let is_selected = spec.selected_ids.contains(&item.id);
        let indicator = if is_selected { "☑" } else { "☐" };
        let color = if is_selected { accent } else { text_color };

        el = el.child(
            ui_element::div()
                .flex_row().items_center().gap(item_gap)
                .pl(item_px).pr(item_px).pt(item_py).pb(item_py)
                .child(ui_element::label(indicator).text_color(color).text_size(indicator_size))
                .child(ui_element::label(&item.label).text_color(text_color).text_size(font_size))
        );
    }

    el
}
