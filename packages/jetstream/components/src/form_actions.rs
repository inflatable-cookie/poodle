//! FormActions — Jetstream form action bar backed by FormActionsSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::{FormActionAlign, FormActionsSpec};

use crate::theme_ext::resolve_px;

pub fn js_form_actions(spec: &FormActionsSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let gap = resolve_px(theme, spec.action_gap_token());

    let mut el = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .flex_wrap();

    match spec.align {
        FormActionAlign::Start => {}
        FormActionAlign::End => { el = el.justify_end(); }
        FormActionAlign::Between => { el = el.justify_between(); }
    }

    for child in children {
        el = el.child(child);
    }

    el
}
