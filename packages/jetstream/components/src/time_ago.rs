//! TimeAgo — Jetstream relative time label backed by TimeAgoSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::TimeAgoSpec;

use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_time_ago(spec: &TimeAgoSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, spec.text_color_token());
    let label = ui_element::label(&spec.timestamp).text_color(text_color);

    if spec.inherits_typography() {
        label
    } else {
        label.text_size(resolve_px(theme, spec.font_size_token()))
    }
}
