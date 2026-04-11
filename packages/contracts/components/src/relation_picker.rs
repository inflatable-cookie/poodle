use crate::picker_shell::PickerShellSpec;
use crate::composite_types::{BrowseState, PickerItemSpec, PickerVariant, SelectionMode};
use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

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
        Self { levels, leaf_groups }
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
        let Some(leaf_id) = path.last() else { return &[]; };
        self.leaf_groups
            .iter()
            .find(|g| &g.parent_id == leaf_id)
            .map(|g| g.items.as_slice())
            .unwrap_or(&[])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationPickerSpec {
    pub items: Vec<PickerItemSpec>,
    pub selected_ids: Vec<String>,
    pub query: String,
    pub selection_mode: SelectionMode,
    pub variant: PickerVariant,
    pub state: BrowseState,
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
            items,
            selected_ids: Vec::new(),
            query: String::new(),
            selection_mode: SelectionMode::Multiple,
            variant: PickerVariant::Inline,
            state: BrowseState::Ready,
            drill_down: None,
            drill_down_path: Vec::new(),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
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

    pub fn selected_item_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| {
                self.selected_ids
                    .iter()
                    .any(|selected| selected == &item.id)
            })
            .count()
    }

    pub fn current_query(&self) -> &str {
        self.query.as_str()
    }

    pub fn as_picker_shell(&self, title: impl Into<String>) -> PickerShellSpec {
        PickerShellSpec::new(title)
            .with_variant(self.variant)
            .with_selection_mode(self.selection_mode)
            .with_state(self.state)
            .with_query(self.query.clone())
            .with_result_count(self.items.len())
            .with_selected_count(self.selected_item_count())
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
