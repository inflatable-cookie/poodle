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

/// Stream-entry severity. Contract §"Types": `"info" | "warn" | "error"`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LogLevel {
    #[default]
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// The contract's wire value, and the key the level chips count by.
    pub fn value(self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}

/// The principal behind an audit entry (Svelte `LogActor`).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LogActor {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    /// Resolved link target for the actor. The web target builds this from a
    /// `getActorHref` callback; the Rust targets take the resolved value
    /// because they carry no callback channel.
    pub href: Option<String>,
}

impl LogActor {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            ..Self::default()
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Contract §"resolveActorName": name, then email, then a truncated id.
    pub fn display_name(&self) -> String {
        if let Some(name) = &self.name {
            return name.clone();
        }
        if let Some(email) = &self.email {
            return email.clone();
        }
        let short: String = self.id.chars().take(8).collect();
        format!("User {short}")
    }
}

/// A stream-mode entry (Svelte `StreamLogEntry`).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StreamLogEntry {
    pub id: Option<String>,
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

impl StreamLogEntry {
    pub fn new(timestamp: impl Into<String>, level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            id: None,
            timestamp: timestamp.into(),
            level,
            message: message.into(),
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// An audit-mode entry (Svelte `AuditLogEntry`).
///
/// `details` is deliberately absent: it exists only to feed the web target's
/// `entryDetails` snippet, and the Rust targets have no snippet channel.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AuditLogEntry {
    pub id: String,
    pub occurred_at: String,
    pub actor: Option<LogActor>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub resource_label: Option<String>,
    /// Resolved link target for the resource, from the web target's
    /// `getResourceHref` callback.
    pub resource_href: Option<String>,
}

impl AuditLogEntry {
    pub fn new(
        id: impl Into<String>,
        occurred_at: impl Into<String>,
        action: impl Into<String>,
        resource_type: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            occurred_at: occurred_at.into(),
            actor: None,
            action: action.into(),
            resource_type: resource_type.into(),
            resource_id: resource_id.into(),
            resource_label: None,
            resource_href: None,
        }
    }

    pub fn with_actor(mut self, actor: LogActor) -> Self {
        self.actor = Some(actor);
        self
    }

    pub fn with_resource_label(mut self, label: impl Into<String>) -> Self {
        self.resource_label = Some(label.into());
        self
    }

    pub fn with_resource_href(mut self, href: impl Into<String>) -> Self {
        self.resource_href = Some(href.into());
        self
    }

    /// Contract §"resolveActorName": an entry without an actor reads "System".
    pub fn actor_name(&self) -> String {
        match &self.actor {
            Some(actor) => actor.display_name(),
            None => "System".to_string(),
        }
    }

    /// Contract §"resolveActionLabel": underscores become spaces.
    pub fn action_label(&self) -> String {
        self.action.replace('_', " ")
    }

    /// Contract §"resolveResourceLabel": underscores become spaces.
    pub fn resource_type_label(&self) -> String {
        self.resource_type.replace('_', " ")
    }
}

/// A log row: stream or audit (Svelte `LogEntry`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogEntry {
    Stream(StreamLogEntry),
    Audit(AuditLogEntry),
}

