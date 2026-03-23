//! Grid — Jetstream grid layout backed by GridSpec.
//!
//! Jetstream/Taffy doesn't support CSS Grid natively, so we emulate it as a
//! flex-wrap container with percentage-width children. The gap is applied
//! uniformly.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::GridSpec;

use crate::theme_ext::resolve_px;

pub fn js_grid(spec: &GridSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let padding = spec.resolved_padding();

    let mut el = ui_element::div()
        .flex_row()
        .flex_wrap();

    if let Some(gap_token) = spec.resolved_column_gap() {
        el = el.gap(resolve_px(theme, gap_token));
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
