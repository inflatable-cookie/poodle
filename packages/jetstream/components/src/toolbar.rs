//! Toolbar — Jetstream toolbar container backed by ToolbarSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::{Alignment, ToolbarSpec};

use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::resolve_px;

pub fn js_toolbar(spec: &ToolbarSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let gap = resolve_px(theme, spec.gap_token());
    let padding = resolve_px(theme, spec.padding_token());
    let height = rem_to_px(control_height_rem(effective_size));

    let mut el = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .min_h(height)
        .pl(padding).pr(padding);

    match spec.alignment {
        Alignment::Start => {}
        Alignment::Center => { el = el.justify_center(); }
        Alignment::End => { el = el.justify_end(); }
        Alignment::Stretch => { el = el.grow(); }
    }

    if spec.has_separator {
        let border_color = crate::theme_ext::resolve_color(theme, spec.border_token());
        el = el.border_b_1().border_color(border_color);
    }

    for child in children {
        el = el.child(child);
    }

    el
}
