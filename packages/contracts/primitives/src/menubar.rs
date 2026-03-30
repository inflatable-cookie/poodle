use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, MenubarEntry, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenubarSpec {
    pub items: Vec<MenubarEntry>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for MenubarSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            value: None,
            default_value: None,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
        }
    }
}

impl MenubarSpec {
    pub fn new(items: Vec<MenubarEntry>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or(self.default_value.as_deref())
            .or_else(|| {
                self.items
                    .iter()
                    .find(|item| !item.is_disabled)
                    .map(|item| item.value.as_str())
            })
    }

    pub fn current_menu(&self) -> Option<&MenubarEntry> {
        let current = self.current_value()?;
        self.items.iter().find(|item| item.value == current)
    }

    pub fn trigger_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn list_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn list_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn list_bg_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
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
