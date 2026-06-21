use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ColumnAlign, TableColumn, TableRow, TableSpec};

use crate::presentation::{
    rem_to_px, resolve_semantic_size, table_cell_pad_block_rem, table_cell_pad_inline_rem,
    table_font_rem, table_header_font_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

pub struct Table {
    spec: TableSpec,
    // Pre-resolved values
    shell_border: Hsla,
    shell_fill: Hsla,
    shell_radius: Pixels,
    header_fill: Hsla,
    header_text: Hsla,
    header_border: Hsla,
    cell_text: Hsla,
    cell_border: Hsla,
    caption_text: Hsla,
    empty_text: Hsla,
}

impl std::ops::Deref for Table {
    type Target = TableSpec;
    fn deref(&self) -> &TableSpec {
        &self.spec
    }
}

impl Table {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(TableSpec::new(), theme)
    }

    pub fn from_spec(spec: TableSpec, theme: &GpuiThemeProvider) -> Self {
        let shell_fill_raw = resolve_color(theme, spec.shell_fill_token());
        let shell_border_raw = resolve_color(theme, spec.shell_border_token());
        let header_border_raw = resolve_color(theme, spec.header_border_token());
        let cell_border_raw = resolve_color(theme, spec.cell_border_token());

        // Contract §8: shell background = color-mix(panel 96%, transparent),
        // shell border = color-mix(border-subtle 78%, transparent) — alpha cuts.
        let shell_fill = Hsla { a: shell_fill_raw.a * 0.96, ..shell_fill_raw };
        let shell_border = Hsla { a: shell_border_raw.a * 0.78, ..shell_border_raw };

        // Contract §8: header bg = color-mix(surface 91%, text-primary).
        let surface = resolve_color(theme, spec.header_surface_token());
        let text_primary = resolve_color(theme, spec.header_mix_text_token());
        let header_fill = color_mix(surface, text_primary, 0.91);

        // Contract §8: header/cell border-bottom = color-mix(border-subtle 72%, transparent).
        let header_border = Hsla { a: header_border_raw.a * 0.72, ..header_border_raw };
        let cell_border = Hsla { a: cell_border_raw.a * 0.72, ..cell_border_raw };

        Self {
            shell_border,
            shell_fill,
            shell_radius: resolve_radius(theme, spec.shell_radius_token()),
            header_fill,
            header_text: resolve_color(theme, spec.header_text_token()),
            header_border,
            cell_text: resolve_color(theme, spec.cell_text_token()),
            cell_border,
            caption_text: resolve_color(theme, spec.caption_text_token()),
            empty_text: resolve_color(theme, spec.empty_text_token()),
            spec,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn columns(mut self, v: Vec<TableColumn>) -> Self {
        self.spec.columns = v;
        self
    }
    pub fn rows(mut self, v: Vec<TableRow>) -> Self {
        self.spec.rows = v;
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.spec.caption = Some(v.into());
        self
    }
    pub fn empty_message(mut self, v: impl Into<String>) -> Self {
        self.spec.empty_message = v.into();
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn size(mut self, v: poodle_specs::ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
}

impl IntoElement for Table {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        // Contract §8: size scales vertical padding-block + fonts; density
        // scales horizontal padding-inline. Resolve the effective size via the
        // semantic size role, then derive every dimension from the rem scales.
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let cell_pad_v = px(rem_to_px(table_cell_pad_block_rem(effective_size)));
        let cell_pad_h = px(rem_to_px(table_cell_pad_inline_rem(self.spec.density)));
        let table_font = px(rem_to_px(table_font_rem(effective_size)));
        let header_font = px(rem_to_px(table_header_font_rem(effective_size)));
        // Svelte table line-height is 1.5 (contract §8 Table type).
        let line = relative(1.5);

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .bg(self.shell_fill)
            .border_1()
            .border_color(self.shell_border)
            .rounded(self.shell_radius)
            .overflow_hidden();

        // Caption — contract §8 caption rule: fixed padding 0.625rem 0.75rem,
        // font-size 0.8125rem (not size-scaled).
        if let Some(caption) = &self.spec.caption {
            root = root.child(
                div()
                    .px(px(rem_to_px(0.75)))
                    .py(px(rem_to_px(0.625)))
                    .text_color(self.caption_text)
                    .text_size(px(rem_to_px(0.8125)))
                    .font_weight(FontWeight::MEDIUM)
                    .line_height(line)
                    .child(caption.clone()),
            );
        }

        // Header row
        if !self.spec.columns.is_empty() {
            let mut header_row = div()
                .flex()
                .flex_row()
                .w_full()
                .bg(self.header_fill)
                .border_b_1()
                .border_color(self.header_border);

            for col in &self.spec.columns {
                let mut cell = div()
                    .flex_1()
                    .px(cell_pad_h)
                    .py(cell_pad_v)
                    .text_color(self.header_text)
                    .text_size(header_font)
                    .line_height(line)
                    .font_weight(FontWeight::SEMIBOLD);

                if col.align == ColumnAlign::End {
                    cell = cell.flex().justify_end();
                }

                // Contract §8: header text-transform uppercase. letter-spacing
                // 0.04em is also specified; GPUI has no per-run letter-spacing,
                // so spacing is an accepted approximation (uppercase applied).
                cell = cell.child(col.label.to_uppercase());
                header_row = header_row.child(cell);
            }

            root = root.child(header_row);
        }

        // Data rows or empty state
        if self.spec.is_empty() {
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(cell_pad_v)
                    .px(cell_pad_h)
                    .text_color(self.empty_text)
                    .text_size(table_font)
                    .line_height(line)
                    .child(self.spec.empty_message.clone()),
            );
        } else {
            let row_count = self.spec.rows.len();
            for (row_idx, row) in self.spec.rows.iter().enumerate() {
                let is_last_row = row_idx == row_count - 1;

                let mut row_el = div().flex().flex_row().w_full();

                if !is_last_row {
                    row_el = row_el.border_b_1().border_color(self.cell_border);
                }

                for col in &self.spec.columns {
                    let value = self.spec.cell_value(row, &col.id);

                    let mut cell = div()
                        .flex_1()
                        .px(cell_pad_h)
                        .py(cell_pad_v)
                        .text_color(self.cell_text)
                        .text_size(table_font)
                        .line_height(line);

                    if col.align == ColumnAlign::End {
                        cell = cell.flex().justify_end();
                    }

                    if col.is_row_header {
                        cell = cell.font_weight(FontWeight::SEMIBOLD);
                    }

                    cell = cell.child(value.to_string());
                    row_el = row_el.child(cell);
                }

                root = root.child(row_el);
            }
        }

        root.into_any_element()
    }
}
