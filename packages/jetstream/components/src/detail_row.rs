//! DetailRow — Jetstream label/value row backed by DetailRowSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::DetailRowSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_detail_row(spec: &DetailRowSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let label_color = resolve_color(theme, spec.label_color_token());
    let value_color = resolve_color(theme, spec.value_color_token());
    let bg = resolve_color(theme, spec.background_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let pad_x = resolve_px(theme, spec.padding_x_token());
    let pad_y = resolve_px(theme, spec.padding_y_token());
    let gap = resolve_px(theme, spec.gap_token());

    let mut el = ui_element::div()
        .bg(bg)
        .rounded(radius)
        .pl(pad_x).pr(pad_x)
        .pt(pad_y).pb(pad_y)
        .flex_row()
        .items_center()
        .gap(gap);

    // Label
    el = el.child(
        ui_element::label(&spec.label)
            .text_color(label_color)
            .text_size(13.0)
    );

    // Value
    if let Some(ref value) = spec.value {
        el = el.child(
            ui_element::label(value)
                .text_color(value_color)
                .text_size(13.0)
                .grow()
        );
    }

    // Description (below, if present)
    if let Some(ref desc) = spec.description {
        let desc_color = resolve_color(theme, spec.description_color_token());
        el = el.child(
            ui_element::label(desc)
                .text_color(desc_color)
                .text_size(12.0)
        );
    }

    el
}
