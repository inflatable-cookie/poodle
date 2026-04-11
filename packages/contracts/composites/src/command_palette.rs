use poodle_primitives::{ControlDensity, ControlSize, OverlayPlacement, SemanticControlSizeRole};
use poodle_tokens::semantic;

use crate::types::{CommandActionItem, DiscoveryState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandPaletteSpec {
    pub query: String,
    pub actions: Vec<CommandActionItem>,
    pub state: DiscoveryState,
    pub active_action_id: Option<String>,
    pub placement: OverlayPlacement,
    /// Visibility state. Matches Svelte `open` prop — when false the
    /// palette surface is not rendered.
    pub is_open: bool,
    /// Optional header title shown above the query field.
    pub title: Option<String>,
    /// Optional header description shown under the title.
    pub description: Option<String>,
    /// Placeholder for the query field when empty. Matches Svelte
    /// `invocationHint` prop.
    pub invocation_hint: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for CommandPaletteSpec {
    fn default() -> Self {
        Self {
            query: String::new(),
            actions: Vec::new(),
            state: DiscoveryState::Ready,
            active_action_id: None,
            placement: OverlayPlacement::BottomStart,
            is_open: false,
            title: None,
            description: None,
            invocation_hint: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl CommandPaletteSpec {
    pub fn new(actions: Vec<CommandActionItem>) -> Self {
        Self {
            actions,
            ..Self::default()
        }
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    pub fn with_state(mut self, state: DiscoveryState) -> Self {
        self.state = state;
        self
    }

    pub fn with_active_action_id(mut self, active_action_id: impl Into<String>) -> Self {
        self.active_action_id = Some(active_action_id.into());
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_invocation_hint(mut self, hint: impl Into<String>) -> Self {
        self.invocation_hint = Some(hint.into());
        self
    }

    pub fn active_action(&self) -> Option<&CommandActionItem> {
        let id = self.active_action_id.as_deref()?;
        self.actions.iter().find(|action| action.id == id)
    }

    pub fn grouped_action_count(&self) -> usize {
        self.actions
            .iter()
            .filter(|action| action.group.is_some())
            .count()
    }

    pub fn results_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
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
