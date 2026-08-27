use crate::icon::IconSize;
use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// Direction the collapse toggle points when in the "collapsed" position.
///
/// The contract specifies "left", "right", "up", "down". When expanded, the
/// icon points in the given direction; when collapsed, it points in the
/// opposite direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum CollapseDirection {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

impl CollapseDirection {
    /// The opposite direction (used when collapsed to indicate "expand toward").
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    /// Icon name for this direction (e.g. "chevron-left").
    pub fn icon_name(self) -> &'static str {
        match self {
            Self::Left => "chevron-left",
            Self::Right => "chevron-right",
            Self::Up => "chevron-up",
            Self::Down => "chevron-down",
        }
    }
}

/// Spec for the CollapseToggle component per the contract.
///
/// A standalone collapse/expand toggle button with a directional chevron icon.
/// The host determines what content gets collapsed; this component only
/// provides the interactive toggle affordance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollapseToggleSpec {
    /// Whether the associated content is currently collapsed.
    pub is_collapsed: bool,
    /// The direction the chevron points when expanded.
    pub direction: CollapseDirection,
    /// Whether interaction is suppressed.
    pub is_disabled: bool,
    /// Custom accessible label. Defaults to "Collapse"/"Expand".
    pub aria_label: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for CollapseToggleSpec {
    fn default() -> Self {
        Self {
            is_collapsed: false,
            direction: CollapseDirection::Left,
            is_disabled: false,
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Chrome,
            density: None,
        }
    }
}

impl CollapseToggleSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_collapsed(mut self, is_collapsed: bool) -> Self {
        self.is_collapsed = is_collapsed;
        self
    }

    pub fn with_direction(mut self, direction: CollapseDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Whether clicking this toggle should be allowed.
    pub fn activation_allowed(&self) -> bool {
        !self.is_disabled
    }

    /// The effective icon name based on collapsed state and direction.
    ///
    /// When expanded, the icon points in the collapse `direction`.
    /// When collapsed, the icon points in the opposite direction.
    pub fn effective_icon_name(&self) -> &'static str {
        if self.is_collapsed {
            self.direction.opposite().icon_name()
        } else {
            self.direction.icon_name()
        }
    }

    /// The effective aria label for the toggle.
    pub fn effective_aria_label(&self) -> &str {
        if let Some(ref label) = self.aria_label {
            label.as_str()
        } else if self.is_collapsed {
            "Expand"
        } else {
            "Collapse"
        }
    }

    // ── Size / density resolution ────────────────────────────
    //
    // Mirrors the Svelte `resolveSemanticControlSize` and the
    // `.poodle-collapse-toggle[data-size]` / `[data-density]` padding ladders in
    // `CollapseToggle.svelte` verbatim. The arithmetic is duplicated here (rather
    // than referencing a per-target presentation module) so both Rust targets
    // resolve identical geometry from a single spec source of truth.

    /// Effective control size after applying `size_role` to the resolved
    /// `size` (omission already inherited from the presentation context).
    /// `Chrome` role (the default) shifts one stop smaller.
    pub fn effective_size(&self, size: ControlSize) -> ControlSize {
        match (size, self.size_role) {
            (s, SemanticControlSizeRole::Control) => s,

            (ControlSize::Xs, SemanticControlSizeRole::Chrome) => ControlSize::Xs,
            (ControlSize::Sm, SemanticControlSizeRole::Chrome) => ControlSize::Sm,
            (ControlSize::Md, SemanticControlSizeRole::Chrome) => ControlSize::Sm,
            (ControlSize::Lg, SemanticControlSizeRole::Chrome) => ControlSize::Md,
            (ControlSize::Xl, SemanticControlSizeRole::Chrome) => ControlSize::Lg,

            (ControlSize::Xs, SemanticControlSizeRole::Prominent) => ControlSize::Sm,
            (ControlSize::Sm, SemanticControlSizeRole::Prominent) => ControlSize::Md,
            (ControlSize::Md, SemanticControlSizeRole::Prominent) => ControlSize::Lg,
            (ControlSize::Lg, SemanticControlSizeRole::Prominent) => ControlSize::Xl,
            (ControlSize::Xl, SemanticControlSizeRole::Prominent) => ControlSize::Xl,
        }
    }

    /// Full button padding in rem for the effective size (contract §8 size table:
    /// xs 0.0625, sm/md 0.125, lg 0.1875, xl 0.25).
    pub fn padding_rem(&self, size: ControlSize) -> f32 {
        match self.effective_size(size) {
            ControlSize::Xs => 0.0625,
            ControlSize::Sm => 0.125,
            ControlSize::Md => 0.125,
            ControlSize::Lg => 0.1875,
            ControlSize::Xl => 0.25,
        }
    }

    /// Horizontal (inline) padding in rem for the density (contract §8 density
    /// table: comfortable 0.375, compact/default 0.125 = base). Density only
    /// overrides `padding-inline`; it never touches vertical padding or height.
    pub fn padding_inline_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Comfortable => 0.375,
            ControlDensity::Compact | ControlDensity::Default => 0.125,
        }
    }

    /// The chevron icon size, scaling with the effective control size to match
    /// Svelte's `<Icon size={resolvedSize} />`.
    pub fn effective_icon_size(&self, size: ControlSize) -> IconSize {
        IconSize::from(self.effective_size(size))
    }

    // ── Token methods ────────────────────────────────────────

    /// Icon size token for the chevron, scaled by the effective control size.
    pub fn icon_size_token(&self, size: ControlSize) -> &'static str {
        self.effective_icon_size(size).size_token()
    }

    /// Radius for the toggle button (small, like the Svelte radius-sm).
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Icon/text color (muted when idle).
    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Icon/text color on hover.
    pub fn text_color_hover_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Background on hover.
    pub fn hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    /// Disabled opacity.
    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    /// Focus ring color.
    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    /// Focus ring width.
    pub fn focus_ring_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_FOCUS
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_and_explicit_labels_follow_collapsed_state() {
        assert_eq!(CollapseToggleSpec::new().effective_aria_label(), "Collapse");
        assert_eq!(
            CollapseToggleSpec::new()
                .with_collapsed(true)
                .effective_aria_label(),
            "Expand"
        );
        assert_eq!(
            CollapseToggleSpec::new()
                .with_collapsed(false)
                .with_aria_label("Collapse left dock")
                .effective_aria_label(),
            "Collapse left dock"
        );
        assert_eq!(
            CollapseToggleSpec::new()
                .with_collapsed(true)
                .with_aria_label("Collapse left dock")
                .effective_aria_label(),
            "Collapse left dock"
        );
    }

    #[test]
    fn every_direction_maps_to_its_opposite_when_collapsed() {
        let pairs = [
            (CollapseDirection::Left, "chevron-left", "chevron-right"),
            (CollapseDirection::Right, "chevron-right", "chevron-left"),
            (CollapseDirection::Up, "chevron-up", "chevron-down"),
            (CollapseDirection::Down, "chevron-down", "chevron-up"),
        ];
        for (direction, expanded, collapsed) in pairs {
            let expanded_spec = CollapseToggleSpec::new().with_direction(direction);
            assert_eq!(expanded_spec.effective_icon_name(), expanded);
            let collapsed_spec = expanded_spec.with_collapsed(true);
            assert_eq!(collapsed_spec.effective_icon_name(), collapsed);
        }
    }
}
