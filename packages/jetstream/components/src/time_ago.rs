//! TimeAgo — Jetstream relative time label backed by TimeAgoSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::TimeAgoSpec;

use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_time_ago(spec: &TimeAgoSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, spec.text_color_token());
    let font_size = resolve_px(theme, spec.font_size_token());

    ui_element::label(&spec.timestamp)
        .text_color(text_color)
        .text_size(font_size)
}
