use pug_tokens::semantic;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Outlined,
    Elevated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardLayout {
    #[default]
    Vertical,
    Horizontal,
    Compact,
}

#[derive(Debug, Clone)]
pub struct CardSpec {
    pub variant: CardVariant,
    pub layout: CardLayout,
    pub is_interactive: bool,
    pub is_selected: bool,
    pub aria_label: Option<String>,
}

impl Default for CardSpec {
    fn default() -> Self {
        Self {
            variant: CardVariant::default(),
            layout: CardLayout::default(),
            is_interactive: false,
            is_selected: false,
            aria_label: None,
        }
    }
}

impl CardSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_layout(mut self, layout: CardLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn interactive(mut self) -> Self {
        self.is_interactive = true;
        self
    }

    pub fn selected(mut self) -> Self {
        self.is_selected = true;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    // Token methods

    pub fn fill_token(&self) -> &'static str {
        match self.variant {
            CardVariant::Default | CardVariant::Outlined => semantic::COLOR_BACKGROUND_SURFACE,
            CardVariant::Elevated => semantic::COLOR_BACKGROUND_ELEVATED,
        }
    }

    pub fn border_token(&self) -> Option<&'static str> {
        match self.variant {
            CardVariant::Default => Some(semantic::COLOR_BORDER_SUBTLE),
            CardVariant::Outlined => Some(semantic::COLOR_BORDER_DEFAULT),
            CardVariant::Elevated => Some(semantic::COLOR_BORDER_SUBTLE),
        }
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn shadow_token(&self) -> Option<&'static str> {
        match self.variant {
            CardVariant::Default | CardVariant::Outlined => None,
            CardVariant::Elevated => Some(semantic::ELEVATION_OVERLAY),
        }
    }

    pub fn selected_border_token(&self) -> Option<&'static str> {
        if self.is_selected {
            Some(semantic::COLOR_ACCENT_BASE)
        } else {
            None
        }
    }

    pub fn hover_fill_token(&self) -> Option<&'static str> {
        if self.is_interactive {
            Some(semantic::COLOR_BACKGROUND_ELEVATED)
        } else {
            None
        }
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn gap_token(&self) -> &'static str {
        match self.layout {
            CardLayout::Compact => semantic::SPACE_STACK_SM,
            _ => semantic::SPACE_STACK_MD,
        }
    }

    pub fn padding_x_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    pub fn padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }
}
