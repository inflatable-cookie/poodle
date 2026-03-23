//! MediaThumbnail — Jetstream media thumbnail backed by MediaThumbnailSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::MediaThumbnailSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_media_thumbnail(spec: &MediaThumbnailSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.frame_fill_token());
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_color = resolve_color(theme, "semantic.color.text.secondary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .w(120.0).h(80.0)
        .flex_col().items_center().justify_center();

    if let Some(ref title) = spec.title {
        el = el.child(ui_element::label(title).text_color(text_color).text_size(11.0));
    }

    el
}
