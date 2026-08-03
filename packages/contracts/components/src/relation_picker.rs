use crate::composite_types::{BrowseState, PickerItemSpec, PickerVariant, SelectionMode};
use crate::picker_shell::PickerShellSpec;
use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// One option inside a toolbar filter (`PickerFilterConfig.options`).
/// Mirrors Svelte `PickerFilterOption`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PickerFilterOption {
    pub id: String,
    pub label: String,
}

impl PickerFilterOption {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

/// One labeled filter control rendered in the picker toolbar. Each entry
/// becomes a labeled `Select` over its `options`. Mirrors Svelte
/// `PickerFilterConfig` (§3 Types).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PickerFilterConfig {
    pub key: String,
    pub label: String,
    pub options: Vec<PickerFilterOption>,
    /// When true (default), the option list is prefixed with an "All" entry.
    pub include_all: bool,
    /// Label for the synthesized "All" option (defaults to "All").
    pub all_label: Option<String>,
}

impl PickerFilterConfig {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<String>,
        options: Vec<PickerFilterOption>,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            options,
            include_all: true,
            all_label: None,
        }
    }

    pub fn with_include_all(mut self, include_all: bool) -> Self {
        self.include_all = include_all;
        self
    }

    pub fn with_all_label(mut self, all_label: impl Into<String>) -> Self {
        self.all_label = Some(all_label.into());
        self
    }

    /// Sentinel value used by the "All" option (matches Svelte `"__all__"`).
    pub const ALL_VALUE: &'static str = "__all__";

    /// Resolve the full option set for this filter, prepending the "All"
    /// option when `include_all` is set. Mirrors Svelte `getFilterOptions`.
    pub fn resolved_options(&self) -> Vec<(String, String)> {
        let mut out = Vec::new();
        if self.include_all {
            out.push((
                Self::ALL_VALUE.to_string(),
                self.all_label.clone().unwrap_or_else(|| "All".to_string()),
            ));
        }
        for option in &self.options {
            out.push((option.id.clone(), option.label.clone()));
        }
        out
    }
}

/// One row in a drill-down level — e.g. a category or subcategory the
/// user can navigate into before reaching leaf `PickerItemSpec`s.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrillDownItem {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    /// Optional count suffix shown on the row (e.g. "4 items").
    pub count: Option<usize>,
}

impl DrillDownItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            count: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }
}

/// One level in a drill-down picker. Items at this level can be
/// navigated into, exposing either another level (via `DrillDownConfig
/// .levels`) or the leaf `final_items` set (keyed by the last-level id).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrillDownLevel {
    /// Machine key used to address this level in the navigation path.
    pub key: String,
    /// Human label rendered in the breadcrumb.
    pub label: String,
    /// Items available at this level.
    pub items: Vec<DrillDownItem>,
    pub search_placeholder: Option<String>,
}

impl DrillDownLevel {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<String>,
        items: Vec<DrillDownItem>,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            items,
            search_placeholder: None,
        }
    }

    pub fn with_search_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.search_placeholder = Some(placeholder.into());
        self
    }
}

/// Leaf items keyed by the parent drill-down node id. This is the flat,
/// data-only form that replaces Svelte's closure-based `finalItems`
/// function — callers populate all leaf groups up front.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrillDownLeafGroup {
    pub parent_id: String,
    pub items: Vec<PickerItemSpec>,
}

impl DrillDownLeafGroup {
    pub fn new(parent_id: impl Into<String>, items: Vec<PickerItemSpec>) -> Self {
        Self {
            parent_id: parent_id.into(),
            items,
        }
    }
}

/// Drill-down configuration — a stack of navigation levels plus the
/// leaf item groups they terminate at. Matches the Svelte
/// `DrillDownConfig` type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrillDownConfig {
    pub levels: Vec<DrillDownLevel>,
    pub leaf_groups: Vec<DrillDownLeafGroup>,
}

impl DrillDownConfig {
    pub fn new(levels: Vec<DrillDownLevel>, leaf_groups: Vec<DrillDownLeafGroup>) -> Self {
        Self {
            levels,
            leaf_groups,
        }
    }

    /// Given the current navigation path (a list of ids, one per level
    /// already entered), return the slice of levels not yet navigated.
    pub fn next_level(&self, path: &[String]) -> Option<&DrillDownLevel> {
        self.levels.get(path.len())
    }

    /// Whether the caller has drilled all the way to a leaf group.
    pub fn is_at_leaf(&self, path: &[String]) -> bool {
        path.len() >= self.levels.len()
    }

