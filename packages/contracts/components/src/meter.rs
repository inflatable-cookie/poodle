use crate::types::{ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// Meter geometry. `Linear` is the parent-width bar; `Ring` is the intrinsically
/// sized circular indicator (contract §2, §8).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MeterShape {
    #[default]
    Linear,
    Ring,
}

/// Base fill tone. `high` escalates any tone to `Warning` (contract §4).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MeterTone {
    #[default]
    Success,
    Accent,
    Warning,
    Danger,
    Neutral,
}

/// Where the value sits relative to the `low` / `high` hints. `High` wins.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterLevel {
    Low,
    Normal,
    High,
}

impl MeterLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            MeterLevel::Low => "low",
            MeterLevel::Normal => "normal",
            MeterLevel::High => "high",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeterSpec {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub optimum: Option<f64>,
    pub aria_label: Option<String>,
    pub shape: MeterShape,
    pub tone: MeterTone,
    pub show_value: bool,
    /// Explicit readout text. When `None` the computed percentage is used.
    pub value_text: Option<String>,
    /// Explicit track-thickness size override. When `None`, resolves from
    /// `size_role` against the inherited presentation scale.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
}

impl Default for MeterSpec {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            // Contract §3: max default is 100 (matches Svelte).
            max: 100.0,
            low: None,
            high: None,
            optimum: None,
            aria_label: None,
            shape: MeterShape::Linear,
            tone: MeterTone::Success,
            show_value: false,
            value_text: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
        }
    }
}

