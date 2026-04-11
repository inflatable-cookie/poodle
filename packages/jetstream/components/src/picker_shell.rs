//! PickerShell — Jetstream picker shell backed by PickerShellSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::PickerShellSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_picker_shell(spec: &PickerShellSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col().gap(rem_to_px(0.5))
        .pl(rem_to_px(1.0)).pr(rem_to_px(1.0)).pt(rem_to_px(0.75)).pb(rem_to_px(0.75))
        .min_w(rem_to_px(20.0));

    el = el.child(ui_element::label(&spec.title).text_color(text_primary).text_size(rem_to_px(1.0)).text_weight(600));

    if let Some(c) = content {
        el = el.child(c);
    }

    el
}
