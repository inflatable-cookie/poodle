use poodle_tokens::semantic;

use crate::types::{ControlDensity, FormActionAlign};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormActionsSpec {
    pub align: FormActionAlign,
    pub show_top_separation: bool,
    pub show_top_border: bool,
    /// Spacing density. Controls inline gap, top separation, and border
    /// gap per contract §8 (Density Variants + Divider Offset Variants).
    pub density: ControlDensity,
}

impl Default for FormActionsSpec {
    fn default() -> Self {
        Self {
            align: FormActionAlign::End,
            show_top_separation: true,
            show_top_border: false,
            density: ControlDensity::Default,
        }
    }
}

impl FormActionsSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_align(mut self, align: FormActionAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_top_separation(mut self, show_top_separation: bool) -> Self {
        self.show_top_separation = show_top_separation;
        self
    }

    pub fn with_top_border(mut self, show_top_border: bool) -> Self {
        self.show_top_border = show_top_border;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn action_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    pub fn stack_separation_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    /// Inline gap between actions, in rem, when density overrides the
    /// token default. `None` means use [`action_gap_token`]. Contract §8
    /// Density Variants: compact `0.5rem`, comfortable `0.875rem`;
    /// default inherits `--poodle-space-inline-md`.
    pub fn gap_rem(&self) -> Option<f32> {
        match self.density {
            ControlDensity::Compact => Some(0.5),
            ControlDensity::Default => None,
            ControlDensity::Comfortable => Some(0.875),
        }
    }

    /// Top separation (padding-top), in rem, when density overrides the
    /// token default. `None` means use [`stack_separation_token`].
    /// Contract §8: compact `0.375rem`, comfortable `0.75rem`; default
    /// inherits `--poodle-space-stack-sm`.
    pub fn top_separation_rem(&self) -> Option<f32> {
        match self.density {
            ControlDensity::Compact => Some(0.375),
            ControlDensity::Default => None,
            ControlDensity::Comfortable => Some(0.75),
        }
    }

    /// Top margin applied when `show_top_border` is true, in rem.
    /// Contract §8 Divider Offset Variants: compact `0.25rem`, default
    /// `0.5rem`, comfortable `0.625rem` (raw literals, matching Svelte
    /// `--poodle-form-actions-border-gap`).
    pub fn border_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.625,
        }
    }

    pub fn shows_top_separation(&self) -> bool {
        self.show_top_separation
    }

    pub fn shows_top_border(&self) -> bool {
        self.show_top_border
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn wraps_on_narrow_widths(&self) -> bool {
        true
    }
}
