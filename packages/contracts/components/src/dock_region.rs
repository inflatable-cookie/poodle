use poodle_tokens::semantic;

use crate::composite_types::{DockEdge, PanelTabItem};
use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockTabsPlacement {
    Edge,
    Top,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DockRegionSpec {
    pub edge: DockEdge,
    pub is_collapsed: bool,
    /// When true, renders the CollapseToggle; when false, no collapse affordance.
    pub is_collapsible: bool,
    pub tabs_placement: DockTabsPlacement,
    pub items: Vec<PanelTabItem>,
    pub value: Option<String>,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl DockRegionSpec {
    pub fn new(edge: DockEdge, items: Vec<PanelTabItem>) -> Self {
        Self {
            edge,
            is_collapsed: false,
            is_collapsible: false,
            tabs_placement: DockTabsPlacement::Edge,
            items,
            value: None,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_collapsed(mut self, is_collapsed: bool) -> Self {
        self.is_collapsed = is_collapsed;
        self
    }

    pub fn with_collapsible(mut self, is_collapsible: bool) -> Self {
        self.is_collapsible = is_collapsible;
        self
    }

    pub fn with_tabs_placement(mut self, tabs_placement: DockTabsPlacement) -> Self {
        self.tabs_placement = tabs_placement;
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or_else(|| self.items.first().map(|item| item.value.as_str()))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn strip_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
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
