use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState};

#[derive(Clone, Debug, PartialEq)]
pub struct DurationInputSpec {
    pub value: Option<String>,
    pub is_disabled: bool,
    pub validation_state: ValidationState,
    pub show_seconds: bool,
    /// Maximum hours value for the hours segment. Defaults to 99.
    pub max_hours: u32,
    /// Minimum total duration in seconds. Zero means no minimum.
    pub min_total_seconds: u64,
    /// Maximum total duration in seconds. None means no maximum.
    pub max_total_seconds: Option<u64>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
    /// The three segments, which are what the host actually binds. `value` is
    /// the formatted string; these are the numbers behind it, matching the
    /// Svelte component's bindable `hours` / `minutes` / `seconds`.
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Default for DurationInputSpec {
    fn default() -> Self {
        Self {
            value: None,
            is_disabled: false,
            validation_state: ValidationState::None,
            show_seconds: false,
            max_hours: 99,
            min_total_seconds: 0,
            max_total_seconds: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            aria_label: None,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

impl DurationInputSpec {
    /// Set all three segments at once — the host binds them together.
    pub fn with_segments(mut self, hours: u32, minutes: u32, seconds: u32) -> Self {
        self.hours = hours;
        self.minutes = minutes;
        self.seconds = seconds;
        self
    }

    /// Total duration in seconds, the same reduction Svelte's
    /// `durationTotalSeconds` performs.
    pub fn total_seconds(&self) -> u64 {
        (self.hours as u64) * 3600 + (self.minutes as u64) * 60 + (self.seconds as u64)
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    pub fn with_show_seconds(mut self, show_seconds: bool) -> Self {
        self.show_seconds = show_seconds;
        self
    }

    pub fn with_max_hours(mut self, max_hours: u32) -> Self {
        self.max_hours = max_hours;
        self
    }

    pub fn with_min_total_seconds(mut self, min: u64) -> Self {
        self.min_total_seconds = min;
        self
    }

    pub fn with_max_total_seconds(mut self, max: u64) -> Self {
        self.max_total_seconds = Some(max);
        self
    }

    pub fn border_token(&self) -> &'static str {
        self.validation_state.border_token()
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn text_secondary_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn body_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    pub fn body_line_height_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_LINE_HEIGHT
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

#[cfg(test)]
mod segment_tests {
    use super::*;

    /// The spec stored only the formatted `value`, so the three numbers a host
    /// actually binds were unreachable natively. `seconds` looked covered to
    /// the drift gate because `show_seconds` exists — a false positive the
    /// gate's `show_` normalisation used to hide.
    #[test]
    fn segments_reduce_to_a_total() {
        let spec = DurationInputSpec::new().with_segments(2, 30, 15);
        assert_eq!(spec.hours, 2);
        assert_eq!(spec.minutes, 30);
        assert_eq!(spec.seconds, 15);
        assert_eq!(spec.total_seconds(), 9015);
    }

    #[test]
    fn an_empty_duration_totals_zero() {
        assert_eq!(DurationInputSpec::new().total_seconds(), 0);
    }
}