impl MeterSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn with_min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn with_low(mut self, low: f64) -> Self {
        self.low = Some(low);
        self
    }

    pub fn with_high(mut self, high: f64) -> Self {
        self.high = Some(high);
        self
    }

    pub fn with_optimum(mut self, optimum: f64) -> Self {
        self.optimum = Some(optimum);
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_shape(mut self, shape: MeterShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_tone(mut self, tone: MeterTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    pub fn with_value_text(mut self, value_text: impl Into<String>) -> Self {
        self.value_text = Some(value_text.into());
        self
    }

    pub fn normalized_progress(&self) -> f64 {
        let range = self.max - self.min;
        if range <= 0.0 {
            0.0
        } else {
            ((self.value - self.min) / range).clamp(0.0, 1.0)
        }
    }

    /// Clamped value inside `[min, safe_max]` (contract §3 `safeValue`).
    pub fn safe_value(&self) -> f64 {
        let max = if self.max <= self.min {
            self.min + 1.0
        } else {
            self.max
        };
        self.value.clamp(self.min, max)
    }

    /// Contract §3 `level`: `high` wins over `low`.
    pub fn level(&self) -> MeterLevel {
        let value = self.safe_value();
        if self.high.is_some_and(|high| value >= high) {
            MeterLevel::High
        } else if self.low.is_some_and(|low| value <= low) {
            MeterLevel::Low
        } else {
            MeterLevel::Normal
        }
    }

    /// Contract §8: fill base is `--poodle-color-status-success` (gradient
    /// endpoint + base) unless `tone` says otherwise, and `data-level="high"`
    /// escalates any tone to warning. Previously returned accent-base
    /// unconditionally — that was a bug inherited by any target trusting the spec.
    pub fn fill_token(&self) -> &'static str {
        if self.level() == MeterLevel::High {
            return semantic::COLOR_STATUS_WARNING;
        }
        match self.tone {
            MeterTone::Success => semantic::COLOR_STATUS_SUCCESS,
            MeterTone::Accent => semantic::COLOR_ACCENT_BASE,
            MeterTone::Warning => semantic::COLOR_STATUS_WARNING,
            MeterTone::Danger => semantic::COLOR_STATUS_DANGER,
            MeterTone::Neutral => semantic::COLOR_TEXT_SECONDARY,
        }
    }

    /// Readout text: explicit `value_text` wins, else the rounded percentage.
    pub fn value_display_text(&self) -> String {
        match &self.value_text {
            Some(text) => text.clone(),
            None => format!("{}%", (self.normalized_progress() * 100.0).round() as i64),
        }
    }

    /// Value-readout colour (contract §8 token reference).
    pub fn value_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Track background base. Contract §8: `color-mix(in srgb,
    /// var(--poodle-surface) 96%, var(--poodle-color-text-primary))`.
    /// This is the base side of that mix.
    pub fn track_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// The token mixed into the track base (contract §8 mix partner).
    pub fn track_mix_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Track-background mix ratio for the base token (contract §8: 96%).
    pub fn track_mix_ratio(&self) -> f32 {
        0.96
    }

    /// Contract §8 Size Variants: track thickness in rem per size.
    ///
    /// | xs 0.25 | sm 0.375 | md 0.5 | lg 0.625 | xl 0.75 |
    pub fn track_thickness_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.25,
            ControlSize::Sm => 0.375,
            ControlSize::Md => 0.5,
            ControlSize::Lg => 0.625,
            ControlSize::Xl => 0.75,
        }
    }

    /// Track height for the default (md) size. Retained for back-compat;
    /// prefer `track_thickness_rem(size)` once a size is resolved.
    pub fn track_height_rem(&self) -> f32 {
        self.track_thickness_rem(ControlSize::Md)
    }

    /// Contract §8 Size Variants: ring outer diameter in rem per size.
    ///
    /// | xs 0.875 | sm 1.125 | md 1.375 | lg 1.75 | xl 2.25 |
    pub fn ring_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.875,
            ControlSize::Sm => 1.125,
            ControlSize::Md => 1.375,
            ControlSize::Lg => 1.75,
            ControlSize::Xl => 2.25,
        }
    }

    /// Contract §8 Size Variants: ring stroke thickness in rem per size.
    ///
    /// | xs 0.125 | sm 0.1875 | md 0.1875 | lg 0.25 | xl 0.3125 |
    pub fn ring_thickness_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.125,
            ControlSize::Sm => 0.1875,
            ControlSize::Md => 0.1875,
            ControlSize::Lg => 0.25,
            ControlSize::Xl => 0.3125,
        }
    }

    /// Ring track mix ratio for the base token (contract §8 ring shape: 88%).
    pub fn ring_track_mix_ratio(&self) -> f32 {
        0.88
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_prefers_high_over_low() {
        let spec = MeterSpec::new()
            .with_value(90.0)
            .with_low(95.0)
            .with_high(80.0);
        // Both thresholds match; high wins (contract §3).
        assert_eq!(spec.level(), MeterLevel::High);
        assert_eq!(spec.fill_token(), semantic::COLOR_STATUS_WARNING);
    }

    #[test]
    fn level_normal_without_thresholds() {
        let spec = MeterSpec::new().with_value(50.0);
        assert_eq!(spec.level(), MeterLevel::Normal);
        assert_eq!(spec.fill_token(), semantic::COLOR_STATUS_SUCCESS);
    }

    #[test]
    fn low_keeps_the_base_tone() {
        let spec = MeterSpec::new()
            .with_value(10.0)
            .with_low(25.0)
            .with_tone(MeterTone::Accent);
        assert_eq!(spec.level(), MeterLevel::Low);
        assert_eq!(spec.fill_token(), semantic::COLOR_ACCENT_BASE);
    }

    #[test]
    fn value_text_overrides_computed_percentage() {
        let spec = MeterSpec::new().with_value(35.0);
        assert_eq!(spec.value_display_text(), "35%");
        assert_eq!(
            MeterSpec::new()
                .with_value(35.0)
                .with_value_text("35k / 100k")
                .value_display_text(),
            "35k / 100k"
        );
    }

    #[test]
    fn ring_ladder_steps_monotonically() {
        let spec = MeterSpec::new();
        let sizes = [
            ControlSize::Xs,
            ControlSize::Sm,
            ControlSize::Md,
            ControlSize::Lg,
            ControlSize::Xl,
        ];
        for pair in sizes.windows(2) {
            assert!(spec.ring_size_rem(pair[0]) < spec.ring_size_rem(pair[1]));
            assert!(spec.ring_thickness_rem(pair[0]) <= spec.ring_thickness_rem(pair[1]));
        }
    }
}
