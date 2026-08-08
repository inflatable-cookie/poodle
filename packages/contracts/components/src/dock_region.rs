use poodle_tokens::semantic;

use crate::composite_types::{DockEdge, PanelTabItem};
use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockTabsPlacement {
    Edge,
    Top,
}

/// Sizing mode — `Flexible` for tabbed/collapsible docks, `Static` for fixed
/// stacked panels. Mirrors Svelte `sizing` prop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockSizing {
    Flexible,
    Static,
}

/// Collapsed-state chrome posture. `IconStrip` renders narrow icon-only tabs;
/// `Hidden` renders only the CollapseToggle. Mirrors Svelte `collapsedPosture`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockCollapsedPosture {
    IconStrip,
    Hidden,
}

/// Region visual emphasis. Mirrors Svelte `emphasis` prop and the
/// `data-emphasis` attribute (`standard` / `quiet` / `strong`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockEmphasis {
    Standard,
    Quiet,
    Strong,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DockRegionSpec {
    pub edge: DockEdge,
    pub sizing: DockSizing,
    pub is_collapsed: bool,
    /// When true, renders the CollapseToggle; when false, no collapse affordance.
    pub is_collapsible: bool,
    pub collapsed_posture: DockCollapsedPosture,
    pub emphasis: DockEmphasis,
    pub tabs_placement: DockTabsPlacement,
    /// When true the region advertises a cross-region drop target (drag-over
    /// affordance). Render-only proxy for Svelte's `canAcceptPanel` callback,
    /// whose validation logic lives in the host event loop.
    pub can_accept_panel: bool,
    pub items: Vec<PanelTabItem>,
    pub value: Option<String>,
    pub aria_label: Option<String>,
    /// Exact drop-zone identity for the same-zone drop guard; when None the
    /// edge is the zone. Hosts mapping several regions onto one edge set a
    /// per-region id here.
    ///
    /// Carried for surface parity and read by nothing today: panel drag is a
    /// drag-with-payload gesture the native event vocabulary does not have, so
    /// the natives draw the drop zone and run no gesture. Recorded in
    /// `dock-region.md` §12.
    pub drag_zone_id: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl DockRegionSpec {
    pub fn new(edge: DockEdge, items: Vec<PanelTabItem>) -> Self {
        Self {
            edge,
            sizing: DockSizing::Flexible,
            is_collapsed: false,
            is_collapsible: false,
            collapsed_posture: DockCollapsedPosture::IconStrip,
            emphasis: DockEmphasis::Standard,
            tabs_placement: DockTabsPlacement::Edge,
            can_accept_panel: false,
            items,
            value: None,
            aria_label: None,
            drag_zone_id: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_sizing(mut self, sizing: DockSizing) -> Self {
        self.sizing = sizing;
        self
    }

    pub fn with_collapsed_posture(mut self, collapsed_posture: DockCollapsedPosture) -> Self {
        self.collapsed_posture = collapsed_posture;
        self
    }

    pub fn with_emphasis(mut self, emphasis: DockEmphasis) -> Self {
        self.emphasis = emphasis;
        self
    }

    pub fn with_can_accept_panel(mut self, can_accept_panel: bool) -> Self {
        self.can_accept_panel = can_accept_panel;
        self
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
