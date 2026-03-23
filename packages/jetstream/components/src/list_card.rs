//! ListCard — Jetstream list card backed by ListCardSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::ListCardSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_list_card(spec: &ListCardSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(12.0).pr(12.0).pt(8.0).pb(8.0)
        .flex_row().items_center().gap(8.0);

    el = el.child(ui_element::label(&spec.title).text_color(text_primary).text_size(13.0).text_weight(500));

    if let Some(ref subtitle) = spec.subtitle {
        el = el.child(ui_element::label(subtitle).text_color(text_secondary).text_size(12.0));
    }

    el
}
