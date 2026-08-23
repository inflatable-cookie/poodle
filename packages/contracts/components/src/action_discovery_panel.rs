use crate::{ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone};
use poodle_tokens::semantic;

use crate::composite_types::{ActionDiscoverySection, DiscoveryState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionDiscoveryPanelSpec {
    pub sections: Vec<ActionDiscoverySection>,
    pub state: DiscoveryState,
    pub empty_message: Option<String>,
    /// Id of the currently active/highlighted action item (contract §9
    /// active-item accent treatment). Keyboard nav that mutates this lives in
    /// the consuming app's event loop; the spec only carries the resolved id.
    pub active_id: Option<String>,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
}

impl Default for ActionDiscoveryPanelSpec {
    fn default() -> Self {
        Self {
            sections: Vec::new(),
            state: DiscoveryState::Ready,
            empty_message: None,
            active_id: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
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

    pub fn with_active_id(mut self, active_id: impl Into<String>) -> Self {
        self.active_id = Some(active_id.into());
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
