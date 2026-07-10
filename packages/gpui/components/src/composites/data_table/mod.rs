//! DataTable — real GPUI component backed by DataTableSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, SemanticControlSizeRole,
};
use poodle_specs::{
    DataTableSpec, TableColumnSpec, TableFilter, TablePagination, TableRowSpec, TableSortDirection,
};


/// A real GPUI data table component backed by `DataTableSpec`.
///
/// Renders a header row with column labels, data rows with cell values,
/// and highlights selected rows.
pub struct DataTable {
    spec: DataTableSpec,
    theme: GpuiThemeProvider,
    on_row_click: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_sort: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    /// Fired when a row selection checkbox is toggled.
    /// Args: row_id, new selected state.
    on_row_select: Option<Box<dyn Fn(&str, bool, &mut Window, &mut App) + 'static>>,
    /// Fired when the "select all" header checkbox is toggled.
    on_select_all: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    /// Fired when the expand chevron on a row is clicked.
    on_row_expand: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    /// Fired when the page changes via the pagination footer.
    on_page_change: Option<Box<dyn Fn(u32, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for DataTable {
    type Target = DataTableSpec;
    fn deref(&self) -> &DataTableSpec {
        &self.spec
    }
}

impl DataTable {
    pub fn new(
        columns: Vec<TableColumnSpec>,
        rows: Vec<TableRowSpec>,
        theme: &GpuiThemeProvider,
    ) -> Self {
        Self {
            spec: DataTableSpec::new(columns, rows),
            theme: theme.clone(),
            on_row_click: None,
            on_sort: None,
            on_row_select: None,
            on_select_all: None,
            on_row_expand: None,
            on_page_change: None,
        }
    }

    pub fn from_spec(spec: DataTableSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_row_click: None,
            on_sort: None,
            on_row_select: None,
            on_select_all: None,
            on_row_expand: None,
            on_page_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn columns(mut self, v: Vec<TableColumnSpec>) -> Self {
        self.spec.columns = v;
        self
    }
    pub fn rows(mut self, v: Vec<TableRowSpec>) -> Self {
        self.spec.rows = v;
        self
    }
    pub fn selected_row_ids(mut self, v: Vec<String>) -> Self {
        self.spec.selected_row_ids = v;
        self
    }
    pub fn sort_column_id(mut self, v: impl Into<String>) -> Self {
        self.spec.sort_column_id = Some(v.into());
        self
    }
    pub fn sort_direction(mut self, v: TableSortDirection) -> Self {
        self.spec.sort_direction = v;
        self
    }
    pub fn row_action_label(mut self, v: impl Into<String>) -> Self {
        self.spec.row_action_label = v.into();
        self
    }
    pub fn show_row_actions(mut self, v: bool) -> Self {
        self.spec.show_row_actions = v;
        self
    }
    pub fn empty_message(mut self, v: impl Into<String>) -> Self {
        self.spec.empty_message = Some(v.into());
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = v.into();
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn on_row_click(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_row_click = Some(Box::new(handler));
        self
    }

    pub fn on_sort(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_sort = Some(Box::new(handler));
        self
    }

    pub fn on_row_select(
        mut self,
        handler: impl Fn(&str, bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_row_select = Some(Box::new(handler));
        self
    }

    pub fn on_select_all(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select_all = Some(Box::new(handler));
        self
    }

    pub fn on_row_expand(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_row_expand = Some(Box::new(handler));
        self
    }

    pub fn on_page_change(
        mut self,
        handler: impl Fn(u32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_page_change = Some(Box::new(handler));
        self
    }

    // ── Forwarded spec builders for the new fields ─────────────
    pub fn selectable(mut self, v: bool) -> Self {
        self.spec.selectable = v;
        self
    }
    pub fn hidden_column_ids(mut self, v: Vec<String>) -> Self {
        self.spec.hidden_column_ids = v;
        self
    }
    pub fn show_column_visibility(mut self, v: bool) -> Self {
        self.spec.show_column_visibility = v;
        self
    }
    pub fn show_export(mut self, v: bool) -> Self {
        self.spec.show_export = v;
        self
    }
    pub fn filters(mut self, v: Vec<TableFilter>) -> Self {
        self.spec.filters = v;
        self
    }
    pub fn pagination(mut self, v: TablePagination) -> Self {
        self.spec.pagination = Some(v);
        self
    }
    pub fn expanded_row_ids(mut self, v: Vec<String>) -> Self {
        self.spec.expanded_row_ids = v;
        self
    }
    pub fn compact(mut self, v: bool) -> Self {
        self.spec.compact = v;
        self
    }
    pub fn striped(mut self, v: bool) -> Self {
        self.spec.striped = v;
        self
    }
    pub fn sticky_header(mut self, v: bool) -> Self {
        self.spec.sticky_header = v;
        self
    }
}

impl IntoElement for DataTable {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        self.render()
    }
}

mod render;
