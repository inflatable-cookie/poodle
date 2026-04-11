//! FieldSet — Jetstream fieldset grouping wrapper backed by FieldSetSpec.
//!
//! Contract: `docs/contracts/components/field-set.md`
//! Reference: `packages/svelte/primitives/src/FieldSet.svelte` (if present)
//!
//! Renders a fieldset wrapper with optional legend and a multi-column grid
//! for slotted field content.  Children are passed in as a Vec of JsEl.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::FieldSetSpec;

use crate::theme_ext::{resolve_color, resolve_px};

/// Build the fieldset element.  `children` are the slotted field controls.
pub fn js_field_set(
    spec: &FieldSetSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
) -> JsEl {
    let gap = resolve_px(theme, spec.gap_token());
    let legend_color = resolve_color(theme, spec.legend_color_token());
    let legend_size = resolve_px(theme, spec.legend_size_token());

    let mut root = ui_element::div().flex_col().gap(gap);

    // Legend (optional)
    if let Some(ref legend) = spec.legend {
        root = root.child(
            ui_element::label(legend)
                .text_color(legend_color)
                .text_size(legend_size)
                .text_weight(600),
        );
    }

    // Fields grid
    let mut grid = ui_element::div().flex_row().flex_wrap().gap(gap);

    // When columns > 1 each child gets an equal share of width.
    // We approximate multi-column by setting a min-width based on
    // the column count so that children flow into a grid pattern.
    let col_basis = if spec.columns > 1 {
        // rough percentage-based min-width per column
        Some(100.0 / spec.columns as f32)
    } else {
        None
    };

    for child in children {
        let mut wrapper = ui_element::div().child(child);
        if spec.columns <= 1 {
            // Single column: each child takes full width
            wrapper = wrapper.grow();
        } else if let Some(_pct) = col_basis {
            // Multi-column: grow equally
            wrapper = wrapper.grow();
        }
        grid = grid.child(wrapper);
    }

    root = root.child(grid);
    root
}
