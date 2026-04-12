use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogListSpec {
    pub entry_count: usize,
    pub max_entries: usize,
    pub auto_scroll: bool,
    pub filter_level: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl LogListSpec {
    pub fn new() -> Self {
        Self {
            entry_count: 0,
            max_entries: 500,
            auto_scroll: true,
            filter_level: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_entry_count(mut self, entry_count: usize) -> Self {
        self.entry_count = entry_count;
        self
    }

    pub fn with_max_entries(mut self, max_entries: usize) -> Self {
        self.max_entries = max_entries;
        self
    }

    pub fn with_auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.auto_scroll = auto_scroll;
        self
    }

    pub fn with_filter_level(mut self, filter_level: impl Into<String>) -> Self {
        self.filter_level = Some(filter_level.into());
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn entry_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
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
