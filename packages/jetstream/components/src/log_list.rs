//! LogList — Jetstream log list backed by LogListSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::LogListSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_log_list(spec: &LogListSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let label_font = rem_to_px(size_font_rem(effective_size) - 0.0625);
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let entry_gap = resolve_px(theme, spec.entry_gap_token());

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, "color.border.subtle");
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let code_font_size = rem_to_px(0.8125);

    // Token colors for log levels
    let info_color = resolve_color(theme, "color.accent.base");
    let warn_color = resolve_color(theme, "color.status.warning");
    let error_color = resolve_color(theme, "color.status.danger");

    // Root
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y);

    // Toolbar: level filter pills + info
    let mut toolbar = ui_element::div()
        .flex_row().items_center().gap(rem_to_px(0.5))
        .pb(rem_to_px(0.5));

    // Level filter chips
    for level in &["info", "warn", "error"] {
        let is_active = spec.filter_level.as_deref() == Some(level);
        let chip_color = match *level {
            "info" => info_color,
            "warn" => warn_color,
            "error" => error_color,
            _ => text_secondary,
        };
        toolbar = toolbar.child(
            ui_element::button(*level)
                .text_color(if is_active { chip_color } else { text_secondary })
                .text_size(label_font)
                .text_weight(if is_active { 600 } else { 400 })
                .focusable()
        );
    }

    toolbar = toolbar.child(ui_element::div().grow());

    // Entry count
    toolbar = toolbar.child(
        ui_element::label(&format!("{} entries", spec.entry_count))
            .text_color(text_secondary).text_size(label_font)
    );
    el = el.child(toolbar);

    // Log entries area (placeholder representation)
    let mut entries_area = ui_element::div()
        .flex_col().gap(entry_gap).grow()
        .overflow_hidden();

    // Show placeholder entries based on entry_count (capped by max_entries)
    let visible_count = spec.entry_count.min(spec.max_entries).min(10); // cap preview at 10
    for i in 0..visible_count {
        let level = match i % 3 {
            0 => "info",
            1 => "warn",
            _ => "error",
        };
        let level_color = match level {
            "info" => info_color,
            "warn" => warn_color,
            _ => error_color,
        };

        let entry_row = ui_element::div()
            .flex_row().items_center().gap(rem_to_px(0.5))
            .child(
                ui_element::label(&format!("00:{:02}:{:02}", i / 60, i % 60))
                    .text_color(text_secondary).text_size(code_font_size)
            )
            .child(
                ui_element::label(level)
                    .text_color(level_color).text_size(label_font).text_weight(600)
            )
            .child(
                ui_element::label(&format!("Log message {}", i + 1))
                    .text_color(text_primary).text_size(font_size)
            );
        entries_area = entries_area.child(entry_row);
    }

    el = el.child(entries_area);

    // Auto-scroll indicator
    if spec.auto_scroll {
        el = el.child(
            ui_element::label("Auto-scrolling enabled")
                .text_color(text_secondary).text_size(label_font)
        );
    }

    el
}
