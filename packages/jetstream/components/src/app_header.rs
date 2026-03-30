//! AppHeader — Jetstream app header backed by AppHeaderSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::AppHeaderSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub fn js_app_header(spec: &AppHeaderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.background_token());
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .h(rem_to_px(3.0)).grow()
        .flex_row().items_center()
        .pl(rem_to_px(1.0)).pr(rem_to_px(1.0));

    if let Some(ref title) = spec.title {
        el = el.child(ui_element::label(title).text_color(text_color).text_size(rem_to_px(0.875)).text_weight(600));
    }

    el
}
