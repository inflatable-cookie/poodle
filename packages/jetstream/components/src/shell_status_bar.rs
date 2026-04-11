//! ShellStatusBar — Jetstream bottom status bar with leading/trailing slots
//! backed by ShellStatusBarSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ShellStatusBarSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub fn js_shell_status_bar(
    spec: &ShellStatusBarSpec,
    theme: &JetstreamThemeProvider,
    leading: Vec<JsEl>,
    trailing: Vec<JsEl>,
) -> JsEl {
    let bg = resolve_color(theme, spec.background_token());
    let border = resolve_color(theme, "color.border.subtle");
    let text_color = resolve_color(theme, "color.text.secondary");

    let bar_height = rem_to_px(1.5);
    let pad_x = rem_to_px(0.5);
    let gap = if spec.is_dense() { rem_to_px(0.375) } else { rem_to_px(0.5) };
    let font_size = rem_to_px(0.6875);

    let mut el = ui_element::div()
        .bg(bg)
        .border(1.0).border_color(border)
        .h(bar_height)
        .pl(pad_x).pr(pad_x)
        .flex_row().items_center();

    // Leading slot
    let mut leading_row = ui_element::div().flex_row().items_center().gap(gap);
    for item in leading {
        leading_row = leading_row.child(item);
    }
    el = el.child(leading_row);

    // Summary (center/grow)
    if let Some(ref summary) = spec.summary {
        el = el.child(
            ui_element::label(summary)
                .text_color(text_color).text_size(font_size)
                .grow()
                .pl(gap).pr(gap)
        );
    } else {
        // Spacer
        el = el.child(ui_element::div().grow());
    }

    // Trailing slot
    let mut trailing_row = ui_element::div().flex_row().items_center().gap(gap);
    for item in trailing {
        trailing_row = trailing_row.child(item);
    }
    el = el.child(trailing_row);

    el
}
