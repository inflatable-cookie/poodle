//! PageHeader — Jetstream page header backed by PageHeaderSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::PageHeaderSpec;
use crate::theme_ext::resolve_color;

pub fn js_page_header(spec: &PageHeaderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

    let mut el = ui_element::div().flex_col().gap(4.0).pb(12.0);

    el = el.child(ui_element::label(&spec.title).text_color(text_primary).text_size(20.0).text_weight(700));

    if let Some(ref desc) = spec.subtitle {
        el = el.child(ui_element::label(desc).text_color(text_secondary).text_size(13.0));
    }

    el
}
