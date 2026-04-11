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
    /// When true, tabs can be reordered via drag. Defers to the
    /// consumer to actually commit the new order. Matches Svelte
    /// `reorderable` prop.
    pub is_reorderable: bool,
    /// When true, the Underline variant renders the bottom border
    /// line under the whole tab list (the default). When false the
    /// indicator is suppressed — useful for flush layouts. Matches
    /// Svelte `bordered` prop (default true).
    pub is_bordered: bool,
    /// Optional key used to persist the active tab across sessions.
    /// The consumer owns the actual storage; this field carries the
    /// key so the render surface can expose a stable id for tests /
    /// state.
    pub history_key: Option<String>,
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
            is_reorderable: false,
            is_bordered: true,
            history_key: None,
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

    pub fn with_reorderable(mut self, is_reorderable: bool) -> Self {
        self.is_reorderable = is_reorderable;
        self
    }

    pub fn with_bordered(mut self, is_bordered: bool) -> Self {
        self.is_bordered = is_bordered;
        self
    }

    pub fn with_history_key(mut self, history_key: impl Into<String>) -> Self {
        self.history_key = Some(history_key.into());
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

    /// Block variant: opacity of the panel background tint applied to the
    /// tab list. Matches Svelte `color-mix(... panel 90%, transparent)`.
    pub fn block_list_bg_opacity(&self) -> f32 {
        0.9
    }

    /// Block variant: opacity of the accent tint layered onto the surface
    /// for the selected item. Matches Svelte `accent 14% + surface`.
    pub fn block_selected_accent_mix(&self) -> f32 {
        0.14
    }

    /// Block variant: opacity of the accent tint on hover of a selected
    /// item. Matches Svelte `accent 18% + surface`.
    pub fn block_selected_hover_accent_mix(&self) -> f32 {
        0.18
    }

    /// Block variant: opacity of the separator border between items.
    /// Matches Svelte `border-subtle 72%`.
    pub fn block_separator_opacity(&self) -> f32 {
        0.72
    }

    /// Block variant: opacity of the elevated background used on unselected
    /// item hover. Matches Svelte `elevated 50%`.
    pub fn block_hover_bg_opacity(&self) -> f32 {
        0.5
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
