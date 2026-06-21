use poodle_tokens::semantic;

use crate::types::{Alignment, ControlDensity, ControlSize, Orientation, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolbarSpec {
    pub alignment: Alignment,
    /// Layout axis for the toolbar items. Horizontal (default) renders
    /// items in a row; Vertical renders them stacked in a column.
    /// Matches the contract doc's `orientation` prop.
    pub orientation: Orientation,
    pub has_separator: bool,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for ToolbarSpec {
    fn default() -> Self {
        Self {
            alignment: Alignment::Start,
            orientation: Orientation::Horizontal,
            has_separator: false,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
        }
    }
}

impl ToolbarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_separator(mut self, has_separator: bool) -> Self {
        self.has_separator = has_separator;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn bg_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Container border-radius. Contract §8: `radius.surface`.
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn padding_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
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
