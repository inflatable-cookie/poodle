use pug_gpui_tokens::semantic;

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
}

impl Default for CollapseToggleSpec {
    fn default() -> Self {
        Self {
            is_collapsed: false,
            direction: CollapseDirection::Left,
            is_disabled: false,
            aria_label: None,
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

    // ── Token methods ────────────────────────────────────────

    /// Small icon size for the chevron.
    pub fn icon_size_token(&self) -> &'static str {
        semantic::SIZE_ICON_SM
    }

    /// Radius for the toggle button.
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
}
