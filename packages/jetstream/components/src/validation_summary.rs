//! ValidationSummary — Jetstream error/warning validation list backed by ValidationSummarySpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::ValidationSummarySpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub fn js_validation_summary(spec: &ValidationSummarySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let border = resolve_color(theme, spec.border_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let danger_color = resolve_color(theme, "color.status.danger");
    let accent_color = resolve_color(theme, "color.accent.base");

    let font_size = rem_to_px(0.8125);
    let small_size = rem_to_px(0.75);
    let pad_x = rem_to_px(0.75);
    let pad_y = rem_to_px(0.5);
    let gap = rem_to_px(0.5);
    let item_gap = rem_to_px(0.375);

    let mut el = ui_element::div()
        .border(1.0).border_color(border)
        .rounded(rem_to_px(0.375))
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .flex_col().gap(gap);

    // Note: ARIA role from spec.accessibility_role() applied by host runtime.

    // Title
    if let Some(ref title) = spec.title {
        el = el.child(
            ui_element::label(title)
                .text_color(text_primary).text_size(font_size).text_weight(600)
        );
    }

    // Entry list
    let entries = spec.active_entries();
    for entry in &entries {
        let entry_color = if entry.is_blocking() { danger_color } else { accent_color };

        let mut entry_el = ui_element::div()
            .flex_row().items_center().gap(item_gap);

        // Bullet indicator
        entry_el = entry_el.child(
            ui_element::div()
                .w(rem_to_px(0.375)).h(rem_to_px(0.375))
                .rounded(rem_to_px(0.1875))
                .bg(entry_color)
        );

        // Field label and message
        let mut text_col = ui_element::div().flex_col().gap(rem_to_px(0.125));

        text_col = text_col.child(
            ui_element::label(&entry.label)
                .text_color(text_primary).text_size(small_size).text_weight(500)
        );

        text_col = text_col.child(
            ui_element::label(&entry.message)
                .text_color(text_secondary).text_size(small_size)
        );

        entry_el = entry_el.child(text_col);
        el = el.child(entry_el);
    }

    // Summary count
    let blocking = spec.blocking_entry_count();
    if blocking > 0 {
        let summary_text = format!("{} blocking issue{}", blocking, if blocking == 1 { "" } else { "s" });
        el = el.child(
            ui_element::label(&summary_text)
                .text_color(danger_color).text_size(small_size).text_weight(500)
        );
    }

    el
}
