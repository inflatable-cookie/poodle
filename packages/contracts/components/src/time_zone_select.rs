use crate::types::{
    ChoiceOption, ControlDensity, ControlSize, SemanticControlSizeRole, TimeZoneOption,
};
use poodle_tokens::semantic;

/// Empty-search message passed through to `Select` when no zone matches
/// the query. Mirrors Svelte `emptyMessage="No matching time zones"`.
pub const TIME_ZONE_EMPTY_MESSAGE: &str = "No matching time zones";

/// Default placeholder, matching Svelte `placeholder = "Search time zones..."`.
pub const TIME_ZONE_PLACEHOLDER: &str = "Search time zones...";

/// Rust equivalent of Svelte `defaultTimeZoneOptions()` (contract §10).
///
/// The Svelte version prefers `Intl.supportedValuesOf("timeZone")` (the full
/// IANA set) and falls back to this small hardcoded list when the runtime has
/// no `Intl` support. The Rust targets have no `Intl`, so this curated list is
/// the canonical fallback. Labels are `value.replaceAll("_", " ")`, matching
/// `formatTimeZoneLabel`.
pub fn default_time_zone_options() -> Vec<TimeZoneOption> {
    [
        "UTC",
        "America/New_York",
        "America/Chicago",
        "America/Denver",
        "America/Los_Angeles",
        "Europe/London",
        "Europe/Paris",
        "Asia/Tokyo",
        "Australia/Sydney",
    ]
    .into_iter()
    .map(|value| TimeZoneOption::new(value, value.replace('_', " ")))
    .collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeZoneSelectSpec {
    pub id: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub placeholder: Option<String>,
    /// Host-provided timezone option list. When empty, the component falls
    /// back to `default_time_zone_options()` (contract §3).
    pub options: Vec<TimeZoneOption>,
    pub is_open: bool,
    pub is_disabled: bool,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    pub aria_label: Option<String>,
    pub described_by: Option<String>,
    /// Current search/filter query for the always-on searchable dropdown.
    /// The host owns this and feeds it back via the query callback so the
    /// option list is filtered.
    pub search_query: Option<String>,
}

impl Default for TimeZoneSelectSpec {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            value: None,
            default_value: None,
            placeholder: None,
            options: Vec::new(),
            is_open: false,
            is_disabled: false,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            aria_label: None,
            described_by: None,
            search_query: None,
        }
    }
}

impl TimeZoneSelectSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_options(mut self, options: Vec<TimeZoneOption>) -> Self {
        self.options = options;
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_described_by(mut self, described_by: impl Into<String>) -> Self {
        self.described_by = Some(described_by.into());
        self
    }

    pub fn with_search_query(mut self, query: impl Into<String>) -> Self {
        self.search_query = Some(query.into());
        self
    }

    /// Resolved selected value (`value` falling back to `default_value`).
    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    /// The effective option set: host options when present, else the
    /// `default_time_zone_options()` fallback (contract §3).
    pub fn effective_options(&self) -> Vec<TimeZoneOption> {
        if self.options.is_empty() {
            default_time_zone_options()
        } else {
            self.options.clone()
        }
    }

    /// Effective placeholder, defaulting to the Svelte search placeholder.
    pub fn effective_placeholder(&self) -> &str {
        self.placeholder.as_deref().unwrap_or(TIME_ZONE_PLACEHOLDER)
    }

    /// Map the effective timezone options into `Select` `ChoiceOption`s, so a
    /// target can compose the shared `Select` implementation directly.
    pub fn select_options(&self) -> Vec<ChoiceOption> {
        self.effective_options()
            .into_iter()
            .map(|o| ChoiceOption::new(o.value, o.label))
            .collect()
    }

    /// Build a `SelectSpec` configured exactly as the Svelte wrapper does:
    /// searchable always on, the timezone empty message, mapped options, and
    /// all shared field state forwarded. Targets that compose `Select` use
    /// this; targets that hand-roll can still read the individual fields.
    pub fn to_select_spec(&self) -> crate::select::SelectSpec {
        let mut spec = crate::select::SelectSpec::new(self.select_options())
            .with_searchable(true)
            .with_empty_message(TIME_ZONE_EMPTY_MESSAGE)
            .with_placeholder(self.effective_placeholder())
            .with_size_role(self.size_role)
            .with_open(self.is_open);
        // Omission propagates unchanged: `None` still means "inherit from the
        // presentation context" on the composed Select.
        spec.size = self.size;
        spec.density = self.density;
        spec.is_disabled = self.is_disabled;
        spec.id = self.id.clone();
        spec.name = self.name.clone();
        spec.value = self.value.clone();
        spec.default_value = self.default_value.clone();
        spec.aria_label = self.aria_label.clone();
        spec.description_id = self.described_by.clone();
        spec.search_query = self.search_query.clone();
        spec
    }

    /// Trigger label: the selected zone's display label when a value is set,
    /// otherwise the placeholder.
    pub fn trigger_text(&self) -> Option<String> {
        if let Some(value) = self.current_value() {
            // Prefer the matching option's label; fall back to the raw value
            // with `_`→space formatting, matching `formatTimeZoneLabel`.
            let label = self
                .effective_options()
                .into_iter()
                .find(|o| o.value == value)
                .map(|o| o.label)
                .unwrap_or_else(|| value.replace('_', " "));
            Some(label)
        } else {
            self.placeholder.clone()
        }
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn overlay_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }
}
