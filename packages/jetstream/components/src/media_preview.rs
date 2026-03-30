//! MediaPreview — Jetstream media preview backed by MediaPreviewSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::MediaPreviewSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_media_preview(spec: &MediaPreviewSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col().gap(rem_to_px(0.5))
        .pl(rem_to_px(0.75)).pr(rem_to_px(0.75)).pt(rem_to_px(0.75)).pb(rem_to_px(0.75));

    el = el.child(ui_element::label(&spec.title).text_color(text_primary).text_size(rem_to_px(0.875)).text_weight(500));

    el
}
