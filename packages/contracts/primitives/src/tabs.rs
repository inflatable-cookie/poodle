use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, Orientation, SemanticControlSizeRole, TabActivationMode, TabDefinition, TabVariant};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabsSpec {
    pub tabs: Vec<TabDefinition>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub variant: TabVariant,
    pub orientation: Orientation,
    pub activation_mode: TabActivationMode,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for TabsSpec {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            value: None,
            default_value: None,
            variant: TabVariant::Underline,
            orientation: Orientation::Horizontal,
            activation_mode: TabActivationMode::Automatic,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
        }
    }
}

impl TabsSpec {
    pub fn new(tabs: Vec<TabDefinition>) -> Self {
        Self {
            tabs,
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

    pub fn with_variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_activation_mode(mut self, activation_mode: TabActivationMode) -> Self {
        self.activation_mode = activation_mode;
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
                self.tabs
                    .iter()
                    .find(|tab| !tab.is_disabled)
                    .map(|tab| tab.value.as_str())
            })
    }

    pub fn selected_tab(&self) -> Option<&TabDefinition> {
        let current = self.current_value()?;
        self.tabs.iter().find(|tab| tab.value == current)
    }

    pub fn uses_manual_activation(&self) -> bool {
        self.activation_mode == TabActivationMode::Manual
    }

    pub fn list_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn indicator_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn list_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn pill_border_opacity(&self) -> f32 {
        0.68
    }

    pub fn pill_active_bg_opacity(&self) -> f32 {
        0.18
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
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
