//! DataTable — real GPUI component backed by DataTableSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::{DataTableSpec, TableColumnSpec, TableRowSpec, TableSortDirection};

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI data table component backed by `DataTableSpec`.
///
/// Renders a header row with column labels, data rows with cell values,
/// and highlights selected rows.
pub struct DataTable {
    spec: DataTableSpec,
    theme: GpuiThemeProvider,
    on_row_click: Option<Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
    on_sort: Option<Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for DataTable {
    type Target = DataTableSpec;
    fn deref(&self) -> &DataTableSpec { &self.spec }
}

impl DataTable {
    pub fn new(columns: Vec<TableColumnSpec>, rows: Vec<TableRowSpec>, theme: &GpuiThemeProvider) -> Self {
        Self { spec: DataTableSpec::new(columns, rows), theme: theme.clone(), on_row_click: None, on_sort: None }
    }

    pub fn from_spec(spec: DataTableSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_row_click: None,
            on_sort: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn columns(mut self, v: Vec<TableColumnSpec>) -> Self { self.spec.columns = v; self }
    pub fn rows(mut self, v: Vec<TableRowSpec>) -> Self { self.spec.rows = v; self }
    pub fn selected_row_ids(mut self, v: Vec<String>) -> Self { self.spec.selected_row_ids = v; self }
    pub fn sort_column_id(mut self, v: impl Into<String>) -> Self { self.spec.sort_column_id = Some(v.into()); self }
    pub fn sort_direction(mut self, v: TableSortDirection) -> Self { self.spec.sort_direction = v; self }
    pub fn row_action_label(mut self, v: impl Into<String>) -> Self { self.spec.row_action_label = v.into(); self }
    pub fn show_row_actions(mut self, v: bool) -> Self { self.spec.show_row_actions = v; self }
    pub fn empty_message(mut self, v: impl Into<String>) -> Self { self.spec.empty_message = Some(v.into()); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = v.into(); self }


    pub fn on_row_click(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_row_click = Some(Box::new(handler));
        self
    }

    pub fn on_sort(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_sort = Some(Box::new(handler));
        self
    }
}

impl IntoElement for DataTable {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_padding = resolve_px(theme, "semantic.space.inline.md");

        let header_bg = resolve_color(theme, spec.header_fill_token());
        let border_color = resolve_color(theme, "semantic.color.border.subtle");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let mut table = div()
            .w_full()
            .flex()
            .flex_col()
            .border_1()
            .border_color(border_color)
            .rounded(px(4.0))
            .overflow_hidden();

        // Header row
        let mut header_row = div()
            .w_full()
            .flex()
            .bg(header_bg)
            .border_b_1()
            .border_color(border_color);

        for col in &spec.columns {
            let label = col.label.clone();
            let is_sorted = spec
                .sort_column_id
                .as_ref()
                .is_some_and(|sid| sid == &col.id);

            let mut header_cell = div()
                .flex_1()
                .px(inline_padding)
                .py(px(8.0))
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_secondary)
                .when(col.align_end, |el| el.text_right());

            if is_sorted {
                let direction_indicator = match spec.sort_direction {
                    pug_composites::TableSortDirection::Asc => " \u{2191}",
                    pug_composites::TableSortDirection::Desc => " \u{2193}",
                };
                header_cell = header_cell.child(format!("{}{}", label, direction_indicator));
            } else {
                header_cell = header_cell.child(label);
            }

            if col.is_sortable {
                header_cell = header_cell.cursor_pointer();
            }

            header_row = header_row.child(header_cell);
        }

        // Row action column header
        if spec.show_row_actions {
            header_row = header_row.child(
                div()
                    .w(px(80.0))
                    .px(inline_padding)
                    .py(px(8.0))
                    .text_xs()
                    .text_color(text_secondary),
            );
        }

        table = table.child(header_row);

        // Data rows or empty message
        if spec.rows.is_empty() {
            let empty_msg = spec
                .empty_message
                .as_deref()
                .unwrap_or("No data available");
            table = table.child(
                div()
                    .w_full()
                    .px(inline_padding)
                    .py(px(24.0))
                    .text_size(px(14.0))
                    .text_color(text_secondary)
                    .text_center()
                    .child(String::from(empty_msg)),
            );
        } else {
            for row in &spec.rows {
                let is_selected = spec
                    .selected_row_ids
                    .iter()
                    .any(|sid| sid == &row.id);

                let row_hover_bg = accent.opacity(0.04);
                let selected_bg = accent.opacity(0.08);

                let mut data_row = div()
                    .w_full()
                    .flex()
                    .border_b_1()
                    .border_color(border_color.opacity(0.5))
                    .when(is_selected, |el| el.bg(selected_bg))
                    .when(!is_selected, move |el| {
                        el.hover(move |s| s.bg(row_hover_bg))
                    });

                for col in &spec.columns {
                    let cell_value = row
                        .cells
                        .iter()
                        .find(|(key, _)| key == &col.id)
                        .map(|(_, val)| val.clone())
                        .unwrap_or_default();

                    let cell = div()
                        .flex_1()
                        .px(inline_padding)
                        .py(px(10.0))
                        .text_size(px(14.0))
                        .text_color(text_primary)
                        .when(col.align_end, |el| el.text_right())
                        .child(cell_value);

                    data_row = data_row.child(cell);
                }

                // Row action button
                if spec.show_row_actions {
                    let action_label = spec.row_action_label.clone();
                    data_row = data_row.child(
                        div()
                            .w(px(80.0))
                            .px(inline_padding)
                            .py(px(10.0))
                            .text_xs()
                            .text_color(accent)
                            .child(action_label),
                    );
                }

                table = table.child(data_row);
            }
        }

        table.into_any_element()
    }
}
