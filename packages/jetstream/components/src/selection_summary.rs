//! SelectionSummary — Jetstream selection summary backed by SelectionSummarySpec.
use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_composites::SelectionSummarySpec;
use crate::theme_ext::resolve_color;

pub fn js_selection_summary(spec: &SelectionSummarySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div().flex_row().items_center().gap(8.0).flex_wrap();

    for item in &spec.items {
        el = el.child(
            ui_element::label(&item.label)
                .text_color(text_color).text_size(13.0)
                .pl(8.0).pr(8.0).pt(2.0).pb(2.0)
                .rounded(4.0)
                .bg(resolve_color(theme, "semantic.color.background.surface"))
                .border(1.0).border_color(resolve_color(theme, "semantic.color.border.subtle"))
        );
    }

    el
}
