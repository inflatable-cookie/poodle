//! FilterToolbar — Jetstream filter toolbar backed by FilterToolbarSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_composites::FilterToolbarSpec;
use crate::theme_ext::resolve_px;

pub fn js_filter_toolbar(spec: &FilterToolbarSpec, theme: &JetstreamThemeProvider, search: Option<JsEl>, filters: Option<JsEl>) -> JsEl {
    let gap = resolve_px(theme, spec.gap_token());

    let mut el = ui_element::div().flex_row().items_center().gap(gap);

    if let Some(search_el) = search {
        el = el.child(search_el);
    }

    if let Some(filter_el) = filters {
        el = el.child(filter_el);
    }

    el
}
