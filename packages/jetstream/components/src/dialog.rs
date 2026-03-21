//! Dialog — Jetstream dialog container backed by DialogSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::DialogSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_dialog(spec: &DialogSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let title_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(24.0).pr(24.0).pt(20.0).pb(20.0)
        .flex_col().gap(16.0)
        .min_w(400.0);

    if let Some(ref title) = spec.title {
        el = el.child(
            ui_element::label(title).text_color(title_color).text_size(16.0).text_weight(600)
        );
    }

    if let Some(content_el) = content {
        el = el.child(content_el);
    }

    el
}
