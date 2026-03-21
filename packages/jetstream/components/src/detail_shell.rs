//! DetailShell — Jetstream detail page shell backed by DetailShellSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_composites::DetailShellSpec;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_detail_shell(spec: &DetailShellSpec, theme: &JetstreamThemeProvider, header: Option<JsEl>, content: Option<JsEl>) -> JsEl {
    let bg = resolve_color(theme, spec.body_fill_token());

    let mut el = ui_element::div().bg(bg).flex_col().grow();

    if let Some(h) = header {
        el = el.child(h);
    }

    if let Some(c) = content {
        el = el.child(c);
    }

    el
}