impl LogEntry {
    pub fn is_audit(&self) -> bool {
        matches!(self, LogEntry::Audit(_))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogListSpec {
    /// Stream or audit rows (Svelte `entries`). The rendered variant is
    /// inferred from these: any audit entry makes the list an audit list.
    pub entries: Vec<LogEntry>,
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

impl Default for LogListSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl LogListSpec {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
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

    pub fn with_entries(mut self, entries: impl IntoIterator<Item = LogEntry>) -> Self {
        self.entries.extend(entries);
        self
    }

    /// How many rows the list holds (Svelte `entries.length`).
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Whether the list renders as an audit list. Contract §"resolvedVariant":
    /// `auto` resolves to audit when any entry is an audit entry.
    pub fn is_audit(&self) -> bool {
        self.entries.iter().any(LogEntry::is_audit)
    }

    /// Audit rows only, in order.
    pub fn audit_entries(&self) -> impl Iterator<Item = &AuditLogEntry> {
        self.entries.iter().filter_map(|entry| match entry {
            LogEntry::Audit(audit) => Some(audit),
            LogEntry::Stream(_) => None,
        })
    }

    /// Stream rows only, in order, after the level and text filters.
    /// Contract §"filteredEntries".
    pub fn stream_entries(&self) -> Vec<&StreamLogEntry> {
        let needle = self.filter_text.trim().to_lowercase();
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                LogEntry::Stream(stream) => Some(stream),
                LogEntry::Audit(_) => None,
            })
            .filter(|stream| match self.filter_level.as_deref() {
                Some(level) if !level.is_empty() => stream.level.value() == level,
                _ => true,
            })
            .filter(|stream| needle.is_empty() || stream.message.to_lowercase().contains(&needle))
            .collect()
    }

    /// Per-level stream counts for the level chips (Svelte `levelCounts`).
    pub fn level_count(&self, level: LogLevel) -> usize {
        self.entries
            .iter()
            .filter(|entry| match entry {
                LogEntry::Stream(stream) => stream.level == level,
                LogEntry::Audit(_) => false,
            })
            .count()
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
        self.loading && self.audit_entries().next().is_none()
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
        assert_eq!(spec.entry_count(), 0);
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
        // Loading is the audit surface, so audit rows are what suppress it.
        assert!(!LogListSpec::new()
            .with_loading(true)
            .with_entries([LogEntry::Audit(AuditLogEntry::new(
                "a1",
                "2026-01-01T00:00:00Z",
                "create",
                "project",
                "p-1",
            ))])
            .is_loading());
        assert!(!LogListSpec::new().is_loading());
    }

    #[test]
    fn variant_resolves_to_audit_when_any_entry_is_audit() {
        let stream = LogListSpec::new().with_entries([LogEntry::Stream(StreamLogEntry::new(
            "10:23:01",
            LogLevel::Info,
            "Server started",
        ))]);
        assert!(!stream.is_audit());

        let mixed = stream
            .clone()
            .with_entries([LogEntry::Audit(AuditLogEntry::new(
                "a1",
                "2026-01-01T00:00:00Z",
                "create",
                "project",
                "p-1",
            ))]);
        assert!(mixed.is_audit());
        assert_eq!(mixed.entry_count(), 2);
    }

    #[test]
    fn stream_entries_apply_level_and_text_filters() {
        let spec = LogListSpec::new().with_entries([
            LogEntry::Stream(StreamLogEntry::new("1", LogLevel::Info, "cache warm")),
            LogEntry::Stream(StreamLogEntry::new("2", LogLevel::Warn, "cache miss")),
            LogEntry::Stream(StreamLogEntry::new("3", LogLevel::Error, "boom")),
        ]);
        assert_eq!(spec.stream_entries().len(), 3);
        assert_eq!(spec.level_count(LogLevel::Warn), 1);

        let by_level = spec.clone().with_filter_level("warn");
        assert_eq!(by_level.stream_entries().len(), 1);

        // Text filter is case-insensitive and matches the message.
        let by_text = spec.clone().with_filter_text("CACHE");
        assert_eq!(by_text.stream_entries().len(), 2);
    }

    #[test]
    fn actor_name_falls_back_through_name_email_then_id() {
        let entry = |actor: Option<LogActor>| AuditLogEntry {
            actor,
            ..AuditLogEntry::new("a1", "2026-01-01T00:00:00Z", "user_login", "project", "p-1")
        };
        assert_eq!(entry(None).actor_name(), "System");
        assert_eq!(
            entry(Some(LogActor::new("0123456789").with_name("Alice"))).actor_name(),
            "Alice"
        );
        assert_eq!(
            entry(Some(LogActor::new("0123456789").with_email("a@b.c"))).actor_name(),
            "a@b.c"
        );
        assert_eq!(
            entry(Some(LogActor::new("0123456789"))).actor_name(),
            "User 01234567"
        );
    }

    #[test]
    fn audit_labels_replace_underscores() {
        let entry = AuditLogEntry::new(
            "a1",
            "2026-01-01T00:00:00Z",
            "user_login",
            "work_space",
            "w-1",
        );
        assert_eq!(entry.action_label(), "user login");
        assert_eq!(entry.resource_type_label(), "work space");
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
