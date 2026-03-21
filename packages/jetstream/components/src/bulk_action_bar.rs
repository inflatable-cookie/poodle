//! BulkActionBar — Jetstream bulk action bar backed by BulkActionBarSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::BulkActionBarSpec;

use crate::theme_ext::resolve_color;

pub fn js_bulk_action_bar(spec: &BulkActionBarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .flex_row().items_center().gap(8.0)
        .pl(12.0).pr(12.0).pt(6.0).pb(6.0);

    // Selected count
    let count_text = format!("{} selected", spec.selection_count);
    el = el.child(ui_element::label(&count_text).text_color(text_color).text_size(13.0).text_weight(500));

    // Actions
    for action in &spec.actions {
        el = el.child(
            ui_element::button(&action.label)
                .text_color(text_color).text_size(13.0)
                .pl(8.0).pr(8.0)
                .focusable()
        );
    }

    el
}
