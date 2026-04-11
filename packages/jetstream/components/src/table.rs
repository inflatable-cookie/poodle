//! Table — Jetstream structured data table backed by TableSpec.
//!
//! Contract: `docs/contracts/components/table.md`
//! Reference: `packages/svelte/primitives/src/Table.svelte` (if present)
//!
//! Renders a shell container with column headers, body rows, and optional
//! caption / empty state.  All visual properties resolve from tokens.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ColumnAlign, TableSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius, tint};

pub fn js_table(spec: &TableSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Token resolution
    let shell_border_raw = resolve_color(theme, spec.shell_border_token());
    let shell_border = tint(shell_border_raw, 0.78); // 78% per contract
    let shell_fill_raw = resolve_color(theme, spec.shell_fill_token());
    let shell_fill = tint(shell_fill_raw, 0.96); // 96% per contract
    let shell_radius = resolve_radius(theme, spec.shell_radius_token());

    let header_fill = resolve_color(theme, spec.header_fill_token());
    let header_text = resolve_color(theme, spec.header_text_token());
    let header_border_raw = resolve_color(theme, spec.header_border_token());
    let header_border = tint(header_border_raw, 0.72); // 72% per contract

    let cell_text = resolve_color(theme, spec.cell_text_token());
    let cell_border_raw = resolve_color(theme, spec.cell_border_token());
    let cell_border = tint(cell_border_raw, 0.72);

    let caption_text = resolve_color(theme, spec.caption_text_token());
    let empty_text = resolve_color(theme, spec.empty_text_token());

    // Layout constants from contract
    let cell_py = rem_to_px(0.6875);
    let cell_px = rem_to_px(0.875);
    let header_font_size = rem_to_px(0.6875);
    let caption_font_size = rem_to_px(0.75);

    // Shell
    let mut shell = ui_element::div()
        .border(1.0)
        .border_color(shell_border)
        .rounded(shell_radius)
        .bg(shell_fill)
        .overflow_hidden()
        .flex_col();

    // Caption (optional)
    if let Some(ref caption) = spec.caption {
        shell = shell.child(
            ui_element::label(caption)
                .text_color(caption_text)
                .text_size(caption_font_size)
                .pl(cell_px)
                .pr(cell_px)
                .pt(cell_py),
        );
    }

    // Header row
    let mut header_row = ui_element::div()
        .bg(header_fill)
        .flex_row()
        .border_b_1()
        .border_color(header_border);

    for col in &spec.columns {
        let mut header_cell = ui_element::label(&col.label)
            .text_color(header_text)
            .text_size(header_font_size)
            .text_weight(600)
            .pl(cell_px)
            .pr(cell_px)
            .pt(cell_py)
            .pb(cell_py)
            .grow();

        if col.align == ColumnAlign::End {
            header_cell = header_cell.text_right();
        }

        header_row = header_row.child(header_cell);
    }
    shell = shell.child(header_row);

    // Body rows
    if spec.is_empty() {
        // Empty state
        let empty_row = ui_element::div()
            .flex_row()
            .child(
                ui_element::label(&spec.empty_message)
                    .text_color(empty_text)
                    .text_size(rem_to_px(0.8125))
                    .pl(cell_px)
                    .pr(cell_px)
                    .pt(cell_py)
                    .pb(cell_py)
                    .grow(),
            );
        shell = shell.child(empty_row);
    } else {
        let row_count = spec.rows.len();
        for (i, row) in spec.rows.iter().enumerate() {
            let is_last = i == row_count - 1;

            let mut row_el = ui_element::div().flex_row();

            // Add bottom border except on last row
            if !is_last {
                row_el = row_el.border_b_1().border_color(cell_border);
            }

            for col in &spec.columns {
                let value = spec.cell_value(row, &col.id);
                let mut cell = ui_element::label(value)
                    .text_color(cell_text)
                    .text_size(rem_to_px(0.8125))
                    .pl(cell_px)
                    .pr(cell_px)
                    .pt(cell_py)
                    .pb(cell_py)
                    .grow();

                // Row header gets bold weight
                if col.is_row_header {
                    cell = cell.text_weight(600);
                }

                // End-alignment
                if col.align == ColumnAlign::End {
                    cell = cell.text_right();
                }

                row_el = row_el.child(cell);
            }

            shell = shell.child(row_el);
        }
    }

    shell
}
