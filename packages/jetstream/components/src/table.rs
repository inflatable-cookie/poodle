//! Table — Jetstream structured data table backed by TableSpec.
//!
//! Contract: `docs/contracts/components/table.md`
//! Reference: `packages/svelte/components/src/Table.svelte`
//!
//! Renders a shell container with column headers, body rows, and optional
//! caption / empty state.  All visual properties resolve from tokens; size
//! scales typography + vertical padding, density scales horizontal padding.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ColumnAlign, TableSpec};

use crate::presentation::{
    rem_to_px, resolve_semantic_size, table_cell_pad_block_rem, table_cell_pad_inline_rem,
    table_font_rem, table_header_font_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_radius, tint};

pub fn js_table(spec: &TableSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Token resolution
    let shell_border_raw = resolve_color(theme, spec.shell_border_token());
    let shell_border = tint(shell_border_raw, 0.78); // contract §8: border-subtle 78%
    let shell_fill_raw = resolve_color(theme, spec.shell_fill_token());
    let shell_fill = tint(shell_fill_raw, 0.96); // contract §8: panel 96%
    let shell_radius = resolve_radius(theme, spec.shell_radius_token());

    // Contract §8: header bg = color-mix(surface 91%, text-primary).
    let surface = resolve_color(theme, spec.header_surface_token());
    let text_primary = resolve_color(theme, spec.header_mix_text_token());
    let header_fill = color_mix(surface, text_primary, 0.91);

    let header_text = resolve_color(theme, spec.header_text_token());
    let header_border_raw = resolve_color(theme, spec.header_border_token());
    let header_border = tint(header_border_raw, 0.72); // contract §8: border-subtle 72%

    let cell_text = resolve_color(theme, spec.cell_text_token());
    let cell_border_raw = resolve_color(theme, spec.cell_border_token());
    let cell_border = tint(cell_border_raw, 0.72);

    let caption_text = resolve_color(theme, spec.caption_text_token());
    let empty_text = resolve_color(theme, spec.empty_text_token());

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

    // Shell
    let mut shell = ui_element::div()
        .min_w_0()
        .border(1.0)
        .border_color(shell_border)
        .rounded(shell_radius)
        .bg(shell_fill)
        .overflow_hidden()
        .flex_col();

    // Caption (optional) — contract §8 caption rule.
    if let Some(ref caption) = spec.caption {
        shell = shell.child(
            ui_element::label(caption)
                .text_color(caption_text)
                .text_size(caption_font)
                .text_weight(500)
                .pl(caption_px)
                .pr(caption_px)
                .pt(caption_py)
                .pb(caption_py),
        );
    }

    // Header row
    let mut header_row = ui_element::div()
        .bg(header_fill)
        .flex_row()
        .border_b_1()
        .border_color(header_border);

    for col in &spec.columns {
        // Contract §8: header text-transform uppercase + letter-spacing 0.04em.
        let mut header_cell = ui_element::label(col.label.to_uppercase())
            .text_color(header_text)
            .text_size(header_font)
            .text_weight(600)
            .letter_spacing_em(0.04)
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
        // Empty state — single message cell spanning the row.
        let empty_row = ui_element::div().flex_row().child(
            ui_element::label(&spec.empty_message)
                .text_color(empty_text)
                .text_size(table_font)
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

            // Contract §8: last-row cell border removed.
            if !is_last {
                row_el = row_el.border_b_1().border_color(cell_border);
            }

            for col in &spec.columns {
                let value = spec.cell_value(row, &col.id);
                let mut cell = ui_element::label(value)
                    .text_color(cell_text)
                    .text_size(table_font)
                    .pl(cell_px)
                    .pr(cell_px)
                    .pt(cell_py)
                    .pb(cell_py)
                    .grow();

                // Row header gets bold weight (contract §8 row-header cell).
                if col.is_row_header {
                    cell = cell.text_weight(600);
                }

                if col.align == ColumnAlign::End {
                    cell = cell.text_right();
                }

                row_el = row_el.child(cell);
            }

            shell = shell.child(row_el);
        }
    }

    crate::aria::with_aria_label(shell, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{
        ColumnAlign, ControlDensity, ControlSize, TableColumn, TableRow, TableSpec,
    };

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn standard_spec() -> TableSpec {
        TableSpec::new()
            .with_columns(vec![
                TableColumn::new("name", "Name").with_row_header(true),
                TableColumn::new("role", "Role"),
                TableColumn::new("hours", "Hours").with_align(ColumnAlign::End),
            ])
            .with_rows(vec![
                TableRow::new(
                    "r1",
                    vec![
                        ("name".into(), "Ada".into()),
                        ("role".into(), "Lead".into()),
                        ("hours".into(), "40".into()),
                    ],
                ),
                TableRow::new(
                    "r2",
                    vec![
                        ("name".into(), "Bo".into()),
                        ("role".into(), "Eng".into()),
                        ("hours".into(), "32".into()),
                    ],
                ),
            ])
    }

    #[test]
    fn header_labels_uppercased_and_present() {
        let th = theme();
        let tree = probe(&js_table(&standard_spec(), &th), 600.0, 300.0);
        assert!(!tree.is_empty(), "table produced no nodes");
        // Header labels are uppercased per contract §8.
        assert!(tree.has_text("NAME"), "header NAME missing: {:?}", tree.texts());
        assert!(tree.has_text("ROLE"), "header ROLE missing: {:?}", tree.texts());
        assert!(tree.has_text("HOURS"), "header HOURS missing: {:?}", tree.texts());
    }

    #[test]
    fn body_rows_render_cell_values() {
        let th = theme();
        let tree = probe(&js_table(&standard_spec(), &th), 600.0, 300.0);
        for v in ["Ada", "Lead", "40", "Bo", "Eng", "32"] {
            assert!(tree.has_text(v), "cell {v:?} missing: {:?}", tree.texts());
        }
    }

    #[test]
    fn header_fill_is_surface_text_mix() {
        let th = theme();
        let surface = resolve_color(&th, "color.background.surface");
        let text_primary = resolve_color(&th, "color.text.primary");
        let expected = color_mix(surface, text_primary, 0.91);
        let tree = probe(&js_table(&standard_spec(), &th), 600.0, 300.0);
        let color = crate::render_probe::ProbeColor {
            r: expected.x,
            g: expected.y,
            b: expected.z,
            a: expected.w,
        };
        assert!(
            tree.has_background(color, 0.02),
            "header surface/text mix fill not found"
        );
    }

    #[test]
    fn empty_state_shows_message_not_rows() {
        let th = theme();
        let spec = TableSpec::new()
            .with_columns(vec![
                TableColumn::new("name", "Name"),
                TableColumn::new("role", "Role"),
            ])
            .with_empty_message("No team members found.");
        let tree = probe(&js_table(&spec, &th), 600.0, 200.0);
        assert!(
            tree.has_text("No team members found."),
            "empty message missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn density_scales_horizontal_padding_not_height() {
        let th = theme();
        let compact = probe(
            &js_table(&standard_spec().with_density(ControlDensity::Compact), &th),
            600.0,
            300.0,
        );
        let comfortable = probe(
            &js_table(
                &standard_spec().with_density(ControlDensity::Comfortable),
                &th,
            ),
            600.0,
            300.0,
        );
        // Header row is at index 1 (root=0, no caption). Same row height across
        // densities (density never touches vertical padding); the row is wider
        // when comfortable because horizontal padding-inline grows.
        let compact_header_h = compact.nodes[1].h;
        let comfortable_header_h = comfortable.nodes[1].h;
        assert!(
            (compact_header_h - comfortable_header_h).abs() < 0.01,
            "density changed header height: {compact_header_h} vs {comfortable_header_h}"
        );
    }

    #[test]
    fn size_scales_vertical_padding() {
        let th = theme();
        let xs = probe(
            &js_table(&standard_spec().with_size(ControlSize::Xs), &th),
            600.0,
            300.0,
        );
        let xl = probe(
            &js_table(&standard_spec().with_size(ControlSize::Xl), &th),
            600.0,
            300.0,
        );
        // xl has larger padding-block → taller header row than xs.
        assert!(
            xl.nodes[1].h > xs.nodes[1].h,
            "xl header ({}) not taller than xs ({})",
            xl.nodes[1].h,
            xs.nodes[1].h
        );
    }
}
