use crate::composite_types::{SplitOrientation, SplitToggleVisibility};
use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, PartialEq)]
pub struct SplitViewSpec {
    /// Stable native instance scope. The split composes a ResizeHandle, whose
    /// backend focus and gesture state need an identity no two splits share;
    /// this is where that identity enters, and the divider's scope is derived
    /// from it.
    pub instance_id: String,
    pub orientation: SplitOrientation,
    pub ratio: Option<f32>,
    pub default_ratio: f32,
    pub min_primary_size: Option<f32>,
    pub min_secondary_size: Option<f32>,
    /// Fixed primary pane size in px. When set, primary uses this
    /// absolute size and secondary fills the remaining space, bypassing
    /// ratio-based allocation. Mutually exclusive with
    /// `secondary_size` at the layout level — setting both is
    /// caller error; the primary wins.
    pub primary_size: Option<f32>,
    /// Fixed secondary pane size in px. When set, secondary uses this
    /// absolute size and primary fills the remaining space, bypassing
    /// ratio-based allocation.
    pub secondary_size: Option<f32>,
    pub is_primary_collapsed: bool,
    pub is_secondary_collapsed: bool,
    /// Takes zero space without being a collapse: no toggle, no collapsed
    /// data attribute. For panes that are absent rather than user-collapsed.
    pub is_primary_hidden: bool,
    /// See `is_primary_hidden`.
    pub is_secondary_hidden: bool,
    /// When true the divider cannot be dragged and the split renders
    /// in a non-interactive state. Collapse toggles (if shown) are
    /// disabled as well.
    pub is_disabled: bool,
    /// When true the split renders a collapse-toggle affordance on
    /// the primary side of the divider.
    pub show_collapse_primary: bool,
    /// When true the split renders a collapse-toggle affordance on
    /// the secondary side of the divider.
    pub show_collapse_secondary: bool,
    /// When the collapse-toggle pill is visible. `Hover` keeps it out of the
    /// way until the pointer reaches the seam; a collapsed pane's expand
    /// toggle stays visible regardless, since it is the only way back.
    pub toggle_visibility: SplitToggleVisibility,
    /// When true the divider paints a visible line. Default false: pane
    /// borders read as the separator and the resize handle's grab area is an
    /// overlay with no layout footprint either way.
    pub divider: bool,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
    /// Below this pane size (px) the primary pane collapses.
    pub collapse_primary_below_size: Option<f32>,
    pub collapse_secondary_below_size: Option<f32>,
    /// Size (px) the primary pane holds while collapsed.
    pub primary_collapsed_size: Option<f32>,
    pub secondary_collapsed_size: Option<f32>,
}

impl SplitViewSpec {
    /// Below `below_size` px the pane collapses to `collapsed_size` px.
    pub fn with_primary_collapse(mut self, below_size: f32, collapsed_size: f32) -> Self {
        self.collapse_primary_below_size = Some(below_size);
        self.primary_collapsed_size = Some(collapsed_size);
        self
    }

    pub fn with_secondary_collapse(mut self, below_size: f32, collapsed_size: f32) -> Self {
        self.collapse_secondary_below_size = Some(below_size);
        self.secondary_collapsed_size = Some(collapsed_size);
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    /// The instance scope has no default, for the reason `ResizeHandleSpec`
    /// states: a derived or render-order key cannot tell two identical splits
    /// apart, and the divider they compose would share one focus handle.
    pub fn new(instance_id: impl Into<String>, orientation: SplitOrientation) -> Self {
        Self {
            instance_id: instance_id.into(),
            orientation,
            ratio: None,
            default_ratio: 0.5,
            min_primary_size: None,
            min_secondary_size: None,
            primary_size: None,
            secondary_size: None,
            is_primary_collapsed: false,
            is_secondary_collapsed: false,
            is_primary_hidden: false,
            is_secondary_hidden: false,
            is_disabled: false,
            show_collapse_primary: false,
            show_collapse_secondary: false,
            toggle_visibility: SplitToggleVisibility::Always,
            divider: false,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            aria_label: None,
            collapse_primary_below_size: None,
            collapse_secondary_below_size: None,
            primary_collapsed_size: None,
            secondary_collapsed_size: None,
        }
    }

    /// Scope for the ResizeHandle this split composes. Derived, not passed:
    /// one split has exactly one divider, so a second caller-stated id would
    /// be a second chance to get it wrong.
    pub fn divider_instance_id(&self) -> String {
        format!("{}:divider", self.instance_id)
    }

    pub fn with_ratio(mut self, ratio: f32) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn with_default_ratio(mut self, default_ratio: f32) -> Self {
        self.default_ratio = default_ratio;
        self
    }

    pub fn with_min_primary_size(mut self, min_primary_size: f32) -> Self {
        self.min_primary_size = Some(min_primary_size);
        self
    }

    pub fn with_min_secondary_size(mut self, min_secondary_size: f32) -> Self {
        self.min_secondary_size = Some(min_secondary_size);
        self
    }

    pub fn with_primary_size(mut self, primary_size: f32) -> Self {
        self.primary_size = Some(primary_size);
        self
    }

    pub fn with_secondary_size(mut self, secondary_size: f32) -> Self {
        self.secondary_size = Some(secondary_size);
        self
    }

    pub fn with_primary_collapsed(mut self, is_primary_collapsed: bool) -> Self {
        self.is_primary_collapsed = is_primary_collapsed;
        self
    }

    pub fn with_secondary_collapsed(mut self, is_secondary_collapsed: bool) -> Self {
        self.is_secondary_collapsed = is_secondary_collapsed;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_show_collapse_primary(mut self, show: bool) -> Self {
        self.show_collapse_primary = show;
        self
    }

    pub fn with_show_collapse_secondary(mut self, show: bool) -> Self {
        self.show_collapse_secondary = show;
        self
    }

    pub fn with_toggle_visibility(mut self, visibility: SplitToggleVisibility) -> Self {
        self.toggle_visibility = visibility;
        self
    }

    /// The toggle cluster's resting opacity: hidden when the split reveals its
    /// toggles on hover and neither pane is collapsed. A collapsed pane keeps
    /// its expand toggle on screen — with the pane gone there is no seam left
    /// to hover toward, so hiding it would strand the pane.
    pub fn toggles_hidden_until_hover(&self) -> bool {
        self.toggle_visibility == SplitToggleVisibility::Hover
            && !self.is_primary_collapsed
            && !self.is_secondary_collapsed
    }

    pub fn current_ratio(&self) -> f32 {
        self.ratio.unwrap_or(self.default_ratio).clamp(0.0, 1.0)
    }

    pub fn keyboard_resize_supported(&self) -> bool {
        !self.is_primary_collapsed && !self.is_secondary_collapsed
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
