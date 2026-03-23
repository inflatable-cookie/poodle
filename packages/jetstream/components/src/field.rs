//! Field — Jetstream form field wrapper backed by FieldSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::{FieldSpec, ValidationState};

use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_field(spec: &FieldSpec, theme: &JetstreamThemeProvider, control: Option<JsEl>) -> JsEl {
    let label_size = resolve_px(theme, spec.label_typography_token());
    let desc_color = resolve_color(theme, spec.description_color_token());
    let error_color = resolve_color(theme, spec.error_color_token());
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let row_gap = resolve_px(theme, spec.row_gap_token());

    let mut el = ui_element::div()
        .flex_col()
        .gap(row_gap);

    // Label row
    let mut label_row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(resolve_px(theme, spec.header_gap_token()));

    label_row = label_row.child(
        ui_element::label(&spec.label)
            .text_color(text_primary)
            .text_size(label_size)
            .text_weight(500)
    );

    if spec.shows_optional_label() {
        if let Some(ref opt_label) = spec.optional_label {
            label_row = label_row.child(
                ui_element::label(opt_label)
                    .text_color(desc_color)
                    .text_size(label_size)
            );
        }
    }

    el = el.child(label_row);

    // Control slot
    if let Some(control_el) = control {
        el = el.child(control_el);
    }

    // Description
    if let Some(ref desc) = spec.description {
        el = el.child(
            ui_element::label(desc)
                .text_color(desc_color)
                .text_size(12.0)
        );
    }

    // Error message
    if spec.validation_state == ValidationState::Invalid {
        if let Some(ref error) = spec.error {
            el = el.child(
                ui_element::label(error)
                    .text_color(error_color)
                    .text_size(12.0)
            );
        }
    }

    // Pending message
    if spec.validation_state == ValidationState::Pending {
        if let Some(ref pending) = spec.pending_message {
            el = el.child(
                ui_element::label(pending)
                    .text_color(desc_color)
                    .text_size(12.0)
            );
        }
    }

    el
}
