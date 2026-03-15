use pug_gpui_tokens::semantic;

use crate::types::{Orientation, TabActivationMode, TabDefinition, TabVariant};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabsSpec {
    pub tabs: Vec<TabDefinition>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub orientation: Orientation,
    pub activation_mode: TabActivationMode,
    pub variant: TabVariant,
    pub aria_label: Option<String>,
}

impl Default for TabsSpec {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            value: None,
            default_value: None,
            orientation: Orientation::Horizontal,
            activation_mode: TabActivationMode::Automatic,
            variant: TabVariant::Underline,
            aria_label: None,
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

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_activation_mode(mut self, activation_mode: TabActivationMode) -> Self {
        self.activation_mode = activation_mode;
        self
    }

    pub fn with_variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
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
        match self.variant {
            // Pill gap: 0.125rem (2px) — no named token, hardcode the rem value
            TabVariant::Pill => "0.125rem",
            _ => semantic::SPACE_INLINE_SM,
        }
    }

    pub fn indicator_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn list_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// The opacity multiplier for the pill variant's container border.
    /// Svelte applies: `color-mix(in srgb, border-subtle 68%, transparent)`.
    pub fn pill_border_opacity(&self) -> f32 {
        0.68
    }

    /// The opacity multiplier for the pill variant's active tab background.
    /// Svelte applies: `color-mix(in srgb, accent 18%, transparent)`.
    pub fn pill_active_bg_opacity(&self) -> f32 {
        0.18
    }
}
