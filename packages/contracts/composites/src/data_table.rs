use pug_primitives::CheckState;
use pug_tokens::semantic;

use crate::types::{TableColumnSpec, TableRowSpec, TableSortDirection};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataTableSpec {
    pub columns: Vec<TableColumnSpec>,
    pub rows: Vec<TableRowSpec>,
    pub selected_row_ids: Vec<String>,
    pub sort_column_id: Option<String>,
    pub sort_direction: TableSortDirection,
    pub row_action_label: String,
    pub show_row_actions: bool,
    pub empty_message: Option<String>,
    pub aria_label: String,
}

impl DataTableSpec {
    pub fn new(columns: Vec<TableColumnSpec>, rows: Vec<TableRowSpec>) -> Self {
        Self {
            columns,
            rows,
            selected_row_ids: Vec::new(),
            sort_column_id: None,
            sort_direction: TableSortDirection::Asc,
            row_action_label: String::from("Open"),
            show_row_actions: true,
            empty_message: None,
            aria_label: String::from("Data table"),
        }
    }

    pub fn with_selected_row_ids(mut self, selected_row_ids: Vec<String>) -> Self {
        self.selected_row_ids = selected_row_ids;
        self
    }

    pub fn with_sort(
        mut self,
        column_id: impl Into<String>,
        direction: TableSortDirection,
    ) -> Self {
        self.sort_column_id = Some(column_id.into());
        self.sort_direction = direction;
        self
    }

    pub fn with_row_action_label(mut self, row_action_label: impl Into<String>) -> Self {
        self.row_action_label = row_action_label.into();
        self
    }

    pub fn with_show_row_actions(mut self, show_row_actions: bool) -> Self {
        self.show_row_actions = show_row_actions;
        self
    }

    pub fn with_empty_message(mut self, empty_message: impl Into<String>) -> Self {
        self.empty_message = Some(empty_message.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    pub fn visible_row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn selected_visible_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| {
                self.selected_row_ids
                    .iter()
                    .any(|selected| selected == &row.id)
            })
            .count()
    }

    pub fn select_all_state(&self) -> CheckState {
        let selected_count = self.selected_visible_row_count();
        if selected_count == 0 {
            CheckState::Unchecked
        } else if selected_count == self.visible_row_count() && selected_count > 0 {
            CheckState::Checked
        } else {
            CheckState::Mixed
        }
    }

    pub fn sortable_column_count(&self) -> usize {
        self.columns
            .iter()
            .filter(|column| column.is_sortable)
            .count()
    }

    pub fn header_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
}
