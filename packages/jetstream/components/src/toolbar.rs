//! Toolbar — Jetstream toolbar container backed by ToolbarSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::{Alignment, ToolbarSpec};

use crate::theme_ext::resolve_px;

pub fn js_toolbar(spec: &ToolbarSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let gap = resolve_px(theme, spec.gap_token());

    let mut el = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap);

    match spec.alignment {
        Alignment::Start => {}
        Alignment::Center => { el = el.justify_center(); }
        Alignment::End => { el = el.justify_end(); }
        Alignment::Stretch => { el = el.grow(); }
    }

    for child in children {
        el = el.child(child);
    }

    el
}
