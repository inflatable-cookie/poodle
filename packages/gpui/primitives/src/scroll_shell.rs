use pug_gpui_tokens::semantic;

use crate::types::{Direction, Inset, PaddingScale, SurfaceRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScrollShellSpec {
    pub direction: Direction,
    pub padding: PaddingScale,
    pub role: Option<SurfaceRole>,
    pub label: Option<String>,
    pub is_focusable: bool,
}

impl Default for ScrollShellSpec {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            padding: PaddingScale::None,
            role: None,
            label: None,
            is_focusable: false,
        }
    }
}

impl ScrollShellSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_padding(mut self, padding: PaddingScale) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_role(mut self, role: SurfaceRole) -> Self {
        self.role = Some(role);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_focusable(mut self, is_focusable: bool) -> Self {
        self.is_focusable = is_focusable;
        self
    }

    pub fn resolved_padding(&self) -> Inset {
        self.padding.panel_inset()
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn focus_ring_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_FOCUS
    }
}
