//! DetailSection — Jetstream detail section backed by DetailSectionSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::DetailSectionSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_detail_section(spec: &DetailSectionSpec, theme: &JetstreamThemeProvider, body: Option<JsEl>) -> JsEl {
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.description_color_token());
    let border = resolve_color(theme, spec.separator_color_token());
    let gap = resolve_px(theme, spec.body_gap_token());

    let mut el = ui_element::div().flex_col().gap(gap);

    if let Some(ref title) = spec.title {
        el = el.child(ui_element::label(title).text_color(text_primary).text_size(rem_to_px(0.875)).text_weight(600));
    }

    if let Some(ref desc) = spec.description {
        el = el.child(ui_element::label(desc).text_color(text_secondary).text_size(rem_to_px(0.8125)));
    }

    if spec.is_separated {
        el = el.child(ui_element::div().h(1.0).grow().bg(border));
    }

    if let Some(body_el) = body {
        el = el.child(body_el);
    }

    el
}
