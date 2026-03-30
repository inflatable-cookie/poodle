use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone};
use poodle_tokens::semantic;

use crate::types::{ActionDiscoverySection, DiscoveryState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionDiscoveryPanelSpec {
    pub sections: Vec<ActionDiscoverySection>,
    pub state: DiscoveryState,
    pub empty_message: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for ActionDiscoveryPanelSpec {
    fn default() -> Self {
        Self {
            sections: Vec::new(),
            state: DiscoveryState::Ready,
            empty_message: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl ActionDiscoveryPanelSpec {
    pub fn new(sections: Vec<ActionDiscoverySection>) -> Self {
        Self {
            sections,
            ..Self::default()
        }
    }

    pub fn with_state(mut self, state: DiscoveryState) -> Self {
        self.state = state;
        self
    }

    pub fn with_empty_message(mut self, empty_message: impl Into<String>) -> Self {
        self.empty_message = Some(empty_message.into());
        self
    }

    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    pub fn action_count(&self) -> usize {
        self.sections
            .iter()
            .map(|section| section.actions.len())
            .sum()
    }

    pub fn summary_tone(&self) -> StatusTone {
        match self.state {
            DiscoveryState::Error => StatusTone::Danger,
            DiscoveryState::Loading => StatusTone::Pending,
            DiscoveryState::Empty | DiscoveryState::NoResults => StatusTone::Neutral,
            DiscoveryState::Ready => StatusTone::Info,
        }
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
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
