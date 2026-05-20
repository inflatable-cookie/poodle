//! FormActions — Jetstream form action bar backed by FormActionsSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{FormActionAlign, FormActionsSpec};

use crate::theme_ext::resolve_px;

pub fn js_form_actions(spec: &FormActionsSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let gap = resolve_px(theme, spec.action_gap_token());
    let separation = if spec.shows_top_separation() {
        resolve_px(theme, spec.stack_separation_token())
    } else {
        0.0
    };
    let border_gap = if spec.shows_top_border() {
        separation.max(4.0) * 0.5
    } else {
        0.0
    };

    let mut el = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .flex_wrap()
        .pt(separation)
        .mt(border_gap);

    if spec.shows_top_border() {
        let border_color = crate::theme_ext::resolve_color(theme, spec.border_token());
        el = el.border_t_1().border_color(border_color);
    }

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
