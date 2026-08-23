use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressSpec {
    pub value: Option<f64>,
    pub max: f64,
    pub is_indeterminate: bool,
    pub aria_label: Option<String>,
    pub value_text: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for ProgressSpec {
    fn default() -> Self {
        Self {
            value: None,
            max: 100.0,
            is_indeterminate: false,
            aria_label: None,
            value_text: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl ProgressSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_indeterminate(mut self, is_indeterminate: bool) -> Self {
        self.is_indeterminate = is_indeterminate;
        self
    }

    pub fn normalized_progress(&self) -> Option<f64> {
        if self.is_indeterminate {
            None
        } else {
            self.value.map(|value| {
                if self.max <= 0.0 {
                    0.0
                } else {
                    (value / self.max).clamp(0.0, 1.0)
                }
            })
        }
    }

    pub fn indicator_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Track background base color token.
    /// Contract §8 Root: `color-mix(in srgb, surface 96%, text-primary)` —
    /// this is the `surface` side of that mix.
    pub fn track_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// Track background mix-toward color token (the `text-primary` side of the
    /// contract §8 track `color-mix`).
    pub fn track_mix_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Ratio of `track_fill_token` retained in the track `color-mix`
    /// (contract §8: surface kept at 96%, text-primary supplies the remainder).
    pub fn track_mix_ratio(&self) -> f32 {
        0.96
    }

    /// Ratio of `indicator_fill_token` (accent) retained in the indicator
    /// gradient's leading stop, mixed with white. Contract §8 Indicator:
    /// `linear-gradient(90deg, color-mix(accent 88%, white), accent)`.
    pub fn indicator_gradient_accent_ratio(&self) -> f32 {
        0.88
    }

    /// Track `min-height` in rem for an already-resolved effective size.
    /// Contract §8 Size Variants: xs/sm `0.375rem`, md `0.5rem`, lg/xl `0.75rem`.
    ///
    /// Callers pass the size after resolving `size`/`size_role` through their
    /// presentation helper (size-role resolution is target-side); this owns the
    /// size→height mapping so neither impl inlines the height ladder.
    pub fn min_height_rem(effective_size: ControlSize) -> f32 {
        match effective_size {
            ControlSize::Xs | ControlSize::Sm => 0.375,
            ControlSize::Md => 0.5,
            ControlSize::Lg | ControlSize::Xl => 0.75,
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
