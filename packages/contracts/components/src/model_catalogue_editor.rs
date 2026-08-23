use poodle_headless::model_connection::{ModelCatalogueItem, ModelCatalogueState};

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// ModelCatalogueEditor — a controlled surface for ordering shown models and
/// hiding or restoring models from one configured connection.
///
/// Contract: `docs/contracts/components/model-catalogue-editor.md`
///
/// Structural only: this orders and hides rows the host supplied. It owns no
/// model policy — no defaults, no favourites, no per-model options, no
/// discovery, and no schema authority. Every change is a request; the host
/// applies it and supplies the next `items`.
///
/// `grabbed_id`, `drop_target_id`, `hidden_open`, and `live_message` are host
/// state for the same reason overlay open state is: the Rust targets keep
/// transient interaction state outside the pure renderer, and a callback asks
/// for the next value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogueEditorSpec {
    /// Shown items keep source order; hidden order carries no meaning.
    pub items: Vec<ModelCatalogueItem>,
    pub state: ModelCatalogueState,
    pub title: String,
    pub hidden_title: String,
    pub aria_label: Option<String>,
    /// Disables editing; the list stays readable.
    pub is_disabled: bool,
    /// Temporary mutation lock while the host applies a change.
    pub is_pending: bool,
    /// Pointer drag. Keyboard and explicit moves remain regardless.
    pub is_drag_enabled: bool,
    /// Explicit up/down IconButtons.
    pub show_move_actions: bool,
    /// Host override for the non-ready heading.
    pub state_title: Option<String>,
    /// Host-safe posture explanation.
    pub state_message: Option<String>,
    /// The row currently held by a keyboard grab.
    pub grabbed_id: Option<String>,
    /// The row a pointer drag is currently over.
    pub drop_target_id: Option<String>,
    /// Whether the hidden-models section is disclosed.
    pub hidden_open: bool,
    /// The live-region copy the host last received from an announcement.
    pub live_message: String,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for ModelCatalogueEditorSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            state: ModelCatalogueState::Ready,
            title: "Models".to_string(),
            hidden_title: "Hidden models".to_string(),
            aria_label: None,
            is_disabled: false,
            is_pending: false,
            is_drag_enabled: true,
            show_move_actions: true,
            state_title: None,
            state_message: None,
            grabbed_id: None,
            drop_target_id: None,
            hidden_open: false,
            live_message: String::new(),
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl ModelCatalogueEditorSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_items(mut self, items: Vec<ModelCatalogueItem>) -> Self {
        self.items = items;
        self
    }

    pub fn with_item(mut self, item: ModelCatalogueItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn with_state(mut self, state: ModelCatalogueState) -> Self {
        self.state = state;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_hidden_title(mut self, hidden_title: impl Into<String>) -> Self {
        self.hidden_title = hidden_title.into();
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_pending(mut self, is_pending: bool) -> Self {
        self.is_pending = is_pending;
        self
    }

    pub fn with_drag_enabled(mut self, is_drag_enabled: bool) -> Self {
        self.is_drag_enabled = is_drag_enabled;
        self
    }

    pub fn with_move_actions(mut self, show_move_actions: bool) -> Self {
        self.show_move_actions = show_move_actions;
        self
    }

    pub fn with_state_title(mut self, state_title: impl Into<String>) -> Self {
        self.state_title = Some(state_title.into());
        self
    }

    pub fn with_state_message(mut self, state_message: impl Into<String>) -> Self {
        self.state_message = Some(state_message.into());
        self
    }

    pub fn with_grabbed(mut self, grabbed_id: Option<String>) -> Self {
        self.grabbed_id = grabbed_id;
        self
    }

    pub fn with_drop_target(mut self, drop_target_id: Option<String>) -> Self {
        self.drop_target_id = drop_target_id;
        self
    }

    pub fn with_hidden_open(mut self, hidden_open: bool) -> Self {
        self.hidden_open = hidden_open;
        self
    }

    pub fn with_live_message(mut self, live_message: impl Into<String>) -> Self {
        self.live_message = live_message.into();
        self
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

    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(&self.title)
    }

    /// Editing is locked while the host is applying a change or the surface is
    /// disabled. Reading is never locked.
    pub fn is_locked(&self) -> bool {
        self.is_disabled || self.is_pending
    }

    /// Row handle id, so a keyboard move can name its focus destination.
    pub fn row_handle_id(&self, item_id: &str) -> String {
        format!("model-catalogue-editor:{item_id}:handle")
    }

}

/// The hidden-section disclosure id — the focus destination when hiding the
/// last shown model.
pub const MODEL_CATALOGUE_HIDDEN_SECTION_ID: &str = "model-catalogue-editor:hidden";
