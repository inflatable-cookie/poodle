//! ResizeHandle — Jetstream resize handle backed by ResizeHandleSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::{Orientation, ResizeHandleSpec};

use crate::theme_ext::resolve_color;

pub fn js_resize_handle(spec: &ResizeHandleSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let handle_color = resolve_color(theme, "semantic.color.border.subtle");

    match spec.orientation {
        Orientation::Horizontal => {
            ui_element::div()
                .h(4.0).grow()
                .bg(handle_color)
        }
        Orientation::Vertical => {
            ui_element::div()
                .w(4.0).grow()
                .bg(handle_color)
        }
    }
}
