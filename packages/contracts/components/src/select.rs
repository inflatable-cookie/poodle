use poodle_headless::select::{SelectContext, SelectOptionState};
use poodle_tokens::semantic;

use crate::types::{ChoiceOption, ControlDensity, ControlSize, SemanticControlSizeRole};

/// Controls how the Select renders its dropdown.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectMode {
    /// Auto: native unless searchable or custom rendering needed.
    #[default]
    Auto,
    /// Always native `<select>`.
    Native,
    /// Always custom dropdown.
    Custom,
}

/// Visual variant for the Select.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectVariant {
    /// Default variant with border, background, padding, and chevron.
    #[default]
    Default,
    /// Ghost variant strips all field chrome (border, background, shadow,
    /// padding, min-height) and hides the chevron indicator.
    Ghost,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectSpec {
    pub id: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub placeholder: Option<String>,
    pub options: Vec<ChoiceOption>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub description_id: Option<String>,
    pub open: Option<bool>,
    pub default_open: bool,
    /// Refuses outside-interact dismissal when false. Matches Svelte
    /// `dismissOnOutsideInteract` (default `true`).
    pub dismiss_on_outside_interact: bool,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Rendering mode: Auto, Native, or Custom.
    pub mode: SelectMode,
    /// When true, shows a filter/search input inside the dropdown.
    pub searchable: bool,
    /// When true, allows typing arbitrary values not in the options list.
    pub freeform: bool,
    /// Message shown when filtering produces no matches.
    pub empty_message: String,
    /// Visual variant: Default or Ghost.
    pub variant: SelectVariant,
    /// Optional minimum width for the dropdown listbox (CSS length string, e.g. "12rem").
    /// When set, listbox uses `width: max-content` with viewport-aware anchor flipping.
    pub menu_min_width: Option<String>,
    /// When true, shows a clear button when a value is selected (custom dropdown only).
    pub clearable: bool,
    /// When true, the select is required for form submission. Surfaces
    /// as an Invalid validation state when left unset on submit.
    /// Matches Svelte `required`.
    pub is_required: bool,
    /// Current validation state. Drives border/focus-ring colour the
    /// same way TextInput does.
    pub validation_state: crate::types::ValidationState,
    /// Temporary label for the current selection before lazy options
    /// load. Rendered in the trigger when value is set but the
    /// matching option hasn't loaded yet. Matches `valueLabel`.
    pub value_label: Option<String>,
    /// Invalidates cached lazy options when it changes. Callers bump
    /// this to force a re-fetch from loadOptions. Matches `loadKey`.
    pub load_key: Option<String>,
    /// Current search/filter query string for the inline search input.
    /// When `searchable` is true and this is set, only options whose
    /// labels contain the query (case-insensitive) are shown.
    pub search_query: Option<String>,
    /// Host-authored highlighted option value. `None` means no highlight.
    pub highlighted_value: Option<String>,
    /// Host-authored search-editor `anchor` as a character offset into
    /// `search_query`. Independent of `search_selection_end`; a backward
    /// selection has `end < start`.
    pub search_selection_start: usize,
    /// Host-authored search-editor `head` (the moving end) as a character
    /// offset into `search_query`.
    pub search_selection_end: usize,
}

impl Default for SelectSpec {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            value: None,
            default_value: None,
            placeholder: None,
            options: Vec::new(),
            is_disabled: false,
            aria_label: None,
            description_id: None,
            open: None,
            default_open: false,
            dismiss_on_outside_interact: true,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            mode: SelectMode::default(),
            searchable: false,
            freeform: false,
            empty_message: String::from("No matches"),
            variant: SelectVariant::default(),
            menu_min_width: None,
            clearable: false,
            is_required: false,
            validation_state: crate::types::ValidationState::None,
            value_label: None,
            load_key: None,
            search_query: None,
            highlighted_value: None,
            search_selection_start: 0,
            search_selection_end: 0,
        }
    }
}

