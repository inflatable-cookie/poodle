use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, PartialEq)]
pub struct DurationInputSpec {
    pub is_disabled: bool,
    /// Whether the Seconds segment is drawn. Defaults to `true`; hiding it
    /// does not drop the stored seconds value.
    pub show_seconds: bool,
    /// Maximum hours value for the hours segment. Defaults to 99.
    pub max_hours: u32,
    /// Minimum total duration in seconds. Inclusive. Zero is a valid total.
    pub min_total_seconds: u64,
    /// Maximum total duration in seconds. Inclusive. None means no maximum.
    pub max_total_seconds: Option<u64>,
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// Omitted (`None`) inherits from the presentation context; an explicit
    /// value always wins.
    pub density: Option<ControlDensity>,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
    /// Bindable hours segment. Together with `minutes` and `seconds` this is
    /// the only controlled duration value.
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Default for DurationInputSpec {
    fn default() -> Self {
        Self {
            is_disabled: false,
            show_seconds: true,
            max_hours: 99,
            min_total_seconds: 0,
            max_total_seconds: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
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

    /// Total duration in seconds. Hours is `u32`, so the product always fits
    /// in `u64`; this is the same reduction the web helper performs, without
    /// wrapping the total.
    pub fn total_seconds(&self) -> u64 {
        u64::from(self.hours) * 3600 + u64::from(self.minutes) * 60 + u64::from(self.seconds)
    }

    /// Inclusive min/max bounds are validation, not edit clamps.
    pub fn is_out_of_bounds(&self) -> bool {
        let total = self.total_seconds();
        if total < self.min_total_seconds {
            return true;
        }
        match self.max_total_seconds {
            Some(max) => total > max,
            None => false,
        }
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
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
        if self.is_out_of_bounds() {
            semantic::COLOR_STATUS_DANGER
        } else {
            semantic::COLOR_BORDER_DEFAULT
        }
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
mod segment_tests {
    use super::*;

    #[test]
    fn segments_are_the_only_value_and_reduce_to_a_total() {
        let spec = DurationInputSpec::new().with_segments(2, 30, 15);
        assert_eq!(spec.hours, 2);
        assert_eq!(spec.minutes, 30);
        assert_eq!(spec.seconds, 15);
        assert_eq!(spec.total_seconds(), 9015);
    }

    #[test]
    fn seconds_are_shown_by_default() {
        assert!(DurationInputSpec::new().show_seconds);
    }

    #[test]
    fn an_empty_duration_totals_zero_and_is_valid() {
        let spec = DurationInputSpec::new();
        assert_eq!(spec.total_seconds(), 0);
        assert!(!spec.is_out_of_bounds());
        assert_eq!(spec.border_token(), semantic::COLOR_BORDER_DEFAULT);
    }

    #[test]
    fn inclusive_bounds_are_valid_and_outside_is_invalid() {
        let at_min = DurationInputSpec::new()
            .with_segments(0, 1, 0)
            .with_min_total_seconds(60);
        assert!(!at_min.is_out_of_bounds());

        let under = DurationInputSpec::new()
            .with_segments(0, 0, 59)
            .with_min_total_seconds(60);
        assert!(under.is_out_of_bounds());
        assert_eq!(under.border_token(), semantic::COLOR_STATUS_DANGER);

        let at_max = DurationInputSpec::new()
            .with_segments(1, 0, 0)
            .with_max_total_seconds(3600);
        assert!(!at_max.is_out_of_bounds());

        let over = DurationInputSpec::new()
            .with_segments(1, 0, 1)
            .with_max_total_seconds(3600);
        assert!(over.is_out_of_bounds());
    }

    #[test]
    fn a_large_hours_value_does_not_overflow_the_total() {
        let spec = DurationInputSpec::new().with_segments(u32::MAX, 59, 59);
        assert_eq!(
            spec.total_seconds(),
            u64::from(u32::MAX) * 3600 + 59 * 60 + 59
        );
    }
}
