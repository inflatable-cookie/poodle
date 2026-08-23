use poodle_tokens::semantic;

use crate::types::ControlDensity;

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
    /// Density override driving internal padding, gap, and footer spacing
    /// (contract §8 density table). `None` inherits from the presentation
    /// context; an explicit value always wins.
    pub density: Option<ControlDensity>,
    pub is_interactive: bool,
    pub is_selected: bool,
    /// Enables the media slot region (overflow-clipped, inset radius — contract §2/§8).
    pub has_media: bool,
    pub aria_label: Option<String>,
}

impl Default for CardSpec {
    fn default() -> Self {
        Self {
            variant: CardVariant::default(),
            layout: CardLayout::default(),
            density: None,
            is_interactive: false,
            is_selected: false,
            has_media: false,
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

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    pub fn with_media(mut self, has_media: bool) -> Self {
        self.has_media = has_media;
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

    /// Internal gap token. The `compact` *layout* overrides to `space.stack.sm`;
    /// otherwise the density axis drives the gap (contract §8 density table).
    /// `default` density maps cleanly to `space.stack.md`; `compact`/`comfortable`
    /// have no exact token, so prefer [`Self::gap_rem`] for those.
    pub fn gap_token(&self, density: ControlDensity) -> &'static str {
        match self.layout {
            CardLayout::Compact => semantic::SPACE_STACK_SM,
            _ => match density {
                ControlDensity::Default => semantic::SPACE_STACK_MD,
                // 0.625rem (10px) — no exact token; closest is space.stack.sm (8px).
                ControlDensity::Compact => semantic::SPACE_STACK_SM,
                // 1rem (16px) — maps to space.panel.x / space.inline.lg.
                ControlDensity::Comfortable => semantic::SPACE_INLINE_LG,
            },
        }
    }

    /// Contract-exact internal gap in rem, density/layout-aware. Use when no
    /// exact token exists (compact `0.625rem`, comfortable `1rem`).
    pub fn gap_rem(&self, density: ControlDensity) -> f32 {
        match self.layout {
            CardLayout::Compact => 0.5, // space.stack.sm
            _ => match density {
                ControlDensity::Compact => 0.625,
                ControlDensity::Default => 0.75, // space.stack.md
                ControlDensity::Comfortable => 1.0,
            },
        }
    }

    /// Inline (horizontal) padding token. The `compact` *layout* overrides to
    /// `space.panel.x-sm`-equivalent (`0.625rem`); otherwise density drives it.
    pub fn padding_x_token(&self, density: ControlDensity) -> &'static str {
        match self.layout {
            // compact layout: 0.625rem (10px) — no exact token; space.inline.md (12) closest.
            CardLayout::Compact => semantic::SPACE_INLINE_MD,
            _ => match density {
                ControlDensity::Default => semantic::SPACE_PANEL_X,
                // 0.75rem (12px) → space.panel.y / space.stack.md.
                ControlDensity::Compact => semantic::SPACE_PANEL_Y,
                // 1rem (16px) → space.panel.x.
                ControlDensity::Comfortable => semantic::SPACE_PANEL_X,
            },
        }
    }

    /// Block (vertical) padding token. The `compact` *layout* overrides to
    /// `0.5rem`; otherwise density drives it. Per the contract density table,
    /// non-compact-layout block padding equals the inline padding value.
    pub fn padding_y_token(&self, density: ControlDensity) -> &'static str {
        match self.layout {
            // compact layout: 0.5rem (8px) → space.stack.sm / space.control.y.
            CardLayout::Compact => semantic::SPACE_STACK_SM,
            _ => match density {
                // density table: padding is square (block == inline).
                ControlDensity::Default => semantic::SPACE_PANEL_X,
                ControlDensity::Compact => semantic::SPACE_PANEL_Y,
                ControlDensity::Comfortable => semantic::SPACE_PANEL_X,
            },
        }
    }

    /// Contract-exact inline padding in rem, density/layout-aware.
    pub fn padding_x_rem(&self, density: ControlDensity) -> f32 {
        match self.layout {
            CardLayout::Compact => 0.625,
            _ => match density {
                ControlDensity::Compact => 0.75,
                ControlDensity::Default => 1.0, // space.panel.x
                ControlDensity::Comfortable => 1.0,
            },
        }
    }

    /// Contract-exact block padding in rem, density/layout-aware.
    pub fn padding_y_rem(&self, density: ControlDensity) -> f32 {
        match self.layout {
            CardLayout::Compact => 0.5,
            _ => match density {
                ControlDensity::Compact => 0.75,
                ControlDensity::Default => 1.0, // space.panel.x (square)
                ControlDensity::Comfortable => 1.0,
            },
        }
    }

    /// Footer top-padding token, density-aware (contract §8). Only `default`
    /// density maps to an exact token (`space.stack.sm`); prefer
    /// [`Self::footer_padding_top_rem`] for compact/comfortable.
    pub fn footer_padding_top_token(&self, density: ControlDensity) -> &'static str {
        match density {
            ControlDensity::Default => semantic::SPACE_STACK_SM,
            // 0.625rem (10px) — no exact token.
            ControlDensity::Compact => semantic::SPACE_STACK_SM,
            // 0.875rem (14px) — no exact token; space.inline.lg (16) closest.
            ControlDensity::Comfortable => semantic::SPACE_INLINE_LG,
        }
    }

    /// Contract-exact footer top-padding in rem, density-aware.
    pub fn footer_padding_top_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.5, // space.stack.sm
            ControlDensity::Comfortable => 0.875,
        }
    }

    /// Footer divider color token (contract §8: border-subtle at 52%).
    pub fn footer_divider_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// Border-width token for the root border (`0.0625rem`, contract §8).
    pub fn border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }

    /// Media region inset-radius reduction in rem (`radius - 0.1875rem`, contract §8).
    pub fn media_radius_inset_rem(&self) -> f32 {
        0.1875
    }
}
