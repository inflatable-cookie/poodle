//! ToastStack — Jetstream toast notification stack backed by ToastStackSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_composites::ToastStackSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_toast_stack(spec: &ToastStackSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div().flex_col().gap(8.0);

    for toast in &spec.toasts {
        let toast_el = ui_element::div()
            .bg(fill)
            .border(1.0).border_color(border)
            .rounded(radius)
            .pl(12.0).pr(12.0).pt(8.0).pb(8.0)
            .flex_row().items_center().gap(8.0)
            .child(ui_element::label(toast.message.as_deref().unwrap_or("")).text_color(text_color).text_size(13.0));
        el = el.child(toast_el);
    }

    el
}
