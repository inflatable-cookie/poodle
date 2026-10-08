use poodle_headless::listbox::{listbox_initial_focus, ListboxContext};

pub use poodle_headless::listbox::{
    ListboxBoundary, ListboxEvent, ListboxItem, ListboxOrientation, ListboxSelectionMode,
};

/// Renderer-neutral inputs and current host-owned state for a Listbox.
///
/// `value` / `values` distinguish controlled state from the corresponding
/// defaults: `None` means uncontrolled, while `Some(None)` is a controlled
/// empty single selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListboxSpec {
    pub items: Vec<ListboxItem>,
    pub selection_mode: ListboxSelectionMode,
    pub orientation: ListboxOrientation,
    pub value: Option<Option<String>>,
    pub values: Option<Vec<String>>,
    pub default_value: Option<String>,
    pub default_values: Vec<String>,
    pub disabled: bool,
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
    /// The host's current focus value. When omitted, the enabled selected
    /// option or first enabled option is the initial roving target.
    pub focused_value: Option<String>,
    pub anchor_value: Option<String>,
    pub typeahead: String,
    pub typeahead_at: u64,
}

impl Default for ListboxSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selection_mode: ListboxSelectionMode::Single,
            orientation: ListboxOrientation::Vertical,
            value: None,
            values: None,
            default_value: None,
            default_values: Vec::new(),
            disabled: false,
            aria_label: None,
            aria_labelledby: None,
            focused_value: None,
            anchor_value: None,
            typeahead: String::new(),
            typeahead_at: 0,
        }
    }
}

impl ListboxSpec {
    pub fn new(items: Vec<ListboxItem>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn with_items(mut self, items: Vec<ListboxItem>) -> Self {
        self.items = items;
        self
    }

    pub fn with_selection_mode(mut self, mode: ListboxSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    pub fn with_orientation(mut self, orientation: ListboxOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_value(mut self, value: Option<String>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_values(mut self, values: Vec<String>) -> Self {
        self.values = Some(values);
        self
    }

    pub fn with_default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn with_default_values(mut self, values: Vec<String>) -> Self {
        self.default_values = values;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_aria_labelledby(mut self, id: impl Into<String>) -> Self {
        self.aria_labelledby = Some(id.into());
        self
    }

    pub fn with_focused_value(mut self, value: impl Into<String>) -> Self {
        self.focused_value = Some(value.into());
        self
    }

    /// Carry the machine-owned navigation state across a host rerender.
    pub fn with_interaction_state(mut self, context: &ListboxContext) -> Self {
        self.focused_value = context.focused_value.clone();
        self.anchor_value = context.anchor_value.clone();
        self.typeahead = context.typeahead.clone();
        self.typeahead_at = context.typeahead_at;
        self
    }

    /// Selected values resolved from the current controlled value or the
    /// uncontrolled default for the active selection mode.
    pub fn current_values(&self) -> Vec<String> {
        match self.selection_mode {
            ListboxSelectionMode::Single => self
                .value
                .clone()
                .unwrap_or_else(|| self.default_value.clone())
                .into_iter()
                .collect(),
            ListboxSelectionMode::Multiple => self
                .values
                .clone()
                .unwrap_or_else(|| self.default_values.clone()),
        }
    }

    pub fn current_value(&self) -> Option<&str> {
        match self.selection_mode {
            ListboxSelectionMode::Single => match &self.value {
                Some(value) => value.as_deref(),
                None => self.default_value.as_deref(),
            },
            ListboxSelectionMode::Multiple => self
                .values
                .as_ref()
                .unwrap_or(&self.default_values)
                .first()
                .map(String::as_str),
        }
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.current_values()
            .iter()
            .any(|selected| selected == value)
    }

    pub fn is_item_disabled(&self, item: &ListboxItem) -> bool {
        self.disabled || item.disabled
    }

    pub fn listbox_context(&self) -> ListboxContext {
        let selected_values = self.current_values();
        let focused_value = if self.disabled {
            None
        } else {
            self.focused_value
                .clone()
                .or_else(|| listbox_initial_focus(&self.items, &selected_values, false))
        };
        ListboxContext {
            items: self.items.clone(),
            selection_mode: self.selection_mode,
            orientation: self.orientation,
            disabled: self.disabled,
            selected_values,
            focused_value,
            anchor_value: self.anchor_value.clone(),
            typeahead: self.typeahead.clone(),
            typeahead_at: self.typeahead_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items() -> Vec<ListboxItem> {
        vec![
            ListboxItem::new("a", "Alpha"),
            ListboxItem::new("b", "Beta").with_disabled(true),
        ]
    }

    #[test]
    fn builders_preserve_controlled_and_default_selection_modes() {
        let uncontrolled = ListboxSpec::new(items()).with_default_value("a");
        assert_eq!(uncontrolled.current_values(), vec!["a"]);
        assert_eq!(
            uncontrolled.listbox_context().focused_value.as_deref(),
            Some("a")
        );

        let controlled_empty = uncontrolled.clone().with_value(None);
        assert!(controlled_empty.current_values().is_empty());

        let multiple = ListboxSpec::new(items())
            .with_selection_mode(ListboxSelectionMode::Multiple)
            .with_default_values(vec!["a".into()])
            .with_values(vec!["b".into(), "a".into()])
            .with_orientation(ListboxOrientation::Horizontal)
            .with_aria_label("Saved items")
            .with_aria_labelledby("list-label");
        assert_eq!(multiple.current_values(), vec!["b", "a"]);
        assert_eq!(multiple.orientation, ListboxOrientation::Horizontal);
        assert_eq!(multiple.aria_label.as_deref(), Some("Saved items"));
        assert_eq!(multiple.aria_labelledby.as_deref(), Some("list-label"));
    }

    #[test]
    fn default_focus_skips_disabled_items_and_global_disable_removes_it() {
        let spec = ListboxSpec::new(items());
        assert_eq!(spec.listbox_context().focused_value.as_deref(), Some("a"));
        assert_eq!(
            spec.with_disabled(true).listbox_context().focused_value,
            None
        );
    }
}
