//! Table — structured data table shell with header, rows, caption/empty.
//!
//! Contract: `docs/contracts/components/table.md`
//! Ported from: `packages/jetstream/components/src/table.rs`.
//!
//! All visual properties resolve from tokens; size scales typography +
//! vertical padding, density scales horizontal padding.

use poodle_adapter::ThemeProvider;
use poodle_node::{LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node};
use poodle_specs::{ColumnAlign, TableSpec};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{
    rem_to_px, resolve_semantic_size, table_cell_pad_block_rem, table_cell_pad_inline_rem,
    table_font_rem, table_header_font_rem,
};

pub fn table(spec: &TableSpec, theme: &dyn ThemeProvider) -> Node {
    // Token resolution
    let shell_border_raw = theme.resolve_color(spec.shell_border_token());
    let shell_border = with_alpha(shell_border_raw, shell_border_raw.3 * 0.78); // contract §8: border-subtle 78%
    let shell_fill_raw = theme.resolve_color(spec.shell_fill_token());
    let shell_fill = with_alpha(shell_fill_raw, shell_fill_raw.3 * 0.96); // contract §8: panel 96%
    let shell_radius = theme.resolve_radius(spec.shell_radius_token());

    // Contract §8: header bg = color-mix(surface 91%, text-primary).
    let surface = theme.resolve_color(spec.header_surface_token());
    let text_primary = theme.resolve_color(spec.header_mix_text_token());
    let header_fill = mix_srgb(surface, text_primary, 0.91);

    let header_text = theme.resolve_color(spec.header_text_token());
    let header_border_raw = theme.resolve_color(spec.header_border_token());
    let header_border = with_alpha(header_border_raw, header_border_raw.3 * 0.72); // contract §8: 72%

    let cell_text = theme.resolve_color(spec.cell_text_token());
    let cell_border_raw = theme.resolve_color(spec.cell_border_token());
    let cell_border = with_alpha(cell_border_raw, cell_border_raw.3 * 0.72);

    let caption_text = theme.resolve_color(spec.caption_text_token());
    let empty_text = theme.resolve_color(spec.empty_text_token());

    // Contract §8: size scales font + vertical padding-block; density scales
    // horizontal padding-inline only. Caption padding/font are fixed rules.
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let cell_py = rem_to_px(table_cell_pad_block_rem(effective_size));
    let cell_px = rem_to_px(table_cell_pad_inline_rem(spec.density));
    let table_font = rem_to_px(table_font_rem(effective_size));
    let header_font = rem_to_px(table_header_font_rem(effective_size));
    let caption_px = rem_to_px(0.75);
    let caption_py = rem_to_px(0.625);
    let caption_font = rem_to_px(0.8125);

    let cell = |content: String, color, size, weight: Option<u16>, align_end: bool| -> Node {
        let mut c = Node::text(content);
        {
            let s = &mut c.style;
            s.descriptor.text_color = Some(color);
            s.text_size = Some(size);
            s.text_weight = weight;
            s.line_height = Some(1.5);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = cell_px;
            pad.right = cell_px;
            pad.top = cell_py;
            pad.bottom = cell_py;
            s.descriptor.layout.width = LayoutSizing::Grow;
            // GPUI's `.flex_1()` uses a zero basis; pairing it with grow keeps
            // all table columns equal instead of sizing from intrinsic text.
            s.flex_basis = Some(0.0);
            if align_end {
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            }
        }
        c
    };

    // Shell
    let mut shell = Node::container();
    {
        let s = &mut shell.style;
        s.min_width = Some(0.0);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = shell_border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = shell_radius;
        c.top_right = shell_radius;
        c.bottom_right = shell_radius;
        c.bottom_left = shell_radius;
        s.descriptor.background = Some(shell_fill);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.layout.direction = LayoutDirection::Column;
    }
    let mut shell = shell;

    // Caption (optional) — contract §8 caption rule.
    if let Some(ref caption) = spec.caption {
        let mut label = Node::text(caption);
        {
            let s = &mut label.style;
            s.descriptor.text_color = Some(caption_text);
            s.text_size = Some(caption_font);
            s.text_weight = Some(500);
            s.line_height = Some(1.5);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = caption_px;
            pad.right = caption_px;
            pad.top = caption_py;
            pad.bottom = caption_py;
        }
        shell = shell.child(label);
    }

    // Header row
    let mut header_row = Node::container();
    {
        let s = &mut header_row.style;
        s.descriptor.background = Some(header_fill);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.border_bottom_width = Some(1.0);
        s.descriptor.border.color = header_border;
    }
    let mut header_row = header_row;

    for col in &spec.columns {
        // Contract §8: header text-transform uppercase + letter-spacing 0.04em.
        let mut header_cell = cell(
            col.label.to_uppercase(),
            header_text,
            header_font,
            Some(600),
            col.align == ColumnAlign::End,
        );
        header_cell.style.letter_spacing_em = Some(0.04);
        header_row = header_row.child(header_cell);
    }
    shell = shell.child(header_row);

    // Body rows
    if spec.is_empty() {
        // Empty state — single message cell spanning the row.
        let mut empty_row = Node::container();
        empty_row.style.descriptor.layout.direction = LayoutDirection::Row;
        shell = shell.child(empty_row.child(cell(
            spec.empty_message.clone(),
            empty_text,
            table_font,
            None,
            false,
        )));
    } else {
        let row_count = spec.rows.len();
        for (i, row) in spec.rows.iter().enumerate() {
            let is_last = i == row_count - 1;

            let mut row_el = Node::container();
            row_el.style.descriptor.layout.direction = LayoutDirection::Row;
            // Contract §8: last-row cell border removed.
            if !is_last {
                row_el.style.border_bottom_width = Some(1.0);
                row_el.style.descriptor.border.color = cell_border;
            }
            let mut row_el = row_el;

            for col in &spec.columns {
                let value = spec.cell_value(row, &col.id);
                row_el = row_el.child(cell(
                    value.to_string(),
                    cell_text,
                    table_font,
                    // Row header gets bold weight (contract §8 row-header cell).
                    if col.is_row_header { Some(600) } else { None },
                    col.align == ColumnAlign::End,
                ));
            }

            shell = shell.child(row_el);
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            shell.a11y.label = Some(label.to_string());
        }
    }
    shell
}
