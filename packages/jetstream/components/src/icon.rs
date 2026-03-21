//! Icon — Jetstream icon component backed by IconSpec.
//!
//! Jetstream has no SVG rendering. Icons render as text glyphs.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::IconSpec;

use crate::theme_ext::resolve_px;

pub fn js_icon(spec: &IconSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let size = resolve_px(theme, spec.size_token());

    ui_element::label(&spec.name)
        .text_size(size)
        .w(size)
        .h(size)
        .flex_row()
        .items_center()
        .justify_center()
}
