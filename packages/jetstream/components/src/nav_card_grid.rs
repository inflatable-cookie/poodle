//! NavCardGrid — Jetstream nav card grid backed by NavCardGridSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::NavCardGridSpec;

pub fn js_nav_card_grid(_spec: &NavCardGridSpec, _theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let mut el = ui_element::div().flex_row().flex_wrap().gap(12.0);

    for child in children {
        el = el.child(child);
    }

    el
}
