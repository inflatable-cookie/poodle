//! PickerShell — Jetstream picker shell backed by PickerShellSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_composites::PickerShellSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_picker_shell(spec: &PickerShellSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col().gap(8.0)
        .pl(16.0).pr(16.0).pt(12.0).pb(12.0)
        .min_w(320.0);

    el = el.child(ui_element::label(&spec.title).text_color(text_primary).text_size(16.0).text_weight(600));

    if let Some(c) = content {
        el = el.child(c);
    }

    el
}
