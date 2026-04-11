//! Stack — Jetstream vertical stack layout backed by StackSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::{Alignment, StackSpec};

use crate::theme_ext::resolve_px;

pub fn js_stack(spec: &StackSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let padding = spec.resolved_padding();

    let mut el = ui_element::div()
        .flex_col();

    if let Some(gap_token) = spec.resolved_gap() {
        el = el.gap(resolve_px(theme, gap_token));
    }

    match spec.align {
        Alignment::Start => { el = el.items_start(); }
        Alignment::Center => { el = el.items_center(); }
        Alignment::End => { el = el.items_end(); }
        Alignment::Stretch => {} // default flex behavior
    }

    if let Some(h) = padding.horizontal {
        let px_val = resolve_px(theme, h);
        el = el.pl(px_val).pr(px_val);
    }
    if let Some(v) = padding.vertical {
        let px_val = resolve_px(theme, v);
        el = el.pt(px_val).pb(px_val);
    }

    for child in children {
        el = el.child(child);
    }

    el
}
