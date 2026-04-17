use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ColumnAlign, TableColumn, TableRow, TableSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

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
    label_size: Pixels,
}

impl std::ops::Deref for Table {
    type Target = TableSpec;
    fn deref(&self) -> &TableSpec {
        &self.spec
    }
}

impl Table {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        let spec = TableSpec::new();
        let shell_fill_raw = resolve_color(theme, spec.shell_fill_token());
        let shell_border_raw = resolve_color(theme, spec.shell_border_token());
        let _header_fill_raw = resolve_color(theme, spec.header_fill_token());
        let header_border_raw = resolve_color(theme, spec.header_border_token());
        let cell_border_raw = resolve_color(theme, spec.cell_border_token());

        // Svelte: color-mix(X N%, transparent) = alpha reduction
        let shell_fill = Hsla { a: shell_fill_raw.a * 0.96, ..shell_fill_raw };
        let shell_border = Hsla { a: shell_border_raw.a * 0.78, ..shell_border_raw };

        // Svelte: header bg = color-mix(surface 91%, text-primary)
        let surface = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let header_fill = color_mix(surface, text_primary, 0.91);

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
            label_size: resolve_px(theme, "typography.label.size"),
            spec,
        }
    }

    pub fn from_spec(spec: TableSpec, theme: &GpuiThemeProvider) -> Self {
        let shell_fill_raw = resolve_color(theme, spec.shell_fill_token());
        let shell_border_raw = resolve_color(theme, spec.shell_border_token());
        let _header_fill_raw = resolve_color(theme, spec.header_fill_token());
        let header_border_raw = resolve_color(theme, spec.header_border_token());
        let cell_border_raw = resolve_color(theme, spec.cell_border_token());

        // Svelte: color-mix(X N%, transparent) = alpha reduction
        let shell_fill = Hsla { a: shell_fill_raw.a * 0.96, ..shell_fill_raw };
        let shell_border = Hsla { a: shell_border_raw.a * 0.78, ..shell_border_raw };

        // Svelte: header bg = color-mix(surface 91%, text-primary)
        let surface = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let header_fill = color_mix(surface, text_primary, 0.91);

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
            label_size: resolve_px(theme, "typography.label.size"),
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
}

impl IntoElement for Table {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let cell_pad_v = px(rem_to_px(0.5));    // Svelte: 0.5rem (Md default)
        let cell_pad_h = px(rem_to_px(0.75));   // Svelte: 0.75rem (default density)
        let _column_count = self.spec.columns.len();

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .bg(self.shell_fill)
            .border_1()
            .border_color(self.shell_border)
            .rounded(self.shell_radius)
            .overflow_hidden();

        // Caption
        if let Some(caption) = &self.spec.caption {
            root = root.child(
                div()
                    .px(cell_pad_h)
                    .py(px(rem_to_px(0.625))) // Svelte: 0.625rem
                    .text_color(self.caption_text)
                    .text_size(px(rem_to_px(0.8125))) // Svelte: 0.8125rem
                    .line_height(relative(1.4))
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
                    .text_size(px(rem_to_px(0.6875)))
                    .line_height(relative(1.4))
                    .font_weight(FontWeight::SEMIBOLD);

                if col.align == ColumnAlign::End {
                    cell = cell.flex().justify_end();
                }

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
                    .py(px(rem_to_px(2.0)))
                    .px(cell_pad_h)
                    .text_color(self.empty_text)
                    .text_size(self.label_size)
                    .line_height(relative(1.4))
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
                        .text_size(self.label_size)
                        .line_height(relative(1.4));

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