impl SelectSpec {
    pub fn new(options: Vec<ChoiceOption>) -> Self {
        Self {
            options,
            ..Self::default()
        }
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

    /// Accessible name for the trigger.
    ///
    /// A select has no caption of its own, so without one it is announced as
    /// "combo box" and its current value, with nothing to say what it chooses.
    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_dismiss_on_outside_interact(mut self, dismiss_on_outside_interact: bool) -> Self {
        self.dismiss_on_outside_interact = dismiss_on_outside_interact;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn selected_option(&self) -> Option<&ChoiceOption> {
        let current = self.current_value()?;
        self.options.iter().find(|option| option.value == current)
    }

    pub fn trigger_text(&self) -> Option<&str> {
        self.selected_option()
            .map(|option| option.label.as_str())
            .or(self.placeholder.as_deref())
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

    pub fn with_mode(mut self, mode: SelectMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    pub fn with_freeform(mut self, freeform: bool) -> Self {
        self.freeform = freeform;
        self
    }

    pub fn with_empty_message(mut self, msg: impl Into<String>) -> Self {
        self.empty_message = msg.into();
        self
    }

    /// Returns true when the dropdown should render a search/filter input.
    /// Search is `searchable` only; `freeform` alone does not show it.
    pub fn shows_search_input(&self) -> bool {
        self.searchable
    }

    /// Returns true when the select must use a custom dropdown
    /// (either explicitly requested or required by searchable).
    pub fn requires_custom_dropdown(&self) -> bool {
        match self.mode {
            SelectMode::Custom => true,
            SelectMode::Native => false,
            SelectMode::Auto => self.searchable,
        }
    }

    /// Set the visual variant (Default or Ghost).
    pub fn with_variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the dropdown listbox minimum width (CSS length string).
    pub fn with_menu_min_width(mut self, width: impl Into<String>) -> Self {
        self.menu_min_width = Some(width.into());
        self
    }

    /// Enable the clear button when a value is selected.
    pub fn with_clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    pub fn with_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    pub fn with_validation_state(mut self, state: crate::types::ValidationState) -> Self {
        self.validation_state = state;
        self
    }

    pub fn with_value_label(mut self, label: impl Into<String>) -> Self {
        self.value_label = Some(label.into());
        self
    }

    pub fn with_load_key(mut self, key: impl Into<String>) -> Self {
        self.load_key = Some(key.into());
        self
    }

    pub fn with_search_query(mut self, query: impl Into<String>) -> Self {
        let query = query.into();
        let len = query.chars().count();
        self.search_query = Some(query);
        self.search_selection_start = len;
        self.search_selection_end = len;
        self
    }

    pub fn with_highlighted_value(mut self, value: impl Into<String>) -> Self {
        self.highlighted_value = Some(value.into());
        self
    }

    pub fn with_search_selection(mut self, start: usize, end: usize) -> Self {
        self.search_selection_start = start;
        self.search_selection_end = end;
        self
    }

    /// Host-authored `(anchor, head)` pair, clamped to the current search
    /// query without swapping direction.
    pub fn search_selection(&self) -> (usize, usize) {
        let len = self.search_query.as_deref().unwrap_or("").chars().count();
        (
            self.search_selection_start.min(len),
            self.search_selection_end.min(len),
        )
    }

    /// Ordered `(start, end)` pair for paint and range replacement.
    pub fn search_selection_range(&self) -> (usize, usize) {
        let (a, b) = self.search_selection();
        (a.min(b), a.max(b))
    }

    /// Apply one complete transition context. Hosts rebuild from this rather
    /// than merging individual fields: highlight events emit no effects.
    pub fn applying_context(mut self, context: &SelectContext) -> Self {
        self.value = if context.value.is_empty() {
            None
        } else {
            Some(context.value.clone())
        };
        self.open = Some(context.open);
        self.search_query = Some(context.query.clone());
        self.highlighted_value = context.highlighted_value.clone();
        let len = context.query.chars().count();
        self.search_selection_start = self.search_selection_start.min(len);
        self.search_selection_end = self.search_selection_end.min(len);
        self
    }

    pub fn effective_value(&self) -> String {
        self.current_value().unwrap_or("").to_string()
    }

    pub fn select_context(&self) -> SelectContext {
        SelectContext {
            value: self.effective_value(),
            open: self.current_open(),
            query: self.search_query.clone().unwrap_or_default(),
            highlighted_value: self.highlighted_value.clone(),
            options: self
                .options
                .iter()
                .map(|option| SelectOptionState {
                    value: option.value.clone(),
                    label: option.label.clone(),
                    disabled: option.is_disabled,
                })
                .collect(),
            clear_value: self.default_value.clone().unwrap_or_default(),
            searchable: self.searchable,
            freeform: self.freeform,
            disabled: self.is_disabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ChoiceOption;

    fn fruit() -> Vec<ChoiceOption> {
        vec![
            ChoiceOption::new("apple", "Apple"),
            ChoiceOption::new("banana", "Banana"),
            ChoiceOption::new("cherry", "Cherry"),
        ]
    }

    #[test]
    fn defaults_match_the_contract() {
        let spec = SelectSpec::default();
        assert_eq!(spec.value, None);
        assert_eq!(spec.default_value, None);
        assert!(!spec.is_disabled);
        assert_eq!(spec.open, None);
        assert!(!spec.default_open);
        assert!(spec.dismiss_on_outside_interact);
        assert!(!spec.searchable);
        assert!(!spec.freeform);
        assert!(!spec.clearable);
        assert_eq!(spec.empty_message, "No matches");
        assert_eq!(spec.menu_min_width, None);
        assert_eq!(spec.mode, SelectMode::Auto);
        assert_eq!(spec.variant, SelectVariant::Default);
    }

    #[test]
    fn builders_cover_the_public_prop_surface() {
        let spec = SelectSpec::new(fruit())
            .with_placeholder("Choose")
            .with_value("banana")
            .with_default_value("apple")
            .with_open(true)
            .with_searchable(true)
            .with_freeform(true)
            .with_clearable(true)
            .with_menu_min_width("12rem")
            .with_aria_label("Fruit")
            .with_highlighted_value("cherry");
        assert_eq!(spec.placeholder.as_deref(), Some("Choose"));
        assert_eq!(spec.value.as_deref(), Some("banana"));
        assert_eq!(spec.default_value.as_deref(), Some("apple"));
        assert_eq!(spec.open, Some(true));
        assert!(spec.searchable);
        assert!(spec.freeform);
        assert!(spec.clearable);
        assert_eq!(spec.menu_min_width.as_deref(), Some("12rem"));
        assert_eq!(spec.aria_label.as_deref(), Some("Fruit"));
        assert_eq!(spec.highlighted_value.as_deref(), Some("cherry"));
    }

    #[test]
    fn current_open_and_value_prefer_controlled_fields() {
        let uncontrolled = SelectSpec::new(fruit()).with_default_value("apple");
        assert_eq!(uncontrolled.current_value(), Some("apple"));
        assert!(!uncontrolled.current_open());
        let controlled = SelectSpec::new(fruit())
            .with_default_value("apple")
            .with_value("banana")
            .with_default_open(true)
            .with_open(false);
        assert_eq!(controlled.current_value(), Some("banana"));
        assert!(!controlled.current_open());
    }

    #[test]
    fn overlay_fill_token_is_elevated() {
        assert_eq!(
            SelectSpec::new(fruit()).overlay_fill_token(),
            semantic::COLOR_BACKGROUND_ELEVATED
        );
    }

    #[test]
    fn applying_context_rebuilds_host_owned_fields() {
        let spec = SelectSpec::new(fruit())
            .with_value("apple")
            .with_open(true)
            .with_search_query("ap");
        let mut context = spec.select_context();
        context.value = "banana".to_owned();
        context.open = false;
        context.query = "Banana".to_owned();
        context.highlighted_value = Some("banana".to_owned());
        let next = spec.applying_context(&context);
        assert_eq!(next.value.as_deref(), Some("banana"));
        assert_eq!(next.open, Some(false));
        assert_eq!(next.search_query.as_deref(), Some("Banana"));
        assert_eq!(next.highlighted_value.as_deref(), Some("banana"));
        assert_eq!(next.select_context().clear_value, "");
    }

    #[test]
    fn authored_default_is_the_clear_value() {
        let spec = SelectSpec::new(fruit()).with_default_value("apple");
        assert_eq!(spec.select_context().clear_value, "apple");
    }
}
