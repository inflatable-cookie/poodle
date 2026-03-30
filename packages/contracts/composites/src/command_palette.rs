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
