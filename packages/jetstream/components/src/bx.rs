//! Box — Jetstream container component backed by BoxSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{BoxSpec, Overflow};

use crate::theme_ext::resolve_px;

pub fn js_box(spec: &BoxSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let padding = spec.resolved_padding();

    let mut el = ui_element::div();

    if let Some(h) = padding.horizontal {
        let px_val = resolve_px(theme, h);
        el = el.pl(px_val).pr(px_val);
    }
    if let Some(v) = padding.vertical {
        let px_val = resolve_px(theme, v);
        el = el.pt(px_val).pb(px_val);
    }

    if spec.overflow == Overflow::Hidden {
        el = el.overflow_hidden();
    }

    for child in children {
        el = el.child(child);
    }

    el
}
