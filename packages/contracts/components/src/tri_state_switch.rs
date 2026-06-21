use poodle_tokens::semantic;

use crate::types::{
    CheckState, ControlDensity, ControlSize, SemanticControlSizeRole, TriStateValue,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TriStateSwitchSpec {
    pub state: CheckState,
    pub size: ControlSize,
    pub label: Option<String>,
    pub excluded_label: Option<String>,
    pub default_label: Option<String>,
    pub included_label: Option<String>,
    pub excluded_color: Option<String>,
    pub default_color: Option<String>,
    pub included_color: Option<String>,
    pub is_disabled: bool,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for TriStateSwitchSpec {
    fn default() -> Self {
        Self {
            state: CheckState::Unchecked,
            size: ControlSize::Md,
            label: None,
            excluded_label: None,
            default_label: None,
            included_label: None,
            excluded_color: None,
            default_color: None,
            included_color: None,
            is_disabled: false,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl TriStateSwitchSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(mut self, state: CheckState) -> Self {
        self.state = state;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_excluded_label(mut self, label: impl Into<String>) -> Self {
        self.excluded_label = Some(label.into());
        self
    }

    pub fn with_default_label(mut self, label: impl Into<String>) -> Self {
        self.default_label = Some(label.into());
        self
    }

    pub fn with_included_label(mut self, label: impl Into<String>) -> Self {
        self.included_label = Some(label.into());
        self
    }

    pub fn with_excluded_color(mut self, color: impl Into<String>) -> Self {
        self.excluded_color = Some(color.into());
        self
    }

    pub fn with_default_color(mut self, color: impl Into<String>) -> Self {
        self.default_color = Some(color.into());
        self
    }

    pub fn with_included_color(mut self, color: impl Into<String>) -> Self {
        self.included_color = Some(color.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn excluded_label(&self) -> &str {
        self.excluded_label.as_deref().unwrap_or("Exclude")
    }

    pub fn default_label(&self) -> &str {
        self.default_label.as_deref().unwrap_or("Default")
    }

    pub fn included_label(&self) -> &str {
        self.included_label.as_deref().unwrap_or("Include")
    }

    pub fn aria_checked(&self) -> &'static str {
        self.state.aria_checked()
    }

    /// The ternary value in contract terms (excluded/default/included),
    /// derived from the legacy `CheckState` storage.
    pub fn value(&self) -> TriStateValue {
        TriStateValue::from_check_state(self.state)
    }

    /// Selected segment index (excluded 0, default 1, included 2). Drives the
    /// sliding-capsule offset (`translateX(index * 100%)` in Svelte).
    pub fn selected_index(&self) -> usize {
        self.value().index()
    }

    // ── Per-state semantic colors (contract §8 root custom props) ──
    //
    // excluded → status-danger, default → text-primary, included →
    // status-success. Honor the optional per-instance color overrides when
    // present (the override string is a hex value parsed by the targets).

    pub fn excluded_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    pub fn default_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn included_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_SUCCESS
    }

    /// Track base color the per-state fills blend over.
    /// Contract: `color-mix(canvas 70%, black)`. Targets resolve
    /// `background.canvas` then mix toward black at this ratio.
    pub fn track_base_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_CANVAS
    }

    /// Root track background base, mixed toward black.
    /// Contract: `color-mix(canvas 75%, black)`.
    pub fn root_bg_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_CANVAS
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn unselected_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn label_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    pub fn label_weight_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_WEIGHT
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