    /// Leaf items under the current path. Returns an empty slice when
    /// the path doesn't address a known leaf group.
    pub fn leaf_items_for(&self, path: &[String]) -> &[PickerItemSpec] {
        let Some(leaf_id) = path.last() else {
            return &[];
        };
        self.leaf_groups
            .iter()
            .find(|g| &g.parent_id == leaf_id)
            .map(|g| g.items.as_slice())
            .unwrap_or(&[])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationPickerSpec {
    pub title: String,
    pub description: Option<String>,
    pub items: Vec<PickerItemSpec>,
    pub selected_ids: Vec<String>,
    pub query: String,
    pub selection_mode: SelectionMode,
    pub variant: PickerVariant,
    pub state: BrowseState,
    pub aria_label: Option<String>,
    /// Placeholder text for the main search field (Svelte `searchPlaceholder`,
    /// default "Search picker results").
    pub search_placeholder: String,
    /// Toolbar filter controls; each renders a labeled `Select` over its
    /// options. Mirrors Svelte `filters` (default empty).
    pub filters: Vec<PickerFilterConfig>,
    /// Current value for each filter, keyed by filter `key`. A missing key (or
    /// the `__all__` sentinel) means "All". Mirrors Svelte `filterValues`.
    pub filter_values: std::collections::BTreeMap<String, String>,
    pub confirm_label: String,
    pub cancel_label: String,
    /// Overrides the default selection-mode footer note text. When `None`, no
    /// footer note is rendered. Mirrors Svelte `footerNote` (default `null`).
    pub footer_note: Option<String>,
    /// When false, the confirm/cancel footer is not rendered (Svelte
    /// `showFooter`, default true).
    pub show_footer: bool,
    /// When false, the selection-summary region is not rendered (Svelte
    /// `showSelectionSummary`, default true).
    pub show_selection_summary: bool,
    /// Optional drill-down configuration. When present the picker
    /// renders a breadcrumbed navigation instead of the flat `items`
    /// list and the caller owns `drill_down_path` as the current state.
    pub drill_down: Option<DrillDownConfig>,
    /// Current drill-down navigation path — one entry per level the
    /// user has entered so far. Empty means the top-level items.
    pub drill_down_path: Vec<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl RelationPickerSpec {
    pub fn new(items: Vec<PickerItemSpec>) -> Self {
        Self {
            title: String::from("Select items"),
            description: None,
            items,
            selected_ids: Vec::new(),
            query: String::new(),
            selection_mode: SelectionMode::Multiple,
            variant: PickerVariant::Inline,
            state: BrowseState::Ready,
            aria_label: None,
            search_placeholder: String::from("Search picker results"),
            filters: Vec::new(),
            filter_values: std::collections::BTreeMap::new(),
            confirm_label: String::from("Confirm selection"),
            cancel_label: String::from("Cancel"),
            footer_note: None,
            show_footer: true,
            show_selection_summary: true,
            drill_down: None,
            drill_down_path: Vec::new(),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_drill_down(mut self, config: DrillDownConfig) -> Self {
        self.drill_down = Some(config);
        self
    }

    pub fn with_drill_down_path(mut self, path: Vec<String>) -> Self {
        self.drill_down_path = path;
        self
    }

    pub fn is_drill_down(&self) -> bool {
        self.drill_down.is_some()
    }

    pub fn with_selected_ids(mut self, selected_ids: Vec<String>) -> Self {
        self.selected_ids = selected_ids;
        self
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    pub fn with_selection_mode(mut self, selection_mode: SelectionMode) -> Self {
        self.selection_mode = selection_mode;
        self
    }

    pub fn with_variant(mut self, variant: PickerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_state(mut self, state: BrowseState) -> Self {
        self.state = state;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_confirm_label(mut self, confirm_label: impl Into<String>) -> Self {
        self.confirm_label = confirm_label.into();
        self
    }

    pub fn with_cancel_label(mut self, cancel_label: impl Into<String>) -> Self {
        self.cancel_label = cancel_label.into();
        self
    }

    pub fn with_search_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.search_placeholder = placeholder.into();
        self
    }

    pub fn with_filters(mut self, filters: Vec<PickerFilterConfig>) -> Self {
        self.filters = filters;
        self
    }

    pub fn with_filter_value(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.filter_values.insert(key.into(), value.into());
        self
    }

    pub fn with_footer_note(mut self, note: impl Into<String>) -> Self {
        self.footer_note = Some(note.into());
        self
    }

    pub fn with_show_footer(mut self, show_footer: bool) -> Self {
        self.show_footer = show_footer;
        self
    }

    pub fn with_show_selection_summary(mut self, show: bool) -> Self {
        self.show_selection_summary = show;
        self
    }

    /// Current value for a filter, falling back to the `__all__` sentinel
    /// (Svelte `filterValues[filter.key] ?? "__all__"`).
    pub fn filter_value(&self, key: &str) -> &str {
        self.filter_values
            .get(key)
            .map(String::as_str)
            .unwrap_or(PickerFilterConfig::ALL_VALUE)
    }

    pub fn selected_item_count(&self) -> usize {
        self.selected_ids.len()
    }

    pub fn current_query(&self) -> &str {
        self.query.as_str()
    }

    pub fn current_items(&self) -> Vec<PickerItemSpec> {
        if let Some(ref drill_down) = self.drill_down {
            if drill_down.is_at_leaf(&self.drill_down_path) {
                return drill_down.leaf_items_for(&self.drill_down_path).to_vec();
            }
        }

        self.items.clone()
    }

    pub fn drill_items(&self) -> Vec<DrillDownItem> {
        self.drill_down
            .as_ref()
            .and_then(|dd| dd.next_level(&self.drill_down_path))
            .map(|level| level.items.clone())
            .unwrap_or_default()
    }

    pub fn current_result_count(&self) -> usize {
        if self.drill_down.is_some() && !self.drill_items().is_empty() {
            return self.drill_items().len();
        }

        self.current_items().len()
    }

    pub fn selection_summary_items(&self) -> Vec<crate::composite_types::SelectionSummaryItem> {
        let mut items = self
            .items
            .iter()
            .map(|item| (item.id.clone(), item.label.clone(), item.meta.clone()))
            .collect::<Vec<_>>();

        if let Some(ref drill_down) = self.drill_down {
            for group in &drill_down.leaf_groups {
                for item in &group.items {
                    items.push((item.id.clone(), item.label.clone(), item.meta.clone()));
                }
            }
        }

        self.selected_ids
            .iter()
            .filter_map(|selected_id| {
                items
                    .iter()
                    .find(|(id, _, _)| id == selected_id)
                    .map(|(_, label, meta)| {
                        let mut item = crate::composite_types::SelectionSummaryItem::new(
                            selected_id.clone(),
                            label.clone(),
                        );
                        if let Some(meta) = meta {
                            item = item.with_meta(meta.clone());
                        }
                        item
                    })
            })
            .collect()
    }

    pub fn as_picker_shell(&self) -> PickerShellSpec {
        let mut shell = PickerShellSpec::new(self.title.clone())
            .with_variant(self.variant)
            .with_selection_mode(self.selection_mode)
            .with_state(self.state)
            .with_query(self.query.clone())
            .with_result_count(self.current_result_count())
            .with_selected_count(self.selected_item_count());

        if let Some(ref description) = self.description {
            shell = shell.with_description(description.clone());
        }
        if let Some(ref aria_label) = self.aria_label {
            shell = shell.with_aria_label(aria_label.clone());
        }

        shell
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
        let spec = RelationPickerSpec::new(vec![]);
        assert_eq!(spec.search_placeholder, "Search picker results");
        assert!(spec.filters.is_empty());
        assert!(spec.filter_values.is_empty());
        assert_eq!(spec.footer_note, None);
        assert!(spec.show_footer);
        assert!(spec.show_selection_summary);
    }

    #[test]
    fn filter_resolved_options_prepends_all_by_default() {
        let filter = PickerFilterConfig::new(
            "kind",
            "Kind",
            vec![
                PickerFilterOption::new("a", "Alpha"),
                PickerFilterOption::new("b", "Beta"),
            ],
        );
        let opts = filter.resolved_options();
        assert_eq!(
            opts[0],
            (PickerFilterConfig::ALL_VALUE.to_string(), "All".to_string())
        );
        assert_eq!(opts[1], ("a".to_string(), "Alpha".to_string()));
        assert_eq!(opts.len(), 3);
    }

    #[test]
    fn filter_include_all_false_and_custom_all_label() {
        let no_all = PickerFilterConfig::new("k", "K", vec![PickerFilterOption::new("a", "A")])
            .with_include_all(false);
        assert_eq!(no_all.resolved_options().len(), 1);

        let custom = PickerFilterConfig::new("k", "K", vec![PickerFilterOption::new("a", "A")])
            .with_all_label("Any");
        assert_eq!(custom.resolved_options()[0].1, "Any");
    }

    #[test]
    fn filter_value_falls_back_to_all_sentinel() {
        let spec = RelationPickerSpec::new(vec![]).with_filter_value("kind", "a");
        assert_eq!(spec.filter_value("kind"), "a");
        assert_eq!(spec.filter_value("missing"), PickerFilterConfig::ALL_VALUE);
    }

    #[test]
    fn footer_and_summary_toggles() {
        let spec = RelationPickerSpec::new(vec![])
            .with_footer_note("Pick up to 3")
            .with_show_footer(false)
            .with_show_selection_summary(false);
        assert_eq!(spec.footer_note.as_deref(), Some("Pick up to 3"));
        assert!(!spec.show_footer);
        assert!(!spec.show_selection_summary);
    }
}
