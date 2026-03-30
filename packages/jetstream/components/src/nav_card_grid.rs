//! NavCardGrid — Jetstream nav card grid backed by NavCardGridSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::NavCardGridSpec;

use crate::theme_ext::resolve_px;

pub fn js_nav_card_grid(spec: &NavCardGridSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let gap = resolve_px(theme, spec.gap_token());

    let mut el = ui_element::div().flex_row().flex_wrap().gap(gap);

    for child in children {
        el = el.child(child);
    }

    el
}
