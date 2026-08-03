use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;
use std::collections::BTreeMap;

/// A single option for a `select`-type [`LogFilter`].
///
/// Mirrors the Svelte `LogFilter.options` shape (`{ value, label }`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogFilterOption {
    pub value: String,
    pub label: String,
}

impl LogFilterOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// The control kind a [`LogFilter`] renders as. Matches Svelte
/// `LogFilter.type` (`"select" | "date"`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LogFilterKind {
    #[default]
    Select,
    Date,
}

/// An audit-mode filter definition for the LogList toolbar.
///
/// Mirrors the Svelte `LogFilter` type: a labelled `select` (with options) or
/// `date` control keyed by `field`. The current value is looked up from
/// [`LogListSpec::filter_values`] by `field`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogFilter {
    /// Key under which the value is stored in `filter_values` and reported
    /// back through the filter-change callback.
    pub field: String,
    /// Human-readable label rendered above the control.
    pub label: String,
    /// Control kind — select or date.
    pub kind: LogFilterKind,
    /// Options for `select` filters. Empty for `date` filters.
    pub options: Vec<LogFilterOption>,
    /// Placeholder / "All" option label for `select` filters.
    pub placeholder: Option<String>,
}

impl LogFilter {
    /// New `select` filter with the given field key and label.
    pub fn select(field: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            label: label.into(),
            kind: LogFilterKind::Select,
            options: Vec::new(),
            placeholder: None,
        }
    }

    /// New `date` filter with the given field key and label.
    pub fn date(field: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            label: label.into(),
            kind: LogFilterKind::Date,
            options: Vec::new(),
            placeholder: None,
        }
    }

    pub fn with_option(mut self, value: impl Into<String>, label: impl Into<String>) -> Self {
        self.options.push(LogFilterOption::new(value, label));
        self
    }

    pub fn with_options(mut self, options: impl IntoIterator<Item = LogFilterOption>) -> Self {
        self.options.extend(options);
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogListSpec {
    pub entry_count: usize,
    pub max_entries: usize,
    pub auto_scroll: bool,
    pub filter_level: Option<String>,
    /// Client-side text filter for stream entries (Svelte `filterText`).
    pub filter_text: String,
    /// Audit-mode loading flag; the loading surface shows when there are no
    /// current entries (Svelte `loading`).
    pub loading: bool,
    /// Audit-mode error message; rendered in the error surface (Svelte `error`).
    pub error: Option<String>,
    /// Empty-state copy for audit mode (Svelte `emptyMessage`).
    pub empty_message: String,
    /// Audit-mode filter definitions for the toolbar (Svelte `filters`).
    pub filters: Vec<LogFilter>,
    /// Current filter values keyed by `LogFilter::field` (Svelte `filterValues`).
    /// `BTreeMap` for deterministic iteration order.
    pub filter_values: BTreeMap<String, String>,
    /// Current page for audit pagination, 1-based (Svelte `page`).
    pub page: usize,
    /// Page size used for pagination copy (Svelte `pageSize`).
    pub page_size: usize,
    /// Total row count; enables pagination when greater than `page_size`
    /// (Svelte `total`). `None` disables pagination.
    pub total: Option<usize>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl LogListSpec {
    pub fn new() -> Self {
        Self {
            entry_count: 0,
            max_entries: 500,
            auto_scroll: true,
            filter_level: None,
            filter_text: String::new(),
            loading: false,
            error: None,
            empty_message: String::from("No log entries found"),
            filters: Vec::new(),
            filter_values: BTreeMap::new(),
            page: 1,
            page_size: 50,
            total: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_entry_count(mut self, entry_count: usize) -> Self {
        self.entry_count = entry_count;
        self
    }

    pub fn with_max_entries(mut self, max_entries: usize) -> Self {
        self.max_entries = max_entries;
        self
    }

    pub fn with_auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.auto_scroll = auto_scroll;
        self
    }

    pub fn with_filter_level(mut self, filter_level: impl Into<String>) -> Self {
        self.filter_level = Some(filter_level.into());
        self
    }

    pub fn with_filter_text(mut self, filter_text: impl Into<String>) -> Self {
        self.filter_text = filter_text.into();
        self
    }

    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn with_empty_message(mut self, empty_message: impl Into<String>) -> Self {
        self.empty_message = empty_message.into();
        self
    }

    pub fn with_filter(mut self, filter: LogFilter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn with_filters(mut self, filters: impl IntoIterator<Item = LogFilter>) -> Self {
        self.filters.extend(filters);
        self
    }

    pub fn with_filter_value(mut self, field: impl Into<String>, value: impl Into<String>) -> Self {
        self.filter_values.insert(field.into(), value.into());
        self
    }

    pub fn with_page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    pub fn with_page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn with_total(mut self, total: usize) -> Self {
        self.total = Some(total);
        self
    }

    // ── Accessors ────────────────────────────────────────────────

    /// Whether audit mode should show the loading surface (loading and no
    /// current entries). Matches Svelte `loading && auditEntries.length === 0`.
    pub fn is_loading(&self) -> bool {
        self.loading && self.entry_count == 0
    }

    /// Whether any filter value is non-empty (Svelte `hasActiveFilters`).
    pub fn has_active_filters(&self) -> bool {
        self.filter_values.values().any(|v| !v.trim().is_empty())
    }

    /// Whether the audit toolbar should render — at least one filter present
    /// (Svelte adds refresh/export to this; those are callback-driven).
    pub fn has_audit_toolbar(&self) -> bool {
        !self.filters.is_empty()
    }

    /// Total page count derived from `total` and `page_size`. Matches Svelte
    /// `total ? Math.max(1, Math.ceil(total / pageSize)) : 1`.
    pub fn total_pages(&self) -> usize {
        match self.total {
            Some(total) if self.page_size > 0 => (total.div_ceil(self.page_size)).max(1),
            _ => 1,
        }
    }

    /// Whether pagination controls should show. Matches Svelte
    /// `total !== undefined && total > pageSize`.
    pub fn show_pagination(&self) -> bool {
        matches!(self.total, Some(total) if total > self.page_size)
    }

    /// Current filter value for a field, or empty string.
    pub fn filter_value(&self, field: &str) -> &str {
        self.filter_values
            .get(field)
            .map(String::as_str)
            .unwrap_or("")
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn entry_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_contract() {
        let spec = LogListSpec::new();
        assert_eq!(spec.entry_count, 0);
        assert_eq!(spec.max_entries, 500);
        assert!(spec.auto_scroll);
        assert_eq!(spec.filter_level, None);
        assert_eq!(spec.filter_text, "");
        assert!(!spec.loading);
        assert_eq!(spec.error, None);
        assert_eq!(spec.empty_message, "No log entries found");
        assert!(spec.filters.is_empty());
        assert!(spec.filter_values.is_empty());
        assert_eq!(spec.page, 1);
        assert_eq!(spec.page_size, 50);
        assert_eq!(spec.total, None);
        assert_eq!(spec.size, ControlSize::Md);
        assert_eq!(spec.size_role, SemanticControlSizeRole::Control);
        assert_eq!(spec.density, ControlDensity::Default);
    }

    #[test]
    fn total_pages_ceils_and_clamps_to_one() {
        // No total → single page.
        assert_eq!(LogListSpec::new().total_pages(), 1);
        // Exact multiple.
        assert_eq!(
            LogListSpec::new()
                .with_page_size(50)
                .with_total(100)
                .total_pages(),
            2
        );
        // Ceils remainder.
        assert_eq!(
            LogListSpec::new()
                .with_page_size(50)
                .with_total(101)
                .total_pages(),
            3
        );
        // total < page_size still yields 1.
        assert_eq!(
            LogListSpec::new()
                .with_page_size(50)
                .with_total(10)
                .total_pages(),
            1
        );
        // total = 0 clamps to 1.
        assert_eq!(
            LogListSpec::new()
                .with_page_size(50)
                .with_total(0)
                .total_pages(),
            1
        );
    }

    #[test]
    fn show_pagination_only_when_total_exceeds_page_size() {
        assert!(!LogListSpec::new().show_pagination());
        assert!(!LogListSpec::new()
            .with_page_size(50)
            .with_total(50)
            .show_pagination());
        assert!(LogListSpec::new()
            .with_page_size(50)
            .with_total(51)
            .show_pagination());
    }

    #[test]
    fn is_loading_requires_no_entries() {
        assert!(LogListSpec::new().with_loading(true).is_loading());
        // Loading but entries present → not the loading surface.
        assert!(!LogListSpec::new()
            .with_loading(true)
            .with_entry_count(3)
            .is_loading());
        assert!(!LogListSpec::new().is_loading());
    }

    #[test]
    fn has_active_filters_ignores_blank_values() {
        assert!(!LogListSpec::new().has_active_filters());
        assert!(!LogListSpec::new()
            .with_filter_value("action", "   ")
            .has_active_filters());
        assert!(LogListSpec::new()
            .with_filter_value("action", "create")
            .has_active_filters());
    }

    #[test]
    fn has_audit_toolbar_tracks_filters() {
        assert!(!LogListSpec::new().has_audit_toolbar());
        assert!(LogListSpec::new()
            .with_filter(LogFilter::select("action", "Action"))
            .has_audit_toolbar());
    }

    #[test]
    fn filter_value_falls_back_to_empty() {
        let spec = LogListSpec::new().with_filter_value("action", "delete");
        assert_eq!(spec.filter_value("action"), "delete");
        assert_eq!(spec.filter_value("missing"), "");
    }

    #[test]
    fn log_filter_builders() {
        let filter = LogFilter::select("action", "Action")
            .with_placeholder("All actions")
            .with_option("create", "Create")
            .with_option("delete", "Delete");
        assert_eq!(filter.kind, LogFilterKind::Select);
        assert_eq!(filter.field, "action");
        assert_eq!(filter.options.len(), 2);
        assert_eq!(filter.placeholder.as_deref(), Some("All actions"));

        let date = LogFilter::date("from", "From");
        assert_eq!(date.kind, LogFilterKind::Date);
        assert!(date.options.is_empty());
    }
}
