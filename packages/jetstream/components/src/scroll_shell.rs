//! ScrollShell — Jetstream scrollable container backed by ScrollShellSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::ScrollShellSpec;

pub fn js_scroll_shell(_spec: &ScrollShellSpec, _theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let mut el = ui_element::div()
        .flex_col()
        .overflow_hidden()
        .grow();

    for child in children {
        el = el.child(child);
    }

    el
}
