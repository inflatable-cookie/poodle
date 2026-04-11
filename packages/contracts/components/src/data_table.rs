use crate::{CheckState, ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

use crate::composite_types::{
    TableColumnSpec, TableFilter, TablePagination, TableRowSpec, TableSortDirection,
};

#[derive(Clone, Debug, PartialEq)]
pub struct DataTableSpec {
    pub columns: Vec<TableColumnSpec>,
    pub rows: Vec<TableRowSpec>,
    /// When true the table renders a row-selection checkbox column and
    /// a "select all" header checkbox. Matches Svelte `selectable` prop.
    pub selectable: bool,
    pub selected_row_ids: Vec<String>,
    pub sort_column_id: Option<String>,
    pub sort_direction: TableSortDirection,
    pub row_action_label: String,
    pub show_row_actions: bool,
    /// Hidden column ids — rendered rows skip these columns.
    pub hidden_column_ids: Vec<String>,
    /// When true the header renders a column-visibility menu button.
    pub show_column_visibility: bool,
    /// When true the header renders an export button.
    pub show_export: bool,
    /// Active filters keyed by column id. Rendered as chips in a
    /// filter row above the header.
    pub filters: Vec<TableFilter>,
    /// Pagination state. When Some, the table renders a pager footer.
    pub pagination: Option<TablePagination>,
    /// Expanded row ids. When a row id is in this list, the row renders
    /// its `summary` below the main row. Matches Svelte `expandedRowIds`.
    pub expanded_row_ids: Vec<String>,
    /// When true the table uses compact row heights.
    pub compact: bool,
    /// When true alternate rows receive a subtle background tint.
    pub striped: bool,
    /// When true the header sticks to the top of its scroll container
    /// (consumer must provide the scrolling parent).
    pub sticky_header: bool,
    pub empty_message: Option<String>,
    pub aria_label: String,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl DataTableSpec {
    pub fn new(columns: Vec<TableColumnSpec>, rows: Vec<TableRowSpec>) -> Self {
        Self {
            columns,
            rows,
            selectable: false,
            selected_row_ids: Vec::new(),
            sort_column_id: None,
            sort_direction: TableSortDirection::Asc,
            row_action_label: String::from("Open"),
            show_row_actions: true,
            hidden_column_ids: Vec::new(),
            show_column_visibility: false,
            show_export: false,
            filters: Vec::new(),
            pagination: None,
            expanded_row_ids: Vec::new(),
            compact: false,
            striped: false,
            sticky_header: false,
            empty_message: None,
            aria_label: String::from("Data table"),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
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

    pub fn with_selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    pub fn with_hidden_column_ids(mut self, hidden: Vec<String>) -> Self {
        self.hidden_column_ids = hidden;
        self
    }

    pub fn with_show_column_visibility(mut self, show: bool) -> Self {
        self.show_column_visibility = show;
        self
    }

    pub fn with_show_export(mut self, show: bool) -> Self {
        self.show_export = show;
        self
    }

    pub fn with_filters(mut self, filters: Vec<TableFilter>) -> Self {
        self.filters = filters;
        self
    }

    pub fn with_pagination(mut self, pagination: TablePagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    pub fn with_expanded_row_ids(mut self, ids: Vec<String>) -> Self {
        self.expanded_row_ids = ids;
        self
    }

    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn with_striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn with_sticky_header(mut self, sticky: bool) -> Self {
        self.sticky_header = sticky;
        self
    }

    /// Whether a column is currently visible (not in hidden_column_ids).
    pub fn is_column_visible(&self, column_id: &str) -> bool {
        !self.hidden_column_ids.iter().any(|id| id == column_id)
    }

    /// Iterator over currently-visible columns.
    pub fn visible_columns(&self) -> impl Iterator<Item = &TableColumnSpec> {
        self.columns
            .iter()
            .filter(|col| self.is_column_visible(&col.id))
    }

    pub fn is_row_expanded(&self, row_id: &str) -> bool {
        self.expanded_row_ids.iter().any(|id| id == row_id)
    }

    pub fn has_filters(&self) -> bool {
        !self.filters.is_empty()
    }

    pub fn has_pagination(&self) -> bool {
        self.pagination.is_some()
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

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
