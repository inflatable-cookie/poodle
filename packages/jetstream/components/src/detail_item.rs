//! DetailItem — Jetstream label/value row backed by DetailItemSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::DetailItemSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_detail_item(spec: &DetailItemSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let label_color = resolve_color(theme, spec.label_color_token());
    let value_color = resolve_color(theme, spec.value_color_token());
    let bg = resolve_color(theme, spec.background_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let pad_x = resolve_px(theme, spec.padding_x_token());
    let pad_y = resolve_px(theme, spec.padding_y_token());
    let gap = resolve_px(theme, spec.gap_token());

    // Contract: body font-size 0.8125rem (13px), description 0.75rem (12px)
    let body_font = rem_to_px(0.8125);
    let desc_font = rem_to_px(0.75);

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
            .text_size(body_font)
    );

    // Value
    if let Some(ref value) = spec.value {
        el = el.child(
            ui_element::label(value)
                .text_color(value_color)
                .text_size(body_font)
                .grow()
        );
    }

    // Description (below, if present)
    if let Some(ref desc) = spec.description {
        let desc_color = resolve_color(theme, spec.description_color_token());
        el = el.child(
            ui_element::label(desc)
                .text_color(desc_color)
                .text_size(desc_font)
        );
    }

    el
}
