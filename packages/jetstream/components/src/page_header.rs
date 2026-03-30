//! PageHeader — Jetstream page header backed by PageHeaderSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::PageHeaderSpec;

use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_page_header(spec: &PageHeaderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_size = rem_to_px(size_font_rem(effective_size) + 0.4375);
    let subtitle_size = rem_to_px(size_font_rem(effective_size));
    let gap = resolve_px(theme, spec.gap_token());
    let pad_y = resolve_px(theme, spec.padding_y_token());

    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.subtitle_color_token());

    let mut el = ui_element::div().flex_col().gap(gap).pb(pad_y);

    el = el.child(ui_element::label(&spec.title).text_color(text_primary).text_size(title_size).text_weight(700));

    if let Some(ref desc) = spec.subtitle {
        el = el.child(ui_element::label(desc).text_color(text_secondary).text_size(subtitle_size));
    }

    el
}
