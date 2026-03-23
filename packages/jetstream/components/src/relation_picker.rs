//! RelationPicker — Jetstream relation picker backed by RelationPickerSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::RelationPickerSpec;
use crate::theme_ext::resolve_color;

pub fn js_relation_picker(spec: &RelationPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let accent = resolve_color(theme, "semantic.color.accent.base");
    let border = resolve_color(theme, "semantic.color.border.subtle");

    let mut el = ui_element::div().flex_col().gap(2.0);

    for item in &spec.items {
        let is_selected = spec.selected_ids.contains(&item.id);
        let indicator = if is_selected { "☑" } else { "☐" };
        let color = if is_selected { accent } else { text_color };

        el = el.child(
            ui_element::div()
                .flex_row().items_center().gap(6.0)
                .pl(8.0).pr(8.0).pt(4.0).pb(4.0)
                .child(ui_element::label(indicator).text_color(color).text_size(14.0))
                .child(ui_element::label(&item.label).text_color(text_color).text_size(13.0))
        );
    }

    el
}
