use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// Tone for a Switch's left/right labels. Resolves to a token-side color
/// used to tint the corresponding track side (off-color for `left_tone`,
/// on-color for `right_tone`) when no explicit custom color is supplied.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SwitchTone {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
}

impl SwitchTone {
    /// Semantic token that backs this tone. `Default` returns `None`
    /// meaning "use the Switch's native track colour".
    pub fn color_token(self) -> Option<&'static str> {
        match self {
            Self::Default => None,
            Self::Primary => Some(semantic::COLOR_ACCENT_BASE),
            Self::Success => Some(semantic::COLOR_STATUS_SUCCESS),
            Self::Warning => Some(semantic::COLOR_STATUS_WARNING),
            Self::Danger => Some(semantic::COLOR_STATUS_DANGER),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwitchSpec {
    pub checked: Option<bool>,
    pub default_checked: bool,
    pub is_disabled: bool,
    pub is_read_only: bool,
    /// HTML `name` attribute for form submission. Required when the switch
    /// participates in a form (the Svelte implementation passes this to the
    /// underlying hidden input).
    pub name: Option<String>,
    pub label: Option<String>,
    /// Left-side label for dual-label mode. When either `left_label` or
    /// `right_label` is set the Switch renders in dual-label layout
    /// (label, track, label) instead of (track, label).
    pub left_label: Option<String>,
    /// Right-side label for dual-label mode.
    pub right_label: Option<String>,
    pub aria_label: Option<String>,
    /// Custom color for the on (checked) track state (CSS hex string).
    pub on_color: Option<String>,
    /// Custom color for the off (unchecked) track state (CSS hex string).
    pub off_color: Option<String>,
    /// Tone applied to the left (off) track side. Only takes effect when
    /// `off_color` is not set — a custom colour always wins.
    pub left_tone: SwitchTone,
    /// Tone applied to the right (on) track side. Only takes effect when
    /// `on_color` is not set.
    pub right_tone: SwitchTone,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for SwitchSpec {
    fn default() -> Self {
        Self {
            checked: None,
            default_checked: false,
            is_disabled: false,
            is_read_only: false,
            name: None,
            label: None,
            left_label: None,
            right_label: None,
            aria_label: None,
            on_color: None,
            off_color: None,
            left_tone: SwitchTone::Default,
            // Right tone defaults to Primary to match the Svelte prop default.
            right_tone: SwitchTone::Primary,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl SwitchSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn with_default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }

    pub fn with_on_color(mut self, color: impl Into<String>) -> Self {
        self.on_color = Some(color.into());
        self
    }

    pub fn with_off_color(mut self, color: impl Into<String>) -> Self {
        self.off_color = Some(color.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_left_label(mut self, left_label: impl Into<String>) -> Self {
        self.left_label = Some(left_label.into());
        self
    }

    pub fn with_right_label(mut self, right_label: impl Into<String>) -> Self {
        self.right_label = Some(right_label.into());
        self
    }

    pub fn with_left_tone(mut self, tone: SwitchTone) -> Self {
        self.left_tone = tone;
        self
    }

    pub fn with_right_tone(mut self, tone: SwitchTone) -> Self {
        self.right_tone = tone;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_read_only(mut self, is_read_only: bool) -> Self {
        self.is_read_only = is_read_only;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    /// Whether either dual-label side is populated.
    pub fn is_dual_label(&self) -> bool {
        self.left_label.is_some() || self.right_label.is_some()
    }

    pub fn current_checked(&self) -> bool {
        self.checked.unwrap_or(self.default_checked)
    }

    pub fn track_fill_token(&self) -> &'static str {
        if self.current_checked() {
            semantic::COLOR_ACCENT_BASE
        } else {
            semantic::COLOR_BACKGROUND_SURFACE
        }
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_contract() {
        let spec = SwitchSpec::default();
        assert_eq!(spec.checked, None);
        assert!(!spec.default_checked);
        assert!(!spec.is_disabled);
        assert!(!spec.is_read_only);
        assert_eq!(spec.name, None);
        assert_eq!(spec.label, None);
        assert_eq!(spec.left_label, None);
        assert_eq!(spec.right_label, None);
        assert_eq!(spec.aria_label, None);
        assert_eq!(spec.on_color, None);
        assert_eq!(spec.off_color, None);
        assert_eq!(spec.left_tone, SwitchTone::Default);
        assert_eq!(spec.right_tone, SwitchTone::Primary);
        assert_eq!(spec.size, None);
        assert_eq!(spec.size_role, SemanticControlSizeRole::Control);
        assert_eq!(spec.density, None);
        assert!(!spec.is_dual_label());
        assert!(!spec.current_checked());
        assert_eq!(spec.track_fill_token(), semantic::COLOR_BACKGROUND_SURFACE);
    }

    #[test]
    fn builders_cover_the_public_prop_surface() {
        let spec = SwitchSpec::new()
            .with_checked(true)
            .with_default_checked(false)
            .with_name("theme-toggle")
            .with_label("Dark theme")
            .with_left_label("Light")
            .with_right_label("Dark")
            .with_aria_label("Select application theme")
            .with_on_color("#22c55e")
            .with_off_color("#cbd5e1")
            .with_left_tone(SwitchTone::Warning)
            .with_right_tone(SwitchTone::Success)
            .with_disabled(true)
            .with_read_only(true)
            .with_size(ControlSize::Lg)
            .with_size_role(SemanticControlSizeRole::Prominent)
            .with_density(ControlDensity::Comfortable);

        assert_eq!(spec.checked, Some(true));
        assert!(!spec.default_checked);
        assert_eq!(spec.name.as_deref(), Some("theme-toggle"));
        assert_eq!(spec.label.as_deref(), Some("Dark theme"));
        assert_eq!(spec.left_label.as_deref(), Some("Light"));
        assert_eq!(spec.right_label.as_deref(), Some("Dark"));
        assert_eq!(spec.aria_label.as_deref(), Some("Select application theme"));
        assert_eq!(spec.on_color.as_deref(), Some("#22c55e"));
        assert_eq!(spec.off_color.as_deref(), Some("#cbd5e1"));
        assert_eq!(spec.left_tone, SwitchTone::Warning);
        assert_eq!(spec.right_tone, SwitchTone::Success);
        assert!(spec.is_disabled);
        assert!(spec.is_read_only);
        assert_eq!(spec.size, Some(ControlSize::Lg));
        assert_eq!(spec.size_role, SemanticControlSizeRole::Prominent);
        assert_eq!(spec.density, Some(ControlDensity::Comfortable));
        assert!(spec.is_dual_label());
        assert!(spec.current_checked());
        assert_eq!(spec.track_fill_token(), semantic::COLOR_ACCENT_BASE);
    }

    #[test]
    fn current_checked_resolves_default_when_controlled_is_absent() {
        let uncontrolled = SwitchSpec::new().with_default_checked(true);
        assert!(uncontrolled.current_checked());
        assert_eq!(uncontrolled.track_fill_token(), semantic::COLOR_ACCENT_BASE);

        let controlled = SwitchSpec::new()
            .with_default_checked(true)
            .with_checked(false);
        assert!(!controlled.current_checked());
        assert_eq!(controlled.track_fill_token(), semantic::COLOR_BACKGROUND_SURFACE);
    }

    #[test]
    fn dual_label_mode_derives_from_either_side() {
        let left_only = SwitchSpec::new().with_left_label("Off");
        assert!(left_only.is_dual_label());

        let right_only = SwitchSpec::new().with_right_label("On");
        assert!(right_only.is_dual_label());

        let single = SwitchSpec::new().with_label("Power");
        assert!(!single.is_dual_label());
    }

    #[test]
    fn switch_tone_color_tokens_map_to_semantics() {
        assert_eq!(SwitchTone::Default.color_token(), None);
        assert_eq!(SwitchTone::Primary.color_token(), Some(semantic::COLOR_ACCENT_BASE));
        assert_eq!(SwitchTone::Success.color_token(), Some(semantic::COLOR_STATUS_SUCCESS));
        assert_eq!(SwitchTone::Warning.color_token(), Some(semantic::COLOR_STATUS_WARNING));
        assert_eq!(SwitchTone::Danger.color_token(), Some(semantic::COLOR_STATUS_DANGER));
    }
}
