//! DetailSection — Jetstream detail section backed by DetailSectionSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_composites::DetailSectionSpec;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_detail_section(spec: &DetailSectionSpec, theme: &JetstreamThemeProvider, body: Option<JsEl>) -> JsEl {
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let border = resolve_color(theme, "semantic.color.border.subtle");

    let mut el = ui_element::div().flex_col().gap(8.0);

    if let Some(ref title) = spec.title {
        el = el.child(ui_element::label(title).text_color(text_primary).text_size(14.0).text_weight(600));
    }

    if let Some(ref desc) = spec.description {
        el = el.child(ui_element::label(desc).text_color(text_secondary).text_size(13.0));
    }

    if spec.is_separated {
        el = el.child(ui_element::div().h(1.0).grow().bg(border));
    }

    if let Some(body_el) = body {
        el = el.child(body_el);
    }

    el
}
